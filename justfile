# Produces a changelog using git-cliff and commits it to the active branch.
cliff version:
  git cliff --tag {{version}} -o CHANGELOG.md
  git add CHANGELOG.md
  git commit -m "CHANGELOG.md updated for version {{version}}"

# Installs the latests versions of package-specific tooling from cargo.
tool_up:
  cargo install just git-cliff cargo-audit cargo-auditable omnibor-cli
  cargo install cargo-dist cargo-release --locked

# Plan and build source code and artifacts.
dist:
  dist build
  dist plan

# Runs cargo release, not for use on workspaces.
prepare version: tool_up dist
  cargo release {{version}}

# Load a particular dataset.
load dataset:
  RUST_LOG=info cargo run --release -- -c load -d {{dataset}}

# Dataset loading test suite.
load_all:
  RUST_LOG=info cargo run --release -- -c load -d FixedAssets
  RUST_LOG=info cargo run --release -- -c load -d GdpByIndustry
  # RUST_LOG=info cargo run --release -- -c load -d Mne
  RUST_LOG=info cargo run --release -- -c load -d Nipa
  RUST_LOG=info cargo run --release -- -c load -d NIUnderlyingDetail
  RUST_LOG=info cargo run --release -- -c load -d Iip
  RUST_LOG=info cargo run --release -- -c load -d InputOutput
  RUST_LOG=info cargo run --release -- -c load -d Ita
  RUST_LOG=info cargo run --release -- -c load -d UnderlyingGdpByIndustry
