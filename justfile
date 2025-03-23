cliff version:
  git cliff --tag {{version}} -o CHANGELOG.md

tool_up:
  cargo install just
  cargo install cargo-auditable
  cargo install cargo-dist
  cargo install cargo-release
