use clap::Parser;
use fast_socks5::client::{Config, Socks5Stream};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

static AFTER_HELP: &'static str = color_print::cstr!(
r#"<bold><underline>Examples:</underline></bold>
    <bold>sucks5 -s example.com:1080 private.example.com 22</bold>
        Connect to `private.example.com:22` via a SOCKS proxy server at `example.com:1080`.
    <bold>sucks5 -s 127.0.0.1:1080 -u user -p pass 10.0.0.1 22</bold>
        Connect to `10.0.0.1:22` via a SOCKS proxy server at `127.0.0.1:1080` with username `user` and password `pass`.
    <bold>ProxyCommand sucks5 -s example.com:1080 %h %p</bold>
        Use `sucks5` as a proxy command in SSH.
"#);

#[derive(Parser)]
#[command(version, about, author, long_about = None)]
#[command(
    help_template = "{before-help}{about-with-newline}\nAuthor: {author-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}"
)]
#[command(after_help = AFTER_HELP)]
/// A SOCKS4/SOCKS4a/SOCKS5 proxy client implementation.
struct Cli {
    /// The SOCKS server address. Syntax: `127.0.0.1:1080`.
    #[arg(short, long)]
    server: String,

    /// The username for the SOCKS server, if required. You must also provide a password.
    #[arg(short, long)]
    username: Option<String>,

    /// The password for the SOCKS server, if required. You must also provide a username.
    #[arg(short, long)]
    password: Option<String>,

    /// The timeout ms for the connection. Syntax: `1000`.
    /// Default: `5000`.
    #[arg(short, long)]
    timeout: Option<u64>,

    /// The address to connect to. Syntax: `10.0.0.1`.
    address: String,

    /// The port to connect to. Syntax: `80`.
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    // validate args
    let need_auth = args.username.is_some() && args.password.is_some();
    if args.username.is_some() ^ args.password.is_some() {
        eprintln!("You must provide both a username and a password. The provided username/password will be ignored.");
    }

    let mut config = Config::default();
    config.set_connect_timeout(args.timeout.unwrap_or(5000));
    config.set_skip_auth(false); // always use auth because many servers don't support no-auth

    let mut stream: Socks5Stream<tokio::net::TcpStream> = if need_auth {
        Socks5Stream::connect_with_password(
            args.server,
            args.address,
            args.port,
            args.username.unwrap(),
            args.password.unwrap(),
            config,
        )
        .await?
    } else {
        Socks5Stream::connect(args.server, args.address, args.port, config).await?
    };

    let mut stdio = tokio::io::join(tokio::io::stdin(), tokio::io::stdout());

    // start bidirectional copy
    let result = tokio::io::copy_bidirectional(&mut stdio, &mut stream).await;
    match result {
        Ok((sent, received)) => {
            eprintln!("Connection closed. Sent {} bytes, received {} bytes.", sent, received);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(e.into())
        }
    }
}
