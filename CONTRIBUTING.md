# Contributing to pmcli

Thanks for your interest in improving pmcli. This guide explains how to set up the project and what is expected from a pull request.

## Getting started

```bash
git clone https://github.com/djunekz/pmcli
cd pmcli
cargo build
cargo test
```

pmcli targets stable Rust. Builds use a vendored OpenSSL, so a C compiler and `perl` must be available.

## Environment

Copy `.env.example` if you want to export the optional variables pmcli reads:

- `GITHUB_TOKEN` is used as a fallback for `git-push` and `git-pull` over HTTPS.
- `USER` becomes the owner of new tasks.
- `HOME` and `XDG_CONFIG_HOME` decide where project data and `config.toml` are stored.

## Before you open a pull request

Run the same checks as CI:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

## Guidelines

- Open an issue before starting large changes.
- Keep each pull request focused on a single change.
- Add or update tests in `tests/` for any behavior you change.
- Keep project data human-readable and Git-friendly.
- Update `README.md` when you add or change a command.

## Commit messages

Use short, imperative messages, for example `Add task priority filter`.

## Reporting bugs

Use the bug report template and include your platform, the pmcli version and the exact command you ran.

## License

By contributing you agree that your contributions are licensed under the MIT License.
