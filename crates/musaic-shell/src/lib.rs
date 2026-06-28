use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use wry::{WebView, WebViewBuilder};

fn content_type(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or_default() {
        "html" => "text/html; charset=utf-8",
        "js" => "application/javascript",
        "wasm" => "application/wasm",
        "css" => "text/css",
        "png" => "image/png",
        "svg" => "image/svg+xml",
        "json" => "application/json",
        _ => "application/octet-stream",
    }
}

fn serve(get: impl Fn(&str) -> Option<Vec<u8>> + Send + 'static) -> u16 {
    let server = tiny_http::Server::http("127.0.0.1:0").expect("failed to bind localhost");
    let port = server
        .server_addr()
        .to_ip()
        .expect("expected an ip address")
        .port();
    std::thread::spawn(move || {
        for request in server.incoming_requests() {
            let path = request.url().split('?').next().unwrap_or("/");
            let path = path.trim_start_matches('/');
            let path = if path.is_empty() { "index.html" } else { path };
            match get(path) {
                Some(data) => {
                    let header = tiny_http::Header::from_bytes(
                        &b"Content-Type"[..],
                        content_type(path).as_bytes(),
                    )
                    .expect("static header is valid");
                    let response = tiny_http::Response::from_data(data).with_header(header);
                    let _ = request.respond(response);
                }
                None => {
                    let _ = request.respond(tiny_http::Response::empty(404));
                }
            }
        }
    });
    port
}

struct Shell {
    title: String,
    port: u16,
    window: Option<Window>,
    webview: Option<WebView>,
}

impl ApplicationHandler for Shell {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attributes = Window::default_attributes()
            .with_title(self.title.clone())
            .with_maximized(true);
        let window = event_loop
            .create_window(attributes)
            .expect("failed to create window");

        let builder = WebViewBuilder::new()
            .with_url(format!("http://127.0.0.1:{}/", self.port))
            .with_navigation_handler(|url| {
                url.starts_with("http://127.0.0.1") || url.starts_with("https://127.0.0.1")
            });
        #[cfg(target_os = "windows")]
        let builder = {
            use wry::WebViewBuilderExtWindows;
            builder.with_additional_browser_args("--enable-features=WebGPU")
        };
        let webview = builder.build(&window).expect("failed to create webview");

        self.window = Some(window);
        self.webview = Some(webview);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}

pub fn run(title: impl Into<String>, get: impl Fn(&str) -> Option<Vec<u8>> + Send + 'static) {
    let port = serve(get);
    let event_loop = EventLoop::new().expect("failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut shell = Shell {
        title: title.into(),
        port,
        window: None,
        webview: None,
    };
    event_loop.run_app(&mut shell).expect("event loop failed");
}
