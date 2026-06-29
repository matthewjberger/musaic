#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    leptos::prelude::mount_to_body(gallery::App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("the gallery is a web app and only runs in the browser.");
    eprintln!(
        "Serve it with `just run-gallery-wasm`, or open the desktop shell with `just run-gallery`."
    );
    std::process::exit(1);
}
