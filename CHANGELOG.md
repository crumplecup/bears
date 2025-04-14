# Changelog

All notable changes to this project will be documented in this file.

## [0.1.8] - 2025-04-14

### 🚀 Features

- Component enum added for the IIP dataset.
- InputOutputTable type added to keys module of bears_species to provide variants for keys in the InputOutput dataset.
- `AocSta` type added for AreaOrCountry keys in the IntlServSTA dataset.
- Added the `Channel` enum to represent valid keys for the Channel parameter of the IntlServSTA dataset.
- Added the `IipIndustry` enum to represent valid keys for the Industry parameter of the IntlServSTA dataset.
- Added the `NaicsSector`, `NaicsSubsector` and `NaicsCategory` enums to represent different levels of NAICS code categories.
- The NaicsSector::from_key method is now NaicsSector::from_code.
- Added the `NaicsSubcategory` type to represent Naics industry codes with a length of five digits.

### 🐛 Bug Fixes

- Explicit version numbers added to workspace members.
- NaicsSubcategory variants and method values corrected in response to unit testing.
- `NaicsCategory` variants and methods corrected in response to unit testing.

### 🚜 Refactor

- Library structure changed from crate to workspace.
- Library types moved to bears_species crate.
- Core library functionality moved to the bears_ecology crate.
- Tests moved to the bears_health crate.
- Stub cli moved to the eponymous bears crate.
- Tracker type moved from queue to tracker module.
- Iip key set moved to the key_sets module of the bears_species crate.
- ApiMetada moved to the key_sets module of the bears_species crate.
- InputOutput key set moved to the key_sets module of the bears_ecology crate.
- `IntlServSta`, `IntlServTrade` and `Regional` key sets moved to the key_sets module of the bears_species crate.
- Naics.rs and naics_codes.csv removed.
- Naics codes by classification added to the `cave` directory of `bears_health`.
- NaicsItem moved to the `keys` module of `bears_species`.
- Data module updated to use the new name for NaicsItems.

### 📚 Documentation

- Descriptions added for member crates.

### 🧪 Testing

- Unused tests for json removed.
- Tests added for `Component` and `AocSta` types validating enum variants against the BEA response.
- Verification testings for Naics types added validating variant names, description and codes against the .csv files in the cave.

### ⚙️ Miscellaneous Tasks

- Version incremented to 0.1.7.
- Patch update to the `clap` dependency.
- Unused json and validate modules removed.
- Patch update to the `tokio` dependency.
- Increment version to 0.1.8.
- Rust fmt changes.
- Naics types added to mod and lib files.
- Clippy corrected `manual implementation of ok` corrected to use the ok method.

## [0.1.6] - 2025-03-31

### 🚀 Features

- `ParameterNameMissing` error moved to error location, some metrics added to the type.
- `NipaTableName` enum added with variants for NIPA table name keys.

### 🚜 Refactor

- Location of dataset keys moved from `value` folder to `keys` and `key_sets`.
- Local errors moved from `error` module to the location of origin.
- Local errors moved from the `error` module to the `free` module.
- Local errors moved from the `error` module to the `app` module.
- Dataset key types moved to the `keys` module.
- Dataset key sets moved to the `key_sets` module.
- `lib.rs` updated to reflect name and module changes.

### 🧪 Testing

- Refactor for changes to Nipa key set.

### ⚙️ Miscellaneous Tasks

- Patch update to the `clap` dependency.
- Justfile updated for publishing workflow.
- Version incremented to 0.1.6.

## [0.1.5] - 2025-03-23

### 🚀 Features

- `Indicator` enum added with variants corresponding to valid values for the Indicator parameter.
- `AreaOrCountry` enum added with variants for valid values of the "AreaOrCountry" parameter in BEA requests.
- `ItaFrequency` and `ItaFrequencies` types added to represent valid values for the "Frequency" parameter of the ITA dataset in BEA requests.
- `SelectionSet` enum added as an alternative abstraction to the `SelectionKind` enum.
- `Frequency::all` method added to generate valid request parameters for all frequencies.  Only implemented for a subset of datasets.
- `ItaDatum` and `ItaData` types added to accomodate data responses for the ITA dataset.  Iterator implemented for the `Ita` type.  `Ita::queue` and `Ita::iter` methods added.
- `ITA` variant added to the TryFrom impl for `Results`.
- Convenience method `ParameterValueTable::parameter_fields` added to provide access to the value contained within the variant.
- Added the `Queue::load_par` method to enable parallel loading through the `rayon` library.
- The `Dataset::initial_load_par` method implements parallel queue loading for benchmarking purposes.

### 🚜 Refactor

- Internal macro impl_json_to_bea_error! added to facilitate conversion of JSON parsing errors to the umbrella internal error type.
- Convenience function `result_to_data` added to convert a `serde_json::Value` representing a BEA result, to a `serde_json::Value` representing the `Data` portion of the `Result`.  Steps a couple levels down the JSON value tree, preventing rightward drift in the caller.
- `lib.rs` updated with added types.
- Added `check_indicators` to `mod.rs` to provide access to unit tests.
- ITA variant added to the `download_summary` function.

### 📚 Documentation

- Comments added to the `check_datasets` test.
- README.md updated with ITA stats.

### 🧪 Testing

- The `check_indicators` test ensures all values in the BEA response parse to valid Indicator variants.
- Added `check_indicators` to unit tests.
- Data download tests temporarily refocued on ITA for active development.

### ⚙️ Miscellaneous Tasks

- Incremental version to 0.1.4 in Cargo.toml.
- Patch updates to `jiff` and `reqwest` dependencies.
- Noise trimmed from tracing logs.
- `mod.rs` updated to include added types.
- ITA variant for `Dataset` added to path generation handling in `App::destination` method.
- Loading benchmark added to test parallel loading with `rayon`.
- Update to convenience commands in the `justfile`.
- Changelog updated for version 0.1.5.

## [0.1.4] - 2025-03-15

### 🚀 Features

- Methods `initial_load`, `initial_load_continued`, `retry_load` and `download_with_history` added to the `Dataset` type.
- Method variants for `with_events` added to the Queue and History types to facilitate benchmarking.
- GdpDatum type added to represent return values for the `Dataset::GDPbyIndustry` variant.
- Download support added to GdpByIndustry via the GdpByIndustryIterator type.
- 'Style' type added to facilitate progress bars drawn to the console.
- Method `from_value` added to `Frequency` to provide a canonical means of interpreting BEA parameter values.
- Added the `roman_numeral_quarter` function mapping Roman Numeral values in the `Quarter` field to the `jiff::civil::Date` type.
- `GdpData` type added for the GDPbyIndustry dataset.  Companion variant `Data::GdpData` added.
- Progress bar added to `Queue` file loading.
- Progress bars added to `History` load and download methods.

### 🐛 Bug Fixes

- Note variant added to the Addendum type.
- The MneError type now recognizes multiple error codes returned in an array.
- The size limiting check no longer prevents users from requesting files larger than the 100MB limit.
- Logical bug patched in the impl for `Chunks` from `History`.  The inner vector will now include the final `Chunk`.
- Off-by-one logical error fixed for generating frequency parameters from a list.

### 🚜 Refactor

- The Error cap has been raised to 29, and the Call cap to 89, since size tracking is now enabled.
- The History::contains method has been removed in favor of calling `contains_key` directly on the inner BTreeMap.
- Streamlined error handling for JsonParseError variants.
- Various console logs lowered from Info to Trace, now that the functions of interest are more stable.
- `MneDiData` and `FixedAssetsData` added to the root namespace following the library convention.

### 📚 Documentation

- Method descriptions added to the `dataset` module.
- Descriptions added for `History` methods.
- Description added for the `roman_numeral_quarter` function.
- Descriptions added to `Frequency` methods.
- Module and function level descriptions added the `check` module.
- Progress statistics updated in the root `README.md`.

### 🧪 Testing

- Coverage added for the `History` methods `initial_load`, `initial_load_continued`, `retry_load` and `download_with_history`.
- Benchmarking added to the `with_event` family of methods for the Queue and Chunks types.
- Additional troubleshooting tests added for loading GDPbyIndustry files.
- Duplicate test removed.

### ⚙️ Miscellaneous Tasks

- Increment version to 0.1.3 in Cargo.toml.
- Changelog updated for version 0.1.3.
- Version incremented to 0.1.4 in Cargo.toml.
- Changelog action added to justfile.
- *(dependency)* Patch updates for dependencies.  Bincode dependency removed.
- Bincode removed from internal tests.
- Benchmarks updated to use the 'Style' type.
- *(dependency)* Patch update applied to `tokio`.  Minor update applied to `uuid`.  No changes required to code.
- Deletion of dead legacy code.
- Changelog updated for version 0.1.4.

## [0.1.2] - 2025-02-23

### 🐛 Bug Fixes

- Missing annotations added to Annotation type.  Missing row codes added to row_code module.

### 🚜 Refactor

- ParameterKind added to organize parameter headers.
- BTreeKeyMissing error added for cases when the expected key is not present in internal BTreeMap structures, such as `Options` for `App` types.
- Tests for data load and download in the `check` module remain a work in progress.

### 📚 Documentation

- Description added to Options type.
- Link to playground added.  Examples do not yet work, as `bears` is not a recognized library.

### ⚙️ Miscellaneous Tasks

- Cargo-audit and OmniBOR supply chain security tooling added to release build artifacts.
- Migrate to 2024 Edition.  Patch updates for dependencies.
- Patch update for dependency uuid.
- Changelog updated for version 0.1.2.
- Incremented version to 0.1.2.
- Cargo.lock updated.

## [0.1.1] - 2025-02-17

### 🚀 Features

- TryFrom<(Dataset, Mode)> implemented for History.  This is an internal convenience function used to direct the program to the correct log file based on the given context.
- Support added for deriving a Queue from a log history.
- Size tracking from History added to App and Queue methods.
- Load and download methods added to Chunks, wrapping the load and download methods from Queue.
- The path of NIPA datasets now indicate whether ShowMillions is present appending `_millions` to the file path.
- The History::summary method now includes the total size of successful downloads.
- The bea_data function has replaced use of the EnvError::from_env method, which has been removed.
- Added the impl_json_parse_error! macro to streamline construction of the JsonParseError type.

### 🐛 Bug Fixes

- The map_to_int method now strips commas from strings.

### 🚜 Refactor

- Load and download logs now divert to the `history` folder in the `BEA_DATA` directory.
- NipaIterator updated to use the SelectionKind type.
- The successes and errors methods have been reworked.  The `from_file` method has been renamed to `from_env`.
- The History::from_env method now looks for the `history.log` file in the `history` folder of the `BEA_DATA` directory.
- Clearer flow control within iterators for Nipa and NiUnderlyingDetail.
- Tracing statements added to the read_json method to aid in debugging during deserialization.
- Control flow improved for MneIterator and FixedAssetIterator.  The YearSelection type has been removed in favor of the SelectionKind enum.  The date_by_period function has replaced the quarter function for mapping time periods to the jiff::civil::Date type.
- The standard App::with_options method generated by derive_getters has been removed.  The App::add_options function has been renamed to with_options. Now only one way to set the value for options, and it ensures the query parameters have been properly updated, relieving the burden from the user.
- Pollster removed from dev-dependencies.

### 📚 Documentation

- Bears logo art added to README.
- Can't quite figure out the changelog tooling.
- Preamble and methods documentation added for the App type.

### ⚙️ Miscellaneous Tasks

- Incremented version to 0.1.1.
- Incremented version to 0.1.1.
- Tests updated to reflect API changes.
- Strum dependency updated to 0.27.0.
- Changelog synced to proper release tag.
- Jiff updated to 0.2.0.  Patch updates for other dependencies.
- Cargo dist tool added to CI.
- Changelog updated for version 0.1.1.

## [0.1.0] - 2025-02-08

### 🚀 Features

- Download queue
- Impl TryFrom<&PathBuf> for NipaData
- Async implementations of the load and download methods for Queue. feat: Iterator implementation for the MNE dataset. feat: Structured logging added to loading and downloading actions.
- Configuration file for git-cliff added. chore: Dependency updates.
- README added to the Github landing page.
- Markdown lint file added to suppress spurious warnings in doc markdown.

### 🚜 Refactor

- Remove dead code from prototype period.
- Match statements for error constructions updated to use the map_err method instead.
- The error_info! macro has been removed, leaving no macros in the public API.
- Amendment to error_info! removal.
- Trimmed unused dependencies.

### 📚 Documentation

- Bears logo art added to data folder.
- Crate metadata added to Cargo.toml.
- Category slugs adjusted in Cargo.toml.
- Keywords adjusted in Cargo.toml.

### ⚙️ Miscellaneous Tasks

- Dependency updates.

<!-- generated by git-cliff -->
