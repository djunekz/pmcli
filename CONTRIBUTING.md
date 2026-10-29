# Contributing to pmcli

Thanks for your interest in improving pmcli. This guide explains how to set up the project, which conventions the code follows, how to run the tests, and what is expected from a pull request. By taking part you agree to follow the [Code of Conduct](CODE_OF_CONDUCT.md).

## Ways to contribute

- Report bugs using the bug report template.
- Suggest features using the feature request template.
- Improve the documentation or the examples in the README.
- Fix an open issue or add test coverage.

For anything larger than a small fix, please open an issue first so the approach can be discussed before you invest time in it.

## Setup

pmcli targets stable Rust. The build uses a vendored OpenSSL, so a C compiler, `make` and `perl` must be available. On Termux install them with `pkg install rust git make perl clang`.

```bash
git clone https://github.com/djunekz/pmcli
cd pmcli
cargo build
cargo run -- --help
```

A `Dockerfile` and a `rust-toolchain.toml` are included for a reproducible environment:

```bash
docker build -t pmcli .
docker run --rm pmcli list
```

## Environment

Copy the variables from `.env.example` into your shell if you need them:

- `GITHUB_TOKEN` is the fallback credential for `git-push` and `git-pull` over HTTPS.
- `USER` becomes the owner of new tasks.
- `HOME` and `XDG_CONFIG_HOME` decide where project data and `config.toml` are stored.

## Project layout

- `src/main.rs` defines the command line interface with clap.
- `src/commands/` holds one module per command.
- `src/models.rs` contains the serializable data types.
- `src/config.rs` loads the optional `config.toml`.
- `tests/cli.rs` contains end-to-end tests that run the real binary against a temporary home directory.

## Code style

Formatting and linting are enforced in CI, so run both before every commit:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

Conventions used in this codebase:

- Format everything with `rustfmt` and its default settings.
- Fix every Clippy warning instead of silencing it with `#[allow]`.
- Prefer small functions with clear names over explanatory comments.
- Keep each command in its own module under `src/commands/`.
- Store project data as plain, human-readable JSON or Markdown so it stays friendly to Git.
- Keep user-facing messages short and consistent with the existing output.

## Running tests

```bash
cargo test
```

Tests in `tests/cli.rs` create a throwaway home directory for each case, so they never touch your real `~/.pmcli` data. When you change behavior, add or update a test next to the existing ones, for example:

```rust
#[test]
fn note_appends_markdown_bullet() {
    let sb = Sandbox::new();
    sb.run(&["create", "demo"]);
    sb.run(&["note", "demo", "first note"]);
    let notes = std::fs::read_to_string(sb.project_dir("demo").join("notes.md")).unwrap();
    assert_eq!(notes, "- first note\n");
}
```

## Pull request workflow

1. Fork the repository and create a branch from `master`, for example `fix-task-filter`.
2. Make your change in small, focused commits.
3. Run formatting, Clippy and the tests locally.
4. Update `README.md` if you added or changed a command.
5. Push your branch and open a pull request using the template.
6. Respond to review comments and push follow-up commits to the same branch.

A pull request is ready to merge when CI is green, the change is covered by tests, and a maintainer has approved it.

## Commit messages

Use short, imperative messages such as `Add task priority filter` or `Fix deadline parsing`. Mention the issue number in the pull request description, not in every commit.

## License

By contributing you agree that your contributions are licensed under the MIT License.
