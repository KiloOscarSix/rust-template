## Stack

- Rust 2024 edition, nightly toolchain (pinned in `rust-toolchain.toml`).
  {%- if use_async %}
- Async on `tokio`; logging via `tracing`.
  {%- endif %}
  {%- if use_http %}
- HTTP client `reqwest`; serialisation `serde`.
  {%- endif %}
  {%- if use_cli %}
- CLI args via `clap`; `.env` loaded with `dotenvy`.
  {%- endif %}
  {%- if use_db %}
- Postgres via `sqlx`, always with the compile-time macros (`query!`,
  `query_as!`, `query_scalar!`) — never the runtime `query()` forms.
  {%- endif %}

Dependency versions and features are pinned once in `[workspace.dependencies]`;
crates say `foo.workspace = true` and nothing else. Deps are current — do not
downgrade. Run `cargo machete` before finishing any change that touches a
dependency list.

## Validation

Run validation **at the end of a plan or task**, not between edits. Compute time
is the bottleneck on this machine.

While working, prefer the narrowest check that can fail:

```bash
cargo check -p <crate>
cargo clippy -p <crate> --all-targets --all-features -- -D warnings
```

Full gate, once, before declaring a task done:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
cargo fmt --all
```

Code behind another target triple is invisible to the commands above. When you
touch it, lint it explicitly:

```bash
cargo clippy --target wasm32-unknown-unknown --all-targets --all-features -- -D warnings
```

Fixing the code always beats silencing the lint, and is the first thing to try.
When a lint is genuinely wrong at one specific site, a narrow
`#[expect(lint, reason = "...")]` on the smallest scope that covers it is
acceptable. Reach for it after a code change has been considered and
rejected, not instead of considering one. Widen the lint table in the root
`Cargo.toml` only when the lint is wrong for the whole project.
{%- if use_db %}

## Database

**Never run a `sqlx` command that writes to the remote database.** The
`DATABASE_URL` in `.env` points at it. Migration files are immutable once
written and every schema change needs human sign-off, so `sqlx database
create`, `sqlx database drop` and `sqlx migrate run` only ever run against a
local or throwaway Postgres. Point them at it explicitly:

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/{{crate_name}} sqlx migrate run
```

`cargo sqlx prepare` reads the schema and writes only `.sqlx/`, so it is safe to
run in any environment.

{%- endif %}

## Code style

Follow single responsibility in both code and project structure. One reason to
change per function, per module, per crate. Split a file before it grows a
second concern.

**Write minimal comments.** Code should carry its own meaning through naming and
structure. Comment only these two cases:

- **Non-obvious performance work** — when code is written in an unusual or
  convoluted way for a measurable gain, say why. The next reader will otherwise
  "simplify" it back.
- **Math, algorithms and external sources** — link the paper, specification,
  RFC, or vendor doc the implementation follows.

Everything else — restating what the line does, section banners, changelog
comments, TODOs without an owner — should not be written.

Comments in files you edit may be rewritten or deleted between your passes, by
the user or by tooling. That is normal. Take the file as you find it: do not
restore stripped comments, do not re-add them on a later pass, and do not report
it as a problem. Unexpected changes to _code_ still deserve a mention.

Errors are `thiserror` enums in `error.rs`. Do not introduce `unwrap`, `expect`,
`panic!`, `todo!`, `unimplemented!` or `dbg!` outside `#[cfg(test)]` — the lint
table denies them, and `clippy.toml` already re-permits them in tests.

## Output

Be concise. Report what changed and what the validation said. Skip preamble,
progress narration, and summaries of work the user just watched you do.

## Skills

`.claude/skills/rust-skills` holds 265 Rust rules across 26 categories
(ownership, error handling, async, unsafe, API design, performance). Consult it
when writing or reviewing non-trivial Rust.
