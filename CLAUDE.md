# {{crate_name}}

Rust 2024, nightly toolchain (pinned in `rust-toolchain.toml`).

## Constraints

{%- if use_async %}
- **Async**: `tokio`; logging via `tracing`.
{%- endif %}
{%- if use_http %}
- **HTTP**: `reqwest`; serialisation via `serde`.
{%- endif %}
{%- if use_cli %}
- **CLI**: args via `clap`; `.env` via `dotenvy`.
{%- endif %}
{%- if use_db %}
- **Database**: Postgres via `sqlx`, compile-time macros only (`query!`, `query_as!`, `query_scalar!`). Never runtime `query()`.
{%- endif %}
- **Dependencies**: Versions and features pinned once in `[workspace.dependencies]`; crates say `foo.workspace = true` and nothing else. Do not downgrade. Run `cargo machete` when editing a dependency list.
- **Lints**: Fix the code instead of silencing the lint. A narrow `#[expect(lint, reason = "...")]` is acceptable only after a code fix was considered and rejected. Widen the root `Cargo.toml` lint table only when the lint is wrong project-wide.

## Validation

Run validation **at the end of a plan or task**, not between edits — compute time is the bottleneck on this machine.

While working, prefer the narrowest check that can fail:

```sh
cargo check -p <crate>
cargo clippy -p <crate> --all-targets --all-features -- -D warnings
```

Full gate, once, before declaring a task done:

```sh
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo fmt --all
```

Code behind another target triple is invisible to the above; lint it explicitly when touched:

```sh
cargo clippy --target wasm32-unknown-unknown --all-targets --all-features -- -D warnings
```
{%- if use_db %}

## Database

- **Remote DB (`DATABASE_URL` in `.env`)**: Never run a `sqlx` command that writes to it. Migration files are immutable once written; every schema change needs human sign-off.
- **Local/Throwaway**: `sqlx database create|drop` and `sqlx migrate run` only ever run here, pointed at it explicitly:
    ```sh
    DATABASE_URL=postgres://postgres:postgres@localhost:5432/{{crate_name}} sqlx migrate run
    ```
- **`cargo sqlx prepare`**: Reads the schema and writes only `.sqlx/`; safe in any environment.
{%- endif %}

## Code Style

- Single responsibility per function, module, and crate. Split a file before it grows a second concern.
- **Minimal comments**: Only non-obvious performance work (say why, so the next reader does not "simplify" it back) and math/algorithms/external sources (link the paper, spec, RFC, or vendor doc). No restatements, banners, changelogs, or unowned TODOs.
- Comments in files you edit may be stripped by the user or tooling between passes. Take the file as you find it: do not restore or re-add them, and do not report it. Unexpected changes to _code_ still deserve a mention.
- **Errors**: `thiserror` enums in `error.rs`.
- No `unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!` or `dbg!` outside `#[cfg(test)]` — the lint table denies them and `clippy.toml` re-permits them in tests.

## Output

Concise. Report what changed and what validation said. No preamble, progress narration, or summaries of work the user just watched.

## References

- `.claude/skills/rust-skills/` — 265 Rust rules across 26 categories (ownership, errors, async, unsafe, API design, performance). Consult when writing or reviewing non-trivial Rust.
