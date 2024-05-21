use yoakv::server::server;

#[cfg(not(target_os = "wasi"))]
fn main() {
    server().unwrap();
}
