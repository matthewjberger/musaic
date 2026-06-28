#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    leptos::prelude::mount_to_body(nightshade_demo::App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("the page is a web app and only runs in the browser.");
    eprintln!("Serve it with `just dev`, or run the desktop shell with `just run`.");
    std::process::exit(1);
}
