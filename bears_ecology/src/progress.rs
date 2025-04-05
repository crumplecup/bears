use bears_species::Progress;

#[derive(
    Clone, derive_more::Deref, derive_more::DerefMut, derive_more::AsRef, derive_more::AsMut,
)]
pub struct Style(std::collections::BTreeMap<String, indicatif::ProgressStyle>);

impl Style {
    pub fn try_new() -> Result<Self, Progress> {
        let mut styles = std::collections::BTreeMap::new();

        let key = "queue_loading".to_string();
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} 'Loading files in queue.'",
        )
        .map_err(|e| Progress::new(key.clone(), e, line!(), file!().to_owned()))?;
        styles.insert(key, style);

        let key = "with_queue".to_string();
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Matching event history to queue.'}",
        )
        .map_err(|e| Progress::new(key.clone(), e, line!(), file!().to_owned()))?;
        styles.insert(key, style);

        let key = "queue_download".to_string();
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} 'Downloading files in queue.'",
        )
        .map_err(|e| Progress::new(key.clone(), e, line!(), file!().to_owned()))?;
        styles.insert(key, style);
        Ok(Self(styles))
    }
}
