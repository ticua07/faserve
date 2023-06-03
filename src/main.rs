use clap::Parser;
use rand::distributions::{Alphanumeric, DistString};
mod routes;

#[derive(Parser)]
struct Cli {
    #[arg(long, value_name = "HOST")]
    host: Option<String>,

    #[arg(long, value_name = "PORT")]
    port: Option<String>,

    file: String,
}

#[derive(Clone)]
pub struct State {
    file: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let cli = Cli::parse();

    let random_url: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
    let host = cli.host.unwrap_or("127.0.0.1".to_string());
    let port = cli.port.unwrap_or("8080".to_string());
    println!(
        "Serving file: {} at http://{host}:{port}/{random_url}",
        cli.file
    );

    let mut app = tide::with_state(State { file: cli.file });
    app.at(format!("/{}", random_url).as_str())
        .get(routes::file_route)
        .with(driftwood::DevLogger)
        .with(tide_compress::CompressMiddleware::new());
    app.listen(format!("{host}:{port}").as_str()).await?;
    Ok(())
}
