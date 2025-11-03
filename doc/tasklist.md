### Basic command execution logging — task list

- **Define scope and success criteria**
  - Log when a command is received, started, completed, with duration and outcome (ok/error).
  - Include minimal context: command name, request_id (if available), and error details.

- **Select logging approach**
  - Use `tracing` and `tracing-subscriber` for structured logs and spans.
  - Respect `RUST_LOG` for levels; default to `info`.

- **Initialize logging early**
  - Initialize a `tracing_subscriber` in `src/main.rs` before server startup.
  - Formatting: pretty/human-readable by default; allow JSON via env (future-ready).

- **Introduce request/command correlation**
  - Generate a simple `request_id` per inbound request/command in `src/server.rs`.
  - Propagate `request_id` into the command handling functions in `src/vault.rs`.

- **Instrument command lifecycle**
  - In `src/server.rs`: log “received” at INFO with `command`, `request_id`.
  - In `src/vault.rs`: wrap execution in a span with `command` and `request_id`.
  - Emit: `status=started`, `status=ok` with `duration_ms`, or `status=error` with error details.

- **Add structured fields**
  - Ensure logs include: `command`, `request_id`, `status`, `duration_ms` (on completion), and `error` (on failure).

- **Configuration knobs**
  - Use `RUST_LOG` for level control.
  - Add `LOG_FORMAT=pretty|json` env flag (default pretty). File/rotation out of scope for now.

- **Testing and verification**
  - Manual check: run representative commands; verify INFO/ERROR lines and durations.
  - Optional: unit/integration test asserting emitted events where feasible.

- **Documentation**
  - Add a short README section on enabling logging and adjusting verbosity.
  - Keep examples for dev (pretty) and optional JSON format.

