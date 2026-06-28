set windows-shell := ["powershell.exe"]
export RUST_BACKTRACE := "1"

# Displays the list of available commands
@just:
    just --list

# Installs the tools pinned in mise.toml (rust, wasm-bindgen, wasm-opt, trunk)
init:
    mise install

# Builds the demo worker to wasm and generates web bindings into runtime/
worker:
    cargo build --release -p worker --target wasm32-unknown-unknown
    wasm-bindgen --target web --out-dir examples/nightshade_demo/runtime --out-name engine target/wasm32-unknown-unknown/release/worker.wasm
    wasm-opt -O3 --enable-simd examples/nightshade_demo/runtime/engine_bg.wasm -o examples/nightshade_demo/runtime/engine_bg.wasm

# Builds the worker and the Leptos bundle for the nightshade demo
build: worker
    trunk build

# Builds the bundle and opens the nightshade demo in a native webview window
run: build
    cargo run -p nightshade_demo_desktop

# Builds the worker, then serves the demo in the browser at http://127.0.0.1:8080
dev: worker
    trunk serve

# Produces a production web bundle in examples/nightshade_demo/dist
dist: worker
    trunk build --release

# Builds the standalone desktop executable with the web bundle embedded
build-desktop: dist
    cargo build --release -p nightshade_demo_desktop

# Type-checks the library across all features and the demo workspace
check:
    cargo build --manifest-path crates/musaic/Cargo.toml --target wasm32-unknown-unknown --features full
    cargo check -p protocol -p worker -p nightshade_demo --target wasm32-unknown-unknown
    cargo check -p musaic-shell -p nightshade_demo_desktop
    cargo fmt --all -- --check

# Lints the library and the demo workspace, denying warnings
lint:
    cargo clippy --manifest-path crates/musaic/Cargo.toml --target wasm32-unknown-unknown --features full -- -D warnings
    cargo clippy -p protocol -p worker -p nightshade_demo --target wasm32-unknown-unknown -- -D warnings
    cargo clippy -p musaic-shell -p nightshade_demo_desktop -- -D warnings

# Formats the code
format:
    cargo fmt --all

# Removes build artifacts (Windows)
[windows]
clean:
    cargo clean
    Remove-Item -Recurse -Force examples/nightshade_demo/dist, examples/nightshade_demo/runtime/engine.js, examples/nightshade_demo/runtime/engine_bg.wasm, examples/nightshade_demo/runtime/engine.d.ts, examples/nightshade_demo/runtime/engine_bg.wasm.d.ts -ErrorAction SilentlyContinue

# Removes build artifacts (Unix)
[unix]
clean:
    cargo clean
    rm -rf examples/nightshade_demo/dist examples/nightshade_demo/runtime/engine.js examples/nightshade_demo/runtime/engine_bg.wasm examples/nightshade_demo/runtime/engine.d.ts examples/nightshade_demo/runtime/engine_bg.wasm.d.ts
