use tokio::runtime::Builder;
use tokio_util::sync::CancellationToken;

use super::connection_handlers;
use super::server::Server;

async fn listen_and_accept(server: &mut Server) {
    loop {
        let client = server.accept().await;

        if let Err(err) = client {
            eprintln!("Error while connecting to client - Err: {:?}", err);
            continue;
        }

        // stream, socket_addr
        let client = client.unwrap();
        connection_handlers::handle_connection(client).await;
    }
}

pub async fn run_async(shd_token: &CancellationToken) {
    let mut server = Server::new();
    if let Err(err) = server.open().await {
        eprintln!("{err}");
        return;
    }

    println!("Server started on 127.0.0.1:3458");
    tokio::select! {
        _ = shd_token.cancelled() => {
            println!("Shutting down...");
            return;
        }
        _ = listen_and_accept(&mut server) => {}
    }
}

pub fn run(shd_token: &CancellationToken) {
    let runtime = Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(run_async(shd_token));
}
