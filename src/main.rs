use bea::{get_geofips, getdata, GeoFipsTask, GeoFipsTasks, Method};
use bea::{BeaError, Config, User};
use clap::Parser;
use indicatif::ProgressBar;
use spreadsheet::prelude::BeaData;
// use tracing::subscriber::set_global_default;
use tracing::{info, trace};
// use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
// use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'c', long, help = "Command to execute.")]
    command: String,
    #[arg(short = 's', long, help = "Source of file.")]
    source: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), BeaError> {
    // LogTracer::init().expect("Failed to set logger.");
    // let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // let formatting_layer = BunyanFormattingLayer::new("bea".into(), std::io::stdout);
    // let subscriber = Registry::default()
    //     .with(env_filter)
    //     .with(JsonStorageLayer)
    //     .with(formatting_layer);
    // set_global_default(subscriber).expect("Failed to set subscriber.");
    // trace!("Subscriber initialized.");
    trace_init();
    dotenvy::dotenv().ok();
    trace!("Environmental variables loaded.");

    let url = std::env::var("BEA_URL")?;
    let url = url::Url::parse(&url)?;
    let key = std::env::var("API_KEY")?;
    let checklist = std::env::var("CHECKLIST")?;
    let checklist_update = std::env::var("CHECKLIST_UPDATE")?;
    let bea_data = std::env::var("BEA_CAINC5N")?;
    let csv = std::env::var("BEA_CAINC5N_CSV")?;
    let user = User::new(url, &key);
    let mut config = Config::new(&user, "Regional");
    config.with_table("CAINC5N");

    let style = indicatif::ProgressStyle::with_template(
        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Downloading data.'}",
    )
    .unwrap();
    let cli = Cli::parse();
    match &cli.command as &str {
        "check" => {
            info!("Fetching FIPS.");
            let fips = Method::get_dataset_list(&user).await?;
            info!("Fips obtained: {:?}", fips);
        }
        "checklist" => {
            info!("Make checklist.");
            info!("Fetching FIPS.");
            let fips = get_geofips(&config).await?;
            info!("FIPS obtained.");
            let tasks = GeoFipsTasks::from(&fips.results());
            let encode: Vec<u8> = bincode::serialize(&tasks)?;
            info!("Encoded to binary.");
            std::fs::write(checklist.clone(), encode)?;
            info!("Written to path {}", checklist);
        }
        "checklist_report" => {
            info!("Checklist report.");
            let check = std::fs::read(checklist.clone())?;
            let check: GeoFipsTasks = bincode::deserialize(&check)?;
            check.report();
        }
        "download" => {
            info!("Download data.");
            config.with_linecode("ALL").with_year("ALL");
            let check = std::fs::read(&checklist)?;
            let mut check: GeoFipsTasks = bincode::deserialize(&check)?;
            if std::path::PathBuf::from(checklist_update.clone()).exists() {
                info!("Checklist found.");
                let done = std::fs::read(&checklist_update)?;
                let done: GeoFipsTasks = bincode::deserialize(&done)?;
                for item in done.tasks() {
                    for task in check.tasks_mut() {
                        if item.key() == task.key() {
                            task.with_processed(*item.processed());
                        }
                    }
                }
                info!(
                    "Tasks processed: {}.",
                    check
                        .tasks()
                        .iter()
                        .filter(|v| *v.processed())
                        .collect::<Vec<&GeoFipsTask>>()
                        .len()
                );
                info!(
                    "Tasks remaining: {}.",
                    check
                        .tasks()
                        .iter()
                        .filter(|v| !v.processed())
                        .collect::<Vec<&GeoFipsTask>>()
                        .len()
                );
                std::fs::remove_file(checklist_update.clone())?;
            } else {
                info!("Checklist update not found.");
            }
            let mut data = Vec::new();
            if std::path::PathBuf::from(bea_data.clone()).exists() {
                info!("Existing BEA data found on disk.");
                let file = std::fs::read(bea_data.clone())?;
                let mut file: Vec<getdata::Datum> = bincode::deserialize(&file)?;
                info!("BEA data loaded from disk.");
                data.append(&mut file);
            }
            let bar = ProgressBar::new(check.tasks().len() as u64);
            bar.set_style(style);
            let mut done = Vec::new();
            for task in check.tasks_mut() {
                if !task.processed() {
                    config.with_geofips(task.key());
                    match getdata::get_data(&config).await {
                        Ok(res) => {
                            let mut results = res.results();
                            data.append(&mut results);
                            task.with_processed(true);
                            done.push(task);
                            bar.inc(1);
                        }
                        Err(_) => {
                            let encode: Vec<u8> = bincode::serialize(&done)?;
                            std::fs::write(checklist_update.clone(), encode)?;
                            let encode: Vec<u8> = bincode::serialize(&data)?;
                            std::fs::write(bea_data.clone(), encode)?;
                            info!("GeoFips {} failed to download.", task.key());
                        }
                    }
                }
            }
            let encode: Vec<u8> = bincode::serialize(&data)?;
            std::fs::write(bea_data.clone(), encode)?;
            let mut data = getdata::Data::new(&data);
            data.to_csv(csv.into())?;
            info!("Data download complete.");
            if let Ok(check) = std::fs::exists(&checklist_update) {
                if check {
                    std::fs::remove_file(checklist_update)?;
                    info!("Completed checklist removed.");
                }
            }
        }
        "save" => {
            let path = std::env::var("BEA_CAINC5N_CSV")?;
            let records = BeaData::from_csv(path)?;
            tracing::info!("Records: {}", records.records_ref().len());
            info!("Serializing to binary.");
            let encode = bincode::serialize(&records)?;
            if let Some(source) = cli.source {
                info!("Writing to file.");
                std::fs::write(source, encode)?;
            }
        }
        "inspect" => {
            let data = std::fs::read(&bea_data)?;
            let decode: BeaData = bincode::deserialize(&data)?;
            info!("Records: {}", decode.records().len());
        }
        _ => info!("Command not recognized."),
    };
    Ok(())
}

pub fn trace_init() {
    if tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "bea=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .is_ok()
    {};
    tracing::trace!("Loading Bea...");
}
