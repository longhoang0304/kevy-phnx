use kevy_phnx::transports::tcp::client;
fn main() {
    let mut clt = client::Client::new();
    clt.run();
}
