//! `CraftLoopSession`: the Web Live View product-session API.
//!
//! Execution 03, Phase 04 (Tasks 023-034). Ports native
//! `craftloop-mobile-ffi`'s `CraftLoopSession` (Execution 02, Phase 04)
//! to the browser, coordinating exactly the same real engines through
//! exactly the same mutation discipline (build a scratch clone, call
//! the real already-tested engine function, commit the delta through
//! `DocumentHistory::commit` as one or more `DocumentChange`s -- never
//! mutate `self.document` directly outside a commit) -- see that
//! module's own doc comment for the full rationale, which this port
//! does not repeat line for line.
//!
//! **Differences from native, and why:**
//! - No `Mutex`. Native wraps state in one because a mobile client may
//!   call in from more than one thread (a Kotlin `ViewModel`, a
//!   coroutine). A browser tab's JS is single-threaded; wasm-bindgen
//!   already gives each exported method exclusive `&mut self` access,
//!   so a `Mutex` here would guard against a race that cannot happen.
//! - No `open(path)`/`save(path)`. There is no filesystem on
//!   `wasm32-unknown-unknown` (Phase 03's audit). `to_json`/`from_json`
//!   replace them (Task 034/Article 43: path-independent
//!   serialization) -- the browser persistence adapter (Phase 14) reads
//!   and writes the resulting string through IndexedDB, not a path.
//! - Every method that would need to return a nested/tagged-union type
//!   UniFFI can cross but `wasm-bindgen` cannot (`FfiSceneSnapshot`,
//!   `FfiConstraintKind`, ...) returns/accepts a JSON string built from
//!   this crate's own `types` module instead. See that module's doc
//!   comment for why.

use std::collections::BTreeSet;

use craftloop_command::{
    Command, CommandAction, CommandBus, CommandSource, RiskLevel, UndoMetadata,
};
use craftloop_consistency::{resolve, Conflict, ConflictKind, ConflictStatus, ResolutionChoice};
use craftloop_constraint::SolveStatus;
use craftloop_dimension::{DimensionRole, DimensionTarget, SemanticDimension};
use craftloop_document::{
    axes_for_identity, evaluate_readiness, propagate_confirmed_value, propose_shared_value,
    transition_to_orthographic, Document, DocumentChange, DocumentHistory, EntityId,
    OrthographicReadiness, OrthographicSet, ProjectionConvention, SemanticEntity, SharedAxis,
    ViewBlock,
};
use craftloop_errors::{DomainError, Severity};
use craftloop_geometry::{Circle2, Point2, RelationalRectangle, Segment2};
use craftloop_ids::{
    ConflictId, ConstraintId, CraftLoopId, DimensionId, NoteId, OrthographicSetId, PrimitiveId,
    StrokeId, ViewId,
};
use craftloop_ink::Stroke;
use craftloop_recognition::{beautify, rank_candidates, recognize, RecognitionCandidate};
use craftloop_sketch::{ConstraintOutcome, ConstraintProvenance, EzpzSolver, PointRef};
use wasm_bindgen::prelude::*;

use crate::types::*;

#[cfg(target_arch = "wasm32")]
fn now_seconds() -> f64 {
    js_sys::Date::now() / 1000.0
}

#[cfg(not(target_arch = "wasm32"))]
fn now_seconds() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}

// ---------------------------------------------------------------------
// Error boundary
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum WebSessionError {
    #[error("{detail}")]
    Domain { detail: String },
}

impl From<DomainError> for WebSessionError {
    fn from(err: DomainError) -> Self {
        WebSessionError::Domain {
            detail: err.to_string(),
        }
    }
}

impl From<WebSessionError> for JsValue {
    fn from(err: WebSessionError) -> Self {
        JsValue::from_str(&err.to_string())
    }
}

fn missing(kind: &str, id: impl std::fmt::Display) -> WebSessionError {
    WebSessionError::Domain {
        detail: format!("no {kind} with id {id}"),
    }
}

fn json_err(err: serde_json::Error) -> WebSessionError {
    WebSessionError::Domain {
        detail: format!("json error: {err}"),
    }
}

fn to_json<T: serde::Serialize>(value: &T) -> Result<String, WebSessionError> {
    craftloop_serialization::to_canonical_json(value).map_err(json_err)
}

/// See native `session.rs::parse_id`'s own doc comment -- identical
/// technique, same bare-UUID format every `Web*Summary.id` this module
/// builds emits.
fn parse_id<T: CraftLoopId>(raw: &str) -> Result<T, WebSessionError> {
    let uuid = uuid::Uuid::parse_str(raw).map_err(|err| WebSessionError::Domain {
        detail: format!("invalid id {raw:?}: {err}"),
    })?;
    Ok(T::from_u128(uuid.as_u128()))
}

/// See native `session.rs::resolve_entity_id`'s own doc comment -- same
/// bare-UUID-tried-against-every-kind technique, kept because the same
/// real bug it documents (an `EntityId::Display`-formatted id never
/// matching a bare-UUID-only caller) applies here unchanged.
fn resolve_entity_id(
    page: &craftloop_document::Page,
    raw: &str,
) -> Result<EntityId, WebSessionError> {
    let uuid = uuid::Uuid::parse_str(raw).map_err(|err| WebSessionError::Domain {
        detail: format!("invalid id {raw:?}: {err}"),
    })?;
    let bits = uuid.as_u128();
    let candidates = [
        EntityId::Stroke(StrokeId::from_u128(bits)),
        EntityId::Primitive(PrimitiveId::from_u128(bits)),
        EntityId::Note(NoteId::from_u128(bits)),
        EntityId::Dimension(DimensionId::from_u128(bits)),
        EntityId::Conflict(ConflictId::from_u128(bits)),
    ];
    candidates
        .into_iter()
        .find(|candidate| page.get(*candidate).is_some())
        .ok_or_else(|| missing("entity", raw))
}

fn entity_id_bare_uuid(id: EntityId) -> String {
    match id {
        EntityId::Stroke(inner) => inner.to_string(),
        EntityId::Primitive(inner) => inner.to_string(),
        EntityId::Note(inner) => inner.to_string(),
        EntityId::Dimension(inner) => inner.to_string(),
        EntityId::Conflict(inner) => inner.to_string(),
    }
}

impl WebConstraintKind {
    fn into_domain(self) -> Result<craftloop_sketch::SketchConstraintKind, WebSessionError> {
        use craftloop_sketch::SketchConstraintKind as K;
        Ok(match self {
            WebConstraintKind::Coincident { a, b } => K::Coincident(
                PointRef::LineEnd(parse_id::<PrimitiveId>(&a)?),
                PointRef::LineStart(parse_id::<PrimitiveId>(&b)?),
            ),
            WebConstraintKind::Horizontal { line } => K::Horizontal(parse_id(&line)?),
            WebConstraintKind::Vertical { line } => K::Vertical(parse_id(&line)?),
            WebConstraintKind::Parallel { a, b } => K::Parallel(parse_id(&a)?, parse_id(&b)?),
            WebConstraintKind::Perpendicular { a, b } => {
                K::Perpendicular(parse_id(&a)?, parse_id(&b)?)
            }
            WebConstraintKind::EqualLength { a, b } => K::EqualLength(parse_id(&a)?, parse_id(&b)?),
            WebConstraintKind::EqualRadius { a, b } => K::EqualRadius(parse_id(&a)?, parse_id(&b)?),
            WebConstraintKind::Concentric { a, b } => K::Concentric(parse_id(&a)?, parse_id(&b)?),
        })
    }
}

// ---------------------------------------------------------------------
// Session
// ---------------------------------------------------------------------

#[wasm_bindgen]
pub struct CraftLoopSession {
    document: Document,
    history: DocumentHistory,
    command_bus: CommandBus,
    selection: BTreeSet<EntityId>,
    /// See native `SessionState::pending_shared_value_proposals`'s own
    /// doc comment for exactly what this is and its documented
    /// cross-session limitation.
    pending_shared_value_proposals:
        std::collections::BTreeMap<ConflictId, (ViewId, SharedAxis, f64)>,
}

impl CraftLoopSession {
    fn active_page(&self) -> Result<craftloop_ids::PageId, WebSessionError> {
        self.document
            .active_page()
            .ok_or_else(|| WebSessionError::Domain {
                detail: "document has no active page".to_string(),
            })
    }

    fn low_risk_command(
        &self,
        action: CommandAction,
        namespace: craftloop_command::CommandNamespace,
        description: &str,
    ) -> Command {
        Command {
            id: craftloop_ids::CommandId::new(),
            action,
            source: CommandSource::Toolbar,
            namespace,
            parameters: std::collections::BTreeMap::new(),
            risk: RiskLevel::Low,
            timestamp_seconds: now_seconds(),
            undo: UndoMetadata::undoable(description),
        }
    }

    fn submit(
        &mut self,
        action: CommandAction,
        namespace: craftloop_command::CommandNamespace,
        description: &str,
    ) -> Result<(), WebSessionError> {
        let command = self.low_risk_command(action, namespace, description);
        self.command_bus.submit(command, None)?;
        Ok(())
    }

    fn commit(
        &mut self,
        changes: Vec<DocumentChange>,
    ) -> Result<craftloop_ids::TransactionId, WebSessionError> {
        Ok(self.history.commit(&mut self.document, changes)?)
    }

    fn insert_primitive(
        &mut self,
        beautified: craftloop_recognition::Beautified,
    ) -> Result<String, WebSessionError> {
        let primitive_id = PrimitiveId::new();
        let page_id = self.active_page()?;
        self.commit(vec![
            DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Primitive {
                    id: primitive_id,
                    beautified: beautified.clone(),
                },
            },
            DocumentChange::InsertSketchPrimitive {
                id: primitive_id,
                beautified,
            },
        ])?;
        Ok(primitive_id.to_string())
    }
}

#[wasm_bindgen]
impl CraftLoopSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CraftLoopSession {
        CraftLoopSession {
            document: Document::new("Untitled", now_seconds()),
            history: DocumentHistory::new(),
            command_bus: CommandBus::new(),
            selection: BTreeSet::new(),
            pending_shared_value_proposals: std::collections::BTreeMap::new(),
        }
    }

    /// Task 034 / Article 43: rebuild a session from a previously saved
    /// `to_json()` string. History is intentionally not persisted (same
    /// as native's `open`: a freshly opened session starts with an
    /// empty, session-local undo/redo stack).
    #[wasm_bindgen(js_name = fromJson)]
    pub fn from_json(json: &str) -> Result<CraftLoopSession, WebSessionError> {
        let document: Document = serde_json::from_str(json).map_err(json_err)?;
        document.validate()?;
        Ok(CraftLoopSession {
            document,
            history: DocumentHistory::new(),
            command_bus: CommandBus::new(),
            selection: BTreeSet::new(),
            pending_shared_value_proposals: std::collections::BTreeMap::new(),
        })
    }

    /// Task 034: canonical JSON, the exact same serialization native's
    /// `save_document_atomically` writes to disk (Phase 03's audit note
    /// on `craftloop-document::persistence`) -- just without the
    /// filesystem step, since there is none here.
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> Result<String, WebSessionError> {
        to_json(&self.document)
    }

    #[wasm_bindgen(js_name = sceneSnapshot)]
    pub fn scene_snapshot(&self) -> String {
        let document = &self.document;

        let mut strokes = Vec::new();
        let mut primitives = Vec::new();
        let mut conflicts = Vec::new();
        let mut geometry_by_id = std::collections::BTreeMap::new();
        if let Some(page_id) = document.active_page() {
            if let Some(page) = document.page(page_id) {
                for entity in page.entities() {
                    match entity {
                        SemanticEntity::Stroke(stroke) => strokes.push(WebStrokeSummary {
                            id: stroke.id.to_string(),
                            sample_count: stroke.len() as u32,
                        }),
                        SemanticEntity::Primitive { id, beautified } => {
                            let (kind, bounds) = match &beautified.primitive {
                                craftloop_recognition::BeautifiedPrimitive::Line(s) => {
                                    (WebPrimitiveKind::Line, s.bounds())
                                }
                                craftloop_recognition::BeautifiedPrimitive::Circle(c) => {
                                    (WebPrimitiveKind::Circle, c.bounds())
                                }
                                craftloop_recognition::BeautifiedPrimitive::Arc(a) => {
                                    (WebPrimitiveKind::Arc, a.bounds())
                                }
                                craftloop_recognition::BeautifiedPrimitive::Rectangle(r) => {
                                    (WebPrimitiveKind::Rectangle, r.bounds())
                                }
                            };
                            primitives.push(WebPrimitiveSummary {
                                id: id.to_string(),
                                kind,
                                min_x: bounds.min.x,
                                min_y: bounds.min.y,
                                max_x: bounds.max.x,
                                max_y: bounds.max.y,
                                geometry: beautified.primitive.clone(),
                            });
                            geometry_by_id.insert(*id, beautified.primitive.clone());
                        }
                        SemanticEntity::Dimension(_) => {}
                        SemanticEntity::Conflict(conflict) => {
                            conflicts.push(WebConflictSummary {
                                id: conflict.id.to_string(),
                                kind: conflict.kind.into(),
                                severity: conflict.severity.into(),
                                unresolved: conflict.is_unresolved(),
                                affected_entities: conflict.affected_entities.clone(),
                                existing_truth: conflict.existing_truth.clone(),
                                proposed_truth: conflict.proposed_truth.clone(),
                                evidence: conflict.evidence.clone(),
                                allowed_resolutions: conflict
                                    .resolution_choices
                                    .iter()
                                    .map(|choice| (*choice).into())
                                    .collect(),
                            });
                        }
                        SemanticEntity::Note(_) => {}
                    }
                }
            }
        }
        let dimensions: Vec<WebDimensionSummary> = document
            .dimension_store()
            .dimensions()
            .map(|dimension| WebDimensionSummary {
                id: dimension.id.to_string(),
                kind: dimension.kind.into(),
                role: dimension.role.into(),
                value: dimension.value(),
                target_primitive_ids: dimension
                    .target
                    .primitive_ids()
                    .iter()
                    .map(|id| id.to_string())
                    .collect(),
            })
            .collect();
        let constraints: Vec<WebConstraintSummary> = document
            .sketch()
            .constraints()
            .map(|(id, kind, _provenance)| {
                // A single primitive can supply more than one `PointRef`
                // (e.g. `Horizontal(line)` constrains that line's start
                // *and* end), so dedupe through a `BTreeSet` -- the
                // field means "which primitives does this touch," not
                // "how many points on them."
                let primitive_ids: std::collections::BTreeSet<PrimitiveId> = kind
                    .point_refs()
                    .iter()
                    .map(|point_ref| point_ref.primitive_id())
                    .collect();
                WebConstraintSummary {
                    id: id.to_string(),
                    label: format!("{kind:?}"),
                    primitive_ids: primitive_ids.iter().map(|id| id.to_string()).collect(),
                }
            })
            .collect();
        let view_blocks = document
            .view_blocks()
            .map(|view| {
                let (readiness, issues) = evaluate_readiness(view, &geometry_by_id);
                WebViewBlockSummary {
                    id: view.id.to_string(),
                    identity: view.identity().map(Into::into),
                    geometry_member_ids: view
                        .geometry_members
                        .iter()
                        .map(|id| id.to_string())
                        .collect(),
                    readiness: readiness.into(),
                    blockers: issues
                        .iter()
                        .filter(|issue| issue.is_blocker())
                        .map(|issue| format!("{issue:?}"))
                        .collect(),
                    axis_bindings: document
                        .multiview_graph()
                        .bindings_for_view(view.id)
                        .into_iter()
                        .map(|(axis, dimension_id)| WebAxisBinding {
                            axis: axis.into(),
                            dimension_id: dimension_id.to_string(),
                        })
                        .collect(),
                    unresolved_axes: document
                        .multiview_graph()
                        .unresolved_axes(view)
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                }
            })
            .collect();
        let orthographic_sets = document
            .orthographic_sets()
            .map(|set| WebOrthographicSetSummary {
                id: set.id.to_string(),
                view_ids: set.views().iter().map(|id| id.to_string()).collect(),
            })
            .collect();
        let selected_entity_ids = self
            .selection
            .iter()
            .map(|id| entity_id_bare_uuid(*id))
            .collect();

        let snapshot = WebSceneSnapshot {
            strokes,
            primitives,
            dimensions,
            constraints,
            conflicts,
            view_blocks,
            orthographic_sets,
            selected_entity_ids,
            revision: document.revision(),
            can_undo: self.history.can_undo(),
            can_redo: self.history.can_redo(),
        };
        // Infallible: every field above is already-valid, already-owned
        // data (no interior `NaN`/cycles serde_json would choke on).
        to_json(&snapshot).unwrap_or_else(|err| {
            format!(
                r#"{{"error":{}}}"#,
                serde_json::to_string(&err.to_string()).unwrap()
            )
        })
    }

    #[wasm_bindgen(js_name = debugState)]
    pub fn debug_state(&self) -> String {
        let unresolved_conflicts = self
            .document
            .active_page()
            .and_then(|id| self.document.page(id))
            .map(|page| {
                page.entities()
                    .filter(|e| matches!(e, SemanticEntity::Conflict(c) if c.is_unresolved()))
                    .count()
            })
            .unwrap_or(0);
        let state = WebDebugState {
            revision: self.document.revision(),
            transaction_count: self.history.transaction_count() as u32,
            can_undo: self.history.can_undo(),
            can_redo: self.history.can_redo(),
            unresolved_conflict_count: unresolved_conflicts as u32,
            view_block_count: self.document.view_blocks().count() as u32,
            orthographic_set_count: self.document.orthographic_sets().count() as u32,
        };
        to_json(&state).unwrap_or_default()
    }

    // -- Input / raw ink (Article 9/15) ----------------------------------

    /// `samples_json`: a JSON array of [`WebPointerSample`].
    #[wasm_bindgen(js_name = submitStroke)]
    pub fn submit_stroke(&mut self, samples_json: &str) -> Result<String, WebSessionError> {
        let samples: Vec<WebPointerSample> =
            serde_json::from_str(samples_json).map_err(json_err)?;
        let domain_samples: Vec<craftloop_input::PointerSample> = samples
            .iter()
            .map(craftloop_input::PointerSample::from)
            .collect();
        let stroke = Stroke::new(StrokeId::new(), domain_samples)?;
        let stroke_id = stroke.id;

        let points: Vec<Point2> = stroke.samples().iter().map(|s| s.position).collect();
        let ranked = rank_candidates(recognize(&points));
        let eligible = ranked
            .first()
            .map(|c| !matches!(c, RecognitionCandidate::KeepAsInk))
            .unwrap_or(false);

        let page_id = self.active_page()?;
        self.commit(vec![DocumentChange::InsertEntity {
            page_id,
            entity: SemanticEntity::Stroke(stroke),
        }])?;

        to_json(&WebStrokeOutcome {
            stroke_id: stroke_id.to_string(),
            eligible_for_recognition: eligible,
        })
    }

    #[wasm_bindgen(js_name = acceptRecognition)]
    pub fn accept_recognition(
        &mut self,
        stroke_id: &str,
    ) -> Result<Option<String>, WebSessionError> {
        let stroke_id: StrokeId = parse_id(stroke_id)?;
        let page_id = self.active_page()?;
        let entity = self
            .document
            .page(page_id)
            .and_then(|p| p.get(EntityId::Stroke(stroke_id)))
            .cloned()
            .ok_or_else(|| missing("stroke", stroke_id))?;
        let stroke = match &entity {
            SemanticEntity::Stroke(s) => s.clone(),
            _ => unreachable!("EntityId::Stroke always maps to SemanticEntity::Stroke"),
        };

        let points: Vec<Point2> = stroke.samples().iter().map(|s| s.position).collect();
        let top = rank_candidates(recognize(&points))
            .into_iter()
            .next()
            .unwrap_or(RecognitionCandidate::KeepAsInk);
        let Some(beautified) = beautify(&top) else {
            return Ok(None);
        };

        let primitive_id = PrimitiveId::new();
        self.commit(vec![
            DocumentChange::RemoveEntity { page_id, entity },
            DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Primitive {
                    id: primitive_id,
                    beautified: beautified.clone(),
                },
            },
            DocumentChange::InsertSketchPrimitive {
                id: primitive_id,
                beautified,
            },
        ])?;
        Ok(Some(primitive_id.to_string()))
    }

    // -- Explicit-tool primitive creation --------------------------------

    #[wasm_bindgen(js_name = createPrimitiveLine)]
    pub fn create_primitive_line(
        &mut self,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> Result<String, WebSessionError> {
        self.submit(
            CommandAction::Line,
            craftloop_command::CommandNamespace::Sketch,
            "Created line",
        )?;
        let beautified = craftloop_recognition::Beautified {
            primitive: craftloop_recognition::BeautifiedPrimitive::Line(Segment2::new(
                Point2::new(x0, y0),
                Point2::new(x1, y1),
            )),
            displacement: 0.0,
        };
        self.insert_primitive(beautified)
    }

    #[wasm_bindgen(js_name = createPrimitiveCircle)]
    pub fn create_primitive_circle(
        &mut self,
        center_x: f64,
        center_y: f64,
        radius: f64,
    ) -> Result<String, WebSessionError> {
        self.submit(
            CommandAction::Circle,
            craftloop_command::CommandNamespace::Sketch,
            "Created circle",
        )?;
        let circle = Circle2::new(Point2::new(center_x, center_y), radius)?;
        let beautified = craftloop_recognition::Beautified {
            primitive: craftloop_recognition::BeautifiedPrimitive::Circle(circle),
            displacement: 0.0,
        };
        self.insert_primitive(beautified)
    }

    #[wasm_bindgen(js_name = createPrimitiveRectangle)]
    pub fn create_primitive_rectangle(
        &mut self,
        x0: f64,
        y0: f64,
        x1: f64,
        y1: f64,
    ) -> Result<String, WebSessionError> {
        self.submit(
            CommandAction::Rectangle,
            craftloop_command::CommandNamespace::Sketch,
            "Created rectangle",
        )?;
        let rectangle =
            RelationalRectangle::from_axis_aligned(Point2::new(x0, y0), Point2::new(x1, y1))?;
        let beautified = craftloop_recognition::Beautified {
            primitive: craftloop_recognition::BeautifiedPrimitive::Rectangle(rectangle),
            displacement: 0.0,
        };
        self.insert_primitive(beautified)
    }

    // -- Selection (ephemeral) -------------------------------------------

    pub fn select(&mut self, ids: Vec<String>) -> Result<(), WebSessionError> {
        let page_id = self.active_page()?;
        let mut parsed = BTreeSet::new();
        {
            let page = self
                .document
                .page(page_id)
                .ok_or_else(|| missing("page", page_id))?;
            for raw in &ids {
                parsed.insert(resolve_entity_id(page, raw)?);
            }
        }
        self.submit(
            CommandAction::Select,
            craftloop_command::CommandNamespace::Notebook,
            "Selected entities",
        )?;
        self.selection = parsed;
        Ok(())
    }

    #[wasm_bindgen(js_name = clearSelection)]
    pub fn clear_selection(&mut self) {
        self.selection.clear();
    }

    #[wasm_bindgen(js_name = deleteSelected)]
    pub fn delete_selected(&mut self, ids: Vec<String>) -> Result<(), WebSessionError> {
        let page_id = self.active_page()?;
        let mut changes = Vec::new();
        let mut parsed_ids = Vec::new();
        for raw in &ids {
            let page = self
                .document
                .page(page_id)
                .ok_or_else(|| missing("page", page_id))?;
            let entity_id = resolve_entity_id(page, raw)?;
            parsed_ids.push(entity_id);
            if let Some(entity) = page.get(entity_id).cloned() {
                changes.push(DocumentChange::RemoveEntity { page_id, entity });
            }
        }
        if !changes.is_empty() {
            self.commit(changes)?;
        }
        for id in parsed_ids {
            self.selection.remove(&id);
        }
        Ok(())
    }

    // -- Dimensions ---------------------------------------------------------

    #[wasm_bindgen(js_name = createDimension)]
    pub fn create_dimension(
        &mut self,
        kind: WebDimensionKind,
        target_ids: Vec<String>,
        value: f64,
    ) -> Result<String, WebSessionError> {
        self.submit(
            CommandAction::Dimension,
            craftloop_command::CommandNamespace::Sketch,
            "Created dimension",
        )?;
        let target = match target_ids.as_slice() {
            [a] => DimensionTarget::Single(parse_id::<PrimitiveId>(a)?),
            [a, b] => {
                DimensionTarget::Pair(parse_id::<PrimitiveId>(a)?, parse_id::<PrimitiveId>(b)?)
            }
            _ => {
                return Err(WebSessionError::Domain {
                    detail: "a dimension needs exactly 1 or 2 target primitive ids".to_string(),
                })
            }
        };
        let dimension = SemanticDimension::new(
            DimensionId::new(),
            kind.into(),
            DimensionRole::Driving,
            target,
            value,
        )?;
        let id = dimension.id;
        let page_id = self.active_page()?;
        self.commit(vec![
            DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Dimension(dimension.clone()),
            },
            DocumentChange::SetDimension {
                id,
                previous: None,
                new: Some(dimension),
            },
        ])?;
        Ok(id.to_string())
    }

    #[wasm_bindgen(js_name = editDimension)]
    pub fn edit_dimension(&mut self, id: &str, new_value: f64) -> Result<(), WebSessionError> {
        let dimension_id: DimensionId = parse_id(id)?;
        self.submit(
            CommandAction::Dimension,
            craftloop_command::CommandNamespace::Sketch,
            "Edited dimension",
        )?;

        let mut scratch = self.document.dimension_store().clone();
        match scratch.edit_driving_value(dimension_id, new_value) {
            Ok(_previous_value) => {
                let previous_dimension = self
                    .document
                    .dimension_store()
                    .dimension(dimension_id)
                    .cloned();
                let new_dimension = scratch.dimension(dimension_id).cloned();
                let page_id = self.active_page()?;
                let mut changes = vec![DocumentChange::SetDimension {
                    id: dimension_id,
                    previous: previous_dimension.clone(),
                    new: new_dimension.clone(),
                }];
                if let Some(previous_entity) = previous_dimension.clone() {
                    changes.push(DocumentChange::RemoveEntity {
                        page_id,
                        entity: SemanticEntity::Dimension(previous_entity),
                    });
                }
                if let Some(new_entity) = new_dimension {
                    changes.push(DocumentChange::InsertEntity {
                        page_id,
                        entity: SemanticEntity::Dimension(new_entity),
                    });
                }
                self.commit(changes)?;
                Ok(())
            }
            Err(err) => {
                let existing_value = self
                    .document
                    .dimension_store()
                    .dimension(dimension_id)
                    .map(|d| d.value());
                let conflict = Conflict {
                    id: ConflictId::new(),
                    kind: ConflictKind::DimensionConstraintMismatch,
                    severity: Severity::Error,
                    affected_entities: vec![format!("{dimension_id:?}")],
                    existing_truth: format!("value = {existing_value:?}"),
                    proposed_truth: format!("value = {new_value}"),
                    evidence: err.to_string(),
                    resolution_choices: vec![
                        ResolutionChoice::KeepExisting,
                        ResolutionChoice::Cancel,
                    ],
                    status: ConflictStatus::Unresolved,
                };
                let page_id = self.active_page()?;
                self.commit(vec![DocumentChange::InsertEntity {
                    page_id,
                    entity: SemanticEntity::Conflict(conflict),
                }])?;
                Err(WebSessionError::from(err))
            }
        }
    }

    // -- Constraints ----------------------------------------------------------

    /// `kind_json`: a JSON-encoded [`WebConstraintKind`].
    #[wasm_bindgen(js_name = applyConstraint)]
    pub fn apply_constraint(&mut self, kind_json: &str) -> Result<String, WebSessionError> {
        let kind: WebConstraintKind = serde_json::from_str(kind_json).map_err(json_err)?;
        let domain_kind = kind.into_domain()?;
        let constraint_id = ConstraintId::new();

        let mut scratch = self.document.sketch().clone();
        let outcome = scratch.add_constraint(
            constraint_id,
            domain_kind,
            ConstraintProvenance::UserCreated,
        )?;
        let result = match outcome {
            ConstraintOutcome::Added => {
                self.commit(vec![DocumentChange::SetSketchConstraint {
                    id: constraint_id,
                    previous: None,
                    new: Some((domain_kind, ConstraintProvenance::UserCreated)),
                }])?;
                WebConstraintOutcome::Added {
                    constraint_id: constraint_id.to_string(),
                }
            }
            ConstraintOutcome::Redundant { existing } => WebConstraintOutcome::Redundant {
                existing_constraint_id: existing.to_string(),
            },
        };
        to_json(&result)
    }

    #[wasm_bindgen(js_name = removeConstraint)]
    pub fn remove_constraint(&mut self, id: &str) -> Result<(), WebSessionError> {
        let constraint_id: ConstraintId = parse_id(id)?;
        let (kind, provenance) = self
            .document
            .sketch()
            .constraint(constraint_id)
            .map(|(k, p)| (*k, *p))
            .ok_or_else(|| missing("constraint", id))?;
        self.commit(vec![DocumentChange::SetSketchConstraint {
            id: constraint_id,
            previous: Some((kind, provenance)),
            new: None,
        }])?;
        Ok(())
    }

    #[wasm_bindgen(js_name = solveConstraints)]
    pub fn solve_constraints(&mut self) -> Result<String, WebSessionError> {
        let mut scratch = self.document.sketch().clone();
        let mut solver = EzpzSolver::new();
        let result = scratch.solve(&mut solver);

        if result.status == SolveStatus::Failed {
            return to_json(&WebSolveOutcome {
                status: result.status.into(),
                unsatisfied_constraint_ids: Vec::new(),
                updated_primitive_count: 0,
            });
        }

        let page_id = self.active_page()?;
        let primitive_ids: Vec<PrimitiveId> = self.document.sketch().primitive_ids().collect();
        let mut changes = Vec::new();
        let mut updated_count = 0u32;
        for id in primitive_ids {
            let before = self.document.sketch().primitive(id).cloned();
            let after = scratch.primitive(id).cloned();
            if before == after {
                continue;
            }
            if let (Some(before_val), Some(after_val)) = (before, after) {
                changes.push(DocumentChange::RemoveSketchPrimitive {
                    id,
                    beautified: before_val.clone(),
                });
                changes.push(DocumentChange::InsertSketchPrimitive {
                    id,
                    beautified: after_val.clone(),
                });
                if let Some(SemanticEntity::Primitive { .. }) = self
                    .document
                    .page(page_id)
                    .and_then(|p| p.get(EntityId::Primitive(id)))
                {
                    changes.push(DocumentChange::RemoveEntity {
                        page_id,
                        entity: SemanticEntity::Primitive {
                            id,
                            beautified: before_val,
                        },
                    });
                    changes.push(DocumentChange::InsertEntity {
                        page_id,
                        entity: SemanticEntity::Primitive {
                            id,
                            beautified: after_val,
                        },
                    });
                }
                updated_count += 1;
            }
        }
        if !changes.is_empty() {
            self.commit(changes)?;
        }

        to_json(&WebSolveOutcome {
            status: result.status.into(),
            unsatisfied_constraint_ids: result
                .unsatisfied
                .iter()
                .map(|d| d.constraint_id.to_string())
                .collect(),
            updated_primitive_count: updated_count,
        })
    }

    // -- View Identity / Orthographic ----------------------------------------

    #[wasm_bindgen(js_name = assignViewIdentity)]
    pub fn assign_view_identity(
        &mut self,
        view_id: Option<String>,
        identity: WebPrincipalViewIdentity,
    ) -> Result<String, WebSessionError> {
        self.submit(
            CommandAction::LabelView,
            craftloop_command::CommandNamespace::Orthographic,
            "Assigned view identity",
        )?;
        let (id, previous) = match view_id {
            Some(raw) => {
                let id: ViewId = parse_id(&raw)?;
                let previous = self
                    .document
                    .view_block(id)
                    .cloned()
                    .ok_or_else(|| missing("view", id))?;
                (id, Some(previous))
            }
            None => (ViewId::new(), None),
        };
        let mut block = previous.clone().unwrap_or_else(|| ViewBlock::new(id));
        block.set_identity(identity.into());
        self.commit(vec![DocumentChange::SetViewBlock {
            id,
            previous,
            new: Some(block),
        }])?;
        Ok(id.to_string())
    }

    #[wasm_bindgen(js_name = addGeometryToView)]
    pub fn add_geometry_to_view(
        &mut self,
        view_id: &str,
        primitive_ids: Vec<String>,
    ) -> Result<(), WebSessionError> {
        let view_id: ViewId = parse_id(view_id)?;
        let previous = self
            .document
            .view_block(view_id)
            .cloned()
            .ok_or_else(|| missing("view", view_id))?;
        let mut updated = previous.clone();
        for raw in &primitive_ids {
            updated
                .geometry_members
                .insert(parse_id::<PrimitiveId>(raw)?);
        }
        self.commit(vec![DocumentChange::SetViewBlock {
            id: view_id,
            previous: Some(previous),
            new: Some(updated),
        }])?;
        Ok(())
    }

    #[wasm_bindgen(js_name = enterOrthographic)]
    pub fn enter_orthographic(&mut self, view_id: &str) -> Result<Vec<String>, WebSessionError> {
        let view_id: ViewId = parse_id(view_id)?;
        let source = self
            .document
            .view_block(view_id)
            .cloned()
            .ok_or_else(|| missing("view", view_id))?;

        let mut geometry = std::collections::BTreeMap::new();
        if let Some(page_id) = self.document.active_page() {
            if let Some(page) = self.document.page(page_id) {
                for entity in page.entities() {
                    if let SemanticEntity::Primitive { id, beautified } = entity {
                        if source.geometry_members.contains(id) {
                            geometry.insert(*id, beautified.primitive.clone());
                        }
                    }
                }
            }
        }
        let (readiness, issues) = evaluate_readiness(&source, &geometry);
        if readiness < OrthographicReadiness::LinkReady {
            let blockers: Vec<String> = issues
                .iter()
                .filter(|issue| issue.is_blocker())
                .map(|issue| format!("{issue:?}"))
                .collect();
            return Err(WebSessionError::Domain {
                detail: format!("view is not link-ready yet: {blockers:?}"),
            });
        }

        let command = self.low_risk_command(
            CommandAction::Orthographic,
            craftloop_command::CommandNamespace::Notebook,
            "Entered Orthographic mode",
        );
        self.command_bus.submit(command.clone(), None)?;
        let layouts =
            transition_to_orthographic(&command, &source, ProjectionConvention::ThirdAngle, 100.0)?;

        let mut set = OrthographicSet::new(OrthographicSetId::new());
        set.add_view(&source, &[])?;
        let mut all_views = vec![source];
        let mut new_ids = Vec::new();
        let mut changes = Vec::new();
        for (identity, layout) in &layouts {
            let mut derived = ViewBlock::new(ViewId::new());
            derived.set_identity(*identity);
            derived.move_to(*layout);
            let existing_refs: Vec<&ViewBlock> = all_views.iter().collect();
            set.add_view(&derived, &existing_refs)?;
            new_ids.push(derived.id.to_string());
            changes.push(DocumentChange::SetViewBlock {
                id: derived.id,
                previous: None,
                new: Some(derived.clone()),
            });
            all_views.push(derived);
        }
        changes.push(DocumentChange::SetOrthographicSet {
            id: set.id,
            previous: None,
            new: Some(set),
        });
        self.commit(changes)?;
        Ok(new_ids)
    }

    #[wasm_bindgen(js_name = propagateSharedValue)]
    pub fn propagate_shared_value(
        &mut self,
        view_id: &str,
        axis: WebSharedAxis,
        value: f64,
    ) -> Result<String, WebSessionError> {
        let view_id: ViewId = parse_id(view_id)?;
        let axis: SharedAxis = axis.into();
        let view = self
            .document
            .view_block(view_id)
            .cloned()
            .ok_or_else(|| missing("view", view_id))?;

        self.submit(
            CommandAction::Link,
            craftloop_command::CommandNamespace::Orthographic,
            "Propagated shared dimension",
        )?;

        if let Some(conflict) = propose_shared_value(
            self.document.dimension_store(),
            self.document.multiview_graph(),
            &view,
            axis,
            value,
        ) {
            let conflict_id = conflict.id;
            self.pending_shared_value_proposals
                .insert(conflict_id, (view_id, axis, value));
            let page_id = self.active_page()?;
            self.commit(vec![DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Conflict(conflict),
            }])?;
            return to_json(&WebPropagateOutcome::Conflict {
                conflict_id: conflict_id.to_string(),
            });
        }

        if let Some(existing_dimension) = self.document.multiview_graph().axis_of(view_id, axis) {
            let affected: Vec<String> = self
                .document
                .multiview_graph()
                .affected_views(existing_dimension)
                .into_iter()
                .map(|id| id.to_string())
                .collect();
            return to_json(&WebPropagateOutcome::Propagated {
                affected_views: affected,
            });
        }

        let containing_set = self
            .document
            .orthographic_sets()
            .find(|set| set.views().contains(&view_id))
            .cloned();
        let mut reuse_dimension = None;
        if let Some(set) = &containing_set {
            for other_id in set.views() {
                if *other_id == view_id {
                    continue;
                }
                if let Some(existing) = self.document.multiview_graph().axis_of(*other_id, axis) {
                    reuse_dimension = Some(existing);
                    break;
                }
            }
        }

        let mut scratch_store = self.document.dimension_store().clone();
        let mut scratch_graph = self.document.multiview_graph().clone();
        let previous_dimension_snapshot;
        let dimension_id;
        match reuse_dimension {
            Some(existing_id) => {
                previous_dimension_snapshot = self
                    .document
                    .dimension_store()
                    .dimension(existing_id)
                    .cloned();
                propagate_confirmed_value(&mut scratch_store, &scratch_graph, existing_id, value)?;
                dimension_id = existing_id;
            }
            None => {
                previous_dimension_snapshot = None;
                let dimension = SemanticDimension::new(
                    DimensionId::new(),
                    craftloop_dimension::DimensionKind::Linear,
                    DimensionRole::Driving,
                    DimensionTarget::Single(PrimitiveId::new()),
                    value,
                )?;
                dimension_id = dimension.id;
                scratch_store.set_dimension(dimension);
            }
        }
        scratch_graph.bind_axis(&view, axis, dimension_id)?;

        let mut affected_views = vec![view_id];
        let mut binding_changes = vec![DocumentChange::SetMultiviewBinding {
            view: view_id,
            axis,
            previous: None,
            new: Some(dimension_id),
        }];
        if let Some(set) = &containing_set {
            for other_id in set.views() {
                if *other_id == view_id || scratch_graph.axis_of(*other_id, axis).is_some() {
                    continue;
                }
                if let Some(other_block) = self.document.view_block(*other_id) {
                    let consumes = other_block
                        .identity()
                        .map(|identity| axes_for_identity(identity).contains(&axis))
                        .unwrap_or(false);
                    if consumes {
                        scratch_graph.bind_axis(other_block, axis, dimension_id)?;
                        binding_changes.push(DocumentChange::SetMultiviewBinding {
                            view: *other_id,
                            axis,
                            previous: None,
                            new: Some(dimension_id),
                        });
                        affected_views.push(*other_id);
                    }
                }
            }
        }

        let new_dimension_snapshot = scratch_store.dimension(dimension_id).cloned();
        let mut changes = vec![DocumentChange::SetDimension {
            id: dimension_id,
            previous: previous_dimension_snapshot,
            new: new_dimension_snapshot,
        }];
        changes.extend(binding_changes);
        self.commit(changes)?;

        to_json(&WebPropagateOutcome::Propagated {
            affected_views: affected_views.iter().map(|id| id.to_string()).collect(),
        })
    }

    // -- Conflicts ------------------------------------------------------------

    #[wasm_bindgen(js_name = resolveConflict)]
    pub fn resolve_conflict(
        &mut self,
        id: &str,
        choice: WebResolutionChoice,
    ) -> Result<(), WebSessionError> {
        let conflict_id: ConflictId = parse_id(id)?;
        let page_id = self.active_page()?;
        let entity_id = EntityId::Conflict(conflict_id);
        let previous_entity = self
            .document
            .page(page_id)
            .and_then(|p| p.get(entity_id))
            .cloned()
            .ok_or_else(|| missing("conflict", id))?;
        let mut conflict = match previous_entity.clone() {
            SemanticEntity::Conflict(c) => c,
            _ => unreachable!("EntityId::Conflict always maps to SemanticEntity::Conflict"),
        };

        let choice_domain: ResolutionChoice = choice.into();
        resolve(&mut conflict, choice_domain)?;

        self.submit(
            CommandAction::Resolve,
            craftloop_command::CommandNamespace::Orthographic,
            "Resolved conflict",
        )?;

        let mut changes = vec![
            DocumentChange::RemoveEntity {
                page_id,
                entity: previous_entity,
            },
            DocumentChange::InsertEntity {
                page_id,
                entity: SemanticEntity::Conflict(conflict.clone()),
            },
        ];

        if matches!(choice_domain, ResolutionChoice::ReplaceAndPropagate) {
            if let Some((view_id, axis, value)) = self
                .pending_shared_value_proposals
                .get(&conflict_id)
                .copied()
            {
                let dimension_id = self
                    .document
                    .multiview_graph()
                    .axis_of(view_id, axis)
                    .ok_or_else(|| missing("multiview binding", view_id))?;
                let mut scratch_store = self.document.dimension_store().clone();
                propagate_confirmed_value(
                    &mut scratch_store,
                    self.document.multiview_graph(),
                    dimension_id,
                    value,
                )?;
                let previous_dimension = self
                    .document
                    .dimension_store()
                    .dimension(dimension_id)
                    .cloned();
                let new_dimension = scratch_store.dimension(dimension_id).cloned();
                changes.push(DocumentChange::SetDimension {
                    id: dimension_id,
                    previous: previous_dimension,
                    new: new_dimension,
                });
            }
        }
        self.pending_shared_value_proposals.remove(&conflict_id);
        self.commit(changes)?;
        Ok(())
    }

    // -- Undo/redo --------------------------------------------------------

    pub fn undo(&mut self) -> Result<(), WebSessionError> {
        self.history.undo(&mut self.document)?;
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), WebSessionError> {
        self.history.redo(&mut self.document)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = canUndo)]
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    #[wasm_bindgen(js_name = canRedo)]
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }
}

impl Default for CraftLoopSession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn snapshot(session: &CraftLoopSession) -> Value {
        serde_json::from_str(&session.scene_snapshot()).unwrap()
    }

    #[test]
    fn a_new_session_starts_empty_with_no_undo_redo() {
        let session = CraftLoopSession::new();
        let snap = snapshot(&session);
        assert!(snap["strokes"].as_array().unwrap().is_empty());
        assert!(snap["primitives"].as_array().unwrap().is_empty());
        assert!(snap["dimensions"].as_array().unwrap().is_empty());
        assert!(snap["conflicts"].as_array().unwrap().is_empty());
        assert_eq!(snap["revision"], 0);
        assert_eq!(snap["can_undo"], false);
        assert_eq!(snap["can_redo"], false);
        assert!(!session.can_undo());
        assert!(!session.can_redo());
    }

    #[test]
    fn create_primitive_and_dimension_then_invalid_edit_creates_a_conflict_and_undo_clears_it() {
        let mut session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(0.0, 0.0, 10.0, 0.0).unwrap();
        let snap = snapshot(&session);
        assert_eq!(snap["primitives"].as_array().unwrap().len(), 1);
        assert_eq!(snap["primitives"][0]["kind"], "Line");

        let dimension_id = session
            .create_dimension(WebDimensionKind::Linear, vec![line_id.clone()], 100.0)
            .unwrap();
        let snap = snapshot(&session);
        assert_eq!(snap["dimensions"][0]["value"], 100.0);
        assert_eq!(snap["dimensions"][0]["role"], "Driving");

        let result = session.edit_dimension(&dimension_id, -5.0);
        assert!(result.is_err());
        let snap = snapshot(&session);
        assert_eq!(
            snap["dimensions"][0]["value"], 100.0,
            "invalid edit must not change the value"
        );
        assert_eq!(snap["conflicts"].as_array().unwrap().len(), 1);
        assert_eq!(snap["conflicts"][0]["unresolved"], true);
        assert_eq!(snap["conflicts"][0]["kind"], "DimensionConstraintMismatch");

        session.undo().unwrap();
        assert!(snapshot(&session)["conflicts"]
            .as_array()
            .unwrap()
            .is_empty());

        session.edit_dimension(&dimension_id, 130.0).unwrap();
        assert_eq!(snapshot(&session)["dimensions"][0]["value"], 130.0);
    }

    #[test]
    fn scene_snapshot_reports_a_real_bounding_box_per_primitive_kind() {
        let mut session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(1.0, 2.0, 5.0, 8.0).unwrap();
        let circle_id = session.create_primitive_circle(10.0, 10.0, 3.0).unwrap();

        let snap = snapshot(&session);
        let primitives = snap["primitives"].as_array().unwrap();
        let line = primitives.iter().find(|p| p["id"] == line_id).unwrap();
        assert_eq!(line["min_x"], 1.0);
        assert_eq!(line["min_y"], 2.0);
        assert_eq!(line["max_x"], 5.0);
        assert_eq!(line["max_y"], 8.0);

        let circle = primitives.iter().find(|p| p["id"] == circle_id).unwrap();
        assert_eq!(circle["min_x"], 7.0);
        assert_eq!(circle["max_x"], 13.0);
    }

    /// Execution 03, Phase 05, Task 036: the snapshot must carry enough
    /// to actually render a shape, not just its bounding box.
    #[test]
    fn primitives_expose_real_geometry_coordinates_not_just_bounds() {
        let mut session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(1.0, 2.0, 5.0, 8.0).unwrap();
        let circle_id = session.create_primitive_circle(10.0, 10.0, 3.0).unwrap();

        let snap = snapshot(&session);
        let primitives = snap["primitives"].as_array().unwrap();
        let line = primitives.iter().find(|p| p["id"] == line_id).unwrap();
        assert_eq!(line["geometry"]["Line"]["a"], json!({"x": 1.0, "y": 2.0}));
        assert_eq!(line["geometry"]["Line"]["b"], json!({"x": 5.0, "y": 8.0}));

        let circle = primitives.iter().find(|p| p["id"] == circle_id).unwrap();
        assert_eq!(
            circle["geometry"]["Circle"]["center"],
            json!({"x": 10.0, "y": 10.0})
        );
        assert_eq!(circle["geometry"]["Circle"]["radius"], 3.0);
    }

    /// Task 037: "anchors" -- which primitive(s) a dimension targets.
    #[test]
    fn dimensions_expose_target_primitive_ids() {
        let mut session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(0.0, 0.0, 10.0, 0.0).unwrap();
        session
            .create_dimension(WebDimensionKind::Linear, vec![line_id.clone()], 10.0)
            .unwrap();

        let snap = snapshot(&session);
        assert_eq!(
            snap["dimensions"][0]["target_primitive_ids"],
            json!([line_id])
        );
    }

    /// Task 038: the snapshot must list constraints (native's own
    /// `FfiSceneSnapshot` never did -- this execution's constraint
    /// visibility/badge UI needs it, so it is added here rather than
    /// left absent).
    #[test]
    fn constraints_are_listed_in_the_snapshot_with_real_primitive_ids() {
        let mut session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(0.0, 0.0, 4.0, 3.0).unwrap();
        session
            .apply_constraint(&json!({"Horizontal": {"line": line_id}}).to_string())
            .unwrap();

        let snap = snapshot(&session);
        let constraints = snap["constraints"].as_array().unwrap();
        assert_eq!(constraints.len(), 1);
        assert!(constraints[0]["label"]
            .as_str()
            .unwrap()
            .starts_with("Horizontal"));
        assert_eq!(constraints[0]["primitive_ids"], json!([line_id]));
    }

    /// Task 039: real readiness, real member ids, real shared-axis
    /// binding/unresolved state -- not a re-derived approximation.
    #[test]
    fn view_blocks_expose_readiness_and_axis_bindings() {
        let mut session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(0.0, 0.0, 100.0, 0.0).unwrap();
        let front_id = session
            .assign_view_identity(None, WebPrincipalViewIdentity::Front)
            .unwrap();
        session
            .add_geometry_to_view(&front_id, vec![line_id.clone()])
            .unwrap();

        let snap = snapshot(&session);
        let front = snap["view_blocks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["id"] == front_id)
            .unwrap();
        assert_eq!(front["geometry_member_ids"], json!([line_id]));
        assert_eq!(front["readiness"], "LinkReady");
        assert!(front["axis_bindings"].as_array().unwrap().is_empty());

        session.enter_orthographic(&front_id).unwrap();
        let snap = snapshot(&session);
        let top_id = snap["view_blocks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["identity"] == "Top")
            .unwrap()["id"]
            .as_str()
            .unwrap()
            .to_string();
        session
            .propagate_shared_value(&top_id, WebSharedAxis::Depth, 40.0)
            .unwrap();

        let snap = snapshot(&session);
        let top = snap["view_blocks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["id"] == top_id)
            .unwrap();
        let bindings = top["axis_bindings"].as_array().unwrap();
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0]["axis"], "Depth");
    }

    /// Task 041: a resolvable conflict must carry enough for a real UI
    /// (which entities, what's being proposed vs. what exists, which
    /// resolutions are legal) -- not just a kind and a bool.
    #[test]
    fn conflicts_expose_full_resolution_metadata() {
        let mut session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(0.0, 0.0, 10.0, 0.0).unwrap();
        let dimension_id = session
            .create_dimension(WebDimensionKind::Linear, vec![line_id], 10.0)
            .unwrap();
        session.edit_dimension(&dimension_id, -1.0).unwrap_err();

        let snap = snapshot(&session);
        let conflict = &snap["conflicts"][0];
        assert_eq!(conflict["kind"], "DimensionConstraintMismatch");
        assert_eq!(conflict["severity"], "Error");
        assert!(!conflict["affected_entities"].as_array().unwrap().is_empty());
        assert!(conflict["proposed_truth"].as_str().unwrap().contains("-1"));
        assert!(!conflict["allowed_resolutions"]
            .as_array()
            .unwrap()
            .is_empty());
    }

    /// Task 042: `sceneSnapshot()` returns an owned JSON string, not a
    /// handle into live state -- there is no way for the JS side to
    /// write back into the Rust document through it. Proven here by
    /// mutating the parsed value and re-reading a fresh snapshot: the
    /// real state is unaffected.
    #[test]
    fn scene_snapshot_is_a_disconnected_copy_not_a_mutable_handle() {
        let mut session = CraftLoopSession::new();
        session.create_primitive_line(0.0, 0.0, 1.0, 0.0).unwrap();

        let mut snap = snapshot(&session);
        snap["primitives"] = json!([]);
        snap["revision"] = json!(999);

        let fresh = snapshot(&session);
        assert_eq!(fresh["primitives"].as_array().unwrap().len(), 1);
        assert_ne!(fresh["revision"], 999);
    }

    #[test]
    fn apply_constraint_detects_redundancy_and_solve_moves_the_line() {
        let mut session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(0.0, 0.0, 4.0, 3.0).unwrap();

        let kind = json!({"Horizontal": {"line": line_id}}).to_string();
        let outcome: Value =
            serde_json::from_str(&session.apply_constraint(&kind).unwrap()).unwrap();
        let constraint_id = outcome["Added"]["constraint_id"]
            .as_str()
            .expect("expected Added")
            .to_string();

        let again: Value = serde_json::from_str(&session.apply_constraint(&kind).unwrap()).unwrap();
        assert!(
            again.get("Redundant").is_some(),
            "re-adding must be redundant, not an error"
        );

        let solved: Value = serde_json::from_str(&session.solve_constraints().unwrap()).unwrap();
        assert_eq!(solved["status"], "Solved");
        assert_eq!(solved["updated_primitive_count"], 1);

        session.remove_constraint(&constraint_id).unwrap();
        let dangling = "00000000-0000-0000-0000-000000000001";
        assert!(session.remove_constraint(dangling).is_err());
    }

    #[test]
    fn select_and_delete_selected_removes_entities_and_clears_selection() {
        let mut session = CraftLoopSession::new();
        let line_id = session.create_primitive_line(0.0, 0.0, 1.0, 0.0).unwrap();

        session.select(vec![line_id.clone()]).unwrap();
        assert_eq!(
            snapshot(&session)["selected_entity_ids"],
            json!([line_id.clone()])
        );

        session.clear_selection();
        assert!(snapshot(&session)["selected_entity_ids"]
            .as_array()
            .unwrap()
            .is_empty());

        session.select(vec![line_id.clone()]).unwrap();
        session.delete_selected(vec![line_id]).unwrap();
        assert!(snapshot(&session)["primitives"]
            .as_array()
            .unwrap()
            .is_empty());
        assert!(snapshot(&session)["selected_entity_ids"]
            .as_array()
            .unwrap()
            .is_empty());
    }

    #[test]
    fn select_rejects_an_id_with_no_matching_entity_on_the_active_page() {
        let mut session = CraftLoopSession::new();
        let bogus = uuid::Uuid::new_v4().to_string();
        assert!(
            session.select(vec![bogus]).is_err(),
            "a bare UUID matching no real entity must fail, not silently select nothing"
        );
    }

    #[test]
    fn to_json_then_from_json_restores_an_equal_scene() {
        let mut session = CraftLoopSession::new();
        session.create_primitive_line(0.0, 0.0, 5.0, 5.0).unwrap();

        let json = session.to_json().unwrap();
        let before = snapshot(&session);

        let reopened = CraftLoopSession::from_json(&json).unwrap();
        let after = snapshot(&reopened);
        assert_eq!(before["primitives"], after["primitives"]);
        assert_eq!(before["revision"], after["revision"]);
        assert_eq!(
            after["can_undo"], false,
            "a freshly rebuilt session starts with no undo history"
        );
    }

    /// Article 35's Golden Alpha Journey, proven through the browser
    /// session API rather than native's file-based save/open -- see
    /// native `session.rs`'s own equivalent test for the journey this
    /// mirrors.
    #[test]
    fn golden_alpha_journey_end_to_end_through_json_round_trip() {
        let mut session = CraftLoopSession::new();

        let line_id = session.create_primitive_line(0.0, 0.0, 100.0, 0.0).unwrap();
        let dimension_id = session
            .create_dimension(WebDimensionKind::Linear, vec![line_id.clone()], 100.0)
            .unwrap();
        session
            .apply_constraint(&json!({"Horizontal": {"line": line_id.clone()}}).to_string())
            .unwrap();

        assert!(session.edit_dimension(&dimension_id, -1.0).is_err());
        assert_eq!(snapshot(&session)["conflicts"].as_array().unwrap().len(), 1);
        session.undo().unwrap();
        assert!(snapshot(&session)["conflicts"]
            .as_array()
            .unwrap()
            .is_empty());

        let front_id = session
            .assign_view_identity(None, WebPrincipalViewIdentity::Front)
            .unwrap();
        session
            .add_geometry_to_view(&front_id, vec![line_id.clone()])
            .unwrap();

        let new_views = session.enter_orthographic(&front_id).unwrap();
        assert_eq!(new_views.len(), 3, "Top, Right, and Back are all created");

        let snap = snapshot(&session);
        assert_eq!(snap["orthographic_sets"].as_array().unwrap().len(), 1);
        assert_eq!(
            snap["orthographic_sets"][0]["view_ids"]
                .as_array()
                .unwrap()
                .len(),
            4
        );
        let view_blocks = snap["view_blocks"].as_array().unwrap();
        let top_id = view_blocks
            .iter()
            .find(|v| v["identity"] == "Top")
            .expect("Top must exist")["id"]
            .as_str()
            .unwrap()
            .to_string();
        let right_id = view_blocks
            .iter()
            .find(|v| v["identity"] == "Right")
            .expect("Right must exist")["id"]
            .as_str()
            .unwrap()
            .to_string();

        assert_eq!(
            snap["dimensions"].as_array().unwrap().len(),
            1,
            "only the width dimension exists so far -- depth is not invented"
        );

        let outcome: Value = serde_json::from_str(
            &session
                .propagate_shared_value(&top_id, WebSharedAxis::Depth, 40.0)
                .unwrap(),
        )
        .unwrap();
        let affected = outcome["Propagated"]["affected_views"]
            .as_array()
            .expect("expected Propagated");
        let affected: Vec<&str> = affected.iter().map(|v| v.as_str().unwrap()).collect();
        assert!(affected.contains(&top_id.as_str()));
        assert!(affected.contains(&right_id.as_str()));
        assert!(snapshot(&session)["dimensions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["value"] == 40.0));

        let conflict_outcome: Value = serde_json::from_str(
            &session
                .propagate_shared_value(&right_id, WebSharedAxis::Depth, 999.0)
                .unwrap(),
        )
        .unwrap();
        let conflict_id = conflict_outcome["Conflict"]["conflict_id"]
            .as_str()
            .expect("expected Conflict")
            .to_string();
        assert_eq!(
            snapshot(&session)["conflicts"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|c| c["unresolved"] == true)
                .count(),
            1
        );

        session
            .resolve_conflict(&conflict_id, WebResolutionChoice::ReplaceAndPropagate)
            .unwrap();
        let after_resolve = snapshot(&session);
        assert!(after_resolve["dimensions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["value"] == 999.0));
        assert!(!after_resolve["conflicts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["unresolved"] == true));

        assert!(session.can_undo());
        session.undo().unwrap();
        assert!(session.can_redo());
        session.redo().unwrap();

        let json = session.to_json().unwrap();
        let before_snapshot = snapshot(&session);
        drop(session);

        let reopened = CraftLoopSession::from_json(&json).unwrap();
        let after_snapshot = snapshot(&reopened);
        assert_eq!(before_snapshot["primitives"], after_snapshot["primitives"]);
        assert_eq!(before_snapshot["dimensions"], after_snapshot["dimensions"]);
        assert_eq!(
            before_snapshot["view_blocks"],
            after_snapshot["view_blocks"]
        );
        assert_eq!(
            before_snapshot["orthographic_sets"],
            after_snapshot["orthographic_sets"]
        );
        assert_eq!(before_snapshot["revision"], after_snapshot["revision"]);
        assert_eq!(
            after_snapshot["can_undo"], false,
            "a freshly rebuilt session starts with no undo history"
        );
    }
}
