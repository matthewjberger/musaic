use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "../dist"]
struct Dist;

fn main() {
    if Dist::get("index.html").is_none() {
        eprintln!("the web bundle is missing, build it first with `just run-gallery`");
        std::process::exit(1);
    }
    leptos_musaic_shell::run("musaic gallery", |path| {
        Dist::get(path).map(|file| file.data.into_owned())
    });
}
