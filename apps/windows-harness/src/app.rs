//! The `eframe`/`egui` application shell.
//!
//! Execution 01, Phase 04, Tasks 027-030. Authority: Engine Contract 27.
//!
//! This is the **only** file in this crate allowed to depend on `egui`
//! types (Task 033's module-boundary rule). Everything it calls into
//! (`state`, `viewport`, `tool`, `scenario`, `diagnostic_export`) is plain
//! Rust with its own unit tests that run with no window system at all.

use std::path::PathBuf;

use craftloop_geometry::Point2;
use craftloop_input::{MouseSimulator, PointerButtons, PointerEvent};

use crate::diagnostic_export::HarnessDiagnostic;
use crate::scenario::Scenario;
use crate::state::HarnessState;
use crate::tool::HarnessTool;

pub struct HarnessApp {
    state: HarnessState,
    scenario_path_input: String,
    last_export_path: Option<PathBuf>,
    status_line: String,
    last_canvas_size: (f64, f64),
}

impl HarnessApp {
    pub fn new() -> Self {
        Self {
            state: HarnessState::new(),
            scenario_path_input: String::new(),
            last_export_path: None,
            status_line: String::new(),
            last_canvas_size: (800.0, 600.0),
        }
    }

    fn pointer_buttons(response: &egui::Response, ctx: &egui::Context) -> PointerButtons {
        let _ = response;
        ctx.input(|i| PointerButtons {
            primary: i.pointer.button_down(egui::PointerButton::Primary),
            secondary: i.pointer.button_down(egui::PointerButton::Secondary),
            barrel: false,
        })
    }

    fn feed_canvas_interaction(&mut self, ui: &mut egui::Ui, response: &egui::Response, time: f64) {
        let buttons = Self::pointer_buttons(response, ui.ctx());

        let to_world = |screen_pos: egui::Pos2, rect: egui::Rect| -> Point2 {
            let local = Point2::new(
                (screen_pos.x - rect.min.x) as f64,
                (screen_pos.y - rect.min.y) as f64,
            );
            self.state.viewport.screen_to_world(local)
        };

        if response.drag_started() {
            if let Some(pos) = response.interact_pointer_pos() {
                let world = to_world(pos, response.rect);
                let sample = MouseSimulator::sample(world, time, buttons);
                self.dispatch(PointerEvent::Down(sample));
            }
        } else if response.dragged() {
            if let Some(pos) = response.interact_pointer_pos() {
                let world = to_world(pos, response.rect);
                let sample = MouseSimulator::sample(world, time, buttons);
                self.dispatch(PointerEvent::Move(sample));
            }
        } else if response.drag_stopped() && self.state.is_stroke_in_progress() {
            let pos = response
                .interact_pointer_pos()
                .unwrap_or(response.rect.center());
            let world = to_world(pos, response.rect);
            let sample = MouseSimulator::sample(world, time, buttons);
            self.dispatch(PointerEvent::Up(sample));
        }
    }

    fn dispatch(&mut self, event: PointerEvent) {
        if let Err(err) = self.state.handle_event(event) {
            self.status_line = format!("rejected event: {err}");
        }
    }

    fn draw_canvas(&mut self, ui: &mut egui::Ui) {
        let time = ui.ctx().input(|i| i.time);
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());
        self.last_canvas_size = (response.rect.width() as f64, response.rect.height() as f64);

        painter.rect_filled(response.rect, 0.0, egui::Color32::from_gray(250));

        self.feed_canvas_interaction(ui, &response, time);

        for stroke in &self.state.completed_strokes {
            let points: Vec<egui::Pos2> = stroke
                .events
                .iter()
                .filter_map(|e| e.sample())
                .map(|s| {
                    let screen = self.state.viewport.world_to_screen(s.position);
                    egui::pos2(
                        response.rect.min.x + screen.x as f32,
                        response.rect.min.y + screen.y as f32,
                    )
                })
                .collect();
            if points.len() >= 2 {
                painter.line(
                    points,
                    egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(30, 30, 30)),
                );
            }
        }

        self.handle_zoom_and_pan(ui, &response);
    }

    /// Scroll wheel zooms toward the cursor; a secondary-button drag pans.
    /// Exercises `Viewport::zoom_at`/`pan_by_screen_delta` from real
    /// interaction, not just their unit tests.
    fn handle_zoom_and_pan(&mut self, ui: &mut egui::Ui, response: &egui::Response) {
        if let Some(hover_pos) = response.hover_pos() {
            let scroll = ui.ctx().input(|i| i.smooth_scroll_delta.y);
            if scroll.abs() > f32::EPSILON {
                let local = egui::pos2(
                    hover_pos.x - response.rect.min.x,
                    hover_pos.y - response.rect.min.y,
                );
                let factor = (1.0 + scroll as f64 * 0.002).clamp(0.5, 2.0);
                self.state
                    .viewport
                    .zoom_at(Point2::new(local.x as f64, local.y as f64), factor);
            }
        }

        if response.dragged_by(egui::PointerButton::Secondary) {
            let delta = response.drag_delta();
            self.state
                .viewport
                .pan_by_screen_delta(craftloop_geometry::Vector2::new(
                    delta.x as f64,
                    delta.y as f64,
                ));
        }
    }

    /// Compute the bounds of every recorded sample and fit the viewport to
    /// them, falling back to the identity transform when there is nothing
    /// to fit.
    fn fit_view_to_strokes(&mut self, viewport_size: (f64, f64)) {
        let points: Vec<Point2> = self
            .state
            .completed_strokes
            .iter()
            .flat_map(|stroke| stroke.events.iter())
            .filter_map(|e| e.sample())
            .map(|s| s.position)
            .collect();
        match craftloop_geometry::Bounds2::from_points(&points) {
            Some(bounds) => self.state.viewport.fit_bounds(bounds, viewport_size, 0.1),
            None => self.state.viewport.reset(),
        }
    }

    fn draw_tool_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Tools");
        for tool in HarnessTool::ALL {
            if ui
                .selectable_label(self.state.active_tool == tool, tool.label())
                .clicked()
            {
                self.state.active_tool = tool;
            }
        }

        ui.separator();
        ui.heading("Simulated controls");
        let mut pressure = self.state.simulated_controls.pressure_override();
        if ui
            .add(egui::Slider::new(&mut pressure, 0.0..=1.0).text("pressure override"))
            .changed()
        {
            let _ = self
                .state
                .simulated_controls
                .set_pressure_override(pressure);
        }

        ui.separator();
        if ui.button("Clear canvas").clicked() {
            self.state.clear();
        }
        if ui.button("Fit view").clicked() {
            self.fit_view_to_strokes(self.last_canvas_size);
        }
    }

    fn draw_scenario_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Scenario loader");
        ui.text_edit_singleline(&mut self.scenario_path_input);
        if ui.button("Save current strokes as scenario").clicked() {
            let scenario = Scenario {
                name: "harness-capture".to_string(),
                description: "Strokes captured live from the Windows harness canvas.".to_string(),
                traces: self
                    .state
                    .completed_strokes
                    .iter()
                    .enumerate()
                    .map(|(i, stroke)| {
                        craftloop_input::PointerTrace::new(
                            format!("stroke-{i}"),
                            stroke.events.clone(),
                        )
                    })
                    .collect(),
            };
            match scenario.to_canonical_json() {
                Ok(json) => {
                    let path = std::env::temp_dir().join("craftloop-harness-scenario.json");
                    match std::fs::write(&path, json) {
                        Ok(()) => {
                            self.status_line = format!("saved scenario to {}", path.display())
                        }
                        Err(err) => self.status_line = format!("scenario save failed: {err}"),
                    }
                }
                Err(err) => self.status_line = format!("scenario serialization failed: {err}"),
            }
        }
        if ui.button("Load scenario").clicked() {
            let path = PathBuf::from(&self.scenario_path_input);
            match Scenario::load_from_file(&path) {
                Ok(scenario) => match self.state.load_traces(&scenario.traces) {
                    Ok(()) => {
                        self.status_line = format!("loaded scenario '{}'", scenario.name);
                    }
                    Err(err) => {
                        self.status_line = format!("scenario replay failed: {err}");
                    }
                },
                Err(err) => {
                    self.status_line = format!("scenario load failed: {err}");
                }
            }
        }
    }

    fn draw_inspector_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Diagnostics");
        let diagnostic = HarnessDiagnostic::capture(&self.state);
        ui.label(format!("Active tool: {}", diagnostic.active_tool));
        ui.label(format!(
            "Completed strokes: {}",
            diagnostic.completed_stroke_count
        ));
        ui.label(format!(
            "Stroke in progress: {}",
            diagnostic.stroke_in_progress
        ));
        ui.label(format!("Zoom: {:.2}", diagnostic.viewport_zoom));
        ui.separator();
        ui.small(diagnostic.simulator_disclaimer.clone());
        ui.separator();

        if ui.button("Export diagnostic JSON").clicked() {
            match diagnostic.to_canonical_json() {
                Ok(json) => {
                    let path = std::env::temp_dir().join("craftloop-harness-diagnostic.json");
                    match std::fs::write(&path, json) {
                        Ok(()) => {
                            self.status_line =
                                format!("exported diagnostics to {}", path.display());
                            self.last_export_path = Some(path);
                        }
                        Err(err) => {
                            self.status_line = format!("export failed: {err}");
                        }
                    }
                }
                Err(err) => {
                    self.status_line = format!("export serialization failed: {err}");
                }
            }
        }
        if let Some(path) = &self.last_export_path {
            ui.small(format!("last export: {}", path.display()));
        }

        ui.separator();
        ui.collapsing("Raw stroke events", |ui| {
            for (index, stroke) in self.state.completed_strokes.iter().enumerate() {
                ui.label(format!("Stroke {index}: {} events", stroke.events.len()));
            }
        });
    }
}

impl Default for HarnessApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for HarnessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("disclaimer").show(ctx, |ui| {
            ui.colored_label(
                egui::Color32::from_rgb(150, 90, 0),
                craftloop_input::SIMULATOR_DISCLAIMER,
            );
        });

        egui::SidePanel::left("tools").show(ctx, |ui| {
            self.draw_tool_panel(ui);
            ui.separator();
            self.draw_scenario_panel(ui);
        });

        egui::SidePanel::right("inspector").show(ctx, |ui| {
            self.draw_inspector_panel(ui);
        });

        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.label(&self.status_line);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_canvas(ui);
        });
    }
}
