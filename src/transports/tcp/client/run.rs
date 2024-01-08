use std::env;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter, Stdin, Stdout};
use tokio::runtime::Builder;
use tokio_util::sync::CancellationToken;

use super::client::{CliClient, CliClientError};
use super::configs::ClientCliConfig;
use super::configs_parser::{Options, parse_args, ParseError};

pub fn print_help() {
    println!(r#"Kevy Phnx Database Client Cli

Usage: kevy-cli --host <host> --port <port> --username <username> --password <password>

Options:
    [Required] -H, --host <host>            Kevy's server host
    [Optional] -P, --port <port>            Kevy's server port. Default is 3458
    [Optional] -u, --username <username>    Authenticated user for the server
    [Optional] -p, --password <password>    Password for the user (if required)

Extras:
    -V, --version                           Print version
    -h, --help                              print help
    "#);
}

pub fn print_version() {
    println!("coding");
}

fn handle_help_options(option: &Options) -> bool {
    match option {
        Options::Version => print_version(),
        Options::Help => print_help(),
        _ => return false,
    };

    return true;
}

async fn read_and_send_command(
    client: &mut CliClient, stdin: &mut BufReader<Stdin>, stdout: &mut BufWriter<Stdout>,
) -> Result<bool, CliClientError> {
    let mut command = String::new();
    let _ = stdout.write_all(client.get_cli_prompt().as_bytes()).await.expect("Unable to write prompt to stdout");
    let _ = stdout.flush().await;

    stdin.read_line(&mut command).await.expect("Unable to read command from stdin");
    client.send_command(command.as_bytes()).await
}

async fn wait_and_get_response(
    client: &mut CliClient, stdout: &mut BufWriter<Stdout>,
) -> Result<(), CliClientError> {
    let result = client.read_response().await?;
    let _ = stdout.write_all(result.as_bytes()).await.expect("Unable to write response to stdout");
    let _ = stdout.flush().await;

    Ok(())
}

async fn listen(
    client: &mut CliClient, stdin: &mut BufReader<Stdin>, stdout: &mut BufWriter<Stdout>,
) {
    loop {
        let sent = read_and_send_command(client, stdin, stdout).await;
        if let Err(err) = sent {
            eprintln!("{err}");
            break;
        }

        let sent = sent.unwrap();
        if !sent {
            continue;
        }

        let err = wait_and_get_response(client, stdout).await;
        if let Err(err) = err {
            eprintln!("{err}");
            break;
        }
    }
}

pub async fn run_async(shd_token: &CancellationToken) {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    if args_len == 1 {
        eprintln!("{}", ParseError::MissingOptions(String::from("head")));
        print_help();
        return;
    }

    let options = parse_args(&args[1..]);
    if let Err(err) = options {
        eprintln!("{err}");
        return;
    }

    let options = options.unwrap();
    if options.len() == 1 && handle_help_options(options.first().unwrap()) {
        return;
    }

    let configs = ClientCliConfig::from_options(options);
    if let Err(msg) = configs {
        eprintln!("{msg}");
        return;
    }

    let mut client = CliClient::new(configs.unwrap());
    let connected = client.connect().await;
    if let Err(err) = connected {
        println!("Unable to connect to the server. {err}");
        return;
    }
    println!("Connected to {}", client.configs.get_addr());
    let mut stdin = BufReader::new(io::stdin());
    let mut stdout = BufWriter::new(io::stdout());

    tokio::select! {
        _ = shd_token.cancelled() => {
            println!("Shutting down...");
            return;
        }
        _ = listen(&mut client, &mut stdin, &mut stdout) => {}
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
