# {{project-name}}

{{description}}

## Commands

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo fmt --all
```

`--all-targets` and `--all-features` both default to _off_, and each silently
skips code: without them clippy never compiles tests, benches or examples, and
never sees anything behind a `#[cfg(feature = "...")]`. Always pass both.

`bacon.toml` defines the same commands as watch jobs:

```bash
bacon            # clippy, the default job
bacon test       # tests
bacon fmt        # cargo fmt --all --check
bacon nextest    # needs `cargo install cargo-nextest`
```

Code compiled only for another target is invisible to the commands above. Lint
it explicitly, and add a CI job per extra triple:

```bash
cargo clippy --target wasm32-unknown-unknown --all-targets --all-features -- -D warnings
```

## Conventions

- Lints are declared once in the root `Cargo.toml` under `[workspace.lints]`;
  every crate opts in with `[lints] workspace = true`.
- Dependency versions and features are pinned once in
  `[workspace.dependencies]`; member crates only ever say `foo.workspace = true`.
- `unwrap`/`panic`/`todo`/`dbg!` are **denied** outside tests; `clippy.toml`
  re-permits them under `#[cfg(test)]`.
- `rustfmt.toml` uses unstable options, so `rust-toolchain.toml` pins nightly
  for local work.
- Keep the flags identical across `README.md`, `bacon.toml` and CI. A local
  command weaker than CI is how drift starts.
  {%- if use_ci %}

## CI

`.github/workflows/ci.yml` runs `rustfmt`, `clippy`, tests and `cargo-deny` in
parallel, then a release `build` gated on the first three. It builds on
**stable** (`RUSTUP_TOOLCHAIN=stable` outranks `rust-toolchain.toml`, and
`RUSTFLAGS=""` clears the nightly-only flags in `.cargo/config.toml`), so the
CI format gate is weaker than `cargo fmt` locally — stable `rustfmt` ignores
the unstable options rather than erroring.

Every job passes `--locked`. Run `cargo build` and **commit `Cargo.lock` before
the first push**, or CI fails with _"the lock file needs to be updated"_.
{%- endif %}
{%- if use_docker and kind == "bin" %}

## Docker

```bash
docker compose up --build
```

`docker/Dockerfile` is a cargo-chef build: dependencies are cooked in a cached layer,
then the binary is compiled and copied into a `debian:trixie-slim` runtime that
runs as a non-root `app` user. It builds on **stable** — `.dockerignore` keeps
`rust-toolchain.toml` and `.cargo/` out of the context, so anything in the
manifests must parse on stable cargo.

`.sqlx/` and `migrations/` are deliberately in the build context; almost
everything else, including `.claude/`, is not.
{%- endif %}
{%- if use_db %}

## Database

```bash
cp .env.example .env
sqlx database create
sqlx migrate run
cargo sqlx prepare --workspace -- --all-targets --all-features
```

`.sqlx/` must be committed: CI sets `SQLX_OFFLINE=true` and never connects to a
database, so that cache is the only thing the query macros have to check
against. Re-run `cargo sqlx prepare` in the same commit as any query or
migration change.
{%- endif %}
