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

pub fn use_reconnecting_socket(
    url: impl Into<String>,
    on_message: Callback<String>,
) -> SocketHandle {
    let connected = RwSignal::new(false);
    let socket = StoredValue::new_local(None::<web_sys::WebSocket>);
    let url = url.into();
    let connect_slot: ConnectSlot = Rc::new(RefCell::new(None));

    let slot_for_close = connect_slot.clone();
    let make: Rc<dyn Fn()> = Rc::new(move || {
        let Ok(ws) = web_sys::WebSocket::new(&url) else {
            return;
        };

        let on_open = Closure::<dyn Fn()>::new(move || connected.set(true));
        ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        on_open.forget();

        let on_message_closure =
            Closure::<dyn Fn(web_sys::MessageEvent)>::new(move |event: web_sys::MessageEvent| {
                if let Some(text) = event.data().as_string() {
                    on_message.run(text);
                }
            });
        ws.set_onmessage(Some(on_message_closure.as_ref().unchecked_ref()));
        on_message_closure.forget();

        let reconnect = slot_for_close.clone();
        let on_close = Closure::<dyn Fn()>::new(move || {
            connected.set(false);
            if let Some(again) = reconnect.borrow().clone() {
                set_timeout(move || again(), Duration::from_millis(RECONNECT_MS));
            }
        });
        ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
        on_close.forget();

        socket.set_value(Some(ws));
    });

    *connect_slot.borrow_mut() = Some(make.clone());
    make();

    SocketHandle { connected, socket }
}
