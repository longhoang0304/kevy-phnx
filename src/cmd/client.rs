use std::process::exit;

use tokio;
use tokio::signal;
use tokio_util::sync::CancellationToken;

use kevy_phnx::transports::tcp::client;

#[tokio::main]
async fn main() {
    let shd_token = CancellationToken::new();
    let cshd_token = shd_token.clone();

    tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(_) => {
                println!("\nClosing the client...");
                cshd_token.cancel();
            }
            Err(err) => {
                eprintln!("Unable to listen for shutdown signal: {}", err);
            }
        }
    });

    tokio::select! {
        _ = shd_token.cancelled() => {
            exit(0);
        }
        _ = client::run_async(&shd_token) => {}
    }
}
