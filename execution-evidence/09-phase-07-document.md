# Phase 07 — Document Model, Semantic Paper, and Persistence — Evidence

Recorded: 2026-09-13

## Crate created

`crates/craftloop-document` (Engine Contract 15), depending on
`craftloop-errors`, `craftloop-ids`, `craftloop-geometry`,
`craftloop-serialization`, `craftloop-ink`, `craftloop-recognition`; dev-
depends on `craftloop-input`. Added `NoteId`/`PageId` to `craftloop-ids` and
`DomainError::Document`/`DocumentErrorKind` to `craftloop-errors` (both
minimal, targeted extensions following the established per-subsystem
pattern).

## Tasks 048–055

| Task | Module | Summary |
|---|---|---|
| 048 Native document root | `document.rs`, `metadata.rs`, `units.rs` | `Document { schema_version, metadata, units, pages, active_page }`. "Orthographic sets" and "asset references" (also named in the task) are explicitly **not** fielded yet — no owning phase/type exists for either (Phase 20 and an unreached asset-reference phase respectively); documented in the module doc rather than stubbed. |
| 049 Page space vs. engineering space | `page_layout.rs` | View blocks with real per-view coordinate frames don't exist until Phase 20, so this phase establishes the *type-level* boundary Phase 20 must build inside: `PageLayoutTransform` never appears as a field of any `SemanticEntity`, so there is no code path by which repositioning something on the page could reach into its own coordinates. Demonstrated with a test, not just asserted in prose. |
| 050 Semantic object containers | `entity.rs`, `note.rs` | `SemanticEntity` (`Stroke`, `Primitive` wrapping Phase 06's `Beautified`, `Note`) + `EntityId`. Of Task 050's seven named kinds, only these three have real implemented types; dimensions/view labels/suggestions/conflicts/ephemeral records are documented as deferred to the phases that define them (10, 20, 12-14), not fabricated. |
| 051 Atomic local persistence | `persistence.rs` | `save_document_atomically`: canonical-JSON → unique temp file → `sync_all` → best-effort `.bak` snapshot of the prior file → atomic `rename`. A crash at any point leaves either the fully-old or fully-new file, never a partial write. |
| 052 Migration framework | `migration.rs` | `Migration` trait operating on raw `serde_json::Value` (not a typed historical struct) + `migrate_to_current` runner. No real migration exists yet (schema is still v1); proven with a synthetic v0→v1 migration, documented as synthetic. Never touches recognition logic (operates purely on JSON), satisfying the task's "never re-run recognition on confirmed old entities during migration" by construction. |
| 053 Integrity checks | `document.rs::validate`, `persistence.rs::load_document` | Structural validation (active-page reference exists, schema version readable) plus load-time fallback from a corrupted/missing primary file to its `.bak` snapshot. |
| 054 Autosave journal | `autosave.rs` | `AutosaveJournal`: append-only, newline-delimited records — O(1)-per-write, no whole-document rewrite. A dangling (non-newline-terminated) final line from an interrupted write is skipped on read, not returned as a corrupt record. Off-thread scheduling is explicitly noted as an adapter concern this platform-independent crate does not fabricate. |
| 055 Save/reopen golden tests | `tests/save_reopen_golden.rs` | A representative document (2 pages; a `Stroke` built via the real `MouseSimulator`; an accepted `Beautified` primitive from a real `recognize`→`beautify` pass; a `Note`) round-trips through save/load with full equality, individually-recoverable entities, and stability across repeated round trips. |

## A real, significant bug found by the Task 055 golden test

`EntityId` (`Stroke`/`Primitive`/`Note`, each wrapping a typed ID) originally
used `#[derive(Serialize, Deserialize)]`. serde's default enum
representation serializes a data-carrying variant as a JSON **object**
(`{"Stroke": "<uuid>"}`). `Page` stores its entities in a
`BTreeMap<EntityId, SemanticEntity>`, and JSON object keys must be strings
— so the entire document failed to serialize the moment a page held more
than zero entities. Every smaller unit test (including `entity.rs`'s own
serialization tests) had only ever serialized a *single* `EntityId` value,
never a map keyed by one, so nothing caught this until the golden
persistence test exercised the real, full document shape.

Fixed with a hand-written `Serialize`/`Deserialize`/`Display`/`FromStr` for
`EntityId` producing/parsing one string, `"<Kind>:<uuid>"`. Three new
regression tests lock this in: `entity_id_serializes_as_a_plain_json_string_not_an_object`,
`entity_id_display_and_from_str_round_trip`, and
`a_map_keyed_by_entity_id_serializes_and_round_trips` (the last one directly
reproducing the shape that broke).

This is exactly the class of bug Task 055 exists to catch — a structural
persistence defect invisible to per-type unit tests, visible only when a
realistic document is actually saved and reopened.

## Commands and results

```
cargo build -p craftloop-document
cargo test -p craftloop-document              # 38/38 unit tests
cargo test -p craftloop-document --test save_reopen_golden   # 5/5 (initially
                                                #   4/5 failing on the EntityId bug above)
cargo fmt --all -- --check                     # clean after cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
  # 2 findings: wrong_self_convention on Migration::from_version/to_version
  #   (renamed to source_version/target_version), derivable_impls on
  #   DocumentUnits's manual Default (replaced with #[derive(Default)]).
  # Re-run clean.
cargo test --workspace                         # 286/286 tests passing workspace-wide
```

Full output: `test-reports/phase-07-cargo-test.txt`,
`test-reports/phase-07-cargo-clippy.txt`.

## Deferred (explicitly, not silently)

- Dimensions, view labels, suggestions/conflicts as stored document
  entities — Phases 10, 20, 12-14 respectively.
- Orthographic sets and reference-image assets in the document root —
  Phase 20 and an unreached asset phase.
- Real off-thread autosave scheduling — a platform-adapter concern (Phase
  04's harness is single-threaded; Android/iPad adapters are Phases 28-29).
- A real (non-synthetic) schema migration — none exists yet; schema is
  still v1.

## Phase Gate

- All eight tasks (048–055) represented in repository code with passing
  tests, including one significant structural persistence bug caught by
  the golden test and fixed with three new regression tests.
- `cargo test --workspace`: 286/286 passing. `cargo fmt --check` and
  `cargo clippy -D warnings`: clean.
- No scope boundary crossed: no dimension/constraint/orthographic logic
  fabricated ahead of its phase; `windows-simulator`/`windows-harness`
  untouched.
- Proceeding to Phase 08 automatically.
