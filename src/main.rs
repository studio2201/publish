#[cfg(target_arch = "wasm32")]
mod frontend;
#[cfg(target_arch = "wasm32")]
mod editor;
#[cfg(target_arch = "wasm32")]
mod crdt;

#[cfg(not(target_arch = "wasm32"))]
mod backend;

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    backend::run().await;
}

#[cfg(target_arch = "wasm32")]
fn main() {
    frontend::run();
}
