mod macros;
mod events;
mod server;

fn main() {
    server::serve::start_server();
}