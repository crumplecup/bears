use bears::{trace_init, BeaErr};
use clap::Parser;
// use indicatif::ProgressBar;
use tracing::{info, trace};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'c', long, help = "Command to execute.")]
    command: String,
    #[arg(short = 's', long, help = "Source of file.")]
    source: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), BeaErr> {
    trace_init()?;
    dotenvy::dotenv().ok();
    trace!("Environmental variables loaded.");

    // let url = EnvError::from_env("BEA_URL")?;
    // let url = UrlParseError::into_url(&url)?;
    // let key = EnvError::from_env("API_KEY")?;
    //
    // let style = indicatif::ProgressStyle::with_template(
    //     "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Downloading data.'}",
    // )
    // .unwrap();
    let cli = Cli::parse();
    match &cli.command as &str {
        "download" => tracing::info!("Download not implemented."),
        _ => info!("Command not recognized."),
    };
    Ok(())
}
