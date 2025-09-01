use bears_species::{BeaErr, IoError, SerdeJson};

#[tracing::instrument(skip_all)]
pub fn write_json<T: ?Sized + serde::Serialize, P: AsRef<std::path::Path>>(
    contents: &T,
    path: P,
) -> Result<(), BeaErr> {
    let path = path.as_ref();
    let file = std::fs::File::create(path)
        .map_err(|e| IoError::new(path.to_owned(), e, line!(), file!().to_string()))?;
    let writer = std::io::BufWriter::new(file);

    serde_json::to_writer_pretty(writer, contents)
        .map_err(|e| SerdeJson::new(e, line!(), file!().into()))?;
    Ok(())
}
