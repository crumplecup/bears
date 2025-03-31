cliff version:
  git cliff --tag {{version}} -o CHANGELOG.md
  git add CHANGELOG.md
  git commit -m "CHANGELOG.md updated for version {{version}}"

tool_up:
  cargo install just
  cargo install cargo-auditable
  cargo install omnibor-cli
  cargo install cargo-dist
  cargo install cargo-release

dist:
  dist build
  dist plan

prepare version: tool_up dist
  cargo release {{version}}

