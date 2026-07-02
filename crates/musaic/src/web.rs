use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use leptos::prelude::*;
use serde::Serialize;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|window| window.local_storage().ok().flatten())
}

pub fn download_text(filename: &str, contents: &str) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let encoded: String = js_sys::encode_uri_component(contents).into();
    let href = format!("data:text/plain;charset=utf-8,{encoded}");
    if let Ok(element) = document.create_element("a") {
        let anchor: web_sys::HtmlAnchorElement = element.unchecked_into();
        anchor.set_href(&href);
        anchor.set_download(filename);
        anchor.click();
    }
}

pub fn pick_file_text(on_pick: Callback<String>) {
    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let Ok(element) = document.create_element("input") else {
        return;
    };
    let input: web_sys::HtmlInputElement = element.unchecked_into();
    input.set_type("file");
    let retained = crate::use_retained_closures();
    let on_change = Closure::<dyn Fn(web_sys::Event)>::new(move |event: web_sys::Event| {
        let Some(target) = event.target() else {
            return;
        };
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        let Some(file) = input.files().and_then(|files| files.get(0)) else {
            return;
        };
        let Ok(reader) = web_sys::FileReader::new() else {
            return;
        };
        let reader_ref = reader.clone();
        let on_load = Closure::<dyn Fn()>::new(move || {
            if let Ok(result) = reader_ref.result()
                && let Some(text) = result.as_string()
            {
                on_pick.run(text);
            }
        });
        reader.set_onload(Some(on_load.as_ref().unchecked_ref()));
        retained.retain(on_load);
        let _ = reader.read_as_text(&file);
    });
    input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
    retained.retain(on_change);
    input.click();
}

pub fn use_persisted<T>(key: impl Into<String>, default: T) -> RwSignal<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + Clone + 'static,
{
    let key = key.into();
    let stored = local_storage()
        .and_then(|storage| storage.get_item(&key).ok().flatten())
        .and_then(|raw| serde_json::from_str::<T>(&raw).ok());
    let signal = RwSignal::new(stored.unwrap_or(default));
    Effect::new(move |_| {
        let value = signal.get();
        if let (Some(storage), Ok(raw)) = (local_storage(), serde_json::to_string(&value)) {
            let _ = storage.set_item(&key, &raw);
        }
    });
    signal
}

#[derive(Clone, Copy)]
pub struct SocketHandle {
    connected: RwSignal<bool>,
    socket: StoredValue<Option<web_sys::WebSocket>, LocalStorage>,
}

impl SocketHandle {
    pub fn connected(&self) -> Signal<bool> {
        let connected = self.connected;
        Signal::derive(move || connected.get())
    }

    pub fn send(&self, text: &str) {
        self.socket.with_value(|socket| {
            if let Some(socket) = socket {
                let _ = socket.send_with_str(text);
            }
        });
    }
}

const RECONNECT_MS: u64 = 1000;

type ConnectSlot = Rc<RefCell<Option<Rc<dyn Fn()>>>>;
type ClosureKeep = StoredValue<Vec<Box<dyn std::any::Any>>, LocalStorage>;

pub fn use_reconnecting_socket(
    url: impl Into<String>,
    on_message: Callback<String>,
) -> SocketHandle {
    let connected = RwSignal::new(false);
    let socket = StoredValue::new_local(None::<web_sys::WebSocket>);
    let keep: ClosureKeep = StoredValue::new_local(Vec::new());
    let cancelled = StoredValue::new(false);
    let url = url.into();
    let connect_slot: ConnectSlot = Rc::new(RefCell::new(None));

    let slot_for_close = connect_slot.clone();
    let make: Rc<dyn Fn()> = Rc::new(move || {
        if cancelled.get_value() {
            return;
        }
        let Ok(ws) = web_sys::WebSocket::new(&url) else {
            return;
        };

        let on_open = Closure::<dyn Fn()>::new(move || connected.set(true));
        ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));

        let on_message_closure =
            Closure::<dyn Fn(web_sys::MessageEvent)>::new(move |event: web_sys::MessageEvent| {
                if let Some(text) = event.data().as_string() {
                    on_message.run(text);
                }
            });
        ws.set_onmessage(Some(on_message_closure.as_ref().unchecked_ref()));

        let reconnect = slot_for_close.clone();
        let on_close = Closure::<dyn Fn()>::new(move || {
            connected.set(false);
            if !cancelled.get_value()
                && let Some(again) = reconnect.borrow().clone()
            {
                set_timeout(move || again(), Duration::from_millis(RECONNECT_MS));
            }
        });
        ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));

        keep.update_value(|closures| {
            closures.push(Box::new(on_open));
            closures.push(Box::new(on_message_closure));
            closures.push(Box::new(on_close));
        });
        socket.set_value(Some(ws));
    });

    *connect_slot.borrow_mut() = Some(make.clone());
    make();

    on_cleanup(move || {
        cancelled.set_value(true);
        if let Some(ws) = socket.get_value() {
            ws.set_onopen(None);
            ws.set_onmessage(None);
            ws.set_onclose(None);
            let _ = ws.close();
        }
        keep.update_value(Vec::clear);
    });

    SocketHandle { connected, socket }
}
