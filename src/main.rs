use bea::{
    get_geofips, getdata, trace_init, EnvError, GeoFipsTask, GeoFipsTasks, IoError, Method,
    UrlParseError,
};
use bea::{BeaErr, Config, User};
use clap::Parser;
use indicatif::ProgressBar;
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
    trace_init();
    dotenvy::dotenv().ok();
    trace!("Environmental variables loaded.");

    let url = EnvError::from_env("BEA_URL")?;
    let url = UrlParseError::into_url(&url)?;
    let key = EnvError::from_env("API_KEY")?;
    let checklist = EnvError::from_env("CHECKLIST")?;
    let checklist_update = EnvError::from_env("CHECKLIST_UPDATE")?;
    let bea_data = EnvError::from_env("BEA_CAINC5N")?;
    // let csv = EnvError::from_env("BEA_CAINC5N_CSV")?;
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
            let encode = tasks.serialize()?;
            info!("Encoded to binary.");
            let mut path = std::path::PathBuf::new();
            path.push(checklist.clone());
            match std::fs::write(&path, &encode) {
                Ok(()) => {}
                Err(source) => {
                    let error = IoError::new(path, source, line!(), file!().to_string());
                    return Err(error.into());
                }
            }
            info!("Written to path {}", checklist);
        }
        "checklist_report" => {
            info!("Checklist report.");
            let mut path = std::path::PathBuf::new();
            path.push(checklist.clone());
            let check = match std::fs::read(&path) {
                Ok(f) => f,
                Err(source) => {
                    let error = IoError::new(path, source, line!(), file!().to_string());
                    return Err(error.into());
                }
            };
            let check = GeoFipsTasks::deserialize(&check)?;
            check.report();
        }
        "download" => {
            info!("Download data.");
            config.with_linecode("ALL").with_year("ALL");
            let mut path = std::path::PathBuf::new();
            path.push(checklist.clone());
            let check = match std::fs::read(&path) {
                Ok(f) => f,
                Err(source) => {
                    let error = IoError::new(path, source, line!(), file!().to_string());
                    return Err(error.into());
                }
            };
            let mut check = GeoFipsTasks::deserialize(&check)?;
            if std::path::PathBuf::from(checklist_update.clone()).exists() {
                info!("Checklist found.");
                let path = std::path::PathBuf::from(checklist_update.clone());
                let done = match std::fs::read(&path) {
                    Ok(f) => f,
                    Err(source) => {
                        let error = IoError::new(path, source, line!(), file!().to_string());
                        return Err(error.into());
                    }
                };
                let done = GeoFipsTasks::deserialize(&done)?;
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
                let path = std::path::PathBuf::from(&checklist_update);
                match std::fs::remove_file(&path) {
                    Ok(()) => {}
                    Err(source) => {
                        let error = IoError::new(path, source, line!(), file!().to_string());
                        return Err(error.into());
                    }
                }
            } else {
                info!("Checklist update not found.");
            }
            let mut data = Vec::new();
            if std::path::PathBuf::from(bea_data.clone()).exists() {
                info!("Existing BEA data found on disk.");
                // let path = std::path::PathBuf::from(bea_data.clone());
                // let file = IoError::read(path)?;
                // let mut file: Vec<getdata::Datum> = bincode::deserialize(&file)?;
                // info!("BEA data loaded from disk.");
                // data.append(&mut file);
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
                            // let path = std::path::PathBuf::from(&checklist_update);
                            // let encode: Vec<u8> = bincode::serialize(&done)?;
                            // IoError::write(&encode, path)?;
                            // let encode: Vec<u8> = bincode::serialize(&data)?;
                            // let path = std::path::PathBuf::from(&bea_data);
                            // IoError::write(&encode, path)?;
                            info!("GeoFips {} failed to download.", task.key());
                        }
                    }
                }
            }
            // let encode: Vec<u8> = bincode::serialize(&data)?;
            // let path = std::path::PathBuf::from(&bea_data);
            // IoError::write(&encode, path)?;
            // let mut data = getdata::Data::new(&data);
            // data.to_csv(csv.into())?;
            // info!("Data download complete.");
            // if let Ok(check) = std::fs::exists(&checklist_update) {
            //     if check {
            //         let path = std::path::PathBuf::from(&checklist_update);
            //         IoError::remove_file(path)?;
            //         info!("Completed checklist removed.");
            //     }
            // }
        }
        // "save" => {
        //     let path = EnvError::from_env("BEA_CAINC5N_CSV")?;
        //     let records = BeaData::from_csv(path)?;
        //     tracing::info!("Records: {}", records.records_ref().len());
        //     info!("Serializing to binary.");
        //     let encode = bincode::serialize(&records)?;
        //     if let Some(source) = cli.source {
        //         info!("Writing to file.");
        //         IoError::write(&encode, source)?;
        //     }
        // }
        // "inspect" => {
        //     let path = std::path::PathBuf::from(&bea_data);
        //     let data = IoError::read(path)?;
        //     let decode: BeaData = bincode::deserialize(&data)?;
        //     info!("Records: {}", decode.records().len());
        // }
        _ => info!("Command not recognized."),
    };
    Ok(())
}
