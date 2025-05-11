cliff version:
  git cliff --tag {{version}} -o CHANGELOG.md
  git add CHANGELOG.md
  git commit -m "CHANGELOG.md updated for version {{version}}"

tool_up:
  cargo install just cargo-audit cargo-auditable omnibor-cli
  cargo install cargo-dist cargo-release --locked

dist:
  dist build
  dist plan

prepare version: tool_up dist
  cargo release {{version}}

