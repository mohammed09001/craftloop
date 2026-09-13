# Platform Adapters and Test Applications

This directory holds platform adapters and test applications, kept separate
from the domain crates in `../crates/`.

Per Execution 01 Phase 01, Task 007: the shared core must compile
independently of Android, iOS, or Windows UI frameworks. Nothing under
`crates/` may depend on anything added here.

Planned members (added when their phase begins):

- `windows-harness` — Phase 04, native Rust `eframe`/`egui` engineering test
  harness. Test adapter only, never production authority (Engine Contract 27).
- Android and iPad adapters are native Kotlin/Swift projects and will live in
  their own top-level directories (e.g. `android/`, `ios/`) once Phases 28–29
  begin, not under this Rust `apps/` directory.
