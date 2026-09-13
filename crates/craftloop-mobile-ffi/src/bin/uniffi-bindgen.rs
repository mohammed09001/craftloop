//! UniFFI binding-generator entry point.
//!
//! Execution 01, Phase 28, Task 202. Standard UniFFI proc-macro-only
//! scaffolding mode boilerplate: this binary is what actually reads the
//! compiled `craftloop_mobile_ffi` cdylib's embedded metadata and emits
//! Kotlin (and, from Phase 29 onward, Swift) binding source. Run with
//! `cargo run --bin uniffi-bindgen -- generate --library <path-to-cdylib>
//! --language kotlin --out-dir <dir>` (see
//! `execution-evidence/mobile-ffi/` for a captured real run).

fn main() {
    uniffi::uniffi_bindgen_main()
}
