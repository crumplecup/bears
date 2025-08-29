use bears_ecology::{bea_data, initial_load, trace_init};
use bears_species::{BeaErr, Data, Dataset, IoError, SerdeJson};

/// Attempts to load all files in the download [`History`], without respect to the load `History`.
/// Loads InputOutput files, converts them to row and column codes.
/// Serializes the results to `InputOutput_RowCode.json` and `InputOutput_ColumnCode.json` in the
/// `BEA_DATA` directory.
#[tracing::instrument(skip_all)]
pub async fn io_codes() -> Result<(), BeaErr> {
    trace_init()?;
    let dataset = Dataset::InputOutput;
    let iot = initial_load(dataset, None).await?;
    tracing::info!("{} datasets loaded.", iot.len());
    let mut row_codes = std::collections::BTreeMap::new();
    let mut column_codes = std::collections::BTreeMap::new();
    iot.iter()
        .map(|v| {
            if let Data::InputOutput(data) = v {
                row_codes.append(&mut data.row_codes());
                column_codes.append(&mut data.column_codes());
            }
        })
        .for_each(drop);

    let path = bea_data()?;
    let row_path = path.join("InputOutput_RowCode.json");
    let column_path = path.join("InputOutput_ColumnCode.json");

    let row_file = std::fs::File::create(row_path.clone())
        .map_err(|e| IoError::new(row_path.clone(), e, line!(), file!().to_string()))?;
    let row_writer = std::io::BufWriter::new(row_file);
    let column_file = std::fs::File::create(column_path.clone())
        .map_err(|e| IoError::new(column_path.clone(), e, line!(), file!().to_string()))?;
    let column_writer = std::io::BufWriter::new(column_file);

    serde_json::to_writer_pretty(row_writer, &row_codes)
        .map_err(|e| SerdeJson::new(e, line!(), file!().into()))?;
    serde_json::to_writer_pretty(column_writer, &column_codes)
        .map_err(|e| SerdeJson::new(e, line!(), file!().into()))?;

    Ok(())
}
