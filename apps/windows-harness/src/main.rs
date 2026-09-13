//! Craft Loop Windows Engineering Test Harness.
//!
//! Execution 01, Phase 04. Engine Contract 27: this is a disposable test
//! adapter, never production authority and never the final Craft Loop UI.
//! See `apps/README.md` and `craftloop_input::SIMULATOR_DISCLAIMER`.

mod app;
mod diagnostic_export;
mod scenario;
mod state;
mod tool;
mod viewport;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Craft Loop Windows Engineering Test Harness (disposable — not production UI)",
        options,
        Box::new(|_cc| Ok(Box::new(app::HarnessApp::new()))),
    )
}
