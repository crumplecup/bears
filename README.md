# Welcome to the `bears` library

A Rust wrapper around the Bureau of Economic Analysis (BEA) REST API.

![Bears Logo](./data/bears_logo.jpg)
_Logo produced using Gemini AI._

## Project Goals

This library is similar to the [beaapi](https://github.com/us-bea/beaapi) Python package. Our aim is to provide the same convenience and accessibility to economic data for users of the Rust Programming Language.

A major goal of this project is to improve the discoverability of BEA data by leveraging the strong typing and exhaustive enum matching in the Rust language. Dates and times map to the [`jiff::civil::Date`](https://docs.rs/jiff/latest/jiff/civil/struct.Date.html) type. Dollar values will consider the unit multiplier when mapping to a currency type, so you can never confuse units in millions with thousands or single dollars.

Request parameter values with reasonable scopes map to enums, such as Dataset, ParameterName, State, and DirectionOfInvestment. Larger scopes, such as Country and Region codes, Geofips and LineCodes read from cached responses of the corresponding _GetParameterValues_ method BEA API call.

### Improving Discoverability

Practitioners of the dismal science should be able to enjoy nice things. If the parameter values submitted by the user would result in an error, or a "dataset not implemented" response, this will trigger a warning for the user. The `active_subset` method filters out invalid requests, so the user can quickly obtain the complete range of valid requests.

The `bears` library determines valid requests by iterating through the full range of possible requests, given the valid ranges of parameter values for the dataset, and recording the result of the request. By consuming structured logs, the library constructs a history of successes and failures, used to inform the `active_subset` method.

## Features

- Async load and download API
- Datasets can generate iterators of valid requests options
  - The full request queue will download the complete dataset
  - Filter request queues to target areas of interest

## Roadmap

_This project is currently under active development._ Initial goals are to provide feature parity with the Python package. In the future, we hope to provide integration with the [polars](https://pola.rs/) library to facilitate data analysis for users.

### Dataset Coverage

| Dataset                 | Download | Load | Export |
| ----------------------- | -------- | ---- | ------ |
| NIPA                    | ✅       | ✅   | ☐      |
| NIUnderlyingDetail      | ✅       | ✅   | ☐      |
| FixedAssets             | ✅       | ✅   | ☐      |
| MNE                     | ✅       | ✅   | ☐      |
| Ita                     | ✅       | ✅   | ☐      |
| Iip                     | ✅       | ✅   | ☐      |
| InputOutput             | ✅       | ✅   | ☐      |
| IntlServTrade           | ☐        | ☐    | ☐      |
| IntlServSTA             | ☐        | ☐    | ☐      |
| GDPbyIndustry           | ✅       | ✅   | ☐      |
| Regional                | ☐        | ☐    | ☐      |
| UnderlyingGDPbyIndustry | ✅       | ✅   | ☐      |
| APIDatasetMetadata      | ☐        | ☐    | ☐      |

### Dataset Summary Information

| Dataset                 | Count | Size     | Description                               |
| ----------------------- | ----- | -------- | ----------------------------------------- |
| NIPA                    | 626   | 1.1 GB   | Year and Frequency set to "ALL"           |
| NIUnderlyingDetail      | 60    | 527.7 MB | Year set to "ALL"                         |
| FixedAssets             | 109   | 148.1 MB | Year set to "ALL"                         |
| MNE                     | 10018 | 24.2 GB  | Year and Frequency set to "ALL"           |
| Ita                     | 94    | 71.4 MB  | Year, Frequency & Indicator set to "ALL"  |
| Iip                     | 399   | 11.4 MB  | Year, Frequency & Component set to "ALL"  |
| InputOutput             | 10    | 141.7 MB | Year set to "ALL"                         |
| IntlServTrade           | ☐     | ☐        | ☐                                         |
| IntlServSTA             | ☐     | ☐        | ☐                                         |
| GDPbyIndustry           | 39    | 131 MB   | Year and Frequency set to "ALL"           |
| Regional                | ☐     | ☐        | ☐                                         |
| UnderlyingGDPbyIndustry | 21    | 16.5 MB  | Year, Frequency and Industry set to "ALL" |
| APIDatasetMetadata      | ☐     | ☐        | ☐                                         |

## Usage

_Requires a BEA issued API key._

The `bears` library reads the user API key from the `API_KEY` environmental variable set in the project `.env` file:

```{bash}
API_KEY = "your_BEA_issued_key"
BEA_DATA = "path_to_download_dir"
```

You can also specify the directory to store downloaded data in the `BEA_DATA` variable.

For a description of how to use the current testing process, see the documentation for the [`check`](https://docs.rs/bears/latest/bears/check/index.html) module.

_The current status of the library is immature, or not yet ready for use. Feel free to experiment and offer feedback, we would appreciate hearing from you._
