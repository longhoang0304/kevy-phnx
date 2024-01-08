use kevy_phnx::transports::tcp::server;
fn main() {
    let svr = server::Server::new();
    svr.run();
}
