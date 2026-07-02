#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    leptos::prelude::mount_to_body(landing::App);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eprintln!("the landing page is a web app and only runs in the browser.");
    eprintln!("Serve it with `just run-landing`.");
    std::process::exit(1);
}
