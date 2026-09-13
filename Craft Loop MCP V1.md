# Craft Loop MCP Version 1

## Document Metadata

**Project name:** Craft Loop  
**Document name:** Craft Loop MCP Version 1  
**Document type:** Foundational Product Description and Initial Product Specification  
**Version:** 1.0  
**Document status:** Initial evolving foundation  
**Creation date:** 2026-09-13  
**Research refresh date:** 2026-09-13  
**Primary platforms:** iPad and Android tablets  
**Primary interaction model:** Pen-native, touch-aware, keyboard-optional  
**Primary domain:** Creative two-dimensional engineering drawing, engineering note-taking, structured sketching, dimensioning, and linked orthographic views  
**Explicitly excluded from Version 1:** Three-dimensional reconstruction, three-dimensional modeling, solid modeling, feature-history modeling, computer-aided manufacturing, simulation, generative three-dimensional design, and automatic replacement of the human designer  
**Document evolution policy:** This document is intentionally extensible. Later versions may revise, replace, or deepen individual articles as implementation evidence, user research, engineering validation, standards review, and product testing reveal better solutions.

---

# Article 1 — The Product in One Sentence

Craft Loop is a tablet-first creative engineering notebook that lets people think, sketch, write, dimension, annotate, and construct linked orthographic drawings with the natural immediacy of paper while an invisible engineering layer continuously converts selected ink into precise geometry, validates dimensional relationships, preserves design intent, and keeps multiple views logically consistent.

Craft Loop is deliberately not a reduced version of traditional computer-aided design software. It is not an attempt to make Fusion 360 smaller, to reproduce Shapr3D without three-dimensional modeling, or to place engineering toolbars on top of a generic notes application. The project begins from a different interaction premise: engineering thought often starts informally, with a hand, a pen, a rough line, a note, a proportion, a question, or an incomplete view. Traditional engineering software frequently asks the user to formalize intent before the idea itself has stabilized. Craft Loop reverses that order. It lets the idea exist first, then allows precision to appear progressively as the user needs it.

The surface of Craft Loop should feel creative, calm, tactile, and direct. The internal system should be rigorous, relational, constraint-aware, standards-aware, and mathematically consistent. This duality is a defining property of the product, not a visual theme. The user should experience freedom. The system should maintain structure.

The product can therefore be summarized by three foundational statements:

**Creative on the surface. Engineering underneath.**

**Human-led design. Machine-assisted precision.**

**The hand expresses intent first; formal structure follows second.**

---

# Article 2 — Why Craft Loop Exists

Engineering drawing has a paradox. The output is visual, but many of the tools used to produce it are operationally complex. A person may understand the shape they want, may be able to sketch it on paper in seconds, and may even understand the dimensional relationships, yet still face a significant software barrier before turning that idea into a clean engineering drawing. The barrier is not always a lack of engineering knowledge. Often it is the interaction model of the software.

Traditional computer-aided design applications are optimized for explicit operations, modes, commands, feature trees, panels, selection states, constraints, numerical dialogs, and formal object models. These mechanisms are valuable when building large, precise, editable models, but they impose cognitive overhead when the user is still thinking, explaining, exploring, or communicating a two-dimensional idea.

At the opposite end, conventional note-taking and illustration applications provide fluid expression but do not deeply understand engineering intent. A line is usually a line because it looks like one, not because it is semantically an edge whose length constrains another view. A number written beside a drawing is usually text or ink, not a dimensional variable that participates in geometric consistency. A front view and a top view can sit beside each other visually without the application understanding that they share the same width.

Craft Loop exists to occupy the space between those categories. It treats drawing as a human act and engineering as an invisible computational service. The user is allowed to remain in a creative flow while the system incrementally builds structured meaning.

This is not merely a usability improvement. It creates a different product category. Craft Loop should be understood as a **Creative Engineering Notebook**, not as a sketching utility attached to computer-aided design and not as a notes application with rulers.

---

# Article 3 — Product Category

The closest descriptive category is **Intelligent Engineering Notebook**.

A second useful technical description is **Pen-Native Creative Engineering Workspace**.

A third, more architectural description is **Constraint-Aware Multiview Engineering Notebook**.

None of these descriptions should be treated as a final marketing slogan. They serve different audiences. A consumer-facing product description should remain simple. An engineering team needs terminology that exposes the actual system behavior.

The product category combines six domains:

1. Engineering drawing.
2. Creative digital ink.
3. Human-computer interaction.
4. Computational geometry.
5. Constraint solving and relational modeling.
6. Assistive machine intelligence.

A seventh domain must be added because it determines whether the other six succeed: **human cognition**. The product should be designed around the way ideas become formal, rather than assuming that formal structure exists before interaction begins.

The most important strategic distinction is this:

Craft Loop does not simplify computer-aided design.

Craft Loop creates a different route into precise engineering representation.

---

# Article 4 — Product Constitution

Craft Loop should be governed by a small number of principles that remain stable even as individual features change.

## The user draws; the system assists

The system must never silently assume authorship over the design. Algorithms may recognize, infer, validate, calculate, rank, and suggest. The user remains the author of engineering intent.

## Paper first, engineering underneath

The first interaction should resemble opening a notebook, not creating a formal engineering project. Precision should be available immediately but should not dominate the initial visual experience.

## Precision appears when needed

The system should not force a rough idea to become fully constrained before the user is ready. A sketch may remain intentionally incomplete.

## Never fabricate engineering certainty

Unknown information must remain unknown. Probable information may be suggested. Confirmed information may propagate. A machine-learning model must never convert uncertainty into false engineering truth merely because a visually plausible answer exists.

## Views are relationships, not separate drawings

Front, top, right, left, and back views may be displayed as separate visual objects, but the internal model should understand shared dimensions, shared alignment, corresponding features, and cross-view consistency.

## Simple by default, deep on demand

A beginner should be able to draw without learning a constraint vocabulary. A professional should be able to reach deep geometric control without being restricted by a beginner-only interface.

## Every intelligent action is reversible

Recognition, refinement, auto-constraint suggestions, view linking, and command interpretation must support undo, rejection, or revision.

## Ink is not automatically geometry

Some ink is a drawing. Some is a note. Some is a command. Some is a dimension. Some is a gesture. Some should remain expressive. The system must preserve this distinction.

## Engineering truth is semantic, not merely visual

Two values displayed in different views may represent the same underlying dimension. A line may look correct but still violate a constraint. The internal semantic model is the authority for engineering consistency.

---

# Article 5 — The Meaning of “Hand First, Type Second”

“Hand First, Type Second” is one of the defining interaction principles of Craft Loop.

It does not mean that typing is removed. It means that the user should not be forced to abandon the pen whenever a task becomes formal.

The pen should be capable of expressing geometry, dimensions, angles, notes, view labels, selections, corrections, tool changes, commands, and workflow transitions.

The keyboard remains available because there are cases where typed entry is faster, more accessible, or more accurate for a specific user. It is a secondary path, not a prerequisite.

The principle can be expanded as follows:

**Create by hand.  
Annotate by hand.  
Dimension by hand.  
Navigate by hand.  
Change tools by hand.  
Invoke engineering workflows by hand.  
Type when typing is genuinely the better choice.**

This philosophy should affect the architecture. Handwriting cannot be treated as a decorative overlay. It is a first-class input channel capable of becoming semantic engineering information.

The phrase also establishes a design constraint. A feature that requires frequent switching between pen and keyboard should be challenged. The question should be: can this operation be expressed directly through ink or a discoverable gesture without making the interaction ambiguous?

---

# Article 6 — Pen-Native, Not Merely Pen-Compatible

Many applications support a stylus. Craft Loop should be designed around one.

A pen-compatible application takes an existing pointer-driven interface and permits pen input. A pen-native application makes the physical behavior of the hand part of the product language.

This affects latency, hit targets, toolbar placement, hover behavior, palm rejection, pressure, tilt, gesture interpretation, haptics, left-handed layout, selection feedback, and command invocation.

Apple PencilKit provides a strong platform foundation for low-latency stroke capture on Apple devices. Apple PaperKit extends the platform concept by combining freeform drawing with structured markup elements such as shapes, images, and text. On Android, the Jetpack Ink application programming interface provides low-latency freehand drawing and exposes stroke inputs including position, timestamps, pressure, tilt, and orientation. These platform capabilities confirm that Craft Loop does not need to invent a low-level ink renderer from nothing on each platform. The product should instead focus its proprietary effort on semantic interpretation and engineering behavior.

The likely architectural direction is therefore native or near-native ink capture at the platform boundary, with a shared engineering core for geometry, constraints, dimensions, relationships, and document semantics.

This separation preserves high-quality stylus behavior while avoiding duplication of the engineering model.

---

# Article 7 — Inspiration from Apple Notes

Apple Notes is important to Craft Loop because it demonstrates how drawing and handwriting can coexist with an interface that does not demand formal setup.

The relevant inspiration is not visual imitation. The deeper lesson is immediacy. A user can enter a note and begin writing or drawing without first defining a document type, coordinate system, feature tree, sketch plane, or drawing standard.

Apple Notes also demonstrates real-time handwriting refinement while attempting to preserve the character of the user’s handwriting. That is especially relevant to Craft Loop because the product should often transform without appearing to replace the user’s work.

Another useful lesson is Smart Selection and handwriting transcription. Ink can remain visually handwritten while still becoming searchable, selectable, movable, and convertible. Craft Loop should extend this concept into engineering semantics. A handwritten “30” can remain ink until context makes it a dimension. A handwritten “Front” can become a view identity. A handwritten note should remain a note.

The design principle derived from Apple Notes is:

**Start with a page, not with a procedure.**

Craft Loop should preserve that feeling even when the internal system becomes significantly more complex.

---

# Article 8 — Inspiration from Procreate

Procreate contributes a different lesson: a professional application can hide considerable power behind a minimal canvas-first interface.

Its QuickShape behavior is especially relevant. A user draws a stroke or shape and holds the pen momentarily, after which the form can snap into a more precise line, arc, polyline, ellipse, triangle, or quadrilateral. The important principle is not the exact gesture. It is that formalization happens in place. The user does not leave the canvas, open a conversion dialog, choose a geometry type, confirm a modal window, and return.

Craft Loop should adopt the same interaction philosophy for engineering refinement. A rough line should be able to become a precise line through a controlled morph. A rough circle should become a precise circle. A hand-drawn rectangle may become orthogonal geometry if the user’s intent is sufficiently clear.

Procreate’s QuickMenu also demonstrates how repeated tool access can become muscle memory. Craft Loop should allow experts to become faster without creating a separate professional mode. Written commands, short prefixes, contextual gestures, and configurable quick actions can gradually replace slower toolbar interactions for users who choose to learn them.

The design principle derived from Procreate is:

**The canvas should remain the primary interface, and precision should happen where the hand already is.**

---

# Article 9 — Inspiration from Concepts

Concepts is a critical competitive reference because it already combines infinite canvas behavior, editable vector strokes, stylus pressure and tilt, smoothing, grids, snapping, real-world measurement, drawing scale, shape guides, and precision tools.

This means Craft Loop cannot claim differentiation merely by offering a beautiful vector notebook with measurement.

The relevant inspiration from Concepts is spatial freedom and editable ink. A stroke should not become a dead raster mark. The user should be able to select, move, refine, rescale, and reinterpret appropriate content without redrawing everything.

Concepts also demonstrates that real-world measurement can live within a creative sketching environment. This validates the broader market assumption that users are willing to mix expressive drawing and precision.

Craft Loop should, however, diverge from Concepts in a deliberate way. Concepts exposes many precision capabilities through a visible Precision panel. Craft Loop should move more of that precision into contextual behavior and semantic interpretation. The goal is not to remove control, but to reduce the number of times the user must explicitly operate the precision machinery.

The design principle derived from Concepts is:

**Preserve spatial freedom and editable vectors, but make engineering meaning deeper than measurement alone.**

---

# Article 10 — Inspiration from Shapr3D

Shapr3D is important not because Craft Loop should look like it, but because Shapr3D demonstrates strong direct manipulation and constraint-aware engineering interaction on tablets.

Constraints in Shapr3D encode relationships such as horizontal alignment, vertical alignment, tangency, equality, and other geometric rules. These relationships preserve design intent as geometry changes. This validates one of the core internal principles of Craft Loop: a drawing becomes significantly more useful when its meaning is relational rather than merely visual.

Shapr3D’s adaptive interface is equally important. Tools can be surfaced based on what is selected rather than displaying every possible operation continuously.

Craft Loop should take this principle further. The toolbar should follow the thought. When the user is writing notes, engineering tools should not dominate. When a circle is selected, circle-relevant operations may appear. When the user enters a sketch context, the toolbar can transform into a smaller set of sketch-specific tools. When orthographic relationships are active, view-specific actions can appear.

The design principle derived from Shapr3D is:

**Show the next relevant capability, not the entire engineering system.**

---

# Article 11 — Inspiration from Microsoft Journal

Microsoft Journal is a useful research reference because it was explicitly designed around an ink-first experience and reducing tool switching.

Journal’s Instant Lasso and scratch-out interactions show how gestures can replace explicit mode changes. It also uses timing to disambiguate behavior: content encircled by the lasso must already have existed long enough to distinguish selection from simply drawing a new circle.

This is directly relevant to Craft Loop’s proposed “write the intent, circle to commit” command language.

A circle cannot globally mean “execute command,” because a circle may be geometry, a note annotation, a selection gesture, or a real drawing element. Craft Loop therefore requires temporal and semantic disambiguation. A recently written command word that matches the command vocabulary, immediately enclosed by a new circle, is a strong command candidate. A circle around older content is more likely a selection. A circle drawn inside an engineering sketch may be geometry.

Journal also highlights left-handed configuration and tactile feedback. Craft Loop should treat those concerns as first-class tablet interaction requirements rather than accessibility afterthoughts.

The design principle derived from Journal is:

**Reduce mode switching through natural ink behavior, but never rely on ambiguous gestures without contextual evidence.**

---

# Article 12 — Craft Loop’s Distinctive Identity

Craft Loop should not appear to be Concepts combined with Shapr3D and Procreate.

Those applications should inform different layers of the experience.

Procreate contributes creative flow.

Concepts contributes spatial freedom and editable vector thinking.

Shapr3D contributes geometric discipline and design-intent reasoning.

Apple Notes contributes immediate note-taking simplicity.

Microsoft Journal contributes pen-first command reduction and gesture thinking.

Craft Loop must contribute something the references do not collectively provide:

**semantic engineering intelligence across the notebook.**

The system should understand that a number is not merely text, that two view dimensions may represent the same underlying quantity, that a rough line can become a constrained edge, that an orthographic view can contain known and unknown information simultaneously, and that the user may wish to move between informal and formal representation without changing applications.

The proprietary center of gravity should therefore be:

**Freehand Intent → Structured Geometry → Constraint Resolution → Dimension Semantics → Cross-View Correspondence → Orthographic Consistency → Human-Controlled Intelligent Assistance.**

---

# Article 13 — The Creative Canvas

The canvas should feel more like a notebook or illustration surface than a drafting sheet that immediately demands formal structure.

A single page may contain engineering geometry, handwritten notes, arrows, dimensions, freehand conceptual sketches, view labels, alternative designs, questions, calculations, annotations, and reference images.

The application must not assume that everything on the page belongs to one engineering object.

This creates the need for semantic grouping without forcing explicit layers on every user.

A “view block” may behave like a creative object placed on the page while internally owning structured geometry. A handwritten note may sit beside it without being interpreted as geometry. A dimension annotation may be visually close to an edge while semantically bound to a particular geometric relation.

The page can therefore be thought of as **semantic paper** rather than merely an infinite canvas.

Spatial freedom remains important, but spatial position alone cannot define engineering meaning.

---

# Article 14 — Semantic Paper

Semantic Paper is the internal model that allows a visually simple page to contain heterogeneous objects with different behavior.

A page may contain:

- Raw ink.
- Refined engineering geometry.
- Handwritten text.
- Typed text.
- Dimension annotations.
- Semantic dimensions.
- View labels.
- Orthographic view containers.
- Arrows and callouts.
- Selection marks.
- Ephemeral commands.
- Reference imagery.
- Engineering symbols.
- Generated guides.
- Unresolved geometry indicators.
- Conflict indicators.

The user should not need to manage all of these as formal object classes.

The document engine must.

This distinction is central. If everything is stored only as strokes, the system loses semantics. If everything is immediately forced into engineering entities, the user loses creative freedom.

Semantic Paper allows both.

---

# Article 15 — Raw Ink and Structured Geometry Must Coexist

The product should preserve original ink long enough to support reversibility, reinterpretation, and natural visual transitions.

When a rough line becomes a precise line, the system should not behave as though the original human action never happened.

There are several useful reasons to preserve the ink history:

1. The user may undo the refinement.
2. Recognition may later improve and reinterpret the stroke.
3. A machine-learning engine may need the original timing, velocity, pressure, or curvature.
4. The transition animation can morph from the human stroke to the structured entity.
5. The user may prefer to keep the freehand version.

This does not mean every raw point must be retained forever in an unbounded form. The document architecture should define retention, compression, and history policies.

The user-facing principle is simple:

**The system refines the user’s work; it does not pretend to have authored it.**

---

# Article 16 — Progressive Engineering Formalization

One of the most important concepts in Craft Loop is **Progressive Engineering Formalization**.

Ideas do not always begin as formally constrained geometry. The product should allow the following evolution:

Idea → loose stroke → recognized form → refined primitive → dimensioned geometry → constrained geometry → linked orthographic geometry.

Each transition should be optional.

A rough conceptual sketch may never need dimensions.

A precise view may need several dimensions but remain intentionally under-constrained.

A final engineering view may become fully constrained.

The application should not define success as “everything fully constrained.” Success means the representation is appropriate for the user’s current purpose.

This model also prevents a common failure in engineering applications: forcing premature precision. Premature precision can interrupt exploration. Craft Loop should let precision arrive at the pace of thought.

---

# Article 17 — Ink Intent Classification

The application needs an **Ink Intent Engine** because the same physical act—drawing with a pen—can express multiple semantic categories.

The engine should evaluate whether recent strokes are likely to represent:

- Freehand geometry.
- A precise geometry candidate.
- Handwritten text.
- A numeric dimension.
- An engineering symbol.
- A view label.
- An application command.
- A selection gesture.
- An erase gesture.
- A general annotation.
- A decorative mark.
- A note that must remain untouched.

Classification should use more than appearance.

Useful signals include:

- Stroke shape.
- Stroke order.
- Time between strokes.
- Pressure.
- Tilt.
- Location.
- Proximity to geometry.
- Current selection.
- Current tool context.
- Nearby dimension guides.
- Nearby view labels.
- Whether the text matches known commands.
- Whether the strokes are newly created or older.
- Whether the user encircles the strokes immediately.

The engine should return confidence and alternatives rather than forcing a binary decision whenever ambiguity is high.

---

# Article 18 — Primitive Recognition

The Primitive Recognition Engine interprets eligible ink as candidate geometric primitives.

Version 1 should focus on a deliberately limited set:

- Point.
- Line segment.
- Polyline.
- Arc.
- Circle.
- Ellipse.
- Rectangle.
- Possibly regular polygon only if interaction remains simple.

Recognition does not necessarily require deep learning in the first implementation.

Many shapes can be detected using geometric fitting, curvature analysis, endpoint analysis, stroke direction, residual error, and simple classification.

Machine learning may later improve recognition for noisy, stylized, or ambiguous strokes.

The important separation is architectural:

Recognition proposes a primitive.

The geometric engine validates and represents it.

The user can accept, reject, or continue drawing.

---

# Article 19 — Beautification and Ink Morphing

Craft Loop should not use a crude “delete handwriting, redraw perfect object” transition.

The visual system should support **Ink Morphing**.

When a stroke is refined, the user should perceive continuity between the hand movement and the final engineering form.

A line can gradually settle onto a fitted segment.

A circle can subtly regularize.

A rectangle can straighten while preserving its intended footprint.

This transition should be fast enough not to interrupt work and clear enough that the user knows something changed.

The system should provide visual reversibility. If refinement is uncertain, a ghosted candidate can appear before commitment.

This visual language is one of the places where the “Adobe engineering style” can become real: computational rigor delivered through elegant interaction rather than engineering dialogs.

---

# Article 20 — Engineering Handwriting

Handwriting is a first-class engineering input.

Craft Loop should support handwritten values such as:

- 30
- 45°
- Ø20
- R10
- 30 ± 0.1

The handwriting recognition layer should not directly apply a value to geometry merely because it recognizes characters.

The result must pass through an **Engineering Handwriting Parser** and a **Dimension Association Engine**.

Recognition asks:

“What characters were written?”

Engineering parsing asks:

“What do those characters mean in engineering context?”

Association asks:

“To what does this value belong?”

Only after these stages should the dimension engine change the design.

This separation is essential for reliability.


# Article 21 — Semantic Dimensions and Visible Dimension Annotations

Craft Loop must distinguish between a dimension as an engineering fact and a dimension as something visibly printed on the page.

Suppose the width of an object is 100 millimeters.

The internal drawing may contain a semantic parameter:

`Width = 100 millimeters`

The user may choose to show that value in the front view but not in the top view.

The top view still knows the width is 100 millimeters because it refers to the same semantic quantity.

This distinction prevents duplicated and contradictory data.

It also allows clean drawings. A professional drawing should not need to repeat every shared value in every view merely because the geometry knows it.

The internal architecture should therefore separate:

**Semantic dimension:** the actual dimensional relationship in the model.

**Visible dimension annotation:** the graphic representation of that relationship on a particular view.

This is especially important once orthographic views become linked.

---

# Article 22 — Dimension Association

Recognizing the handwritten number “30” is insufficient.

The system must determine what the number refers to.

The Dimension Association Engine should consider:

- The geometry currently selected.
- The nearest eligible edge.
- The nearest dimension guide.
- The direction of the dimension.
- Whether the user just created a line, arc, or circle.
- Whether the number follows a diameter or radius symbol.
- Whether a dimension gesture preceded the writing.
- Stroke timing.
- Spatial proximity.
- Orthographic view context.
- Existing constraints.
- Whether another dimension already owns the same relationship.

The output should not be an immediate mutation when confidence is weak.

For high confidence, the system may apply the value and provide reversible feedback.

For medium confidence, the application should show a minimal confirmation.

For low confidence, the ink should remain ordinary handwriting.

A valuable principle is:

**Recognition should be eager enough to feel intelligent and conservative enough to remain trustworthy.**

---

# Article 23 — Dimensions Are Relationships, Not Labels

A dimension is not merely a number attached to a line.

Dimensions participate in geometric relationships.

If a triangle has two sides of 30 and 50 units, the third side cannot be an arbitrary number. The triangle inequality restricts its feasible range.

If two lines are perpendicular, a 70-degree angle between them creates a contradiction.

If two circles are constrained as equal, their diameters cannot simultaneously be assigned different driving values.

If a rectangle is defined with opposite sides equal, conflicting opposite-side dimensions should trigger a consistency problem.

The dimension engine must therefore communicate with the constraint solver.

The user should not see mathematical machinery, but the system must understand it.

This principle becomes especially important across orthographic views. A width written in the front view and the same width represented in the top view are not independent numbers.

The system should represent dimensions as variables in a relational system.

---

# Article 24 — Driving, Derived, Shared, Bounded, and Reference Dimensions

Craft Loop should understand several internal dimension states.

A **driving dimension** controls geometry.

A **derived dimension** is calculated from already established relationships.

A **shared dimension** represents the same semantic quantity across multiple views or features.

A **bounded dimension** is not uniquely determined but is restricted to a feasible range.

A **reference dimension** reports geometry without controlling it.

A **conflicting dimension** contradicts the current relationship graph.

The user does not need to learn these terms during onboarding.

The system can communicate states through visual behavior.

For example:

- A driving value can appear normal.
- A derived value can appear subtly lighter.
- An unresolved value can appear as a ghost placeholder.
- A conflict can receive a restrained warning treatment.
- A reference value can remain informative without becoming editable in the same way.

These internal states are critical because they prevent every visible number from being treated as an independent command.

---

# Article 25 — Constraint Engine

The Constraint Engine is one of the central deterministic engines in Craft Loop.

It should represent relationships such as:

- Coincident.
- Horizontal.
- Vertical.
- Parallel.
- Perpendicular.
- Tangent.
- Equal.
- Concentric.
- Collinear.
- Symmetric.
- Fixed or locked, where appropriate.
- Distance.
- Angle.
- Radius.
- Diameter.

The Version 1 constraint vocabulary should remain limited enough to be understandable and testable.

Constraints should be generated from three possible sources:

1. Explicit user action.
2. Deterministic inference from clearly intentional geometry.
3. Assistive suggestion from future machine-learning systems.

The source matters.

A user-created constraint should not be silently removed because a later model prefers another interpretation.

A suggested constraint must be distinguishable internally from one explicitly accepted by the user.

---

# Article 26 — Design Intent

Design intent is the set of relationships the user expects to remain true when the drawing changes.

A drawing can be visually correct at one moment and still contain poor design intent.

For example, two holes may happen to be centered at the moment. If no symmetry or equal-spacing relationship exists, changing the width of the part may cause them to drift into an unintended arrangement.

Autodesk research on automated constraints emphasizes the importance of constraint and dimension choices that preserve editability and intended relationships. This directly supports Craft Loop’s internal separation between appearance and intent.

Craft Loop should treat design intent as a first-class semantic layer.

The system may infer likely intent, but inference must remain distinguishable from explicit user decisions.

Future intelligent assistance can become very powerful here, but it should always operate through proposals rather than silent authority.

---

# Article 27 — Geometric Consistency Engine

The Geometric Consistency Engine verifies whether current dimensions, constraints, and geometry can coexist.

Its responsibility is broader than checking individual numeric values.

It should identify:

- Impossible dimensional combinations.
- Contradictory constraints.
- Duplicate driving values that overdetermine the same relation.
- Cross-view dimension conflicts.
- Incompatible view labels.
- Invalid circle or arc conditions.
- Degenerate geometry.
- Values outside feasible geometric ranges.
- Relationships that became impossible after an edit.

The engine should return structured explanations, not merely a boolean failure.

The presentation layer can then translate those explanations into simple feedback.

For example:

Instead of “Constraint system unsolvable,” the user may see:

“300 cannot form this triangle while the other two sides remain 30 and 50.”

Instead of “Over-constrained,” the user may see:

“This width is already controlled by the front view.”

The mathematics can remain deep while the interaction remains human.

---

# Article 28 — Constraint State Engine

The Constraint State Engine continuously evaluates the current freedom of the drawing.

Useful internal states include:

- Unknown.
- Free.
- Partially constrained.
- Bounded.
- Derived.
- Shared.
- Confirmed.
- Conflicting.

This engine should support both individual geometric entities and higher-level view relationships.

It should answer questions such as:

- Is this dimension still free to change?
- Does this line already have a fixed length?
- Is this value shared with another view?
- Does this dimension have a feasible range?
- Can this geometry move without violating established intent?
- Is a user-entered value redundant?
- Is the current sketch sufficiently resolved for a requested operation?

The user should rarely see the technical state names.

The visual language should communicate certainty and freedom without turning the notebook into a constraint debugger.

---

# Article 29 — Under-Constrained Does Not Mean Wrong

Traditional parametric systems often distinguish under-constrained and fully constrained sketches.

Craft Loop should preserve the mathematical distinction while changing the emotional framing.

An under-constrained sketch is not necessarily an error.

It may represent healthy exploration.

A sketch should only be forced toward full constraint when the user’s current task benefits from it.

This is essential for a notebook product.

A user may intentionally draw a rough front view, label it, enter Orthographic mode, then add dimensions later across multiple views.

The system should support this workflow.

The rule is:

**Orthographic entry does not require complete dimensioning.**

---

# Article 30 — Orthographic View Identity

Before Craft Loop can reason reliably about orthographic relationships, it must know what a particular view represents.

The user should explicitly identify a view as one of the supported principal views, such as:

- Front.
- Top.
- Right.
- Left.
- Back.
- Bottom, if later supported.

This explicit identity is a product rule designed to avoid unnecessary machine guessing.

A user can write the identity by hand beneath or near the view.

For example:

`Front`

The Ink Intent Engine can recognize the word in the expected labeling position.

The system may convert it to a clean view label such as `FRONT`.

Typed entry remains available.

The application should not enter linked orthographic behavior if it cannot establish a source view identity.

If the user invokes Orthographic mode without a view identity, the system should ask a simple question such as:

“Which view is this?”

The product should not silently guess.

---

# Article 31 — Why “View” Is Preferred Over “Face”

In three-dimensional modeling, a face usually refers to a surface belonging to a three-dimensional body.

Craft Loop Version 1 does not operate on three-dimensional solids.

The more accurate term for a two-dimensional orthographic representation is **view**.

Product language should therefore prefer:

Front view.

Top view.

Right view.

Back view.

This terminology reduces future ambiguity if Craft Loop later integrates with three-dimensional systems.

---

# Article 32 — Entering Orthographic Mode

The user should not begin every drawing inside an orthographic workspace.

The preferred flow is:

1. Open a page.
2. Draw a view naturally.
3. Refine as much or as little as desired.
4. Add dimensions if desired.
5. Identify the view.
6. Invoke Orthographic mode when ready.

The invocation can use the toolbar or the Ink Command Language.

A user may write:

`Ortho`

or:

`Orthographic`

and encircle the command to commit it.

The system then evaluates Orthographic Readiness.

The workflow should remain reversible.

A large mode transition should generate clear feedback and support undo.

---

# Article 33 — Orthographic Readiness

Orthographic Readiness should not be a binary “fully finished or rejected” gate.

The system should evaluate whether a view is sufficiently interpretable to participate in linked multiview reasoning.

Possible internal readiness levels include:

**Draft Ready:** recognizable geometry exists.

**Identity Ready:** the source view has an explicit view identity.

**Link Ready:** enough structure exists to create orthographic relationships.

**Resolved:** critical unknowns required by current relationships have been supplied.

**Constrained:** the drawing has sufficient constraints for predictable editing.

The user does not need to see these labels unless they become useful.

A view may enter Orthographic mode before it is dimensioned.

This is a major product decision.

---

# Article 34 — Orthographic Intelligence Is Not an Image Generator

Orthographic Intelligence must not be implemented as:

Front image → generative model → plausible top image.

That would create engineering hallucination.

Craft Loop should instead reason from explicit geometry, dimensions, alignments, view identities, and shared relationships.

The engine should continuously ask:

What is known?

What is unknown?

What can be derived?

What is shared?

What conflicts?

What did the user explicitly decide?

What is merely suggested?

This is the core of trustworthy orthographic assistance.

---

# Article 35 — Multiview Constraint Graph

The internal representation of linked orthographic drawings should be a **Multiview Constraint Graph**.

This graph should contain:

- View nodes.
- Geometric entity nodes.
- Dimension variables.
- Constraint relationships.
- Cross-view correspondences.
- Shared extent relationships.
- Confidence metadata.
- User-authorship metadata.
- Suggestion states.
- Conflict states.

For simple orthographic relationships, the graph can express that:

Front uses width and height.

Top uses width and depth.

Right uses depth and height.

The system does not need a three-dimensional solid to understand that the same width appears in front and top views.

This graph becomes the shared truth of the drawing.

---

# Article 36 — No Permanent Master View

The first view can be the starting view without becoming the permanent master.

Once a multiview drawing exists, the design knowledge belongs to the drawing graph.

The user may add width in the front view.

The user may add depth later in the top view.

The user may resolve another relationship in the right view.

Each confirmed value becomes part of the shared semantic model.

This enables the principle:

**Dimension anywhere. Resolve everywhere.**

The principle does not mean that every dimension is independent.

It means the user can enter valid information where it is easiest to understand, while the system distributes that information to all relationships that legitimately depend on it.

---

# Article 37 — Shared Dimensions Across Views

Suppose:

Front view shows width and height.

Top view shows width and depth.

Right view shows depth and height.

If the user defines width as 100 millimeters in the front view, the top view shares the same semantic width.

The top view should not be allowed to establish a contradictory 130-millimeter width for the same linked geometry.

If the user attempts this, the system should explain the conflict.

This relationship is semantic, not merely visual.

The width label does not need to be visibly repeated in both views.

The graph knows the value even when only one annotation is displayed.

---

# Article 38 — Unresolved Dimensions

An unresolved dimension is not an error.

It is a known gap in the current engineering description.

If the user starts from a front view with known width and height but no depth, the system should not invent depth.

When Orthographic mode opens, the top or right view may contain a ghosted unresolved extent.

The interface should invite completion without presenting a failure state.

The user can write a depth value by hand in the top view.

The value then becomes available to the right view as well.

This is **Progressive Geometric Resolution**.

---

# Article 39 — Ambiguity as a First-Class Product Concept

Craft Loop should represent uncertainty explicitly.

Useful internal states include:

- Confirmed.
- Suggested.
- Unresolved.
- Conflicting.

The user interface can translate these states into a restrained visual language.

Confirmed geometry appears normally.

Suggested geometry appears ghosted or subtly different.

Unresolved geometry may use guides, open handles, placeholders, or incomplete extents.

Conflicting geometry receives a warning treatment that is visible but not hostile.

This makes the notebook honest.

A drawing can be incomplete without being broken.

---

# Article 40 — Cross-View Correspondence

The system must determine when an entity in one view corresponds to an entity or feature in another.

This is the role of the **Cross-View Correspondence Engine**.

Correspondence should consider:

- Shared alignment.
- Shared dimensions.
- Centerlines.
- Relative positions.
- Geometric type.
- View identity.
- Existing user links.
- Temporal workflow.
- Constraint compatibility.
- Future machine-learning confidence.

The engine should not automatically merge ambiguous entities.

When confidence is insufficient, it can show a minimal suggestion such as:

“Link these?”

The user remains the authority.

---

# Article 41 — Back and Hidden Information

The back view is an important example of why Craft Loop must separate known geometry from assumed geometry.

A front view may establish overall width and height.

It does not necessarily reveal every feature on the back.

The back view can inherit dimensions that are logically shared, while unknown back-specific details remain unresolved.

The system should never create holes, slots, or other unseen details merely because a generative model considers them plausible.

This is a central expression of the principle:

**Never fabricate engineering certainty.**

---

# Article 42 — First-Angle and Third-Angle Projection

Orthographic view placement is governed by established drawing conventions.

The International Organization for Standardization maintains standards covering projection methods, including orthographic representation, while the American Society of Mechanical Engineers maintains standards for orthographic and pictorial views.

Craft Loop should become standards-aware.

However, standards awareness should remain mostly invisible during ordinary drawing.

A document can have a projection convention such as:

- First-angle projection.
- Third-angle projection.

The default may be based on user preference, regional settings, or explicit project configuration.

The system should use the selected convention to arrange views, interpret positions, and export formal drawings.

Version 1 does not need to expose a standards control panel during every sketching session.

---

# Article 43 — Standards-Aware, Not Standards-Driven

Standards are essential for professional output, but a notebook experience should not begin by asking a novice to understand standards documents.

Craft Loop should apply a **standards-aware** philosophy.

The user draws naturally.

The system can later help normalize:

- View arrangement.
- Line conventions.
- Centerlines.
- Hidden lines.
- Dimension presentation.
- Leader lines.
- Section views in future versions.
- Drawing-sheet output in future versions.

The standards engine is therefore a background service.

Professional users can access deeper controls when required.

---

# Article 44 — Why Dashed Lines Cannot Be Reserved for Commands or Dimensions

Technical drawing standards assign meanings to different line types.

Dashed lines are commonly associated with hidden geometry and other technical conventions depending on context.

Craft Loop must therefore avoid defining “draw a dashed line” as a universal command for dimension entry.

Doing so would conflict with the language of engineering drawings.

Dimension entry should instead use contextual dimension gestures, selection, proximity, or explicit dimension commands.

This is an example of why the product must respect engineering semantics even while inventing a new interaction language.

---

# Article 45 — Dimension Gesture Language

Craft Loop should explore a dedicated **Dimension Gesture Language**.

Possible patterns include:

- Select an edge, drag outward, and write a value.
- Draw extension marks and a span, then write the value.
- Draw a small angle arc between two lines, then write the angle.
- Draw a diameter symbol or use a circle context, then write the value.
- Invoke Dimension through an ink command, then select geometry.

The best gestures must be validated through user testing.

They should be discoverable.

They should not conflict with established engineering line types.

They should not require users to memorize a large private alphabet.

The product should prioritize natural actions before hidden shortcuts.


# Article 46 — Ink Command Language

Craft Loop should support an **Ink Command Language** that lets users control the application without repeatedly travelling to a toolbar.

The user can write a command word or a recognized abbreviation and confirm it with an intentional gesture.

Examples include:

`Pen`

`Eraser`

`Sketch`

`Line`

`Circle`

`Dimension`

`Ortho`

`Orthographic`

The command system is not intended to replace the toolbar.

It is an alternative path that becomes faster as the user builds muscle memory.

A new user may tap visible tools.

An experienced user may write the full command.

A highly experienced user may use a short unique prefix.

All paths should invoke the same underlying command system.

---

# Article 47 — Write the Intent, Circle to Commit

A signature interaction proposed for Craft Loop is:

**Write the intent. Circle to commit.**

The user writes:

`Line`

and draws a circle around the newly written command.

If the command is recognized, valid in the current context, and intentionally confirmed, the application activates the line tool.

The same pattern can apply to larger workflow commands such as `Ortho`.

The circle is not inherently a command gesture.

Context determines meaning.

This distinction is necessary because a circle can also be geometry or a selection gesture.

---

# Article 48 — Temporal Disambiguation

Temporal information should help distinguish command circles from ordinary circles and lasso selections.

A recent command word immediately enclosed by a circle is a strong command candidate.

Older content enclosed by a circle is more likely to represent selection.

A circle drawn as part of an active engineering sketch is more likely to represent geometry.

Microsoft Journal demonstrates the value of temporal disambiguation by requiring Instant Lasso content to have existed on the page for a period before interpreting a surrounding circle as selection.

Craft Loop should develop its own timing rules through testing.

Time should be one signal among several, not the only rule.

---

# Article 49 — Contextual Command Grammar

Commands should not exist in one global namespace.

Their meaning depends on the current context.

A general notebook context may contain:

- Pen.
- Eraser.
- Select.
- Sketch.
- Orthographic.
- Note.

A sketch context may contain:

- Line.
- Circle.
- Arc.
- Rectangle.
- Dimension.
- Constraint-related actions.

An orthographic context may contain:

- Add View.
- Link.
- Label View.
- Resolve.
- Align.

Context reduces ambiguity.

It also allows short commands without forcing the entire application vocabulary into a single letter space.

---

# Article 50 — Shortest Unique Prefix

Craft Loop should support the **Shortest Unique Prefix** rule.

A full command name always works.

A shortened prefix works only when it uniquely identifies one valid command in the current context.

If `L` can mean both Line and Linear Dimension, `L` is ambiguous.

The application should not guess.

It can request another character or show two small candidate choices.

If `Li` uniquely identifies Line, `Li` can execute.

This creates a progressive expert workflow without sacrificing safety.

---

# Article 51 — Personal Command Vocabulary

A future version may allow users to customize command aliases.

Examples:

`L` → Line.

`LD` → Linear Dimension.

`C` → Circle.

`R` → Rectangle.

Custom aliases should remain explicit and visible in settings.

The system should never silently change command meaning based only on learned behavior.

Machine learning may recommend a shortcut, but the user should approve the vocabulary.

This protects muscle memory.

---

# Article 52 — Ephemeral Ink

Command ink should usually be **ephemeral**.

If the user writes `Line`, circles it, and the command is executed, the written command does not need to remain permanently in the engineering note.

A possible interaction is:

1. Command ink is recognized.
2. Confirmation circle closes.
3. A brief morph or highlight confirms recognition.
4. A subtle haptic signal fires if supported.
5. The command ink disappears.
6. The selected tool becomes visibly active.

The event remains undoable even if the ink disappears.

This prevents the command language from polluting the document.

---

# Article 53 — Command Risk Levels

Not every command should have the same confirmation behavior.

**Low-risk commands** such as selecting Pen, Line, Circle, or Eraser can execute immediately after recognition.

**Medium-risk commands** such as entering Orthographic mode can execute with clear visual feedback and immediate undo.

**High-risk commands** such as deleting a page, clearing a drawing, or replacing a major view should require additional confirmation.

This prevents the elegant command language from becoming dangerous.

The same risk model should apply whether a command originates from ink, touch, keyboard, or toolbar.

---

# Article 54 — One Command Bus

Toolbar actions, ink commands, keyboard shortcuts, and gestures must converge on a shared command architecture.

The application should not implement separate business logic for:

“Line clicked from toolbar.”

“Line requested by handwriting.”

“Line requested by keyboard shortcut.”

“Line requested by future gesture.”

They should all resolve to one command such as:

`SelectTool(Line)`

This prevents behavior drift.

It also makes testing significantly easier.

---

# Article 55 — Contextual Toolbar

The toolbar should follow the user’s thought.

In a general notebook state, it may expose only a small set of tools such as Pen, Select, and Eraser.

When the user enters Sketch, it can transform into a compact sketch toolbar.

When a circle is selected, circle-relevant actions may become available.

When an orthographic view is active, view-specific tools may appear.

The goal is not to hide capability arbitrarily.

The goal is to avoid presenting irrelevant decisions.

This principle is inspired by adaptive engineering interfaces such as Shapr3D but should be expressed with the visual calm of a creative application.

---

# Article 56 — Natural Gesture Before Hidden Gesture

Craft Loop should prefer gestures that users can discover from context.

A gesture should not become mandatory if its meaning is invisible and undocumented.

The hierarchy should be:

1. Natural gesture.
2. Discoverable shortcut.
3. Optional expert gesture.

Never:

Hidden gesture required for essential work.

This principle is especially important for beginners.

A user should always be able to accomplish the same task through visible controls.

---

# Article 57 — Draw and Hold

Draw-and-hold is a strong candidate for geometry refinement.

A user draws a line and holds the pen at the end.

The application recognizes a likely line and refines it.

A user draws a circle and holds.

The system proposes a precise circle.

This behavior is inspired by the success of similar interactions in creative drawing applications, but Craft Loop must adapt it to engineering semantics.

For example, a line may also receive contextual alignment suggestions.

The user should remain able to reject the refinement.

---

# Article 58 — Haptic Feedback Language

Where supported by hardware, Craft Loop can use subtle haptic feedback to confirm interaction states.

A light response may confirm:

- Command accepted.
- Snap acquired.
- Constraint inferred.
- Dimension committed.
- Selection captured.

A different response may indicate:

- Ambiguous command.
- Invalid dimension.
- Conflict.
- Rejected snap.

Haptics should reinforce visual feedback, not replace it.

The experience must remain functional on devices without haptic stylus support.

---

# Article 59 — Left-Handed and Right-Handed Interaction

Craft Loop must treat handedness as a core tablet design concern.

Floating controls should not appear under the user’s palm.

Contextual popovers should prefer the side opposite the writing hand.

Toolbar placement should be configurable.

Command suggestions should not be hidden beneath the active hand.

The application should support both left-handed and right-handed layouts without requiring an entirely separate interface.

The Ink Engine should also respect platform palm-rejection behavior.

---

# Article 60 — Hover and Pre-Contact Feedback

Modern stylus hardware may expose hover or near-surface information.

Craft Loop can use hover carefully for:

- Showing the active tool.
- Previewing snap targets.
- Highlighting the geometry that would receive a dimension.
- Previewing command targets.
- Revealing subtle affordances.

Hover must not become necessary for core functionality because not all supported devices provide equivalent capabilities.

It is an enhancement layer.

---

# Article 61 — The Visual Language of Certainty

Craft Loop should translate mathematical state into visual state.

A user should not need a constraint debugger to know whether something is resolved.

Possible visual treatments include:

- Normal solid geometry for confirmed relationships.
- Soft ghost geometry for suggestions.
- Open-ended or lighter guides for unresolved dimensions.
- Restrained amber or warm emphasis for conflicts.
- Subtle completion animation when a dimension resolves multiple views.
- Slightly lighter typography for reference dimensions.

The system should avoid excessive colors.

The page must remain elegant enough to feel like a creative notebook.

---

# Article 62 — Adobe Engineering Style

“Adobe Engineering Style” should not be interpreted as copying Adobe’s visual assets.

It describes a product philosophy:

- Creative interaction first.
- High visual polish.
- Direct manipulation.
- Contextual tools.
- Fluid motion.
- Clear hierarchy.
- Complex capability hidden behind approachable surfaces.
- A canvas that remains the center of attention.

Craft Loop’s engineering depth should feel like power available inside a creative tool, not like a drafting system cosmetically restyled.

The design objective is emotional as well as functional:

Engineering should feel inviting.

---

# Article 63 — The Toolbar Should Disappear From Attention

A successful Craft Loop session should contain long periods where the user does not consciously think about the toolbar.

This can happen through:

- Pen commands.
- Contextual tools.
- Gestures.
- Inline dimensions.
- Draw-and-hold refinement.
- Intelligent defaults.
- Direct manipulation.

The toolbar still exists.

It provides discoverability and accessibility.

It simply should not demand continuous attention.

---

# Article 64 — Notebook Library

Version 1 should include a simple document library.

The library should support:

- Create notebook or document.
- Rename.
- Duplicate.
- Delete with recovery policy.
- Organize into folders or collections.
- Search titles.
- Search recognized handwritten text where platform and architecture permit.
- Sort by date.
- Show recent documents.
- Pin important documents if simple to support.
- Preview thumbnail.

The library should remain visually calm.

It is not a project-management system.

---

# Article 65 — Pages and Infinite Paper

Craft Loop needs an explicit decision about document topology.

A conventional notes application often uses pages.

Concepts uses an infinite canvas.

Craft Loop can support a hybrid concept:

**Finite export surfaces within a spatially flexible working canvas.**

The user can think freely without immediately committing to a paper size.

For professional export, the system can later map selected content to standard sheet sizes or page frames.

Version 1 can begin with flexible pages or generous canvases while preserving the internal ability to define export regions.

This avoids locking the entire experience to an infinite canvas or a rigid sheet too early.

---

# Article 66 — View Blocks

An orthographic view should behave like a structured block placed on semantic paper.

A View Block has:

- View identity.
- Local coordinate frame.
- Structured geometry.
- View-local annotations.
- Links to shared dimensions.
- Cross-view correspondences.
- Visual bounds.
- Optional title.
- Readiness state.

The user can move the View Block spatially without breaking its semantic identity.

Moving the block on the page does not change the engineering coordinates of the geometry inside it unless the user performs an engineering transform.

This separation prevents document layout changes from altering design truth.

---

# Article 67 — Notes Around Engineering Views

The application should intentionally support informal annotation around formal views.

A user may write:

“Make wall thicker.”

“Try aluminum.”

“Check hole spacing.”

“Alternative version.”

These notes should not be misinterpreted as dimensions or commands merely because they contain numbers or engineering vocabulary.

The Ink Intent Engine should use context.

A note region can remain expressive handwriting.

Engineering geometry and personal thinking can coexist.

This coexistence is a defining reason for using a notebook metaphor.

---

# Article 68 — Engineering Symbols

Version 1 should reserve an extensible system for engineering symbols.

Initial recognition may include:

- Degree symbol.
- Diameter symbol.
- Radius prefix.
- Plus/minus tolerance notation.

Future versions may expand toward:

- Surface finish.
- Geometric dimensioning and tolerancing symbols.
- Datum references.
- Welding symbols.
- Section indicators.

The first version should not attempt to implement the entire professional annotation universe.

The data model should avoid blocking future expansion.

---

# Article 69 — Centerlines and Construction Geometry

Engineering drawings require more than visible object edges.

Craft Loop should support centerlines and construction geometry.

These entities should be semantically different from normal edges.

They may participate in:

- Symmetry.
- Diameter placement.
- Concentric relationships.
- Orthographic alignment.
- View correspondence.

Line style must reflect engineering meaning.

The standards engine should ultimately govern exported appearance.

---

# Article 70 — Hidden Lines

Hidden lines represent geometry not directly visible in a particular view.

Version 1 may initially support manual hidden-line creation before attempting any automatic inference.

Automatic hidden-line generation normally depends on deeper geometric understanding than simple two-dimensional projection relationships.

Because Craft Loop Version 1 explicitly excludes three-dimensional reconstruction, it must be careful not to promise hidden geometry it cannot prove.

A human-in-the-loop approach is appropriate.

---

# Article 71 — The Standards Engine

The Standards Engine should maintain rules related to technical drawing representation.

Potential responsibilities include:

- Line type conventions.
- View arrangement.
- Dimension presentation.
- Leader behavior.
- Centerline representation.
- Projection convention.
- Text orientation.
- Future section and cut conventions.
- Export formatting.

The initial implementation can be limited.

The architecture should allow standards profiles.

For example:

International Organization for Standardization oriented profile.

American Society of Mechanical Engineers oriented profile.

The system should not claim formal compliance until implementation is independently validated against licensed standards where necessary.

---

# Article 72 — Unit System

Craft Loop should support explicit document units.

Initial units may include:

- Millimeters.
- Centimeters.
- Meters where relevant.
- Inches.

The user can write a bare value such as `30` and the system interprets it using document units.

If the user writes an explicit unit, such as `30 cm`, the system can normalize the value.

Unit conversion must be deterministic.

The application should prevent silent unit mismatches.

---

# Article 73 — Scale

Craft Loop must distinguish real engineering dimensions from visual zoom.

Zoom changes how large something appears on screen.

It does not change geometry.

Drawing scale is relevant for printed or exported representation.

Version 1 should maintain real semantic dimensions independently of screen zoom.

If imported references are supported, scale calibration may become important.

The Concepts application demonstrates a useful precedent for real-world scale in a creative drawing environment.

---

# Article 74 — Snap System

Snapping should assist without dominating.

Potential snap targets include:

- Endpoints.
- Midpoints.
- Centers.
- Tangency points.
- Intersections.
- Horizontal and vertical alignment.
- Existing guides.
- View correspondences.
- Grid points when grid is active.

The system should provide clear transient feedback when a snap occurs.

Snapping should be configurable.

Users should be able to draw freely when precision is not desired.

---

# Article 75 — Smart Guides

Smart Guides are temporary geometric relationships suggested during drawing or movement.

Examples:

- Horizontal alignment.
- Vertical alignment.
- Equal spacing.
- Center alignment.
- Parallel direction.
- Perpendicular direction.
- Shared orthographic coordinate.

A guide is not automatically a permanent constraint.

The user can continue moving without committing.

If the interaction indicates intention, the relationship may become a constraint or be offered as one.

This separation keeps the interface fluid.

---

# Article 76 — Grid Philosophy

The grid should be a support surface, not a visual identity.

Craft Loop may offer:

- No grid.
- Dot grid.
- Square grid.
- Engineering graph paper.
- Isometric grid for free sketching, even though Version 1 focuses on two-dimensional engineering representation.
- Custom spacing.

Grid snapping should be optional.

The page should remain readable when the grid is disabled.

---

# Article 77 — Selection

Selection must support both raw ink and structured engineering entities.

The user should be able to:

- Tap.
- Lasso.
- Multi-select.
- Select through recognized semantic groups.
- Select a View Block.
- Select a dimension.
- Select an engineering primitive.

Selection feedback should reveal the minimum information needed.

A raw handwritten note should not suddenly expose engineering handles.

A structured line can expose endpoints only when useful.

---

# Article 78 — Erasing

Erasing should understand object semantics.

Possible behaviors include:

**Stroke erasing:** remove raw ink strokes.

**Pixel-like partial erasing:** where supported for expressive ink.

**Object erasing:** remove an entire geometric primitive.

**Semantic deletion:** remove a dimension annotation while optionally preserving the underlying dimension.

These are not always equivalent.

The application should make deletion behavior predictable.

A scratch-out gesture may be available as an optional fast action.

---

# Article 79 — Undo and Redo

Undo and redo are critical because Craft Loop performs intelligent transformations.

The user must be able to undo:

- Stroke creation.
- Refinement.
- Constraint creation.
- Dimension association.
- Command execution.
- Orthographic linking.
- View creation.
- Automatic propagation.
- Conflict resolution.
- Recognition acceptance.

An intelligent action should ideally be one understandable undo step.

Internal implementation may involve several engine changes, but user history should reflect the user’s mental model.

---

# Article 80 — Event History and Authorship

The document should retain enough event metadata to distinguish:

- User-created geometry.
- System-suggested geometry.
- User-accepted suggestions.
- User-modified suggestions.
- Machine-derived dimensions.
- User-entered dimensions.
- Propagated changes.

This provenance is important for trust and future debugging.

It is also valuable for future machine learning, because accepted and rejected suggestions provide training signals.

User privacy and explicit consent must govern any use of such data for model training.


# Article 81 — Local-First Product Behavior

Craft Loop should be local-first for core drawing activity.

A user should be able to open the application, create a notebook, draw, refine geometry, enter dimensions, undo, redo, and work with orthographic relationships without depending on continuous network connectivity.

This principle has several benefits:

- Pen latency remains independent of network quality.
- A temporary outage does not interrupt engineering thought.
- User trust improves because documents remain accessible.
- The application can work in workshops, classrooms, travel, and field environments.
- Cloud infrastructure can focus on synchronization, sharing, backup, and optional intelligence rather than becoming the primary drawing engine.

Cloud features should extend the product.

They should not be required to make a line appear after the pen touches the screen.

---

# Article 82 — Autosave

The notebook should autosave continuously.

A user should not need to think about saving while sketching.

Autosave should capture:

- Document metadata.
- Raw ink.
- Structured geometry.
- Dimensions.
- Constraints.
- View identities.
- Orthographic relationships.
- Command-relevant state that must survive reopening.
- Undo checkpoints according to product policy.

The implementation should use transactional persistence or equivalent safety mechanisms so that an application crash does not corrupt the entire document.

---

# Article 83 — Version History

Version history is valuable for engineering work because changes can alter relationships across multiple views.

A future paid tier may retain longer history.

Version 1 should at least define a document revision model.

Useful future capabilities include:

- Restore previous version.
- Compare dimension changes.
- See when a view was linked.
- Identify which change introduced a conflict.

History must remain visually simple.

It should not become a source-code control interface.

---

# Article 84 — Cloud Synchronization

Cloud synchronization can support:

- Device backup.
- Multiple tablets.
- Future desktop viewing.
- Sharing.
- Collaboration in later versions.

Conflict resolution is nontrivial because documents contain semantic relationships.

A naive last-write-wins strategy can damage engineering state.

Version 1 can initially limit simultaneous editing and focus on reliable backup and sequential synchronization.

Real-time collaboration should not be introduced until the document model can reconcile semantic operations safely.

---

# Article 85 — Privacy

Engineering notebooks can contain commercially sensitive ideas.

Privacy must therefore be treated as a product feature.

Core principles should include:

- Local processing where practical.
- Clear explanation when content leaves the device.
- Explicit consent for model-improvement data.
- Encryption in transit.
- Encryption at rest for cloud documents.
- Ability to delete cloud data.
- No silent use of private engineering drawings for model training.

Future enterprise requirements may add organization controls, data residency, and managed accounts.

---

# Article 86 — On-Device Intelligence

Where platform capability permits, handwriting recognition and lightweight geometric inference should run on device.

Apple’s current PencilKit direction includes on-device handwriting recognition capabilities, while Android provides local ink and device-side machine-learning frameworks.

On-device execution is particularly attractive for:

- Handwriting recognition.
- Command recognition.
- Primitive fitting.
- Basic intent classification.
- Constraint solving.
- Dimension parsing.

Large future models may still require optional cloud processing.

The architecture should support both without making the cloud authoritative for every stroke.

---

# Article 87 — Cross-Platform Principle

Craft Loop targets iPad and Android tablets.

The product should deliver a consistent mental model across platforms without forcing identical low-level implementation.

The pen experience should use the strongest platform capabilities available.

On Apple platforms, PencilKit and PaperKit should be evaluated.

On Android, Jetpack Ink should be evaluated.

The semantic engineering core should be shared where practical.

This approach is preferable to a lowest-common-denominator input layer if that layer reduces pen quality.

---

# Article 88 — Shared Engineering Core

A shared engineering core can own:

- Vector primitive definitions.
- Constraint graph.
- Constraint solver integration.
- Dimension model.
- Unit conversion.
- Geometric consistency checks.
- Orthographic relationships.
- Command definitions.
- Document semantics.
- Serialization schema.
- Deterministic tests.

A systems language such as Rust is a plausible implementation option because it can target multiple platforms and WebAssembly if later needed.

This is a candidate architecture, not a mandatory decision.

The team should benchmark interoperability, performance, developer velocity, and solver options before committing.

---

# Article 89 — Platform-Specific Presentation Layers

The iPad application may use SwiftUI and UIKit components around PencilKit or PaperKit.

The Android application may use Jetpack Compose and Jetpack Ink.

The platform layer should own:

- Stylus events.
- Native gestures.
- Haptics.
- Platform accessibility.
- System document integration.
- Native sharing.
- Platform-specific tool picker behavior.

The shared core should not need to know the details of Apple Pencil or Android stylus hardware.

---

# Article 90 — Rendering Architecture

Rendering should separate expressive ink from structured vector geometry.

Raw ink may use platform-optimized stroke rendering.

Engineering primitives require deterministic vector rendering.

Annotations and guides require a presentation layer that can scale cleanly across zoom levels.

The renderer should support:

- High refresh rates where hardware permits.
- Stable line weight.
- Crisp geometry.
- Low-latency in-progress strokes.
- Post-stroke recognition without visible jumps.
- Efficient culling for large pages.
- Correct text scaling.
- Export-quality vector output.

The user should never feel the boundary between ink and engineering layers.

---

# Article 91 — Performance Budget

Pen latency is a product-quality metric.

The application should define explicit budgets for:

- Ink response.
- Primitive recognition.
- Constraint solving.
- Orthographic propagation.
- Autosave.
- Large-document rendering.
- Command recognition.

Operations that cannot complete within interactive time should degrade gracefully.

For example, a heavy background recognition pass should not block the live stroke.

Fast deterministic feedback should appear first.

Deeper assistance can arrive asynchronously if it does not change user intent without confirmation.

---

# Article 92 — Determinism

Core geometric behavior must be deterministic.

Given the same confirmed geometry, dimensions, units, constraints, and view relationships, Craft Loop should reach the same engineering result.

Machine-learning suggestions may be probabilistic.

Their accepted result must become deterministic once the user commits it.

This boundary is essential for engineering trust.

---

# Article 93 — Engine Boundaries

The internal system should maintain clear ownership.

The Ink Engine owns physical stroke data.

The Ink Intent Engine classifies strokes.

The Primitive Recognition Engine proposes geometry.

The Vector Geometry Engine owns confirmed geometric primitives.

The Engineering Handwriting Parser interprets recognized text.

The Dimension Association Engine identifies dimensional targets.

The Constraint Engine owns formal relationships.

The Geometric Consistency Engine validates the combined system.

The Orthographic Relationship Engine owns cross-view projection semantics.

The Multiview Constraint Graph owns shared view relationships.

The Standards Engine owns representation rules.

The Document Engine owns persistence and history.

These boundaries should prevent one engine from silently rewriting the authority of another.

---

# Article 94 — Ink Engine

The Ink Engine should capture:

- X and Y position.
- Timestamp.
- Pressure when available.
- Tilt when available.
- Azimuth or orientation when available.
- Tool identity.
- Stroke grouping.
- Pen-down and pen-up events.
- Hover where available.

It should support smoothing without destroying the raw sample history needed for interpretation.

The engine should expose completed strokes to higher-level systems.

It should avoid embedding engineering meaning directly.

---

# Article 95 — Vector Geometry Engine

The Vector Geometry Engine should provide canonical representations for supported primitives.

A line should have exact endpoints.

A circle should have exact center and radius.

An arc should have exact center, radius, start angle, and end angle or an equivalent stable representation.

A rectangle may be represented by constrained lines rather than as a permanently special object if that better supports the solver.

The internal model should favor relational editability.

Rendering data should not become the sole source of truth.

---

# Article 96 — Recognition Confidence

Every recognition event should carry confidence metadata.

For example:

`Circle candidate: 0.96`

`Ellipse candidate: 0.72`

`Raw ink: 0.44`

The user interface does not need to show numeric confidence by default.

Confidence determines behavior.

Very high confidence can allow silent preview.

Medium confidence can produce a suggestion.

Low confidence should preserve raw ink.

The system should be biased toward preservation when wrong conversion would be disruptive.

---

# Article 97 — Human Correction as Signal

When the user rejects a recognition result, the system should record the correction locally as event metadata.

When the user accepts a suggestion, that is also a signal.

These signals can improve personalization or future models if privacy policy allows.

However, the product must not force the user into correction dialogs.

Correction should happen through ordinary interactions such as undo, choosing another candidate, or keeping ink.

---

# Article 98 — Constraint Solver

Craft Loop needs a reliable two-dimensional geometric constraint solver.

The solver must support the Version 1 constraint vocabulary.

It should provide:

- Solved geometry.
- Degrees of freedom or equivalent internal state.
- Conflict detection.
- Redundancy detection where possible.
- Stable behavior during drag.
- Incremental updates.
- Deterministic results.
- Diagnostic information usable by the user interface.

Open-source solver projects may provide implementation references, but license compatibility must be reviewed before embedding them in a proprietary product.

The project should avoid building a full general-purpose solver from scratch unless evidence shows available options are unsuitable.

---

# Article 99 — Solver Feedback Should Be Humanized

The solver may internally report:

- Inconsistent constraint set.
- Redundant equation.
- Unbounded degree of freedom.
- No solution.
- Multiple solutions.

The interface should translate this into user-centered language.

Examples:

“This angle conflicts with the perpendicular relationship.”

“This width is already defined in the front view.”

“This value can vary between these limits.”

“Depth is still unknown.”

This translation layer is critical.

It is one of the mechanisms by which Craft Loop hides mathematical complexity without hiding truth.

---

# Article 100 — Research Foundation: SketchGraphs

SketchGraphs is an important research reference because it models parametric sketches as geometric constraint graphs.

Its dataset contains millions of real-world sketches.

Geometric primitives are represented as nodes.

Designer-imposed relationships are represented as edges.

This representation directly supports Craft Loop’s decision to treat a drawing as relational structure rather than as a bitmap.

Craft Loop should not copy the dataset representation blindly.

The research supports the conceptual direction:

**The drawing is a graph of geometric entities and constraints.**

---

# Article 101 — Research Foundation: Vitruvion

Vitruvion demonstrates machine learning over parametric computer-aided design sketches.

It models primitives and constraints and supports conditioning on partial sketches and hand-drawn images.

The important product lesson is not that Craft Loop needs a generative model immediately.

The important lesson is that machine learning can assist with:

- Sketch completion.
- Constraint inference.
- Conditional interpretation.

This aligns with the future role of machine intelligence in Craft Loop.

Machine learning may propose.

The geometric system must validate.

The user must decide.

---

# Article 102 — Research Foundation: Free2CAD

Free2CAD explores parsing freehand drawings into sequences of simplified computer-aided design commands.

The research is relevant because it addresses two common barriers: users may not know how to decompose a shape into software commands, and they may not know how to execute those commands.

Craft Loop shares the motivation but narrows the first release to two-dimensional engineering representation.

The lesson is that freehand input can serve as a high-level expression of design intent.

Craft Loop should use that insight without forcing every sketch into a command sequence.

---

# Article 103 — Research Foundation: DAVINCI

DAVINCI explores joint inference of parametric sketch geometry and constraints from raster sketches.

It reports effectiveness on precise and hand-drawn sketches and builds on the SketchGraphs research ecosystem.

The important implication for Craft Loop is that primitive recognition and constraint inference can eventually be assisted by learned models.

The product should still separate:

Recognition.

Constraint proposal.

Constraint validation.

User acceptance.

This separation protects engineering trust.

---

# Article 104 — Research Foundation: PICASSO

PICASSO explores parameterizing two-dimensional computer-aided design sketches from both precise and hand-drawn sketch images.

Its self-supervised rendering strategy is relevant to Craft Loop because obtaining perfectly labeled hand-drawn engineering data can be difficult.

The project demonstrates a research path for turning hand-drawn visual input into parametric primitives.

Craft Loop has an additional advantage when processing live ink: it receives stroke order, timing, pressure, and geometry directly rather than only a raster image.

This online information should be preserved because it can reduce ambiguity.

---

# Article 105 — Research Foundation: CadVLM

CadVLM demonstrates multimodal models for parametric sketch tasks including autocompletion, automatic constraint prediction, and image-conditioned generation.

Craft Loop should treat this as evidence that future multimodal assistance is technically plausible.

It should not use the research as justification for autonomous engineering decisions.

Future learned engines may:

- Rank likely constraints.
- Suggest missing entities.
- Recognize repeated design patterns.
- Assist correspondence across views.
- Interpret ambiguous handwriting.

Their output must remain proposals until confirmed or until a deterministic rule makes the relationship certain.

---

# Article 106 — Research Foundation: AutoConstrain

Autodesk’s work on automated constraint prediction illustrates why design intent matters.

A sketch can be fully constrained in many different ways.

Some constraint sets preserve the intended structure better than others.

Craft Loop should therefore avoid a simplistic goal such as:

“Add enough constraints until no freedom remains.”

The better goal is:

“Preserve the relationships the user actually intends.”

This is one reason human approval remains central.

---

# Article 107 — Research Foundation: Dimension Recognition

Older engineering-drawing research remains relevant.

Research on recognition of dimensioning text shows that dimensions are not isolated character strings.

They interact with arrowheads, leader lines, witness lines, and surrounding geometry.

This supports Craft Loop’s decision to use contextual dimension association rather than handwriting recognition alone.

Craft Loop has a stronger signal because the system sees the dimension being created live.

It knows stroke order and timing.

That information should be used.

---

# Article 108 — Research Foundation: Dimension Sets as Constraints

Research on engineering drawing interpretation has treated dimensions as constraint sets that can be integrated with vectorized geometry.

This strongly supports Craft Loop’s semantic-dimension architecture.

A visible `30` should become a relationship in the engineering graph.

The same relationship may appear in more than one view without becoming multiple independent values.

The research direction reinforces the product principle:

**Dimensions are relationships, not labels.**

---

# Article 109 — Research Foundation: Orthographic Drawings

Research on dimensioned orthographic drawings has historically translated detected dimensions into constraints for individual views and then combined them into composite relational networks.

Craft Loop Version 1 does not reconstruct three-dimensional solids.

The relevant lesson is the graph representation itself.

Orthographic views can be related through structured constraints and correspondences.

This supports the Multiview Constraint Graph architecture.

---

# Article 110 — Why Version 1 Excludes Three-Dimensional Reconstruction

Three-dimensional reconstruction from orthographic drawings is a legitimate research field.

It is deliberately outside Craft Loop Version 1.

The exclusion protects product focus.

Adding three-dimensional reconstruction would introduce:

- Solid topology.
- Boundary representation.
- Ambiguity resolution at a much larger scale.
- Hidden geometry inference.
- Three-dimensional editing.
- Camera interaction.
- Three-dimensional export.
- More complex validation.
- Larger machine-learning requirements.

Craft Loop Version 1 should prove the value of the notebook interaction and linked two-dimensional engineering model first.

Three-dimensional integration may later become a separate product phase or integration with another Craft product.

---

# Article 111 — Machine Learning Philosophy

Machine learning in Craft Loop should be **assistive intelligence**.

It should not be the engineering authority.

Future learned engines may handle:

- Handwriting recognition.
- Primitive recognition.
- Constraint suggestions.
- View correspondence suggestions.
- Command recognition.
- Intent classification.
- Completion ranking.
- Personalized shortcuts.
- Error explanation.
- Pattern recognition.

The deterministic geometry and constraint systems remain responsible for what can be mathematically validated.

The core rule is:

**Artificial intelligence interprets. Geometry validates. The user decides.**

---

# Article 112 — Deep Learning Philosophy

Deep learning may become useful for problems where handcrafted rules struggle with variability.

Examples include:

- Very noisy handwriting.
- Complex freehand primitive interpretation.
- Mixed note and geometry scenes.
- Cross-view feature correspondence.
- Suggestion ranking.
- Context-aware command interpretation.

Deep learning should not be introduced simply because a problem contains drawing.

Deterministic methods are preferable when the relationship is known exactly.

Using a neural network to enforce a mathematical equality is unnecessary.

Using one to infer likely human intent may be valuable.

---

# Article 113 — Confidence, Explanation, and Provenance

Every learned suggestion should include internal metadata:

- Confidence.
- Source model.
- Relevant context.
- Whether the result has been accepted.
- Whether the result has been edited by the user.

The interface does not need to expose technical model details during normal use.

It should provide explanation when needed.

For example:

“Make these lines parallel?”

“Likely the same width as the front view.”

“Possible diameter: 20 millimeters.”

This supports trust without creating cognitive overload.

---

# Article 114 — No Silent Model Training From Private Work

Craft Loop should assume engineering drawings may be confidential.

Private notebooks should not automatically become training data.

If the company later offers an opt-in improvement program, consent must be explicit.

Enterprise plans may require a strict no-training policy.

Local personalization can be explored separately.

The product must be designed so that privacy is not dependent on vague language.

---

# Article 115 — Beginner Experience

The beginner should be able to open Craft Loop and draw without knowing:

- Constraint solver terminology.
- Degrees of freedom.
- Projection standards.
- Parametric modeling.
- Feature trees.
- Computer-aided design commands.

The product should guide through interaction.

A rough shape can refine.

A handwritten number can become a dimension.

A view label can activate meaning.

An invalid number can receive a plain-language explanation.

The beginner becomes more precise without being forced to learn the software before using it.

Craft Loop should not claim that it instantly makes someone a professional engineer.

It can lower the barrier to producing organized, precise engineering communication.

---

# Article 116 — Professional Experience

A professional should not feel constrained by beginner simplicity.

The same interface should reveal depth when demanded.

Professional capabilities can include:

- Explicit constraints.
- Reference dimensions.
- Precise unit control.
- Projection conventions.
- Dimension visibility control.
- Detailed snapping.
- Standards-aware export.
- Advanced view relationships.
- Custom command aliases.
- Future tolerancing.

The product should avoid separate beginner and professional applications.

Progressive disclosure is preferable.

---

# Article 117 — Progressive Disclosure

Progressive disclosure means capability appears when it becomes relevant.

A beginner sees:

Pen.

Eraser.

Select.

A user entering Sketch sees:

Line.

Circle.

Arc.

Rectangle.

Dimension.

A selected circle may reveal:

Diameter.

Radius.

Concentric.

Tangent.

An orthographic view may reveal:

View label.

Link.

Alignment.

Resolve.

The application becomes deeper without becoming visually heavier.

---

# Article 118 — Onboarding

Onboarding should teach interaction principles, not present a feature catalog.

A first-run tutorial can demonstrate:

1. Draw a rough line.
2. Hold to refine.
3. Write a dimension by hand.
4. Watch it become a structured value.
5. Write `Sketch`, circle it, and enter sketch tools.
6. Label a view.
7. Write `Ortho`, circle it, and create linked views.
8. Resolve one missing dimension in another view.

This short journey teaches the product’s identity.

The user should be able to skip onboarding.

---

# Article 119 — Learnability

Craft Loop should support learning through visible redundancy.

Every essential ink command must have a visible toolbar equivalent.

Every gesture should have a conventional alternative.

Tooltips should explain shortcuts.

The application may gently reveal faster workflows after repeated use.

For example:

After repeatedly tapping Line, the system may show:

“Tip: write `Line` and circle it.”

This makes expert interaction discoverable rather than hidden.

---

# Article 120 — Accessibility

Accessibility should include:

- Left-handed layout.
- Right-handed layout.
- Large touch targets.
- Sufficient contrast.
- Reduced motion.
- Haptic alternatives.
- VoiceOver or TalkBack support for document navigation where practical.
- Keyboard access.
- External pointer support where practical.
- Clear non-color-only conflict signals.
- Adjustable text size for interface elements.

The engineering canvas itself presents accessibility challenges.

The document model should expose semantic labels so future assistive technologies can identify geometry and dimensions.


# Article 121 — Version 1 Product Goal

Craft Loop Version 1 should be a usable product, not a research demonstration.

A user should be able to install the application on a supported tablet, create a document, sketch naturally with a stylus, refine geometry, write dimensions by hand, correct mistakes, annotate the page, label a principal view, enter an orthographic workflow, create or complete linked views, save the document, reopen it, and export a meaningful result.

Not every advanced engine needs to be fully automated in Version 1.

Where automation is not reliable enough, the product should use explicit human confirmation.

The Version 1 objective is to prove the central experience:

**A notebook can understand enough engineering structure to make two-dimensional technical drawing significantly easier without becoming traditional computer-aided design software.**

---

# Article 122 — Version 1 Functional Scope

The initial usable release should target the following capabilities:

- Notebook library.
- Document creation.
- Pen and pencil ink.
- Eraser.
- Selection.
- Undo and redo.
- Pan and zoom.
- Basic flexible page or canvas.
- Basic geometric primitive recognition.
- Draw-and-hold refinement.
- Line, circle, arc, ellipse, rectangle, and polyline support.
- Handwritten text recognition where available.
- Handwritten dimension recognition.
- Linear dimensions.
- Angular dimensions.
- Radius dimensions.
- Diameter dimensions.
- Basic constraints.
- Geometric consistency checks.
- View identity.
- Front, top, right, and back view support.
- Orthographic workspace.
- Shared extent relationships.
- Unresolved-dimension states.
- Cross-view dimension conflict detection.
- Ink Command Language for a limited safe command vocabulary.
- Local document persistence.
- Autosave.
- Export to portable document format.
- Vector export path, preferably scalable vector graphics.
- Architecture prepared for Drawing Exchange Format export when stable.

This scope is ambitious.

Implementation should be phased internally even if the final Version 1 presents a coherent experience.

---

# Article 123 — Explicit Version 1 Non-Goals

Version 1 does not include:

- Three-dimensional reconstruction.
- Three-dimensional visualization.
- Solid modeling.
- Boundary representation.
- Mesh generation.
- STEP export.
- Computer-aided manufacturing.
- Finite element analysis.
- Simulation.
- Full assembly management.
- Generative three-dimensional design.
- Automatic autonomous product design.
- Full geometric dimensioning and tolerancing authoring suite.
- Full manufacturing drawing certification.
- Automatic hidden-line generation from a three-dimensional model.
- Feature-history tree.
- Parametric three-dimensional operations such as extrusion and revolve.
- Desktop-first workflow.
- Phone-first workflow.

These exclusions protect the product thesis.

---

# Article 124 — Why Tablet-Only Is a Product Decision

Craft Loop is intentionally tablet-first.

The tablet combines:

- Direct pen input.
- Portable screen.
- Touch navigation.
- Enough display area for drawing.
- Natural notebook posture.
- Immediate annotation.
- Field use.
- Classroom use.
- Workshop use.
- Client communication.

Phones are too constrained for the intended drawing experience.

Desktop computers are powerful but do not provide the same direct hand-to-canvas relationship by default.

Desktop access may become useful later for viewing, export, or extended editing.

Version 1 should optimize interaction for tablets rather than compromise for every screen.

---

# Article 125 — iPad as a Reference Platform

The iPad ecosystem provides mature pen interaction and a strong creative-software culture.

PencilKit offers low-latency Apple Pencil capture.

PaperKit provides structured markup alongside freeform drawing.

Apple Notes establishes expectations for immediate handwriting.

Procreate establishes expectations for high-quality creative pen interaction.

Shapr3D establishes expectations for serious engineering work on a tablet.

Craft Loop should meet the interaction quality users already expect from those applications.

It should not assume Android is secondary.

The iPad simply provides a strong reference environment.

---

# Article 126 — Android Tablets as a First-Class Platform

Android tablet support must be treated as a real product target.

Jetpack Ink provides a modern low-latency drawing foundation.

The Android ecosystem contains stylus-capable devices with different hardware capabilities.

Craft Loop should define a supported-device quality floor.

Capabilities such as pressure, tilt, hover, and haptics may vary.

Core workflows must remain functional without optional hardware features.

The application should avoid an architecture that makes Android a delayed port of an Apple-only design.

---

# Article 127 — Device Capability Matrix

The engineering team should maintain a capability matrix covering:

- Stylus pressure.
- Stylus tilt.
- Hover.
- Haptic feedback.
- Palm rejection.
- Refresh rate.
- Screen size.
- Memory.
- Graphics capability.
- Native handwriting recognition availability.
- Operating system version.

Features should declare fallback behavior.

For example:

Hover guide preview is optional.

Dimension entry is not.

---

# Article 128 — Document Model

A Craft Loop document should be a structured package rather than a flat image.

The package can contain:

- Metadata.
- Pages or canvases.
- Raw ink streams.
- Vector primitives.
- View Blocks.
- Dimensions.
- Constraints.
- Cross-view links.
- Command history references.
- Embedded images.
- Style information.
- Document units.
- Projection convention.
- Version identifier.
- Engine schema versions.

The file format should support migration.

A document created in Version 1 must remain readable after later engines evolve.

---

# Article 129 — Stable Identifiers

Every semantic entity should have a stable identifier.

Examples include:

- Stroke.
- Geometric primitive.
- Dimension.
- Constraint.
- View.
- Cross-view correspondence.
- Annotation.
- Command event.

Stable identifiers are necessary for:

- Undo and redo.
- Persistence.
- Synchronization.
- Conflict resolution.
- Model suggestions.
- Cross-view linking.
- Debugging.

Identifiers should not be derived solely from screen coordinates.

---

# Article 130 — Separation of Geometry and Presentation

The data model should separate engineering geometry from visual styling.

A line’s endpoints define engineering geometry.

Its color, line weight, and on-screen emphasis define presentation.

A dimension’s semantic value and relation are separate from the font and placement of the visible label.

This separation supports:

- Standards-aware export.
- Theme changes.
- Accessibility.
- Different presentation modes.
- Clean technical drawings.
- Creative note views.

The same semantic drawing can have multiple visual representations without changing engineering truth.

---

# Article 131 — Coordinate Systems

Each View Block should own a local two-dimensional engineering coordinate system.

Page position should be separate.

This permits the user to move a view around the notebook without changing dimensions.

The Orthographic Relationship Engine should understand how view coordinate axes correspond.

For a conventional arrangement:

Front view can represent width and height.

Top view can represent width and depth.

Right view can represent depth and height.

The exact axis labels are internal implementation concepts.

The user does not need to see a three-dimensional coordinate widget.

---

# Article 132 — Projection Relationships Without a Three-Dimensional Object

Craft Loop can maintain multiview relationships without constructing a three-dimensional solid.

It can model shared extents and corresponding geometric relations.

For example:

A width variable may be shared by front and top views.

A height variable may be shared by front and right views.

A depth variable may be shared by top and right views.

Additional feature-level relationships can be added when correspondence is confirmed.

This is a two-and-a-half-dimensional intent model, not a three-dimensional object.

The distinction should remain explicit in architecture and marketing.

---

# Article 133 — Two-and-a-Half-Dimensional Intent Model

The term **Two-and-a-Half-Dimensional Intent Model** is useful internally.

It describes a representation richer than independent two-dimensional drawings but less complete than a three-dimensional solid.

The model can contain:

- Shared dimensions.
- View orientations.
- Projection alignments.
- Corresponding features.
- Constraints.
- Unresolved depths.
- Known extents.

It cannot necessarily answer arbitrary three-dimensional visibility or topology questions.

The product should never pretend that it can.

---

# Article 134 — Conflict Model

Conflicts should be structured objects.

A conflict can contain:

- Type.
- Affected entities.
- User-entered value.
- Existing value.
- Related views.
- Explanation.
- Possible resolutions.
- Severity.

Common conflict types include:

- Shared dimension mismatch.
- Impossible geometry.
- Constraint contradiction.
- View identity conflict.
- Unit mismatch.
- Duplicate driving dimension.
- Invalid command context.

Structured conflicts enable better user feedback than generic error strings.

---

# Article 135 — Conflict Resolution

Conflict resolution should preserve user control.

A conflict can offer actions such as:

- Use the existing value.
- Use the new value and update related geometry.
- Keep the items separate if the user decides they are not the same feature.
- Remove a constraint.
- Cancel the new input.

The system should explain consequences before large changes.

Simple conflicts should require minimal interaction.

---

# Article 136 — Feasible Ranges

Some dimensions are not uniquely determined but are not arbitrary.

The triangle example demonstrates this.

If two side lengths are fixed, the third side has a feasible range.

The engine should be capable of representing such bounded values.

A bounded dimension may show a subtle range hint when the user enters an invalid value.

This is more informative than a simple red error.

The same concept can apply to other geometry where inequality constraints exist.

---

# Article 137 — Constraint Propagation

When a driving dimension changes, related geometry should update coherently.

Propagation should follow explicit constraints and shared dimensions.

A change should not propagate through an uncertain correspondence unless that correspondence has been confirmed.

This is a critical distinction.

Confirmed semantic relationships can update automatically.

Suggested relationships require confirmation.

---

# Article 138 — Interaction Transaction Model

Complex interactions should behave transactionally.

For example, changing a shared width may update:

- Front geometry.
- Top geometry.
- A dimension annotation.
- Several constraints.
- A conflict state.

The user should perceive one action.

Undo should reverse the entire transaction.

This prevents partial updates that leave the drawing inconsistent.

---

# Article 139 — Orthographic Workspace Layout

The Orthographic workspace should remain creative rather than become a rigid multi-pane computer-aided design grid.

View Blocks can be arranged according to the selected projection convention.

The layout should provide enough structure to reveal relationships.

The user can still move notes, add callouts, and maintain notebook character.

A useful default may place the principal views according to engineering convention while allowing document-level arrangement.

If the user moves a view away for presentation purposes, the semantic view identity remains intact.

---

# Article 140 — Orthographic Projection Guides

Projection guides can help users understand how views correspond.

Guides may appear temporarily between aligned features.

They should not permanently clutter the page.

The system can show a guide when:

- Drawing a corresponding feature.
- Selecting a feature.
- Resolving an unknown dimension.
- Linking two entities.
- Diagnosing a conflict.

The visual language should remain subtle.

---

# Article 141 — Creating Additional Views

When Orthographic mode begins, Craft Loop can create empty or partially resolved View Blocks.

It should generate only what can be justified.

For example:

Known width may establish an extent in the top view.

Unknown depth remains unresolved.

Known height may establish an extent in the right view.

Feature details without sufficient information remain absent or ghosted.

The user completes the missing information.

The system should not use visual plausibility as engineering evidence.

---

# Article 142 — Completing a View by Hand

A user should be able to draw missing geometry directly in any active View Block.

The Ink Intent Engine treats the drawing in the context of that view.

New geometry can create new relationships.

If the geometry aligns with a projected feature, the system can suggest a correspondence.

The user confirms when ambiguity exists.

This makes orthographic construction collaborative rather than automatic.

---

# Article 143 — View-Local Versus Shared Geometry

Not every entity is shared across every view.

A visible edge in one view may not have a direct visible counterpart in another.

The data model should distinguish:

- View-local geometry.
- Shared semantic feature.
- Projection relationship.
- Correspondence suggestion.

This prevents over-linking.

---

# Article 144 — View Labels and Layout Labels

A View Identity is semantic.

A visible label is presentation.

The view can remain semantically Front even if the user hides the `FRONT` text for a clean export.

The system should not lose view identity when the label is hidden.

This mirrors the distinction between semantic dimensions and visible annotations.

---

# Article 145 — General Notes Versus Engineering Notes

Craft Loop should support both casual and formal annotation.

General notes may remain handwriting.

Engineering notes may optionally become typed technical text.

The user can choose whether a note is transformed.

Dimensions should generally use formal technical typography because they participate in the engineering representation.

Personal notes can preserve handwriting character.

This creates a deliberate mixture:

Precise geometry.

Precise dimensions.

Human notes.

Creative arrows.

---

# Article 146 — Handwriting Style Preservation

Apple Notes demonstrates a useful idea: handwriting can be refined while preserving personal style.

Craft Loop may eventually support two refinement paths:

**Technical conversion:** convert recognized content into engineering typography.

**Hand refinement:** smooth and improve handwriting while preserving its visual character.

Dimensions should default toward technical conversion.

Personal annotations may default toward hand refinement.

This distinction reinforces the hybrid creative-engineering identity.

---

# Article 147 — Typography

Craft Loop needs a clear typography system for:

- Interface text.
- View labels.
- Dimensions.
- Engineering notes.
- Conflict messages.
- Command previews.

Engineering annotations should use a highly legible typeface.

The product should avoid excessive stylistic variation in technical output.

Notebook notes can remain expressive.

Typography is part of the boundary between human thought and formal engineering representation.

---

# Article 148 — Color

Color should be used sparingly for engineering semantics.

Possible semantic roles include:

- Active selection.
- Suggested geometry.
- Conflict.
- Shared relationship.
- Orthographic guide.

The page should not become a rainbow of constraint states.

Most geometry should remain neutral.

The user’s drawing and notes can use expressive colors where appropriate.

The system should preserve print-friendly output.

---

# Article 149 — Motion

Motion should explain state transitions.

Useful transitions include:

- Ink refining into geometry.
- Handwritten number becoming a dimension.
- Command ink resolving into a tool.
- A shared dimension propagating to another view.
- An unresolved guide becoming confirmed.
- A conflict being resolved.

Motion should not exist merely for decoration.

Reduced-motion settings must be respected.

---

# Article 150 — Sound

Sound is optional and should be conservative.

Most feedback can use visual and haptic channels.

If sounds are included, they should be subtle and disableable.

Engineering environments can be quiet classrooms, workshops, offices, or client meetings.

The application should never depend on sound for critical state.

---

# Article 151 — Export Philosophy

Craft Loop becomes useful in real engineering workflows when its output can leave the application.

Version 1 should prioritize:

**Portable Document Format** for sharing and printing.

**Scalable Vector Graphics** for vector fidelity and downstream editing.

**Drawing Exchange Format** should be evaluated as a bridge to traditional computer-aided design workflows once the geometry model is stable.

Raster image export may also be useful for quick sharing.

The semantic internal format remains richer than any of these exports.

---

# Article 152 — Portable Document Format Export

Portable Document Format export should preserve:

- Visible geometry.
- Visible dimensions.
- Notes.
- View labels.
- Line types.
- Page layout.
- Vector quality where possible.

The user should be able to define an export region or sheet.

Export should not include hidden command ink or internal uncertainty metadata unless intentionally visualized.

---

# Article 153 — Scalable Vector Graphics Export

Scalable Vector Graphics is useful because it preserves vector geometry and is widely interoperable with creative tools.

The export can represent:

- Lines.
- Curves.
- Text.
- Dimensions as graphical elements.
- Annotations.

Scalable Vector Graphics does not preserve the full Craft Loop semantic model by default.

That limitation must be understood.

The native document remains the authoritative editable source.

---

# Article 154 — Drawing Exchange Format Export

Drawing Exchange Format is strategically important for engineering interoperability.

It can become a bridge between Craft Loop and traditional computer-aided design systems.

However, the export should not be rushed.

Line types, units, layers, blocks, dimension entities, and coordinate conventions need careful mapping.

Version 1 may release Drawing Exchange Format only when the team can validate output in representative downstream tools.

---

# Article 155 — Import

Initial import scope should remain conservative.

Useful early imports may include:

- Images.
- Portable Document Format references.
- Possibly Scalable Vector Graphics.

Imported content can act as reference material.

Automatic conversion of arbitrary imported engineering drawings into editable Craft Loop semantics is a separate advanced problem and should not be implied by basic import.

---

# Article 156 — Reference Images

Users may want to place:

- Photos.
- Scanned sketches.
- Existing drawings.
- Screenshots.

Reference images should remain distinguishable from engineering geometry.

Future recognition tools may trace or infer geometry from them.

Version 1 can focus on placement, opacity, crop, and annotation.

---

# Article 157 — Collaboration

Real-time collaboration is valuable but not required for the first usable product.

A simpler initial model can support:

- Share exported drawing.
- Share a read-only Craft Loop document.
- Duplicate a document.
- Future comment workflow.

Real-time co-editing should wait until semantic operation merging is reliable.

Engineering consistency is more important than collaboration novelty.

---

# Article 158 — Presentation Mode

A future Presentation Mode can hide editing controls and show the notebook cleanly.

This is useful for:

- Design reviews.
- Teaching.
- Client discussion.
- Workshop communication.

The user can still navigate views.

Annotations can remain visible.

This extends the notebook from creation into communication.

---

# Article 159 — Engineering Communication as a Product Pillar

Craft Loop is not only about producing geometry.

Engineering drawings are communication artifacts.

The notebook should help people explain:

- Shape.
- Size.
- Relationships.
- Alternatives.
- Questions.
- Revisions.
- Intent.

Arrows, callouts, handwritten notes, and highlighted regions are therefore not secondary features.

They are part of engineering communication.

The product should support the transition from private thinking to shared explanation.

---

# Article 160 — Education and Hobbyist Use

Craft Loop can serve students and hobbyists without becoming an educational toy.

For beginners, the system can reveal why a dimension conflicts.

It can make orthographic relationships visible.

It can demonstrate shared dimensions between views.

It can help users learn through doing.

The product should avoid grading or lesson-management features in Version 1.

Educational value should emerge from transparent engineering behavior.

---

# Article 161 — Professional Use

Professionals may use Craft Loop for:

- Early concept sketches.
- Shop-floor communication.
- Design review notes.
- Quick dimensioned ideas.
- Orthographic concept documentation.
- Client explanation.
- Field notes.
- Pre-computer-aided-design ideation.
- Markup and revision thinking.

Version 1 should not claim to replace certified manufacturing documentation workflows.

It can become a powerful precursor and communication layer.

---

# Article 162 — Position Relative to Traditional Computer-Aided Design

Craft Loop should integrate with traditional computer-aided design rather than position itself as an immediate replacement.

A plausible workflow is:

Craft Loop concept → clean orthographic representation → vector or Drawing Exchange Format export → detailed computer-aided design.

Later, Craft Loop may integrate with Craft’s other modeling products.

This creates a continuum rather than a competitive dead end.

---

# Article 163 — Future Integration With Craft Modeling

Craft Loop can eventually become an input surface for a broader Craft ecosystem.

A future workflow may allow linked orthographic sketches to inform three-dimensional modeling.

That future is explicitly outside MCP Version 1.

The current architecture should preserve semantic data that could support later integration:

- View identities.
- Shared dimensions.
- Constraints.
- Feature correspondences.
- Design intent.

This is future readiness, not current scope.

---

# Article 164 — Business Model Principle

Craft Loop is intended as a focused software-as-a-service product.

The product should earn payment through sustained user value, not advertising.

A possible model is:

Free or trial access with meaningful capability.

Paid individual subscription for advanced engineering intelligence, synchronization, export, history, or professional features.

Education pricing may be considered.

Business plans may later include organization controls and privacy guarantees.

Pricing should be validated against user value rather than decided solely from competitor prices.

---

# Article 165 — Value Proposition

The value proposition should not be:

“Another drawing app.”

It should be:

“Keep the freedom of sketching while gaining engineering structure without leaving the notebook.”

The strongest practical benefits are:

- Less context switching.
- Faster conversion from idea to clean drawing.
- Handwritten dimensions that become real geometry.
- Fewer inconsistent dimensions.
- Linked orthographic views.
- Lower barrier for beginners.
- Faster concept communication for professionals.
- A creative surface that preserves engineering rigor.

---

# Article 166 — Competitive Moat

The moat should not depend on drawing brushes.

It should not depend on a generic large language model.

It should not depend only on interface beauty.

The defensible system is the interaction and semantic architecture:

- Ink Intent Engine.
- Progressive Engineering Formalization.
- Ink Command Language.
- Engineering Handwriting Parser.
- Dimension Association.
- Constraint and Consistency Engine.
- Design Intent Graph.
- Orthographic Relationship Engine.
- Multiview Constraint Graph.
- Ambiguity management.
- Human-in-the-loop intelligent assistance.

These systems reinforce each other.

---

# Article 167 — What Craft Loop Must Not Become

Craft Loop must not slowly accumulate every feature from professional computer-aided design.

The following warning signs indicate product drift:

- Toolbars continuously growing.
- Feature trees appearing.
- Users required to learn modes before drawing.
- Three-dimensional concepts leaking into Version 1 terminology.
- Every action requiring a dialog.
- Machine intelligence making irreversible changes.
- Notebook notes treated as secondary.
- Orthographic views becoming rigid panes instead of semantic blocks.
- Creativity being sacrificed for formalism.

A new feature should answer:

“Does this make the user think and draw more effectively?”

If not, it should be questioned.

---

# Article 168 — Product Quality Bar

Craft Loop should be judged on more than feature completion.

The quality bar includes:

- Ink feels immediate.
- Recognition feels respectful.
- Refinement feels continuous.
- Dimensions feel natural.
- Invalid values are explained clearly.
- Orthographic relationships are understandable.
- The interface stays calm.
- Undo is trustworthy.
- Documents reopen exactly.
- Export is accurate.
- The product remains useful offline.
- The user remains in control.

A technically correct engine with poor interaction is not sufficient.

A beautiful interface with weak geometric truth is not sufficient.

---

# Article 169 — Reliability Philosophy

Engineering tools should fail conservatively.

If handwriting recognition is uncertain, preserve ink.

If correspondence is uncertain, ask.

If a dimension conflicts, do not silently modify another value.

If an orthographic view lacks information, show unresolved state.

If a standards rule is not implemented, do not claim compliance.

If a machine-learning engine is unavailable offline, deterministic drawing must continue.

Trust is more valuable than appearing intelligent.

---

# Article 170 — Error Philosophy

Errors should be actionable.

Bad:

“Invalid geometry.”

Better:

“This value conflicts with the width already defined in the front view.”

Bad:

“Cannot solve constraints.”

Better:

“This line is locked perpendicular, so the requested 70-degree angle cannot be applied.”

Bad:

“Orthographic failed.”

Better:

“Label the source view as Front, Top, Right, or Back before creating linked views.”

Every error should help the user continue drawing.


# Article 171 — Core User Journey: First Sketch

A new user opens Craft Loop.

The library shows recent notebooks and a clear create action.

The user creates a blank notebook.

The canvas opens immediately.

There is no mandatory project wizard.

The user draws a rough rectangle with the pen.

The stroke remains natural during movement.

After the stroke finishes, the system may identify line-like segments.

If the user holds at the end or invokes refinement, the geometry settles into precise segments.

The user continues drawing.

The experience should feel continuous.

The user should not need to understand the engine that produced the refinement.

---

# Article 172 — Core User Journey: Handwritten Dimension

The user selects or gestures toward one edge.

A light dimension guide appears.

The user writes `30` by hand.

The Ink Engine captures the strokes.

The handwriting recognizer proposes the text `30`.

The Engineering Handwriting Parser interprets it using document units.

The Dimension Association Engine verifies the target edge.

The Constraint Engine evaluates whether applying 30 is valid.

The Geometric Consistency Engine checks related constraints.

If valid, geometry updates.

The handwritten number morphs into a clean dimension annotation.

The action is stored as one undoable transaction.

---

# Article 173 — Core User Journey: Invalid Dimension

The user attempts to enter a dimension that is impossible under current constraints.

The system does not reject the pen stroke before the user completes it.

The writing is recognized.

The solver evaluates the proposed value.

A conflict is detected.

The annotation appears in a restrained conflict state.

A concise explanation becomes available.

The user can:

- Replace the existing relationship.
- Change the new value.
- Remove a conflicting constraint.
- Cancel.

No hidden correction occurs.

---

# Article 174 — Core User Journey: View Identity

The user completes a preliminary sketch.

The user writes `Front` beneath it.

The Ink Intent Engine sees text positioned near a structured sketch.

The command resolver determines that `Front` is more likely a view label than an application command because it is not circled and is placed in a labeling region.

The system offers or applies the Front view identity.

The handwriting may become a clean `FRONT` label.

The semantic identity is stored separately from the visible text.

---

# Article 175 — Core User Journey: Enter Orthographic

The user writes `Ortho`.

The user circles the newly written word.

The Ink Command Engine recognizes the command.

The Contextual Command Grammar verifies that Orthographic is valid from the current state.

The Orthographic Readiness Engine verifies that the source sketch has an explicit view identity.

If ready, the application enters the orthographic workspace.

The command ink dissolves.

The user receives visual and optional haptic confirmation.

The transition is undoable.

---

# Article 176 — Core User Journey: Orthographic With No Dimensions

The source Front view contains recognized geometry but no dimensions.

Craft Loop still allows Orthographic mode.

The system creates linked view blocks based on known projection relationships.

It does not invent physical depth.

Known alignment relationships appear.

Unknown extents remain unresolved.

The user can continue drawing in the top or right view.

Dimensions may be entered later in any appropriate view.

This is an essential Version 1 behavior.

---

# Article 177 — Core User Journey: Dimension Across Views

The Front view receives a width of 100 millimeters.

The Multiview Constraint Graph binds that width to the corresponding width in the Top view.

The Top view updates.

The visible annotation can remain only in Front.

The user later enters depth as 40 millimeters in Top.

The Right view gains the same depth relation.

The drawing becomes progressively more resolved.

The user never creates a three-dimensional solid.

---

# Article 178 — Core User Journey: Cross-View Conflict

The Front view defines width as 100 millimeters.

The Top view shares the width relationship.

The user writes 130 millimeters against the same width in Top.

The Geometric Consistency Engine detects a contradiction.

The application highlights the new value and the existing shared relationship.

The user can choose:

- Keep 100.
- Replace with 130 and propagate.
- Break the correspondence if the two entities were linked incorrectly.
- Cancel.

The application does not store both values as independent truth.

---

# Article 179 — Core User Journey: Freehand Note Beside Geometry

The user writes:

“Move hole later.”

The handwriting is near a drawing but does not follow a dimension gesture.

The Ink Intent Engine classifies it as a note.

The system does not turn `hole` into a command.

It does not convert `later` into a tool.

The note remains handwriting or can be converted to typed text if the user requests it.

This scenario is important because a pen-native engineering application must tolerate mixed semantic content.

---

# Article 180 — Core User Journey: Tool Command

The user wants the line tool.

The user writes `L`.

The user circles it.

In the current Sketch command namespace, `L` uniquely maps to Line.

The command executes.

The command ink disappears.

The toolbar reflects that Line is active.

If `L` is ambiguous in the current vocabulary, the system does not guess.

It requests more characters or shows candidates.

---

# Article 181 — Core User Journey: Circle as Geometry

The user draws a circle inside a sketch.

There is no command word inside it.

The current context is geometry creation.

The Ink Command Engine should not interpret the circle as confirmation.

The Primitive Recognition Engine may instead recognize it as a circle.

This scenario illustrates why confirmation gestures must be contextual.

---

# Article 182 — Core User Journey: Circle as Selection

The user circles older content that has existed on the page.

The content does not match a recently written command.

The gesture resembles a lasso.

The system selects the enclosed items.

A small option can allow “Keep Ink” if the user intended the circle to remain.

This behavior should be tuned through user testing.

---

# Article 183 — Core User Journey: Command With Full Word

A beginner writes `Rectangle`.

The user circles it.

The command resolves without requiring knowledge of abbreviations.

The application activates Rectangle.

This ensures the command language is readable before it becomes fast.

Shortcuts are a progressive enhancement.

---

# Article 184 — Core User Journey: Command With Prefix

An intermediate user writes `Rec`.

The current command namespace contains Rectangle and no other valid command beginning with `Rec`.

The resolver identifies a unique prefix.

Rectangle activates.

The same user can later customize or learn a shorter alias if desired.

---

# Article 185 — User Journey: Professional Precision

A professional user creates a sketch.

They explicitly apply symmetry.

They define a reference dimension.

They hide duplicate annotations in secondary views.

They choose a third-angle projection profile.

They export a clean vector drawing.

The interface remains the same product.

Advanced options appear in context.

No separate “Professional Mode” is required.

---

# Article 186 — User Journey: Student

A student sketches a mechanical form.

They label the first view Front.

They enter Orthographic mode.

The system shows that depth is unresolved.

The student adds a depth in the Top view.

The Right view becomes more complete.

The student tries an inconsistent width.

Craft Loop explains that the width is already shared with Front.

The product teaches orthographic relationships through behavior.

---

# Article 187 — User Journey: Hobbyist

A hobbyist sketches a bracket for fabrication.

They use freehand first.

They hold to refine the edges.

They write dimensions by hand.

Craft Loop catches one impossible dimension.

The user exports a portable document for discussion with a maker.

The product has delivered engineering value without requiring the user to learn a traditional computer-aided design application.

---

# Article 188 — User Journey: Engineer in a Meeting

An engineer sketches a concept during discussion.

They add handwritten notes around the drawing.

Only the critical geometry is refined.

They add a Front label and create a Top view.

The group discusses a dimension.

The engineer changes it by writing the new value.

Linked geometry updates.

The notebook becomes both thinking surface and communication artifact.

---

# Article 189 — User Journey: Field Annotation

A user imports a reference image or document.

They sketch an engineering detail beside it.

They dimension the sketch.

They add handwritten observations.

They export the page as a portable document.

This use case does not require the imported reference to become structured engineering geometry.

The distinction must remain clear.

---

# Article 190 — Product State Model

At a high level, Craft Loop may contain the following user-visible states:

**Notebook state:** free drawing and notes.

**Sketch context:** engineering geometry tools are active.

**Orthographic context:** linked view tools are active.

These should not behave like rigid modal worlds.

The user should be able to write notes in Sketch.

The user should be able to annotate in Orthographic.

The context primarily changes available commands, recognition expectations, and contextual tooling.

---

# Article 191 — Recognition State Model

A piece of ink may move through:

Raw.

Candidate.

Suggested interpretation.

Accepted structure.

Modified structure.

Rejected interpretation.

These states should be explicit internally.

The system should not destroy provenance when transitioning.

A rejected interpretation should return control to raw ink or the prior valid state.

---

# Article 192 — Dimension State Model

A dimension may be:

Handwritten candidate.

Parsed value.

Associated candidate.

Validated.

Driving.

Reference.

Shared.

Derived.

Bounded.

Conflicting.

The state machine should be testable.

Unexpected transitions should fail safely.

---

# Article 193 — View State Model

A View Block may be:

Unlabeled.

Labeled.

Draft.

Linked.

Partially resolved.

Conflicting.

Resolved.

The system should allow drawing in all states except those where corruption would occur.

“Resolved” is not necessarily a final immutable state.

A later edit can introduce new unresolved information.

---

# Article 194 — Command State Model

An ink command may be:

Raw text candidate.

Recognized command candidate.

Ambiguous.

Awaiting confirmation gesture.

Confirmed.

Executing.

Completed.

Rejected.

Cancelled.

The system should handle low-confidence recognition without executing accidentally.

---

# Article 195 — Conflict State Model

A conflict may be:

Detected.

Explained.

Awaiting user decision.

Resolved by keeping existing state.

Resolved by applying new state.

Resolved by unlinking.

Dismissed because the interpretation was wrong.

Conflict state should survive autosave if the document is closed before resolution.

The application should not lose track of unresolved engineering contradictions.

---

# Article 196 — Acceptance Criterion: Ink

A Version 1 release is not acceptable if pen strokes visibly lag under supported device conditions.

It is not acceptable if palm rejection frequently creates unintended marks.

It is not acceptable if the first stroke after opening a document is lost.

It is not acceptable if zooming causes strokes to visibly detach from geometry.

Ink quality is foundational.

---

# Article 197 — Acceptance Criterion: Refinement

Refinement should never occur so aggressively that ordinary notes are repeatedly converted into geometry.

The user must be able to keep raw ink.

Undo must restore the original state.

A refined primitive must preserve intended placement within defined tolerances.

The transition should feel continuous.

---

# Article 198 — Acceptance Criterion: Handwriting Dimensions

A handwritten dimension should not directly modify geometry before recognition and association are validated.

The system must support correction.

Common numeric input must be recognized reliably enough for practical use.

Units must be deterministic.

Conflicting values must not silently overwrite existing design intent.

---

# Article 199 — Acceptance Criterion: Constraints

The solver must return stable results for supported primitives and constraints.

Drag interactions should not oscillate visibly.

Conflicting constraints must produce structured diagnostics.

Undo must restore prior geometry exactly within numerical tolerance.

The system must not silently drop a user-confirmed constraint to find a solution.

---

# Article 200 — Acceptance Criterion: Orthographic Relationships

Shared dimensions must remain consistent across linked views.

Unresolved information must not be fabricated.

Changing a shared value must propagate atomically.

Unlinking a relationship must stop future propagation.

View identity must survive document save and reopen.

Changing page layout must not alter engineering dimensions.

---

# Article 201 — Acceptance Criterion: Ink Commands

A command must never execute from plain handwriting merely because it resembles a command word.

The confirmation gesture and context must be respected.

Ambiguous prefixes must not execute arbitrarily.

Command history must be undoable.

The toolbar and command language must invoke the same underlying action.

---

# Article 202 — Acceptance Criterion: Offline Use

Core drawing must work without network connectivity.

Opening an existing local notebook must work offline.

Creating a notebook must work offline.

Ink, refinement, deterministic dimensions, constraints, and saved linked views must work offline.

Features that genuinely require cloud intelligence must explain their temporary unavailability without breaking the document.

---

# Article 203 — Acceptance Criterion: Persistence

Closing and reopening a document must preserve:

- Geometry.
- Ink.
- Dimensions.
- View identity.
- Constraints.
- Cross-view relationships.
- Notes.
- Units.
- Projection convention.
- Conflict state where applicable.

Reopening must not reinterpret already confirmed engineering content differently merely because a recognition model changed.

Confirmed semantics are stored semantics.

---

# Article 204 — Acceptance Criterion: Export

Portable Document Format export must match visible technical content.

Units and dimensions must not change during export.

Vector lines should remain crisp.

Line types should survive appropriately.

No ephemeral command ink should appear.

No invisible conflict metadata should be printed unless the user chooses to include review marks.

---

# Article 205 — Testing Pyramid

Craft Loop requires several layers of testing.

**Unit tests** validate geometry, unit conversion, parsing, and constraint logic.

**Property-based tests** explore large numerical input spaces.

**Golden visual tests** detect rendering regressions.

**Gesture tests** validate command and selection disambiguation.

**Integration tests** validate end-to-end engine transactions.

**Device tests** validate pen latency, palm rejection, hover, and haptic differences.

**Usability tests** validate whether real people understand the interaction language.

No single test layer is sufficient.

---

# Article 206 — Geometry Property Testing

Property-based testing is particularly valuable for the geometry core.

Examples:

A line length should remain nonnegative.

A circle radius should remain positive.

Perpendicular lines should maintain a ninety-degree relation within tolerance.

Parallel constraints should remain parallel after valid edits.

Shared dimensions should remain equal across views.

Triangle-side constraints should reject impossible combinations.

Undo followed by redo should restore the same semantic state.

These tests can reveal edge cases that scripted examples miss.

---

# Article 207 — Numerical Tolerance

Computational geometry requires tolerance policies.

Craft Loop should define numerical tolerances centrally.

Tolerance policies should cover:

- Coincidence.
- Parallelism.
- Perpendicularity.
- Equality.
- Snap detection.
- Solver convergence.
- Export precision.

Visual tolerance and engineering tolerance are not the same thing.

A stroke may appear close enough for a recognition suggestion without being committed as an exact constraint.

---

# Article 208 — Gesture Testing

Gesture interpretation must be tested with varied handwriting and drawing behavior.

Test users should include:

- Left-handed writers.
- Right-handed writers.
- Fast writers.
- Slow writers.
- Large handwriting.
- Small handwriting.
- Different stylus pressures.
- Different command word styles.

False positive rate matters as much as recognition rate.

A command system that executes accidentally is worse than one that occasionally asks for confirmation.

---

# Article 209 — Orthographic Testing

Orthographic tests should include:

- Front-first workflows.
- Top-first workflows.
- Right-first workflows.
- No initial dimensions.
- Partial dimensions.
- Shared width.
- Shared height.
- Shared depth.
- Conflicts.
- Unlinked geometry.
- Ambiguous correspondence.
- First-angle layout.
- Third-angle layout.
- Save and reopen.
- Undo across propagated changes.

The system must prove that it behaves as a relational drawing engine, not a set of synchronized pictures.

---

# Article 210 — Research Evaluation Metrics

If future machine-learning models are introduced, conventional recognition accuracy is insufficient.

Useful metrics may include:

- Primitive recognition accuracy.
- Command false activation rate.
- Constraint suggestion precision.
- Constraint suggestion acceptance rate.
- Dimension association accuracy.
- Cross-view correspondence precision.
- User correction rate.
- Time saved.
- Number of tool switches avoided.
- Engineering conflict prevention.
- Trust rating.

A model with high raw accuracy may still be unacceptable if its errors are disruptive.

---

# Article 211 — Product Metrics

Business and product metrics should reflect value rather than only engagement.

Potential metrics include:

- Time from blank page to first dimensioned sketch.
- Time from first view to linked orthographic set.
- Percentage of sessions completed without keyboard use.
- Average number of tool switches.
- Number of conflicts caught before export.
- Undo rate after intelligent suggestions.
- Export frequency.
- Document reopen frequency.
- Weekly active engineering users.
- Retention by novice and professional cohorts.

The product should not optimize for unnecessary time spent in the application.

A successful tool may help users finish faster.

---

# Article 212 — Usability Research Questions

Early user research should ask:

Do users understand draw-and-hold refinement?

Do users trust handwritten dimensions?

Do users understand when a value is shared between views?

Can users distinguish suggested and confirmed geometry?

Does circling a command feel natural?

Does the command gesture conflict with selection?

Do users know why Orthographic mode cannot resolve depth?

Do beginners understand conflict explanations?

Do professionals feel the interface is too simplified?

Does the notebook metaphor remain credible after engineering features appear?

These questions should be answered with prototypes, not assumptions.

---

# Article 213 — Prototype Strategy

The first prototypes should isolate risky interactions.

Prototype A: Ink refinement.

Prototype B: Handwritten dimension conversion.

Prototype C: Ink command and circle confirmation.

Prototype D: View labeling.

Prototype E: Orthographic transition.

Prototype F: Shared dimension propagation.

Prototype G: Conflict explanation.

Prototype H: Mixed notes and engineering geometry.

These prototypes can be tested before the entire engine exists.

This reduces architectural risk.

---

# Article 214 — Technical Spike: Constraint Solver

A dedicated technical spike should evaluate available solver approaches.

The spike should measure:

- Supported constraint types.
- Incremental solve performance.
- Numerical stability.
- Mobile portability.
- Licensing.
- Debuggability.
- Deterministic behavior.
- Ease of binding to application code.
- Ability to return conflict information.

The solver choice is a foundational technical decision.

It should not be made solely from popularity.

---

# Article 215 — Technical Spike: Handwriting Recognition

Handwriting recognition should be tested separately on:

- Numbers.
- Decimal values.
- Fractions where relevant.
- Negative values.
- Degree symbol.
- Diameter notation.
- Radius notation.
- Plus/minus tolerance notation.
- Command words.
- View labels.

Platform-native recognizers may perform differently across languages and scripts.

Craft Loop should establish a supported-language policy for Version 1.

---

# Article 216 — Technical Spike: Primitive Recognition

Primitive recognition should be benchmarked using live strokes rather than only raster images.

The team should collect representative anonymous test strokes with consent.

Metrics should include:

- Correct primitive.
- Incorrect forced conversion.
- Time to recognition.
- Stability after zoom.
- Performance with shaky lines.
- Performance with intentional expressive lines.
- Difference between quick conceptual strokes and deliberate engineering strokes.

A “keep as ink” outcome is valid.

---

# Article 217 — Technical Spike: Orthographic Graph

A prototype Multiview Constraint Graph should be implemented before advanced user interface polish.

It should prove:

- Shared width.
- Shared height.
- Shared depth.
- View identity.
- Dimension propagation.
- Conflict detection.
- Unresolved values.
- Unlinking.
- Save and restore.

This demonstrates whether the product’s core differentiation is technically sound.

---

# Article 218 — Technical Spike: Command Grammar

The initial command vocabulary should remain small.

The team should test:

- Full-word recognition.
- Single-letter shortcuts.
- Unique prefixes.
- Ambiguous prefixes.
- Circle confirmation.
- Timing.
- Lasso conflict.
- Geometry-circle conflict.
- Handwriting-note conflict.

The goal is not maximum command count.

The goal is reliable pen-native control.

---

# Article 219 — Release Phasing Inside Version 1

Version 1 can be built in internal phases.

**Phase A — Notebook Foundation:** ink, documents, autosave, selection, undo.

**Phase B — Structured Sketch:** primitive recognition, refinement, vector geometry.

**Phase C — Engineering Dimensions:** handwritten values, dimension association, units, consistency.

**Phase D — Constraint Core:** basic constraint solver, design intent, conflict explanation.

**Phase E — Ink Commands:** contextual command grammar and confirmation gesture.

**Phase F — Orthographic Intelligence:** view identity, multiview graph, unresolved state, propagation.

**Phase G — Standards and Export:** line semantics, projection conventions, portable document and vector export.

**Phase H — Product Hardening:** performance, accessibility, device coverage, crash recovery, usability testing.

These are internal development phases, not necessarily separate public releases.

---

# Article 220 — Definition of a Usable Version 1

Craft Loop Version 1 is usable when a real person can complete the following without developer assistance:

Open the application.

Create a notebook.

Draw a rough engineering view with a supported stylus.

Refine the geometry.

Add at least several dimensions by hand.

Receive a clear warning for an invalid dimension.

Label the view.

Enter Orthographic mode.

See additional linked view space.

Resolve at least one unknown dimension in another view.

Change a shared dimension and observe consistent propagation.

Write ordinary notes without them becoming geometry.

Save automatically.

Close the application.

Reopen the same document with state intact.

Export a clean portable document.

If this journey is unreliable, Version 1 is not complete.


# Article 221 — Detailed Specification of the Ink Intent Engine

The Ink Intent Engine sits between raw stylus activity and every higher-level interpretation system.

Its primary responsibility is not to decide the final meaning of the user’s work.

Its responsibility is to determine which interpretation paths are plausible.

A raw group of strokes can therefore produce multiple candidates.

For example, a circular stroke may be:

- A geometric circle.
- A handwritten zero.
- The letter O.
- A lasso selection.
- A command-confirmation circle.
- A visual annotation.
- A decorative mark.
- A circle that should remain raw ink.

The engine should preserve this ambiguity until context provides sufficient evidence.

The input should include the complete stroke sequence, not merely a raster screenshot.

Useful input signals include point position, time, velocity, pressure, tilt, pen-up intervals, stroke grouping, screen scale, current tool, active View Block, nearby geometry, current selection, and recent commands.

The engine should also receive document context.

A circular mark written immediately around the word `Line` has different meaning from the same circle drawn inside an engineering sketch.

A short numeric stroke beside an active dimension guide has different meaning from the same number written in a paragraph.

A word written beneath a selected View Block may be a view label.

The engine should return a ranked set of intent candidates.

Each candidate should include a confidence estimate.

Each candidate should identify the evidence used.

Each candidate should declare which downstream engine would handle it.

The engine should not directly alter engineering geometry.

The engine should not directly execute commands.

The engine should not directly assign a dimension.

It routes intent.

The product must distinguish between high-confidence convenience and low-confidence interference.

A high-confidence result can trigger a reversible preview.

A medium-confidence result should ask quietly.

A low-confidence result should preserve ink.

The engine should have explicit false-positive budgets.

False interpretation is more damaging than missed interpretation when the user is thinking freely.

The Ink Intent Engine should therefore prefer “leave this alone” when evidence is weak.

This engine is one of the most important protectors of creative freedom in Craft Loop.

Without it, every advanced engine becomes intrusive.

With it, the user can mix formal and informal drawing naturally.

---

# Article 222 — Detailed Specification of the Primitive Recognition Engine

The Primitive Recognition Engine receives stroke groups that the Ink Intent Engine has classified as likely geometry.

Its goal is to estimate a compact geometric representation.

The first release should recognize a limited vocabulary well rather than a large vocabulary poorly.

A line candidate can be estimated through linear regression or endpoint fitting.

The engine should compute residual distance from the fitted line.

The engine should examine curvature.

The engine should inspect directional consistency.

A circle candidate can be fitted using geometric circle-fitting algorithms.

The engine should estimate center and radius.

The engine should measure residual error.

An ellipse candidate requires more complex fitting and should be accepted only when confidence is sufficient.

An arc candidate can be treated as a partial circular fit with start and end parameters.

A rectangle candidate may emerge from multiple strokes or one continuous polyline.

The engine should recognize that a human-drawn rectangle is not required to have perfect ninety-degree corners before refinement.

However, it should not automatically assume that every quadrilateral intends to be a rectangle.

The engine can return multiple primitive candidates.

For a rough oval, it may return ellipse with high confidence and circle with lower confidence.

For a roughly straight stroke, it may return line with high confidence and polyline with lower confidence.

The user can accept a candidate through hold behavior, a contextual suggestion, or explicit refinement.

Recognition should preserve the original stroke group until the transaction commits.

The engine must not introduce dimensions.

It should not infer a line length unless that length is merely a geometric measurement of current shape.

It should not apply permanent horizontal or vertical constraints unless a separate constraint step confirms the relationship.

Recognition identifies geometry.

Constraint interpretation identifies relationships.

These must remain separate responsibilities.

---

# Article 223 — Detailed Specification of the Beautification Engine

The Beautification Engine converts a confirmed interpretation into a visually and mathematically cleaner representation.

Beautification is not the same as primitive recognition.

Recognition says, “This is probably a line.”

Beautification says, “Given that this is a line, this is the precise line that best represents the intended stroke.”

The engine should minimize visible displacement.

The refined object should remain close to the human stroke.

The engine should preserve important endpoints where context indicates they are intentional.

If a line begins at an existing vertex, the fitting process should favor that vertex.

If a circle appears intended to be concentric with another circle, that relationship should remain a suggestion until the constraint system confirms it.

Beautification can include smoothing.

It can include removal of hand tremor.

It can include correction of nearly straight edges.

It can include normalization of nearly circular shapes.

It should not erase expressive ink that the user has not chosen to formalize.

The visual transition should make refinement understandable.

The application can animate the stroke settling into place.

The animation should be short.

The user should be able to undo immediately.

The engine should preserve the original stroke association for provenance.

The internal document can store the structured primitive as authoritative after acceptance.

The original ink can be retained in history or compressed according to policy.

---

# Article 224 — Detailed Specification of the Engineering Handwriting Parser

The Engineering Handwriting Parser receives recognized text that appears likely to carry engineering semantics.

Its role is to turn recognized characters into typed semantic values.

For example:

`30` may become a scalar value.

`30 mm` may become a scalar with explicit millimeter unit.

`3.5` may become a decimal scalar.

`45°` may become an angular value.

`Ø20` may become a diameter value.

`R10` may become a radius value.

`30 ± 0.1` may become a nominal value with symmetric tolerance in a future phase.

The parser should normalize common handwriting variants.

It should recognize that handwritten degree symbols may be imperfect.

It should recognize likely diameter notation only when context supports it.

The parser should never assume that a number is a dimension solely because it can parse it.

A note may contain the number 30.

The Dimension Association Engine decides whether a parsed engineering value binds to geometry.

The parser should produce structured candidates.

A candidate should include raw recognized text.

It should include normalized numeric value.

It should include unit.

It should include semantic type.

It should include confidence.

It should include parse warnings.

The parser should support locale-aware decimal separators where practical.

A user who writes `3,5` in a locale where comma is decimal should not be forced into an English-only numeric convention.

Unit parsing should be explicit.

The document default unit should apply when the user omits a unit.

A written explicit unit should override the default for that input and then normalize to the document’s internal unit.

The parser should reject physically meaningless unit combinations for simple dimensions.

The first release can deliberately limit unit syntax.

A smaller reliable grammar is better than an expansive ambiguous grammar.

---

# Article 225 — Detailed Specification of the Dimension Association Engine

Dimension Association is one of the highest-risk interpretation tasks.

A correct number attached to the wrong edge is an engineering error.

The engine should therefore use multiple independent signals.

The first signal is explicit selection.

If the user selects an edge and then writes a value, association confidence is high.

The second signal is an active dimension gesture.

If the user drags a dimension guide from an edge and writes a value inside it, confidence is high.

The third signal is spatial proximity.

A number written near an edge may refer to that edge, but proximity alone is insufficient when several entities are nearby.

The fourth signal is direction.

A horizontal dimension annotation usually describes a horizontal span or horizontal projection.

The fifth signal is engineering symbol context.

`Ø20` written near a selected circle strongly suggests diameter.

The sixth signal is temporal proximity.

A value written immediately after creating a dimension guide is likely to belong to that guide.

The seventh signal is view context.

A number written inside a Front view should not accidentally attach to geometry in a Top view merely because the screen coordinates are close.

The eighth signal is existing dimension ownership.

If an edge length is already controlled by a shared dimension, a new value may be a conflict or reference rather than a new independent dimension.

The ninth signal is user habit.

Personalization may eventually rank interpretations, but it must not override explicit context.

The engine should return a proposed binding.

The Constraint and Consistency Engines validate the proposal.

If binding confidence is below threshold, Craft Loop should ask.

The user should be able to redirect the dimension easily by dragging or tapping the intended target.

A misassociated dimension should be correctable without deleting and rewriting the value.

---

# Article 226 — Detailed Specification of the Constraint Engine

The Constraint Engine manages exact relationships between structured geometric entities.

The engine must distinguish between geometric constraints and numerical dimensions.

Geometric constraints describe relationships such as parallel, perpendicular, tangent, coincident, concentric, symmetric, equal, horizontal, and vertical.

Numerical constraints describe values such as length, distance, radius, diameter, and angle.

The solver should operate incrementally.

When one value changes, it should solve only the affected relation graph where practical.

The system must provide stable results during interactive drag.

The system should identify when there are multiple mathematically valid solutions.

The solver should preserve continuity during manipulation rather than jumping unpredictably between solution branches.

The engine should understand fixed user anchors.

If the user changes a dimension, the system needs a policy for which geometry moves.

The policy should reflect explicit constraints and local interaction context.

The engine should not silently discard constraints to recover from a conflict.

If no solution exists, the transaction should not commit until the user resolves the conflict.

The engine should expose diagnostic information.

It should identify the constraints most directly participating in a contradiction.

It should report free degrees of freedom internally.

It should report redundant relationships where possible.

The interface can simplify these diagnostics.

The solver implementation must be numerically robust across ordinary engineering scales.

The team should define supported scale ranges.

Extremely small and extremely large coordinates can create floating-point problems.

The document should normalize units internally.

Solver tolerance should be centralized.

Testing should include repeated solve-edit-solve cycles.

Engineering state should not drift over time.

---

# Article 227 — Detailed Specification of the Geometric Consistency Engine

The Geometric Consistency Engine sits above the constraint solver.

The solver answers whether a set of equations can be solved.

The Consistency Engine answers whether the document’s semantic claims remain coherent.

The difference matters.

A drawing may be mathematically solvable while semantically contradictory.

For example, two views might no longer correspond after one is edited independently.

A view may be labeled Front but arranged under a projection convention that contradicts an explicitly locked layout.

A dimension may be syntactically valid but applied to the wrong semantic feature.

The Consistency Engine should check multiple layers.

Local geometry checks validate primitives.

Constraint checks validate relationships.

Dimension checks validate numeric feasibility.

View checks validate shared dimensions.

Correspondence checks validate feature links.

Unit checks validate quantity interpretation.

Standards checks validate representation where enabled.

The engine should output conflicts with severity.

Some issues block commit.

Some issues are warnings.

Some issues merely indicate unresolved information.

The distinction is essential.

Unknown depth is not an error.

Conflicting shared width is an error.

A missing optional view label may be a workflow blocker only when entering Orthographic mode.

The Consistency Engine should never convert “unknown” into “invalid.”

This principle protects exploratory work.

---

# Article 228 — Detailed Specification of the Design Intent Graph

The Design Intent Graph records why geometry should behave a certain way.

A raw geometric graph can state that two lines currently happen to be parallel.

A Design Intent Graph records whether the user expects them to remain parallel.

This graph should represent:

- Explicit user constraints.
- User-accepted inferred constraints.
- Shared dimensions.
- Symmetry intentions.
- Equal-size relationships.
- View correspondences.
- Named semantic relationships.
- Provenance.

The graph should distinguish observed geometry from intended geometry.

Two circles may currently have equal radius without an Equal constraint.

The system can suggest equality.

It should not assume equality permanently unless confirmed or deterministically implied.

The Design Intent Graph is also a key future input to machine learning.

A model can learn patterns of accepted intent.

However, the graph’s confirmed state remains authoritative.

The graph supports coherent edits.

When a dimension changes, the solver uses intent to determine what should remain invariant.

This is one of the places where Craft Loop becomes significantly more than a drawing application.

---

# Article 229 — Detailed Specification of the Constraint State Engine

The Constraint State Engine interprets solver state for product behavior.

A geometric entity can be fully defined in some dimensions and free in others.

A view can be partially resolved.

A dimension can be bounded without being fixed.

The engine should expose a structured state for every relevant semantic object.

Possible geometry states include:

Free.

Partially constrained.

Fully determined.

Conflicting.

Possible dimension states include:

Unknown.

Free.

Bounded.

Driving.

Derived.

Reference.

Shared.

Conflicting.

Possible correspondence states include:

Unlinked.

Suggested.

Confirmed.

Broken.

Conflicting.

These states can drive interface styling.

They can drive command availability.

They can drive Orthographic Readiness.

They can drive export warnings.

The state engine should not use presentation colors as state.

State belongs in the model.

Presentation reads it.

---

# Article 230 — Detailed Specification of the Orthographic Relationship Engine

The Orthographic Relationship Engine manages the semantic connections among View Blocks.

It should know each View Identity.

It should know the projection convention.

It should know which coordinate directions correspond across views.

It should know which semantic dimensions are shared.

It should know confirmed feature correspondences.

It should know unresolved relationships.

The engine should not require a three-dimensional body.

It should operate over a relational representation.

For simple bounding geometry, it can share extents.

For feature-level geometry, it can propagate only when correspondence is known.

The engine should create projected guides.

It should validate alignments.

It should help create additional View Blocks.

It should coordinate with the Consistency Engine.

If the user changes a shared width, the engine propagates that width to linked views.

If the user draws a new feature in Top, the engine may suggest projected alignment in Front.

It should not generate unseen back geometry without evidence.

The engine should support first-angle and third-angle layout semantics.

It should allow the page layout to move without changing engineering relationships.

The engine should serialize all relationships so reopening the document does not require re-inference.

---

# Article 231 — Detailed Specification of the Multiview Constraint Graph

The Multiview Constraint Graph is the authoritative relational structure for orthographic intelligence.

A View Block is a node or container.

Structured primitives within views are nodes.

Semantic dimensions are shared nodes or variables.

Constraints connect entities.

Cross-view correspondences connect entities in different View Blocks.

Projection relationships connect coordinate semantics.

The graph should support partial connectivity.

Not every object must be linked.

The graph should support uncertainty metadata for suggestions.

Confirmed engineering links must remain deterministic.

The graph should support graph queries such as:

Which views depend on this width?

Which dimensions control this feature?

Which unresolved values prevent this view from being fully resolved?

Which correspondences involve this edge?

Which conflict was introduced by this new value?

The graph should support incremental recomputation.

A change in one local note should not trigger global engineering solve.

A change in a shared dimension should update only affected subgraphs.

The graph should remain independent from screen arrangement.

A View Block can move visually.

Its engineering connections remain.

---

# Article 232 — Detailed Specification of the Cross-View Correspondence Engine

Cross-view correspondence is inherently ambiguous in many cases.

The engine should therefore combine deterministic geometric evidence with assistive ranking.

Deterministic evidence includes exact shared coordinates under the current projection.

It includes dimensions.

It includes established centerlines.

It includes existing user links.

It includes constraints.

Probabilistic evidence can include visual similarity.

It can include structural context.

It can include repeated patterns.

It can include learned design patterns.

The engine should produce correspondence candidates.

A candidate should identify source entity.

It should identify target entity.

It should identify confidence.

It should identify evidence.

High-confidence deterministic links may be applied when the relationship follows directly from confirmed geometry.

Learned links should generally remain suggestions.

The user should be able to reject a suggested correspondence.

Rejected correspondences should not immediately reappear.

The system should remember the rejection within the document context.

---

# Article 233 — Detailed Specification of the Ambiguity Engine

The Ambiguity Engine maintains explicit uncertainty.

It should prevent other systems from turning unknown information into false certainty.

An unresolved depth is an ambiguity.

An uncertain circle-to-hole correspondence is an ambiguity.

An unclear handwritten value is an ambiguity.

An ambiguous command prefix is an ambiguity.

The engine should classify ambiguity type.

It should estimate what information would resolve the ambiguity.

For example:

Depth can be resolved by a dimension in Top or Right.

Command ambiguity can be resolved by one additional character.

Dimension-target ambiguity can be resolved by tapping the target edge.

Correspondence ambiguity can be resolved by confirming a suggested link.

The interface can then ask the smallest possible question.

This leads to a powerful design rule:

**When the system does not know, ask for the smallest missing fact.**

The product should avoid large modal forms when a single tap or handwritten value resolves the uncertainty.

---

# Article 234 — Detailed Specification of the Orthographic Readiness Engine

The Orthographic Readiness Engine determines whether it is useful and safe to start linked multiview work.

It should not require full dimensioning.

Minimum readiness can include:

At least one structured or interpretable view.

A known View Identity.

A valid local coordinate frame.

No blocking local geometry corruption.

Readiness can be graded.

A sketch can be “link ready” while still containing unresolved dimensions.

The engine should report blockers separately from incompleteness.

A missing depth is incompleteness.

A missing View Identity is a blocker for automatic view semantics.

A local constraint contradiction may be a blocker if propagation would amplify the conflict.

The interface should translate readiness into simple guidance.

If the only missing piece is View Identity, ask for it.

If the drawing contains a conflict, explain it.

If the drawing is merely incomplete, allow progression.

---

# Article 235 — Detailed Specification of the Standards Engine

The Standards Engine should separate normative representation rules from product interaction.

Its data should be profile-based.

A profile can define:

Projection method.

View arrangement.

Line type definitions.

Line weight relationships.

Dimension typography rules.

Leader conventions.

Centerline conventions.

Hidden-line conventions.

Future section-view rules.

The engine should expose capabilities to the export system.

It may also provide optional live guidance.

However, Version 1 should avoid claiming full standards compliance unless the implemented subset has been reviewed professionally.

The International Organization for Standardization standards are copyrighted documents.

The American Society of Mechanical Engineers standards are copyrighted documents.

Public metadata and summaries can guide product planning.

Formal compliance may require access to full licensed texts and expert review.

The MCP document should therefore distinguish standards awareness from certified compliance.

---

# Article 236 — Detailed Specification of the Ink Command Engine

The Ink Command Engine receives stroke groups classified as likely commands.

It receives recognized text.

It receives current command namespace.

It receives timing.

It receives the confirmation gesture.

It receives the current tool state.

It receives risk level.

The engine resolves the command through the following stages:

Normalize handwriting.

Match full command names.

Match user aliases.

Match unique prefixes.

Check context validity.

Check ambiguity.

Check confirmation gesture.

Check risk policy.

Create command object.

Dispatch through the shared Command Bus.

The engine should not invoke business logic directly.

It should not bypass permission checks.

It should not bypass undo history.

The engine should report command recognition visually before destructive actions.

The engine should support escape or cancel.

The user should be able to undo a mistaken low-risk command immediately.

Command vocabulary should be versioned.

Documents should not depend on the current meaning of ephemeral commands after execution.

The resulting operation is stored, not the original command string as the sole authority.

---

# Article 237 — Detailed Specification of the Contextual Command Grammar

The command grammar defines which commands exist in each context.

The grammar should be small enough to remember.

The Notebook context might include:

Pen.

Eraser.

Select.

Sketch.

Orthographic.

The Sketch context might include:

Line.

Circle.

Arc.

Rectangle.

Dimension.

Exit Sketch.

The Orthographic context might include:

Add View.

Label View.

Link.

Resolve.

Dimension.

The grammar should support synonyms carefully.

For example, `Ortho` can map to `Orthographic`.

Synonyms increase recognition burden.

They should be intentionally curated.

Localization will eventually require translated command vocabularies.

A command should not unexpectedly change meaning across languages.

The shortest unique prefix should be computed per namespace and locale.

The application should expose a command reference for discoverability.

---

# Article 238 — Detailed Specification of the Command Confirmation Gesture

The initial proposed confirmation gesture is a circle around recently written command ink.

The detection system should evaluate:

Whether the enclosed content is recent.

Whether the enclosed content is recognized as a valid command.

Whether the circle encloses most of the command bounds.

Whether the stroke itself looks like a deliberate enclosure.

Whether the current context would interpret the same circle as geometry.

Whether the user has disabled circle-to-command behavior.

If confidence is high, a low-risk command can execute.

If confidence is medium, a small command preview can appear.

If confidence is low, the circle remains ink or selection behavior applies.

The product must test the gesture with many handwriting styles.

A command circle should not require a perfect geometric circle.

The recognizer should tolerate an oval enclosure.

The user should not need to draw an engineering-quality circle to issue a command.

---

# Article 239 — Detailed Specification of the Command Bus

The Command Bus is the unified application action layer.

It receives semantic commands from:

Toolbar buttons.

Ink Command Engine.

Keyboard shortcuts.

Accessibility actions.

Future voice commands.

Future gestures.

Every command object should contain:

Command type.

Target context.

Parameters.

Source channel.

Timestamp.

Undo metadata.

Risk level.

The Command Bus validates whether the command is allowed.

It invokes the appropriate domain operation.

It creates a user-visible transaction.

It returns success or structured failure.

The interface then provides feedback.

This architecture prevents duplicated action logic.

It also provides a complete testing surface.

A test can dispatch the same command that every input channel ultimately uses.

---

# Article 240 — Detailed Specification of the Document Engine

The Document Engine is responsible for durable truth.

It owns document identity.

It owns page or canvas structure.

It owns entity identifiers.

It owns serialization.

It owns schema version.

It coordinates autosave.

It coordinates migration.

It coordinates undo persistence policy.

It owns references to raw ink and structured engineering entities.

The Document Engine should not perform geometric solving itself.

It stores the results and semantic state produced by domain engines.

Serialization should be atomic.

A save operation should not leave a half-written relationship graph.

The file format should include forward-compatible version metadata.

Unknown future fields should be handled safely where possible.

Migration should be explicit.

A new application version should not reinterpret confirmed old geometry through new recognition logic.

Confirmed geometry is data.

Recognition is an input process.

This distinction protects long-term document stability.


# Article 241 — Detailed Specification of the Native Ink Boundary

The native ink boundary is the point where hardware input becomes application data.

On Apple platforms, PencilKit should be evaluated as the default low-latency ink capture layer because it receives Apple Pencil and touch input and produces structured stroke data.

PaperKit should be evaluated for structured markup features that may reduce implementation effort for basic shapes, text boxes, arrows, and annotation behavior.

On Android, Jetpack Ink should be evaluated because it provides low-latency freehand drawing, immutable completed strokes, in-progress stroke representation, pressure, tilt, orientation, and rendering support.

The native layer should normalize platform differences into a Craft Loop stroke representation.

Normalization should not discard platform capabilities.

The shared stroke representation can include optional fields.

A device without tilt simply produces no tilt value.

A device with hover can produce hover events.

The engine should not fake unsupported data.

The native boundary should tag every stroke with:

Platform.

Device capability profile.

Tool type.

Input source.

Timestamp base.

Coordinate transform.

This metadata can assist debugging.

The application should preserve the native rendering path for the active stroke when that path provides superior latency.

After the stroke completes, the shared representation can become authoritative for semantic interpretation.

The team should avoid expensive format conversions during pen-down interaction.

Latency must take priority.

---

# Article 242 — Detailed Specification of Stroke Grouping

Handwriting and geometry recognition often depend on groups of strokes.

The engine must decide which strokes belong together.

A rectangle may be four strokes.

The handwritten number `30` may be two or three strokes depending on writing style.

The word `Line` contains multiple letters.

A command circle follows the command text as a separate stroke.

Stroke grouping should consider:

Time gap.

Spatial distance.

Writing direction.

Current semantic context.

Recognition candidates.

Pen lift duration.

Bounding-box overlap.

Active target.

The system should allow regrouping after later evidence.

An initial grouping should not permanently corrupt interpretation.

For example, the number `3` and the number `0` may initially be separate.

When written next to each other within the dimension region, they become one token.

The command word and confirmation circle should remain separate semantic groups linked by a command-confirmation event.

---

# Article 243 — Detailed Specification of Stroke Smoothing

Stroke smoothing is a presentation operation.

It should not erase the motion characteristics needed for intent classification.

Craft Loop should distinguish:

Raw sample path.

Smoothed visual path.

Fitted engineering geometry.

These are three different representations.

The raw sample path preserves evidence.

The smoothed path improves handwriting and drawing appearance.

The fitted geometry represents formal structure.

A user can therefore write a note and receive smoothing without conversion to engineering geometry.

A line candidate can be smoothed before the user decides whether to refine it.

The engine should avoid smoothing so aggressively that handwriting becomes unrecognizable.

Smoothing parameters may differ by tool.

A pencil may preserve more texture.

An engineering pen may stabilize more strongly.

The settings should remain simple.

---

# Article 244 — Detailed Specification of Pen Tools

Version 1 should avoid recreating a full illustration brush studio.

A small tool family is sufficient.

Potential tools include:

Engineering Pen.

Pencil.

Highlighter.

Eraser.

Select.

The Engineering Pen can emphasize clarity and predictable width.

The Pencil can support exploratory sketching.

The Highlighter can support review and communication.

The user should not need hundreds of brush presets.

This is one of the boundaries that protects Craft Loop from becoming Procreate.

Creative quality comes from response and interaction, not from an enormous brush library.

---

# Article 245 — Detailed Specification of Touch Interaction

Touch should complement the pen.

A default division can be:

Pen draws and writes.

One or two fingers navigate.

Touch selects when appropriate.

Multi-touch zooms and pans.

Two-finger tap can undo if consistent with platform expectations and user settings.

Touch should not accidentally create geometry while the palm is resting.

The application should allow finger drawing as an accessibility option if desired.

The default tablet engineering experience should prioritize stylus precision.

---

# Article 246 — Detailed Specification of Zoom and Pan

Zoom and pan must remain smooth even with large documents.

Screen-space line weights should behave predictably.

Dimension text should remain readable at useful zoom levels.

The product should define minimum and maximum zoom.

At extreme zoom out, detailed annotations may simplify.

At extreme zoom in, raw ink texture can remain high quality.

Semantic selection should use zoom-aware hit testing.

A user should not need pixel-perfect tapping to select a line.

View Block labels can remain accessible across zoom through adaptive presentation.

The user should be able to quickly return to active content if they become lost on a large canvas.

---

# Article 247 — Detailed Specification of Spatial Navigation

If Craft Loop supports a large or flexible canvas, it should help the user navigate semantic content.

Potential mechanisms include:

Document overview.

View Block navigator.

Named bookmarks.

Recent focus locations.

Fit selected view.

Fit all views.

These features should remain secondary.

The primary interaction remains direct pan and zoom.

The system can use semantic content to make navigation smarter than a generic infinite canvas.

---

# Article 248 — Detailed Specification of Page Frames

A Page Frame defines a formal export surface.

The notebook can remain spatially flexible outside the frame.

The user can add or resize frames.

A frame can correspond to a standard paper size.

The Standards Engine can eventually provide standard sheet choices.

The frame should not clip working ink while editing unless the user chooses a print-preview mode.

This approach allows creative exploration beyond the final sheet.

---

# Article 249 — Detailed Specification of Layers

Traditional illustration applications expose explicit layers.

Craft Loop should be cautious with layers because a complex layer stack can conflict with simplicity.

Internally, the system needs semantic layers or categories.

Raw ink.

Engineering geometry.

Dimensions.

Guides.

Notes.

References.

View Blocks.

Presentation state.

The user does not necessarily need to manage these as Photoshop-like layers.

A simple visibility control can be introduced when needed.

Advanced layer management should not appear in Version 1 unless user research shows strong value.

---

# Article 250 — Detailed Specification of Styles

Styles should be role-based rather than manually configured per entity by default.

Possible roles include:

Visible engineering edge.

Construction line.

Centerline.

Hidden line.

Dimension line.

Extension line.

Note ink.

Suggestion.

Conflict.

Selection.

The Standards Engine can map roles to line patterns and weights.

The user can choose themes or limited presentation customization without destroying semantic meaning.

A hidden line should remain a hidden-line role even if the user changes its appearance.

---

# Article 251 — Detailed Specification of Theme

Craft Loop can support light and dark interface themes.

The drawing page may remain paper-like under both themes or offer a dark-paper option.

Engineering output should remain export-neutral.

The application theme should not alter semantic colors in a way that makes conflicts or suggestions indistinguishable.

The visual design should use restrained surfaces.

The page is the hero.

Toolbars should feel lightweight.

The interface should avoid heavy panels that make the product resemble desktop computer-aided design.

---

# Article 252 — Detailed Specification of the Main Toolbar

The main toolbar should contain only high-frequency global tools.

A candidate Version 1 default includes:

Pen.

Select.

Eraser.

Sketch entry.

Undo.

Redo.

A contextual action can reveal more.

The toolbar should not permanently display Line, Circle, Arc, Dimension, Orthographic, export, settings, and every future tool simultaneously.

The toolbar can transform when context changes.

The user should always understand which tool is active.

Ink commands should update the toolbar state.

Toolbar actions should update command state.

There is one tool authority.

---

# Article 253 — Detailed Specification of Sketch Toolbar

When Sketch context becomes active, the toolbar may expose:

Line.

Circle.

Arc.

Rectangle.

Dimension.

Constraint access.

Exit Sketch.

The toolbar should remain compact.

It should not expose every constraint icon by default.

Selecting geometry can reveal applicable constraints.

For example, selecting two lines can reveal Parallel, Perpendicular, Equal, or Angle where relevant.

Selecting a circle and line can reveal Tangent.

The system should avoid presenting impossible constraints.

This reduces cognitive load.

---

# Article 254 — Detailed Specification of Orthographic Toolbar

When Orthographic context is active, the toolbar may expose:

Add View.

Label View.

Dimension.

Link.

Unlink.

Resolve.

View arrangement.

Projection convention access.

The number of visible actions should remain small.

Most shared dimensions should propagate without explicit sync controls.

The user should not need a “Synchronize Views” button.

Synchronization is a property of confirmed relationships.

---

# Article 255 — Detailed Specification of Contextual Popovers

A contextual popover should appear close to the work.

It can show:

Recognition alternatives.

Constraint choices.

Conflict resolution.

Command ambiguity.

Dimension target candidates.

The popover should not cover the active pen tip.

Placement should respect handedness.

It should dismiss easily.

It should not become a floating property inspector that remains permanently open.

The product should use popovers for decisions, not for routine drawing.

---

# Article 256 — Detailed Specification of Inline Editing

Dimensions should be editable inline.

The user can tap a visible dimension and write a replacement value with the pen.

Typed entry can also appear.

A value edit should preview its geometric effect where safe.

If the value creates a conflict, the preview should communicate it before commit.

Inline editing preserves flow.

The user should not have to open a distant numeric panel.

---

# Article 257 — Detailed Specification of Selection Handles

Structured geometry can expose handles when selected.

A line can expose endpoints.

A circle can expose center and radius control.

An arc can expose endpoints and curvature controls.

Handles should not appear on raw notes.

Handles should be large enough for stylus use.

The system should distinguish moving a handle from writing nearby.

Hover can help where available.

Handle movement should flow through the constraint system.

A constrained endpoint may move differently from a free endpoint.

The interface should make this understandable through guides and feedback.

---

# Article 258 — Detailed Specification of Dragging Constrained Geometry

When the user drags constrained geometry, the solver should update interactively.

The product should avoid sudden jumps.

If a point is constrained horizontally, dragging can move it along the allowed direction.

If a circle is concentric, moving its center may move related entities or be disallowed depending on the relationship.

The interface should reveal constraint behavior.

A small guide can indicate the allowed direction.

If no valid solution exists, the drag should not corrupt the model.

The system can stop at the last valid state and show why.

---

# Article 259 — Detailed Specification of Snap Versus Constraint

Snap and constraint are different.

Snap is temporary guidance during an interaction.

Constraint is a persistent relationship.

A line may temporarily snap horizontal while being drawn.

If the user accepts that alignment, the system may offer or create a Horizontal constraint based on policy.

The application should not always convert snap into constraint automatically.

Users often want convenient placement without permanent relational behavior.

The product should make the difference predictable.

---

# Article 260 — Detailed Specification of Automatic Constraint Suggestions

Automatic constraint suggestions should be conservative.

The system can notice:

Two lines are nearly parallel.

Two points nearly coincide.

A line is nearly horizontal.

Two circles have nearly equal radius.

A shape is nearly symmetric.

The system may show subtle cues.

The user can accept through hold, gesture, tap, or a setting that enables safe automatic constraints.

The source of the constraint should be stored.

A machine-learning model may later rank suggestions.

The constraint solver validates them.

The user remains able to remove them.


# Article 261 — Research Synthesis: Apple Notes as an Interaction Baseline

Apple Notes demonstrates several expectations that Craft Loop users will bring with them before learning any engineering-specific behavior.

The first expectation is immediate input.

A user can begin writing or drawing directly on the note.

Craft Loop should preserve that immediacy.

The second expectation is that handwriting can remain handwriting while still becoming computationally useful.

Apple supports handwriting refinement, selection, search, and transcription.

This reinforces Craft Loop’s decision to preserve ink as a meaningful data type.

The third expectation is that system intelligence should often feel like refinement rather than replacement.

Craft Loop should extend this concept from handwriting legibility to geometric precision.

A rough line can become precise without forcing the user to recreate it.

A handwritten value can become structured without forcing keyboard entry.

The Apple Notes reference therefore informs interaction expectations, not engineering logic.

Craft Loop must add the engineering semantics that Notes intentionally does not provide.

---

# Article 262 — Research Synthesis: PencilKit

PencilKit confirms that iPad applications can obtain low-latency stroke data rather than treating the Apple Pencil as a generic mouse.

The framework exposes drawing data and individual strokes.

The current platform direction also includes handwriting recognition capabilities.

This makes PencilKit a strong candidate for Craft Loop’s Apple ink boundary.

However, PencilKit is not a constraint solver.

It does not understand that a line is part of an orthographic view.

It does not know that `Ø20` should constrain a circle.

It does not understand a shared width.

Craft Loop should therefore treat PencilKit as infrastructure.

The proprietary value exists above it.

The product should also avoid binding its semantic document format directly to a platform-specific drawing format.

A platform format can be stored or referenced for raw ink, while the shared Craft Loop document model remains independent.

---

# Article 263 — Research Synthesis: PaperKit

PaperKit is relevant because Apple explicitly positions it as a richer markup layer built on PencilKit.

It supports structured elements such as shapes, images, and text alongside freeform drawing.

This validates a key Craft Loop design decision: freehand content and structured elements can coexist in one canvas.

PaperKit may accelerate some annotation behavior on Apple platforms.

It should be evaluated rather than automatically adopted for every structured object.

Craft Loop’s engineering primitives and constraint graph require semantics that exceed generic markup.

A PaperKit shape can be useful presentation infrastructure.

The engineering engine still needs its own canonical representation.

---

# Article 264 — Research Synthesis: Jetpack Ink

The Android Jetpack Ink application programming interface provides low-latency drawing and structured stroke data.

It includes position, timestamps, and optional pressure, tilt, and orientation.

It supports in-progress and finalized strokes.

This is a strong match for Craft Loop’s need to preserve online ink information.

The ability to use common Ink modules beyond Android-specific rendering also suggests potential shared processing opportunities.

Craft Loop should still normalize its own semantic stroke representation.

The platform library is an input and rendering foundation.

It is not the product model.

---

# Article 265 — Research Synthesis: Concepts

Concepts proves that professional and creative users accept a tablet drawing environment that mixes vector editability, measurement, snapping, scale, and sketching.

This reduces product-market uncertainty around the broad premise.

It also raises the differentiation bar.

Craft Loop cannot merely add measurements to an infinite canvas.

Concepts already provides real-world scale, measurement, grid systems, snapping, editable vector strokes, and shape recognition.

Craft Loop must therefore compete on deeper engineering understanding.

Its distinctive value should include semantic dimensions, constraint validation, design intent, view identity, orthographic relationships, and cross-view consistency.

The comparison should remain respectful.

The goal is not to claim Concepts lacks value.

The goal is to define why Craft Loop exists separately.

---

# Article 266 — Research Synthesis: Procreate

Procreate demonstrates how a highly capable professional tool can keep the artwork at the center of the interface.

Its streamlined visual design and gesture system are relevant.

QuickShape is especially relevant because it turns hand-drawn forms into cleaner geometry through an in-place interaction.

QuickMenu demonstrates the value of learned muscle memory.

Craft Loop should emulate these principles, not Procreate’s illustration features.

A full brush engine, color studio, blending-mode system, and painterly layer stack are not part of Version 1.

The transferable lessons are:

Directness.

Flow.

Gesture efficiency.

Minimal visual obstruction.

Refinement in place.

---

# Article 267 — Research Synthesis: Shapr3D

Shapr3D demonstrates that professional engineering interaction on a tablet can be direct and context-sensitive.

Its constraint model reinforces the importance of design intent.

Its adaptive interface reinforces the value of surfacing valid operations based on selection.

Craft Loop should adopt these deeper principles.

It should not reproduce three-dimensional modeling workflows.

The key distinction is that Shapr3D begins within a formal modeling environment.

Craft Loop begins within a notebook.

The notebook can become formal progressively.

---

# Article 268 — Research Synthesis: Microsoft Journal

Microsoft Journal is important because it explicitly explored an ink-first interaction philosophy.

Its scratch-out and Instant Lasso gestures reduce mode switching.

Its use of timing for lasso disambiguation is a concrete precedent for Craft Loop’s contextual circle gesture.

Its hand-preference settings demonstrate that handedness affects interface placement.

Its tactile feedback support demonstrates that pen interaction can include haptic confirmation.

Craft Loop should use these lessons while developing its own command grammar.

A gesture should be robustly disambiguated.

It should not be copied blindly.

---

# Article 269 — Research Synthesis: SketchGraphs

SketchGraphs gives Craft Loop a research-backed vocabulary for relational geometry.

A sketch is not only a collection of coordinates.

It is a graph of primitives and constraints.

That representation is particularly suitable for:

Constraint inference.

Design intent.

Partial sketches.

Graph queries.

Machine-learning research.

Craft Loop’s production graph may differ substantially.

The key research contribution for product thinking is the relational model.

---

# Article 270 — Research Synthesis: Vitruvion

Vitruvion demonstrates that models can learn distributions over parametric sketches containing both primitives and constraints.

It also shows conditioning on partial and hand-drawn input.

Craft Loop can interpret this as evidence for future assistance.

The immediate Version 1 should not depend on reproducing Vitruvion.

The deterministic product core is more urgent.

The research becomes valuable when Craft Loop has enough structured data and user interaction evidence to decide where learning genuinely improves the workflow.

---

# Article 271 — Research Synthesis: Free2CAD

Free2CAD addresses the gap between freehand expression and formal design operations.

Craft Loop shares the belief that the user should not need to mentally decompose every idea into software commands before drawing.

The two projects differ in scope.

Free2CAD ultimately parses drawings into computer-aided design command sequences.

Craft Loop Version 1 remains in structured two-dimensional engineering representation.

Nevertheless, the paper supports a critical product thesis:

Freehand input can express higher-level design intent than conventional command-first interfaces allow.

---

# Article 272 — Research Synthesis: DAVINCI

DAVINCI jointly predicts sketch parameterization and constraints.

Its results on both precise and hand-drawn inputs support the idea that learned systems can bridge rough visual expression and formal sketch structure.

Craft Loop should consider joint inference only after deterministic boundaries are clear.

A model should not become the final solver.

A predicted constraint can be validated mathematically.

A predicted primitive can be fitted geometrically.

Human correction remains available.

This layered approach converts research capability into trustworthy product behavior.

---

# Article 273 — Research Synthesis: PICASSO

PICASSO uses rendering self-supervision to infer parametric primitives from sketch images.

The work is relevant because labeled parametric datasets are expensive.

Craft Loop can benefit conceptually from self-supervised and weakly supervised approaches in future research.

Craft Loop also possesses richer online input than an image.

The stroke sequence itself contains time and motion.

That information should not be discarded merely to fit an image-based research architecture.

The product can eventually combine raster appearance with online ink features.

---

# Article 274 — Research Synthesis: CadVLM

CadVLM combines visual and symbolic sketch representations for tasks such as completion and constraint prediction.

The work is relevant to future contextual assistance.

Craft Loop eventually has a similar combination:

Visual ink.

Structured primitives.

Text.

Dimensions.

View identities.

Constraints.

The future opportunity is substantial.

The product must nevertheless avoid allowing a general multimodal model to become the source of engineering truth.

A multimodal model is an interpreter and proposal generator.

The deterministic graph remains authoritative.

---

# Article 275 — Research Synthesis: AutoConstrain

Autodesk’s AutoConstrain work is especially relevant because it moves beyond simple geometry recognition.

It attempts to predict useful constraints and dimensions.

Later Autodesk research emphasizes alignment with design intent.

This reinforces Craft Loop’s view that the “correct” constraint set is not merely the one that mathematically removes freedom.

The relationships should reflect what the user means.

The user is therefore part of the inference loop.

Craft Loop’s human-in-the-loop philosophy is not a limitation.

It is an engineering reliability strategy.

---

# Article 276 — Research Synthesis: Engineering Dimension Recognition

Research on dimension recognition in engineering drawings historically combines optical character recognition with surrounding graphical context.

Dimension text alone is insufficient.

Arrowheads, witness lines, leader lines, and nearby geometry help establish meaning.

Craft Loop receives this information at creation time.

It can therefore build association before the page becomes a static image.

This is a major architectural advantage.

The system should use it.

---

# Article 277 — Research Synthesis: Dimension Layout

OpenAlex surfaces historical research studying how the layout of dimensional information affects a reader’s ability to solve dimension problems.

This is a reminder that dimension placement is not merely visual decoration.

Poor annotation layout can reduce comprehension.

Craft Loop should eventually provide intelligent dimension placement suggestions.

Version 1 can begin with collision avoidance and direct manipulation.

Future research can optimize readability without taking control away from the user.

---

# Article 278 — Research Synthesis: Orthographic Constraints

Historical work on dimensioned orthographic drawings has represented detected dimensions as sets of constraints and combined them into composite structural networks.

Craft Loop can adopt the relational insight without adopting the three-dimensional reconstruction goal.

Each view contributes partial knowledge.

Dimensions convert visual geometry into exact relationships.

A composite network allows consistency to be checked across views.

This is directly aligned with the Multiview Constraint Graph.

---

# Article 279 — Standards Research: International Organization for Standardization 128

International Organization for Standardization 128 defines general principles for technical product documentation.

The standard family includes rules for lines and for views, sections, and cuts.

The current International Organization for Standardization 128-2 edition defines basic line conventions.

International Organization for Standardization 128-3 defines principles for views, sections, and cuts and references orthographic projection methods.

Craft Loop should use these standards as professional design references.

The application should not claim compliance merely because it recognizes line styles.

Compliance requires full implementation review.

---

# Article 280 — Standards Research: International Organization for Standardization 5456

International Organization for Standardization 5456 addresses projection methods.

The orthographic representation part is directly relevant to Craft Loop.

The projection convention affects where principal views appear relative to one another.

Craft Loop should model the convention semantically.

The selected convention should affect:

Default view placement.

View interpretation.

Export.

It should not require the user to manually rearrange every view according to a standard.

---

# Article 281 — Standards Research: American Society of Mechanical Engineers Y14.3

The American Society of Mechanical Engineers Y14.3 standard covers requirements for orthographic and pictorial views.

This provides a second major standards family relevant to Craft Loop.

A professional system may need to support both common standards ecosystems.

The Version 1 Standards Engine can begin with projection behavior and basic representation.

Future releases can deepen coverage.

The team should obtain appropriate standards access and expert review before making professional compliance claims.

---

# Article 282 — Standards Research: Dimensioning and Tolerancing

The American Society of Mechanical Engineers Y14 standards family includes dimensioning and tolerancing standards.

International standards also govern technical product specification.

Craft Loop Version 1 should support ordinary linear, angular, radius, and diameter dimensions.

Full geometric dimensioning and tolerancing should be treated as a later specialized system.

It requires significant domain expertise.

The data architecture should avoid blocking future implementation.

The interface should not expose partial geometric dimensioning and tolerancing in a misleading way.

---

# Article 283 — Competitive Boundary: Apple Notes

Apple Notes competes for the user’s habit of opening a tablet and writing immediately.

Craft Loop must be nearly as immediate.

If creating an engineering note requires a long setup process, the user may stay in Notes and never move the idea.

Craft Loop wins when it offers that immediacy plus engineering structure.

The experience should feel like opening a notebook that happens to understand engineering.

---

# Article 284 — Competitive Boundary: Concepts

Concepts competes for freeform spatial thinking and precision sketching.

Craft Loop must offer a reason to move beyond measurement and snapping.

That reason is semantic engineering relationships.

A Concepts user may measure a stroke.

A Craft Loop user can make that measurement part of a constraint graph and share it across an orthographic view.

This is the intended differentiation.

---

# Article 285 — Competitive Boundary: Procreate

Procreate competes for drawing pleasure.

Craft Loop will not beat Procreate by building more brushes.

It should learn from Procreate’s interaction quality.

Craft Loop must make engineering drawing feel similarly direct.

The competition is not feature-for-feature.

It is an expectation of polish.

---

# Article 286 — Competitive Boundary: Shapr3D

Shapr3D competes for tablet-based engineering creation.

Craft Loop should not enter direct competition in three-dimensional modeling.

Its opportunity is earlier in the thought process.

A user may begin in Craft Loop because they want to think, sketch, dimension, and communicate.

A later modeling workflow may begin after the idea becomes formal.

This makes integration more strategically sensible than imitation.

---

# Article 287 — Competitive Boundary: Traditional Computer-Aided Design

Traditional computer-aided design remains the professional destination for detailed modeling, manufacturing documentation, assemblies, and downstream engineering.

Craft Loop can reduce the friction before that stage.

The product should export cleanly.

It should not trap users.

Interoperability strengthens the product.

A notebook that helps a user reach professional tools faster is more valuable than one that tries to replace every professional tool.

---

# Article 288 — Product Language

Craft Loop should use plain language whenever possible.

Prefer:

“Keep these lines parallel.”

Over:

“Add a parallel geometric constraint.”

Prefer:

“This width is already defined in Front.”

Over:

“Redundant driving dimension.”

Prefer:

“Depth is still unknown.”

Over:

“One degree of freedom remains.”

Advanced terminology can be available in professional documentation.

The default interface should explain meaning before jargon.

---

# Article 289 — Engineering Vocabulary Without Fear

The product should not avoid engineering words entirely.

Words such as Front, Top, Radius, Diameter, Dimension, Parallel, and Tangent are useful.

The goal is not to hide the discipline.

The goal is to prevent software mechanics from becoming the learning barrier.

Craft Loop should teach vocabulary through context.

A user who sees `Ø20` become a diameter dimension learns the notation naturally.

---

# Article 290 — Professional Transparency

When professionals need deeper detail, Craft Loop should expose it.

A dimension can reveal whether it is driving or reference.

A relationship can reveal its source.

A conflict can reveal the exact linked views.

A suggestion can reveal that it came from machine inference.

This transparency builds trust.

Simplicity should never mean obscuring information that matters.

---

# Article 291 — Explainable Assistance

Future machine-learning suggestions should be explainable at an appropriate level.

A suggestion might say:

“Likely parallel because the lines differ by less than one degree.”

A cross-view suggestion might say:

“Likely the same feature because centers align and diameter matches.”

The application does not need to expose neural-network internals.

It should expose the practical evidence.

The user can then decide.

---

# Article 292 — Personalization Without Loss of Predictability

Craft Loop may personalize:

Preferred tool aliases.

Toolbar order.

Frequently used units.

Handedness.

Common page style.

Frequently used constraints.

It should be conservative about personalizing engineering semantics.

The same dimension should not behave differently on two devices because a hidden personalization model changed the solver.

Personalization belongs around interaction, not mathematical truth.

---

# Article 293 — Internationalization

Craft Loop should plan for multiple languages.

Handwriting recognition availability varies by platform and language.

Command vocabularies require localization.

Decimal separators vary.

Unit notation varies.

Technical terms vary.

View labels may be localized while internal semantic identities remain language-independent.

A document should not break when the application language changes.

The semantic model should store language-neutral identifiers.

---

# Article 294 — Arabic and Right-to-Left Considerations

A future Arabic interface requires more than translated labels.

Right-to-left layout affects toolbar placement, popovers, text flow, notes, and command interpretation.

Engineering numbers and symbols may mix directional systems.

Handwriting recognition support must be verified.

The user may write Arabic notes while using English engineering command words.

Craft Loop should not assume one language per document.

The architecture should support mixed-language annotations.

---

# Article 295 — Localization of Ink Commands

Ink commands should use a locale-specific vocabulary.

A user may choose to use English commands even in another interface language.

Command aliases can support both.

The application should avoid a situation where the same short prefix maps unpredictably after language changes.

Command namespaces should be versioned by locale.

The visible command reference should explain active vocabulary.

---

# Article 296 — Search

Search can eventually use:

Notebook title.

Typed notes.

Recognized handwriting.

View labels.

Dimension values.

Tags.

Search should not require converting all handwriting permanently into typed text.

An on-device index can store recognized text separately.

Engineering search can later include questions such as:

“Find drawings with a 20 millimeter hole.”

This is a future opportunity.

Version 1 can begin with title and recognized-text search.

---

# Article 297 — Tags and Organization

The product should keep organization lightweight.

Folders and tags may be sufficient.

Craft Loop should not become a project-management suite.

Useful metadata can include:

Project name.

Date.

Tags.

Author.

Revision note.

The system can later integrate with external document-management workflows.

---

# Article 298 — Templates

Templates can accelerate common workflows.

Potential templates include:

Blank engineering notebook.

Graph paper.

Orthographic layout.

Dimensioned sketch page.

Design review page.

Education practice page.

Templates should initialize layout and settings.

They should not constrain what the user can draw.

---

# Article 299 — Orthographic Template

An Orthographic template can pre-create labeled view positions according to a projection convention.

This is different from entering Orthographic mode from a free sketch.

Both workflows should exist.

Some users know in advance that they want a formal multiview drawing.

Others begin with one idea.

The product should support both without requiring different document types.

---

# Article 300 — Blank Notebook as the Default

Despite the value of templates, the default experience should remain a blank notebook.

The product thesis begins with freedom.

A blank page communicates that the user can start anywhere.

Engineering structure appears when requested.

This is a deliberate contrast with software that begins by asking the user to configure a model before they can draw.


# Article 301 — Edge Case: Ambiguous Handwritten Number

The user writes a shape that could be read as `1` or `7`.

The recognizer should not immediately apply a dimension.

The parser can produce both candidates.

The geometric context can eliminate impossible candidates.

If one value creates a contradiction and the other is feasible, the application may rank the feasible value higher.

It should still avoid silently changing the user’s handwriting into the value it prefers when recognition confidence remains low.

A small inline choice is appropriate.

The user can tap the intended value.

The chosen interpretation becomes part of the dimension transaction.

---

# Article 302 — Edge Case: Ambiguous Decimal Separator

The user writes `3,5`.

The document locale may interpret comma as decimal separator.

Another user may intend a list or annotation.

The parser should consider locale.

It should consider dimension context.

If the value is attached to a dimension guide, numeric interpretation is strong.

The normalized internal value can be 3.5 in base units.

The visible annotation can follow the selected locale.

---

# Article 303 — Edge Case: Unit Written With a Number

The document uses millimeters.

The user writes `3 cm`.

The parser recognizes an explicit unit.

The Dimension Engine converts 3 centimeters to 30 millimeters internally.

The visible annotation can remain `3 cm` if the user wants to preserve explicit unit notation.

Alternatively, the document style can normalize display to 30 millimeters.

The behavior should be predictable.

The application must never interpret the value as 3 millimeters merely because document units are millimeters.

---

# Article 304 — Edge Case: Number in a General Note

The user writes:

“Use 30 bolts.”

The number appears near geometry.

The Ink Intent Engine detects sentence context.

No dimension guide is active.

No geometry is selected.

The text remains a note.

This example demonstrates why numeric handwriting recognition cannot automatically create dimensions.

---

# Article 305 — Edge Case: Number Between Two Views

The user writes `50` in the space between Front and Top.

Several edges are nearby.

The Dimension Association Engine finds multiple candidates.

Association confidence is low.

The application can show temporary target highlights.

The user taps the intended target.

The number becomes a dimension.

If the user ignores the suggestion, the ink remains ordinary text.

---

# Article 306 — Edge Case: Dimension Already Exists

A line is already controlled by a 30-millimeter driving dimension.

The user writes another 30 beside the same line.

The system should not create a second independent driving dimension.

It may interpret the new value as redundant.

The user can keep a visible reference annotation if desired.

The engine should explain that the length is already defined.

---

# Article 307 — Edge Case: Redundant but Consistent Dimension

A shared width is 100 millimeters.

The user writes 100 again in the Top view.

The new value is consistent.

The system can treat the annotation as another visible representation of the same semantic dimension rather than a new variable.

This is different from a conflict.

The graph gains a new annotation reference.

The underlying width remains one value.

---

# Article 308 — Edge Case: Shared Dimension Conflict

A shared width is 100 millimeters.

The user writes 120 in another linked view.

The system creates a conflict object.

The new value is not committed as an independent shared width.

The existing geometry remains valid until the user chooses a resolution.

A preview may show how the design would change if 120 replaces 100.

The user remains in control.

---

# Article 309 — Edge Case: Dimension Feasible Range

Two triangle sides are fixed.

The third side has a mathematical range.

The user enters a value outside that range.

The Consistency Engine reports a bounded-value conflict.

The interface can show the valid range.

The system should not recommend a single arbitrary value unless a separate constraint determines one.

This is important because “invalid” and “unknown” are different.

---

# Article 310 — Edge Case: Dimension With Multiple Geometric Solutions

A set of dimensions may permit more than one geometric configuration.

For example, a triangle defined by certain relationships may have mirrored solutions.

The solver should preserve the branch closest to the current geometry during editing.

If the user explicitly requests the alternate configuration, the application can expose a flip or alternate-solution action.

The system should avoid jumping between branches unexpectedly.

---

# Article 311 — Edge Case: Over-Constrained Sketch

The user adds a new constraint that conflicts with existing fully defining relationships.

The solver reports inconsistency.

The transaction remains uncommitted.

The interface identifies the relationship most directly involved.

The user can replace an old constraint or cancel the new one.

Craft Loop should avoid the intimidating phrase “over-constrained” in the default message if a plain explanation is possible.

Professional details can be available on demand.

---

# Article 312 — Edge Case: Redundant Constraint

Two lines are already forced parallel by another structural relationship.

The user explicitly adds Parallel again.

The system can treat the new constraint as redundant.

It should not necessarily create an error.

The user can be told that the relationship is already guaranteed.

The internal graph should avoid unnecessary duplicate constraints.

---

# Article 313 — Edge Case: Constraint Suggestion Rejected

The system suggests that two lines are equal.

The user rejects the suggestion.

The suggestion should disappear.

The system should not immediately propose the same relation again after every edit.

A rejection memory should exist for the current document or local context.

If geometry changes substantially, the suggestion may become relevant again.

The policy should avoid nagging.

---

# Article 314 — Edge Case: Refinement Misrecognizes a Shape

The user draws an intentional trapezoid.

The recognizer suggests Rectangle.

The user does not accept.

The trapezoid remains raw or becomes a generic polyline.

The application should not repeatedly straighten it into a rectangle.

The user can explicitly choose Rectangle later if desired.

Recognition must respect rejection.

---

# Article 315 — Edge Case: Rough Circle Is Actually a Letter

The user writes the letter O in a note.

The shape is circular.

Text context dominates geometry context.

The Ink Intent Engine routes the stroke to handwriting recognition.

The Primitive Recognition Engine may never receive it.

This layered routing reduces false geometry conversion.

---

# Article 316 — Edge Case: Command Word in a Note

The user writes:

“Use Line B as reference.”

The word `Line` appears.

There is no immediate command-confirmation circle.

The sentence remains a note.

The Ink Command Engine should not execute merely because a known command name exists in text.

This is a strict requirement.

---

# Article 317 — Edge Case: Command Word Circled as Annotation

The user writes the word `Line` as a note and circles it for emphasis.

This exactly resembles the proposed command pattern.

Context must decide.

Useful evidence includes:

The text age.

Whether the circle immediately follows the word.

Whether the current interaction state expects a command.

Whether surrounding text forms a sentence.

Whether the user paused.

If ambiguity remains high, the application should ask rather than execute.

This is a key usability test scenario.

---

# Article 318 — Edge Case: Circle Around Existing Content

The user circles existing content to select it.

The content is older.

No valid command token dominates the selection.

The system uses lasso behavior.

If the user intended to draw a real circle, a “Keep Ink” action can restore it.

This matches familiar ink interaction patterns.

---

# Article 319 — Edge Case: Single-Letter Command Conflicts With Handwriting

The user writes `P`.

In Notebook context, `P` may mean Pen.

In a note, `P` may be an ordinary letter.

The confirmation gesture is required for command interpretation.

The current namespace must contain a unique mapping.

Without confirmation, the stroke remains handwriting.

---

# Article 320 — Edge Case: Prefix Becomes Ambiguous After New Tool Added

Version 1 maps `R` to Rectangle.

A future release adds Radius as a command in the same namespace.

The previously unique prefix becomes ambiguous.

Craft Loop should avoid breaking learned muscle memory casually.

Command vocabulary versioning and reserved aliases can prevent this problem.

If `R` was an official shortcut, Radius should receive another prefix.

Command design is a long-term compatibility issue.

---

# Article 321 — Edge Case: Orthographic Command Without View Identity

The user has a structured sketch.

The user writes and confirms `Ortho`.

The source view has no identity.

The Orthographic Readiness Engine blocks the transition.

The interface asks:

“Which view is this?”

The user can write or tap Front, Top, Right, or Back.

The system then continues the requested operation.

The command is not lost.

---

# Article 322 — Edge Case: Orthographic Command With Raw Ink Only

The user draws a completely freehand object and labels it Front.

No geometry has been interpreted.

The user invokes Orthographic.

The system may enter a limited orthographic workspace but should clearly show that projection relationships are unavailable until relevant strokes are structured.

Alternatively, Version 1 may require at least minimal recognized geometry.

This policy must be tested.

The important principle is transparency.

---

# Article 323 — Edge Case: Orthographic Entry With No Dimensions

The user has clean structured geometry and a Front identity.

There are no dimensions.

Orthographic mode is allowed.

The system establishes view relationships without creating arbitrary sizes beyond the geometry currently drawn.

Unknown depth remains unresolved.

The user can add dimensions later.

This is a required supported path.

---

# Article 324 — Edge Case: Orthographic Entry With Conflicting Dimensions

The source view contains a blocking local conflict.

Entering Orthographic mode could propagate invalid assumptions.

The readiness engine should stop and explain the conflict.

The system can offer:

Resolve now.

Enter as unlinked draft.

Cancel.

Version 1 should choose the safest behavior through user testing.

It must not silently propagate the contradiction.

---

# Article 325 — Edge Case: View Label Changed

A user labels a view Front.

Later, the user changes it to Top.

The Orthographic Relationship Engine must reconsider axis mappings.

Existing cross-view relationships may become invalid.

The change should be treated as a significant transaction.

The system should preview consequences.

If the view already participates in many links, confirmation is appropriate.

---

# Article 326 — Edge Case: Duplicate View Identity

A document contains two View Blocks labeled Front.

This may be intentional if they represent alternative designs.

It may be invalid if both are linked into the same orthographic set.

The model should introduce the concept of an **Orthographic Set**.

View identity is unique within a linked set, not necessarily across the entire notebook.

This prevents artificial global restrictions.

---

# Article 327 — Orthographic Set

An Orthographic Set is a group of View Blocks that describe one linked design or one linked state of a design.

A notebook can contain multiple sets.

For example:

Concept A.

Concept B.

Revision 2.

Each set can have its own Front, Top, and Right views.

This concept prevents the document from assuming one object per page.

It also supports creative iteration.

The user should not need to manage set identifiers manually in simple workflows.

Spatial grouping and explicit commands can help.

---

# Article 328 — Edge Case: Two Alternative Designs on One Page

The user sketches two different brackets side by side.

Both have a Front view.

Craft Loop must not link them merely because their labels match.

Each belongs to a different Orthographic Set.

The user can initiate Orthographic from one selected view.

The system creates or joins the correct set.

This reinforces the need for semantic grouping.

---

# Article 329 — Edge Case: Moving a View Block

The user drags the Top view to another part of the page.

The engineering relationship should not break.

The system can update projection guides visually.

The semantic coordinate mapping remains.

If the user intentionally wants to detach the view, they should use Unlink or duplicate as independent content.

Page layout and engineering relation are separate concepts.

---

# Article 330 — Edge Case: Copying a Linked View

The user duplicates a Front view.

Should the copy remain linked to the same Orthographic Set?

The system should ask or use context.

Possible actions include:

Duplicate as linked representation.

Duplicate as independent sketch.

Duplicate entire orthographic set.

The default should avoid creating two representations that silently control the same geometry.

This workflow needs careful design.

---

# Article 331 — Edge Case: Deleting a Shared Dimension Annotation

The user deletes the visible `100 mm` label from Front.

The underlying shared dimension may still control Front and Top.

Deleting the annotation should not necessarily delete the semantic dimension.

The application should distinguish “hide annotation” from “remove dimension.”

A context action can expose both.

This distinction is essential for clean professional drawings.

---

# Article 332 — Edge Case: Removing a Driving Dimension

The user explicitly removes the semantic driving dimension.

The geometry may gain freedom.

The Constraint State Engine updates.

The related visible annotations disappear or become reference measurements according to policy.

Other views remain linked geometrically where other relationships exist.

The system should not preserve a hidden controlling value after the user intentionally removes it.

---

# Article 333 — Edge Case: Converting Driving Dimension to Reference

A user may want a displayed measurement that does not control geometry.

The dimension can be converted to reference.

The solver releases that degree of control.

The annotation updates as geometry changes.

This is a professional capability that can remain behind a contextual menu.

It should not complicate beginner workflows.

---

# Article 334 — Edge Case: Unit Change After Dimensioning

A document uses millimeters.

The user changes display units to inches.

Semantic values should not change physical size.

The display converts.

Constraints remain stable.

Handwritten original input can remain in history.

Export follows the chosen document or export unit policy.

The application should never scale geometry when merely changing display units.

---

# Article 335 — Edge Case: Imported Image With Scale

The user imports a scanned drawing.

The image has no known physical scale.

If the user wants to measure against it, the application must request calibration.

A user can identify a known length.

The reference image receives a scale transform.

This operation must not automatically convert image lines into structured geometry.

Reference calibration and geometry recognition are separate features.

---

# Article 336 — Edge Case: Low-Confidence Handwriting Offline

A cloud-based advanced recognizer is unavailable.

The on-device recognizer has low confidence.

The application should preserve ink.

It can allow manual typing.

It can retry recognition later if the user requests it.

Core drawing remains available.

Offline mode must fail gracefully.

---

# Article 337 — Edge Case: Solver Failure

The solver encounters a numerical failure even though the input may be theoretically valid.

The transaction should not corrupt existing geometry.

The system should return to the last stable state.

A diagnostic event should be recorded.

The user receives a simple message.

The application may offer to remove the latest action.

Engineering state remains safe.

---

# Article 338 — Edge Case: Autosave During Active Stroke

Autosave should not block the active stroke.

The Document Engine can snapshot the last stable state.

The in-progress stroke can be stored after completion.

If the application crashes mid-stroke, losing the unfinished stroke is preferable to corrupting the document.

This behavior should be tested.

---

# Article 339 — Edge Case: Crash During Cross-View Propagation

A dimension transaction updates several views.

The application crashes before persistence completes.

Transactional save should restore either the state before the operation or the complete committed state.

It should not restore half the views updated.

Atomic semantic transactions are therefore necessary.

---

# Article 340 — Edge Case: Document Schema Upgrade

A document created in MCP Version 1 is opened by a future version.

The migration system upgrades the schema.

Confirmed primitives remain confirmed.

Confirmed dimensions remain dimensions.

The new recognizer does not reinterpret old raw ink automatically unless explicitly requested.

Migration should create a backup or recovery path for major transformations.

Long-term document trust depends on migration quality.


# Article 341 — Visual Design Principle: Canvas Dominance

The canvas should visually dominate every editing screen.

Toolbars should feel secondary.

Panels should appear only when they provide immediate value.

The user should always understand where the drawing surface begins.

The interface should avoid permanent sidebars that reduce usable writing space on tablets.

When a temporary panel opens, it should respect the dominant hand and active geometry.

A compact canvas-first interface supports the notebook metaphor.

---

# Article 342 — Visual Design Principle: Calm Engineering

Craft Loop should avoid the visual density commonly associated with professional engineering software.

The goal is not to remove engineering depth.

The goal is to remove unnecessary visual noise.

Neutral surfaces, restrained accents, clear typography, generous spacing, and deliberate motion should create calm.

Technical states should be readable without covering the page in labels.

The user should feel that precision is available, not imposed.

---

# Article 343 — Visual Design Principle: Creative Warmth

A purely sterile interface can make the notebook feel clinical.

Craft Loop can use subtle material qualities associated with high-quality paper and creative tools.

The page background can be warm white rather than harsh pure white if readability remains excellent.

Ink can have enough texture to feel alive.

Structured geometry should remain crisp.

The contrast between expressive ink and formal geometry can become part of the product identity.

---

# Article 344 — Visual Design Principle: Technical Clarity

Creative warmth must never reduce technical clarity.

Dimension text must remain legible.

Line types must remain distinguishable.

Conflict indicators must remain clear.

Guides must not disappear against the page.

Export should produce clean technical output independent of the creative application chrome.

The product can feel artistic without making the engineering ambiguous.

---

# Article 345 — Visual Design Principle: One Accent System

Craft Loop should use a small semantic accent system.

A primary accent can indicate active tools and confirmed interactive focus.

A suggestion accent can indicate proposed relationships.

A warning accent can indicate conflict.

A link accent can indicate shared orthographic relationships.

The system should avoid using a unique color for every constraint type.

The visual vocabulary should remain learnable.

---

# Article 346 — Visual Design Principle: Ghost Geometry

Ghost Geometry is a core Craft Loop visual primitive.

It can represent:

Suggested primitive interpretation.

Unresolved orthographic extent.

Potential correspondence.

Projected guide.

Preview of a dimension change.

Ghost Geometry should be clearly different from confirmed geometry.

It should still be visible enough to invite interaction.

Opacity, line style, and animation can communicate the difference.

---

# Article 347 — Visual Design Principle: Conflict Without Alarm

Engineering conflicts matter.

The interface should not make every conflict feel catastrophic.

A warm highlight, subtle underline, or local badge may be sufficient.

The user can tap for explanation.

Blocking conflicts can receive stronger treatment.

Warnings can remain less intrusive.

The design should preserve flow while making incorrect engineering state impossible to miss at commit or export.

---

# Article 348 — Visual Design Principle: Motion as Explanation

Animation should answer the question:

“What just happened?”

When a handwritten `30` becomes a dimension, the transformation should be visible.

When a shared width updates the Top view, the related geometry can briefly emphasize.

When a command is accepted, the ink can morph or fade.

When a constraint locks, a subtle guide can settle.

These animations create understanding without explanatory dialogs.

---

# Article 349 — Visual Design Principle: Spatial Continuity

Contextual controls should appear near the object that caused them.

A dimension edit belongs near the dimension.

A recognition choice belongs near the stroke.

A conflict explanation belongs near the conflicting value.

A command ambiguity belongs near the command ink.

This spatial continuity reduces eye travel.

It also helps the user understand which object a choice affects.

---

# Article 350 — Visual Design Principle: Tool State

Active tool state must be unmistakable.

Pen, Eraser, Line, and other persistent tools should show clear selection.

Ink commands must update the same state.

Keyboard shortcuts must update the same state.

The cursor or hover indicator can reinforce state.

A user should never unknowingly remain in Eraser after expecting Pen.

The application can return to Pen automatically after one-shot operations where appropriate, but this behavior must be consistent.

---

# Article 351 — Visual Design Principle: Temporary Tool State

Some operations are one-shot.

For example, Add View may complete and return to the previous tool.

Other operations persist.

Line may remain active for repeated drawing.

The command definition should declare persistence behavior.

This keeps tool state predictable.

The interface should show when a temporary tool will end automatically.

---

# Article 352 — Visual Design Principle: Selection Without Chrome

Selection should avoid large bounding boxes when they are not needed.

A single line can highlight and expose endpoints.

A group can show a lightweight boundary.

A View Block can reveal its frame.

The system should not surround every selected object with desktop-style handles if direct manipulation can remain simpler.

Tablet interaction benefits from larger but fewer controls.

---

# Article 353 — Visual Design Principle: Dimension Typography

Dimension typography should be visually disciplined.

Numbers should remain easy to read at common zoom levels.

Decimal alignment should be consistent.

Degree, diameter, radius, and tolerance symbols should render correctly.

The application should support professional output without making on-screen dimensions visually heavy.

User handwriting can morph into this technical typography when it becomes a formal dimension.

---

# Article 354 — Visual Design Principle: Handwriting Preservation

Personal notes should retain human character.

Craft Loop should not turn the entire page into typed text.

The contrast between handwritten thought and structured dimensioning is valuable.

A design review note can look personal.

A formal dimension can look technical.

The page communicates both process and result.

---

# Article 355 — Visual Design Principle: View Identity

View labels such as FRONT, TOP, RIGHT, and BACK should be clear but not oversized.

The semantic identity can exist even if the visible label is hidden.

When a View Block is selected, the identity should be discoverable.

The product may use a small label chip during editing and a conventional label during export.

The representation can adapt without changing semantics.

---

# Article 356 — Visual Design Principle: Orthographic Set Identity

If multiple design alternatives exist on one page, Craft Loop should help the user understand which views belong together.

Possible methods include:

Subtle grouping.

Shared selection highlight.

Temporary connectors when a set is selected.

Set title.

Spatial organization.

The product should not permanently draw connection lines across the page.

The relationship is semantic.

---

# Article 357 — Visual Design Principle: Unresolved Information

Unresolved information should appear as an invitation rather than an error.

A missing depth can show a soft placeholder.

A missing correspondence can show a projected guide.

A missing view identity can show a small label prompt.

The user should understand what is missing without feeling blocked.

This supports progressive completion.

---

# Article 358 — Visual Design Principle: Confidence

The interface can map confidence to treatment.

High confidence may produce a strong preview.

Medium confidence may produce a ghost suggestion.

Low confidence may remain invisible until the user asks.

The application should avoid displaying raw confidence percentages during ordinary work.

Professionals can access diagnostic details if needed.

Confidence is a system variable, not necessarily a user-facing number.

---

# Article 359 — Visual Design Principle: Mode Transition

Entering Sketch or Orthographic context should not replace the entire application visually.

The page remains the page.

The toolbar adapts.

Relevant guides appear.

The interaction language changes subtly.

This continuity reinforces that Craft Loop is one notebook with deeper layers, not separate applications stitched together.

---

# Article 360 — Visual Design Principle: Exit and Recovery

A user should always know how to leave a context.

Visible toolbar control can return to Notebook state.

An ink command can exit.

Undo can reverse recent transitions.

Closing a contextual popover should not destroy work.

The application should not trap the user inside a mode.

---

# Article 361 — Interaction Principle: Preserve Flow

Every product decision should be evaluated against interruption cost.

A dialog interrupts flow more than an inline gesture.

A full-screen settings page interrupts more than a contextual choice.

A required keyboard interrupts a pen session.

This does not mean dialogs are forbidden.

It means the system should reserve them for decisions that genuinely require explicit attention.

Most drawing interactions belong on the canvas.

---

# Article 362 — Interaction Principle: Smallest Missing Fact

When Craft Loop cannot continue because information is missing, it should ask for the smallest fact that resolves the uncertainty.

If View Identity is missing, ask which view.

If a command prefix is ambiguous, ask for another letter or show two candidates.

If a dimension target is ambiguous, highlight the candidates.

If depth is missing, invite a depth dimension.

Do not open a general configuration panel when one answer is enough.

---

# Article 363 — Interaction Principle: Reversible Intelligence

Every machine interpretation should have a clear reversal path.

If a rough circle becomes a precise circle, undo returns the rough state.

If two features are linked across views, unlink is available.

If a suggested constraint is accepted, it can be removed.

If a command is misrecognized, undo restores prior tool state.

This makes intelligent behavior feel safe.

---

# Article 364 — Interaction Principle: Low Ceremony

Common actions should not require confirmation dialogs.

Low-risk reversible actions can happen directly.

The user should not confirm every line.

The user should not confirm every dimension if association is obvious.

The user should not confirm every shared-view propagation if the relationship is already explicit.

Confirmation is reserved for uncertainty, risk, or destructive change.

---

# Article 365 — Interaction Principle: Visible Consequence Before Commitment

For significant edits, Craft Loop should preview consequences.

Changing a shared width can temporarily show affected views.

Changing a view identity can preview reordered relationships.

Replacing a conflicting dimension can show resulting geometry.

The user sees what the decision means.

This is especially important when one handwritten number affects multiple views.

---

# Article 366 — Interaction Principle: Preserve Focus Point

After a command executes, the user’s visual focus should remain near the active drawing.

The application should not jump the viewport unnecessarily.

A tool change should not recenter the page.

A dimension commit should not move the camera.

Entering Orthographic may expand the workspace but should preserve orientation around the source view.

Spatial stability supports cognitive continuity.

---

# Article 367 — Interaction Principle: Context Follows Selection

Selection can drive contextual actions.

Selecting two lines can expose relationship options.

Selecting a circle can expose diameter and radius.

Selecting a dimension can expose edit, reference, or visibility controls.

The application should avoid requiring the user to choose a command before selecting when the inverse order is more natural.

Both orders can be supported through the shared Command Bus.

---

# Article 368 — Interaction Principle: Ink Follows Thought

The user should be able to write a note at any time.

Entering Sketch should not disable handwriting.

Entering Orthographic should not disable annotations.

The Ink Intent Engine uses context without turning context into a hard prison.

This is essential to the notebook identity.

---

# Article 369 — Interaction Principle: No Hidden Auto-Correction of Engineering Values

Craft Loop may refine handwriting.

It may fix a recognized character after user confirmation.

It must not change an engineering value to the nearest valid value without telling the user.

If `300` is impossible, it should not become `30`.

If `45°` conflicts with a perpendicular constraint, it should not silently become `90°`.

Engineering meaning requires explicit resolution.

---

# Article 370 — Interaction Principle: Suggestions Should Not Accumulate

A page should not fill with suggestion badges.

The system should prioritize the highest-value current suggestion.

Old ignored suggestions can fade.

Suggestions can become available again through an assistance command.

This protects visual calm.

The user should not feel managed by the application.

---

# Article 371 — Interaction Principle: Respect Deliberate Imperfection

A user may intentionally draw a nonstandard shape.

A line may intentionally be slightly curved.

A circle may intentionally be an ellipse.

Craft Loop should not worship geometric perfection.

Refinement is offered where intent supports it.

Creative ink remains legitimate.

This is one of the core differences between a notebook and a rigid drafting environment.

---

# Article 372 — Interaction Principle: Professional Override

A professional user should be able to override automatic behavior.

Disable automatic refinement.

Disable automatic constraint suggestions.

Disable command gestures.

Use explicit numeric entry.

Use manual view linking.

The product should remain usable even when intelligence is minimized.

This is important for trust and unusual workflows.

---

# Article 373 — Interaction Principle: Learn Without Forcing

Craft Loop can teach faster behaviors gradually.

A tooltip can reveal the ink command equivalent of a toolbar action.

A dimension gesture can be demonstrated after the user repeatedly opens Dimension manually.

A shortcut suggestion can be dismissed permanently.

Education should feel helpful.

It should not feel like an onboarding campaign.

---

# Article 374 — Interaction Principle: Consistency Across Platforms

The mental model should remain the same on iPad and Android tablets.

Pen is primary.

Canvas is central.

View identity works the same way.

Dimensions behave the same way.

Orthographic relationships behave the same way.

Platform-specific gestures can differ where hardware or operating-system expectations require it.

The semantic product should remain consistent.

---

# Article 375 — Interaction Principle: Native Where It Matters

Craft Loop should not fight platform conventions needlessly.

System share sheets should feel native.

Document pickers should feel native.

Text input should use platform keyboards.

Accessibility should integrate with platform services.

Pen behavior should use platform capabilities.

The distinctive experience belongs in the engineering canvas and semantic engines.

---

# Article 376 — Design System Component: Tool Button

A Tool Button should have:

Icon.

Active state.

Disabled state.

Hover state where supported.

Pressed state.

Accessible label.

Optional shortcut hint.

Optional ink-command hint.

The button should be large enough for touch.

It should remain compact enough for a tablet toolbar.

Tool state should not rely on color alone.

---

# Article 377 — Design System Component: Dimension Label

A Dimension Label should support:

Value.

Unit display policy.

Symbol.

Reference state.

Shared state indication when editing.

Conflict state.

Selection state.

Handwritten-to-technical transition.

Drag repositioning.

Visibility control.

The label must remain linked to its semantic dimension.

---

# Article 378 — Design System Component: View Block

A View Block should support:

Semantic identity.

Title.

Selection.

Move.

Resize viewport area without scaling engineering geometry unintentionally.

Orthographic set membership.

Local coordinate system.

Clip or open presentation according to design.

Guides.

Annotations.

The frame should be subtle when not selected.

The View Block is both a creative layout object and an engineering container.

---

# Article 379 — Design System Component: Command Preview

A Command Preview appears after a command is recognized but before or during execution.

For low-risk commands it can be extremely brief.

It can show:

Line.

Eraser.

Enter Orthographic from FRONT.

The preview confirms understanding.

It can provide a quick cancel path for ambiguous or higher-impact actions.

It should disappear automatically after successful execution.

---

# Article 380 — Design System Component: Conflict Card

A Conflict Card should remain small and contextual.

It should include:

Plain-language explanation.

Affected values.

Primary resolution choices.

Optional “Details.”

Undo or cancel.

It should avoid presenting equations unless the user requests details.

The card should not obscure the geometry being discussed.


# Article 381 — Architecture Principle: Domain Authority

Every important concept should have one authoritative owner.

Raw stroke authority belongs to the ink subsystem.

Confirmed vector geometry authority belongs to the geometry model.

Dimension authority belongs to the semantic dimension model.

Constraint authority belongs to the constraint graph.

Cross-view authority belongs to the orthographic relationship model.

Presentation state must not become engineering truth.

This prevents the interface from accidentally becoming the source of geometry.

---

# Article 382 — Architecture Principle: Unidirectional Meaning Flow

Interpretation should flow through explicit stages.

Raw input becomes candidate intent.

Candidate intent becomes structured proposal.

Proposal becomes validated engineering state.

Engineering state becomes presentation.

Presentation should not silently rewrite engineering state.

User interactions with presentation create commands that return through the domain path.

This architecture makes intelligent behavior traceable.

---

# Article 383 — Architecture Principle: Immutable Confirmed Semantics

Once a recognition result is accepted and stored as confirmed engineering semantics, a future recognition-model update should not reinterpret it automatically.

Recognition models can improve.

Documents must remain stable.

A line accepted as a line remains a line until the user edits it.

A dimension accepted as 30 millimeters remains 30 millimeters.

A view accepted as Front remains Front.

This protects long-term trust.

---

# Article 384 — Architecture Principle: Separate Suggestion From Truth

Suggestions must have their own storage state.

A suggested Parallel relationship is not the same as a confirmed Parallel constraint.

A suggested cross-view link is not a confirmed correspondence.

A suggested dimension association is not a driving dimension.

The data model should make it impossible for presentation code to confuse these categories accidentally.

This separation is critical when machine learning is introduced.

---

# Article 385 — Architecture Principle: Transactional Domain Operations

Domain changes should commit as transactions.

A dimension change may involve multiple internal objects.

The application should either commit the coherent result or keep the previous state.

Partial mutations are dangerous.

The transaction layer should support rollback when the solver rejects a change.

Undo history can operate on committed transactions.

Autosave can snapshot committed state.

---

# Article 386 — Architecture Principle: Deterministic Serialization

The same semantic state should serialize consistently.

Deterministic serialization aids:

Testing.

Diffing.

Sync.

Debugging.

Corruption detection.

Version migration.

Entity ordering should not change randomly.

Floating-point values should use stable precision policies.

The native file should include checksums or equivalent integrity mechanisms where useful.

---

# Article 387 — Architecture Principle: Schema Evolution

The Craft Loop document schema will evolve.

Every schema version should define:

Version identifier.

Migration path.

Deprecated fields.

Required fields.

Optional fields.

Backward-compatibility expectations.

Major changes should have explicit migration tests.

A document should never become unreadable merely because one engine was refactored.

---

# Article 388 — Architecture Principle: Engine Observability

Engineering engines need diagnostics.

The product should support internal tracing for:

Recognition decision.

Dimension association.

Constraint solve.

Orthographic propagation.

Conflict creation.

Command interpretation.

These traces should be developer diagnostics, not user-facing noise.

They can be anonymized and excluded from user content.

Observability is critical when multiple engines interact.

---

# Article 389 — Architecture Principle: Feature Flags

High-risk intelligent features should be controlled by feature flags during development.

Examples:

Automatic primitive suggestion.

Automatic constraint suggestion.

Ink commands.

Cross-view correspondence suggestions.

Experimental handwriting parser.

Feature flags allow staged testing.

They should not become permanent architectural fragmentation.

Successful behavior should eventually graduate to ordinary configuration or product state.

---

# Article 390 — Architecture Principle: Safe Degradation

If an optional intelligent subsystem fails, deterministic editing should continue.

If handwriting recognition fails, the user can type.

If cross-view suggestion is unavailable, manual linking remains.

If the cloud is unavailable, local drawing continues.

If haptics are unavailable, visual feedback remains.

If hover is unavailable, selection still works.

The application should degrade by losing convenience, not by losing the drawing.

---

# Article 391 — Architecture Principle: Capability Detection

The application should detect device capabilities at runtime.

It should know whether pressure is supported.

It should know whether hover is supported.

It should know whether advanced haptics are available.

It should know whether on-device handwriting recognition is available.

The user interface should not expose controls that cannot function.

Fallback paths should be defined.

---

# Article 392 — Architecture Principle: Cross-Platform Semantic Tests

The same semantic input should produce equivalent engineering results on iPad and Android.

A 30-millimeter line remains 30 millimeters.

A Parallel constraint solves equivalently.

A shared width propagates equivalently.

Rendering can differ slightly.

Semantics cannot.

A shared test corpus should run against both platform bindings.

---

# Article 393 — Architecture Principle: Numerical Reproducibility

Different devices may use different processor architectures.

The geometry engine should define acceptable numerical reproducibility.

Tiny floating-point differences should not produce visible divergence.

Serialized confirmed dimensions should remain exact semantic values where possible.

Derived coordinates can use controlled tolerances.

Tests should include cross-device document reopen.

---

# Article 394 — Architecture Principle: Coordinate Precision

Craft Loop should choose an internal numeric representation appropriate for engineering drawings.

Double-precision floating point is a common candidate.

Some dimensions may be stored as exact decimal semantic values plus solved coordinates.

The implementation should avoid repeated unit conversions that accumulate error.

Internal base units should be defined once.

Export conversions should be deterministic.

---

# Article 395 — Architecture Principle: Geometry Kernel Scope

Version 1 does not require a full three-dimensional geometry kernel.

It requires a robust two-dimensional geometry core.

The core should support:

Intersections.

Distances.

Angles.

Projection onto lines.

Circle-line relations.

Circle-circle relations.

Curve evaluation.

Bounding boxes.

Hit testing.

Constraint equations.

Offset operations only if a Version 1 feature requires them.

The team should not import a massive three-dimensional kernel merely to draw lines and circles.

---

# Article 396 — Architecture Principle: Dependency Discipline

Each engine should depend on the minimum lower-level concepts it needs.

The Standards Engine may read semantic geometry.

The Ink Engine should not depend on the Standards Engine.

The Orthographic Engine may depend on dimensions and geometry.

The Constraint Solver should not depend on user-interface components.

The Command Engine should dispatch domain actions rather than manipulate views directly.

This discipline reduces coupling.

---

# Article 397 — Architecture Principle: Background Work

Recognition and heavy analysis can run away from the main rendering thread where platform architecture allows.

The live stroke must remain responsive.

The application can use background workers, coroutines, tasks, or native equivalents.

Results should carry document revision identifiers.

A stale recognition result must not apply after the user changes the relevant ink.

This is especially important for future machine-learning inference.

---

# Article 398 — Architecture Principle: Stale Result Rejection

Every asynchronous intelligent operation should record the state it analyzed.

When the result returns, the system verifies that the source state is still relevant.

If the user erased or modified the stroke, the old recognition is stale.

If the user changed a dimension, an old conflict calculation may be stale.

If the view was unlinked, an old correspondence suggestion is stale.

Stale results should be discarded safely.

---

# Article 399 — Architecture Principle: Cancellation

Long-running operations should support cancellation.

The user may continue drawing.

The user may close the document.

The user may undo.

The user may replace imported content.

A cancelled operation must not commit later.

Cancellation state should be explicit.

This prevents delayed intelligence from surprising the user.

---

# Article 400 — Architecture Principle: Memory Budget

Large notebooks can contain many strokes and semantic objects.

Craft Loop should monitor memory use.

Raw stroke history may be compressed.

Off-screen rendering data may be evicted while semantics remain.

Reference images may be downsampled for editing while original assets remain available for export.

Undo history may use checkpoints and deltas.

The application must remain responsive during long sessions.

---

# Article 401 — Performance Requirement: Inking

In-progress ink should target the lowest practical latency supported by the platform.

Latency should be measured from hardware input to visible pixels.

The engineering engines should never sit synchronously inside this path.

Recognition begins after sufficient stroke data exists.

Any prediction used for low-latency rendering must remain visually stable.

Pen response is a release-blocking metric.

---

# Article 402 — Performance Requirement: Recognition

Simple primitive recognition should feel immediate after pen-up.

A line or circle suggestion should appear quickly enough that the user perceives it as part of the same action.

More expensive recognition can use progressive results.

A fast geometric candidate can appear before a slower machine-learning ranking.

The system should avoid delaying the user while waiting for maximal intelligence.

---

# Article 403 — Performance Requirement: Constraint Solve

Small sketches should solve within interactive frame budgets during drag.

Larger sketches can use incremental solving.

The engine should solve affected subgraphs rather than the entire document where possible.

Performance tests should include worst-case constraint networks.

If the solver cannot maintain interactivity, the interface should degrade gracefully rather than freeze.

---

# Article 404 — Performance Requirement: Orthographic Propagation

A shared dimension edit should update linked views perceptibly immediately.

The transaction may involve multiple view graphs.

Propagation should remain localized.

Rendering should animate the result after semantic commit.

The user should not see views update one by one over a long interval.

Atomic perception matters.

---

# Article 405 — Performance Requirement: Search

Document search can index recognized handwriting in background.

Search should not block drawing.

Index updates can lag slightly.

Confirmed semantic text should be indexed immediately where practical.

Private indexing should remain local unless cloud search is explicitly enabled.

---

# Article 406 — Performance Requirement: Autosave

Autosave must avoid visible pauses.

Small transactions can append to a journal.

Periodic compaction can run in background.

The application should avoid rewriting an entire large document after every stroke.

Crash recovery should replay stable operations.

The exact persistence strategy depends on platform and file format.

---

# Article 407 — Performance Requirement: Export

Export can take longer than interactive actions.

The product should still provide progress.

Large image references may increase time.

Vector export should use semantic geometry directly rather than screen capture.

The user can continue using other parts of the application if platform behavior permits.

Export failure should not affect the source document.

---

# Article 408 — Security Requirement: Data at Rest

Local documents may contain proprietary engineering information.

The application should use platform-provided protected storage where possible.

Cloud data should be encrypted at rest.

Backup policies should be documented.

Temporary export files should be cleaned according to platform behavior.

Security should be included in architecture reviews.

---

# Article 409 — Security Requirement: Data in Transit

Cloud synchronization and optional intelligence must use modern encrypted transport.

Authentication tokens should be protected.

Upload endpoints should validate file size and type.

The service should not expose raw notebook identifiers unnecessarily.

Network failures should not corrupt local state.

---

# Article 410 — Security Requirement: Account Separation

User documents must be isolated between accounts.

Shared documents require explicit authorization.

Deleted shares should revoke access according to product policy.

Future team workspaces need organization boundaries.

The simple Version 1 business model should not reduce the seriousness of data isolation.

---

# Article 411 — Security Requirement: Model Service Isolation

If future cloud machine-learning services process user content, service boundaries must be explicit.

The request should contain only the data required for the task.

The system should avoid uploading entire notebooks when a small stroke region is sufficient.

Logs should avoid storing sensitive drawing contents.

Retention policy should be documented.

Enterprise no-retention options may become important.

---

# Article 412 — Security Requirement: Imported Content

Imported portable documents and images should be treated as untrusted input.

The application should use safe decoders.

File size limits should prevent memory exhaustion.

Embedded scripts or active content should not execute.

The import pipeline should extract only the data required.

This applies especially if vector formats are added.

---

# Article 413 — Privacy Requirement: Consent

Any telemetry containing content-derived information should require careful review.

Basic performance metrics can often avoid collecting content.

Model improvement data requires explicit opt-in.

A user should be able to use Craft Loop without donating engineering drawings.

Privacy settings should be understandable.

---

# Article 414 — Privacy Requirement: Analytics

Useful analytics can include:

Crash events.

Latency.

Feature usage counts.

Command recognition success.

Undo after suggestion.

These events can often be recorded without storing raw drawings.

Analytics identifiers should be minimized.

The company should define retention and deletion policies.

---

# Article 415 — Reliability Requirement: Crash Recovery

The application should reopen the last document safely after a crash if the user chooses.

The autosave journal should recover committed transactions.

An incomplete transaction should be discarded.

The user should not see a corrupted half-propagated orthographic state.

Recovery tests should simulate forced termination.

---

# Article 416 — Reliability Requirement: Corruption Detection

The native file should detect malformed internal data.

A checksum, schema validation, or database integrity check can identify problems.

The application should attempt safe recovery from the last snapshot.

It should never overwrite the only good copy with a corrupted save.

Cloud backup can provide another recovery path.

---

# Article 417 — Reliability Requirement: Export Verification

Export tests should compare semantic source values with output.

A 30-millimeter dimension must remain 30 millimeters.

A dashed hidden-line role should map to the intended export line style.

View placement should follow the selected projection convention.

Fonts should remain available or embed appropriately.

Export is an engineering function, not only a graphics function.

---

# Article 418 — Reliability Requirement: Long Session Stability

Users may keep a notebook open for hours.

The application should test:

Thousands of strokes.

Repeated undo and redo.

Many recognition cycles.

Repeated orthographic edits.

Import and delete cycles.

Zooming across large spatial ranges.

Memory should stabilize rather than grow indefinitely.

Resource disposal is part of product quality.

---

# Article 419 — Reliability Requirement: Device Rotation

Tablet orientation may change.

The document coordinates must not change.

Toolbar layout can adapt.

Handedness preferences should remain.

Popovers should reposition.

The active pen operation should not corrupt if rotation occurs between strokes.

The user’s spatial context should be preserved where possible.

---

# Article 420 — Reliability Requirement: Background and Resume

Mobile operating systems suspend applications.

Craft Loop must save stable state before suspension where possible.

On resume, the active document should restore.

Stale asynchronous tasks should not apply blindly.

Network sync should reconcile.

The user should not lose recent confirmed dimensions because the tablet switched applications.


# Article 421 — Versioning Philosophy

MCP Version 1 is a living foundation.

The document describes the first coherent product model.

It should evolve through evidence.

Future versions can revise terminology.

They can change engine boundaries.

They can change implementation choices.

They should preserve the core human principles unless evidence demonstrates a stronger product direction.

Every major revision should document why it changed.

This creates institutional memory.

---

# Article 422 — What MCP Version 1 Freezes

MCP Version 1 should freeze several strategic decisions until explicitly reconsidered.

Craft Loop is tablet-first.

Version 1 is two-dimensional.

The product is notebook-first.

The user remains the designer.

Pen is the primary interaction language.

Keyboard is secondary.

Orthographic intelligence must not fabricate missing geometry.

Dimensions are semantic relationships.

Views are linked through shared engineering truth.

Three-dimensional reconstruction is out of scope.

These are foundational boundaries.

---

# Article 423 — What MCP Version 1 Does Not Freeze

MCP Version 1 does not permanently freeze:

Programming language.

Exact solver library.

Exact visual theme.

Pricing.

Cloud provider.

Final brand name.

Exact toolbar layout.

Exact gesture delay.

Exact command abbreviations.

Exact supported-device list.

These require implementation evidence and user testing.

The document should distinguish product principle from technical hypothesis.

---

# Article 424 — Research-to-Product Discipline

A research paper demonstrates possibility.

It does not automatically define product behavior.

Craft Loop should evaluate each research idea through:

Reliability.

Latency.

Mobile feasibility.

Explainability.

User control.

Licensing.

Data requirements.

Maintenance.

A model that performs well on a benchmark may still be unsuitable for interactive engineering use.

The product must translate research into controlled capability.

---

# Article 425 — Build Versus Reuse

Craft Loop should reuse platform infrastructure where it is not a source of differentiation.

Low-latency ink capture can use platform frameworks.

Basic file sharing can use platform frameworks.

The product should invest proprietary engineering effort where differentiation is strongest.

Examples include:

Ink Intent Engine.

Dimension semantics.

Constraint consistency.

Multiview graph.

Orthographic relationships.

Command grammar.

Human-in-the-loop assistance.

This allocation protects development capacity.

---

# Article 426 — Open-Source Evaluation

Open-source components should be evaluated for:

License compatibility.

Mobile portability.

Maintenance.

Numerical quality.

Performance.

Security.

Community health.

API stability.

A technically impressive library may still be unsuitable because of licensing.

The team should document license decisions.

This is especially important for geometric constraint solvers.

---

# Article 427 — Patent and Intellectual Property Review

Craft Loop introduces interaction patterns that may overlap with existing pen and computer-aided design patents.

Before commercial launch, the company should conduct appropriate intellectual property review.

This document is not a legal opinion.

Areas worth reviewing include:

Gesture command systems.

Handwriting-to-structured-object interaction.

Constraint inference.

Orthographic automation.

Specific user-interface mechanics.

The product should build original interaction language rather than assuming every published idea is unrestricted.

---

# Article 428 — Accessibility Review

Accessibility should be reviewed before final visual polish.

A design can become difficult to change after interaction assumptions become embedded.

The team should test:

Low vision.

Color blindness.

Motor variability.

Left-handed use.

Reduced motion.

External keyboard use.

Voice assistance where applicable.

Semantic labels in the document model create long-term accessibility opportunities.

---

# Article 429 — Engineering Expert Review

The product team should include practicing engineers or technical-drawing experts in review.

Product designers alone should not define engineering semantics.

Important review areas include:

Dimension conventions.

Projection methods.

Line semantics.

Constraint behavior.

Export.

Terminology.

Professional trust.

Expert review is especially important before marketing Version 1 as suitable for professional engineering communication.

---

# Article 430 — Novice Review

Experts can tolerate complexity that defeats beginners.

Craft Loop’s product thesis requires novice testing.

Novice users should be able to:

Draw.

Refine.

Dimension.

Understand a conflict.

Label a view.

Enter Orthographic.

Complete missing information.

The system should not require prior computer-aided design training.

If novices repeatedly ask what mode they are in, the interaction needs improvement.

---

# Article 431 — Hobbyist Review

Hobbyists represent a distinct group.

They may have strong practical knowledge without formal drafting education.

They value speed.

They may fabricate parts.

They may need clear communication with workshops.

Testing with hobbyists can reveal whether Craft Loop genuinely lowers the barrier without oversimplifying.

---

# Article 432 — Student Review

Students can reveal whether the application teaches or hides concepts.

Craft Loop should not remove the need to understand engineering.

It should remove unnecessary software friction.

A student should still learn that Front and Top share dimensions.

They should learn why impossible geometry fails.

The application can make concepts tangible through interaction.

---

# Article 433 — Professional Review

Professionals should evaluate:

Speed.

Precision.

Trust.

Export quality.

Constraint behavior.

Annotation clarity.

Whether the notebook fits real workflows.

Whether the simplicity becomes restrictive.

Professional feedback should guide the “deep on demand” layer.

---

# Article 434 — Workshop Review

Workshop environments create practical constraints.

Gloves.

Dust.

Bright light.

Fast annotation.

Device mounting.

Offline connectivity.

Simple sharing.

Craft Loop may not target rugged field use initially, but workshop testing can reveal valuable robustness issues.

---

# Article 435 — Education Opportunity

Craft Loop can become a powerful educational surface without building a learning management system.

Interactive orthographic relationships can show students how views relate.

Dimension conflicts can teach geometry.

Constraint suggestions can demonstrate design intent.

A future education mode could add explanatory overlays.

This remains secondary to the core product.

---

# Article 436 — Future Collaboration Opportunity

A future version can allow two users to discuss the same engineering notebook.

One person can annotate.

Another can respond.

Shared view relationships remain stable.

The collaboration model should respect semantic transactions.

A dimension edit should be a structured operation, not merely remote pixels.

This future requirement should influence document operation design now.

---

# Article 437 — Future Desktop Companion

A desktop companion may later provide:

Large-screen review.

Keyboard-heavy editing.

File organization.

Export management.

Enterprise administration.

It should not redefine the tablet-first interaction.

The tablet remains the primary creative surface.

A desktop client can consume the same semantic document model.

---

# Article 438 — Future Web Viewer

A web viewer may allow recipients to inspect Craft Loop documents without installing the application.

It can render views.

It can show dimensions.

It can allow comments.

Editing can remain limited initially.

The native semantic file format should therefore have a platform-neutral representation or service translation path.

---

# Article 439 — Future Artificial Intelligence Assistant

A future assistant can answer document-aware questions.

Examples:

“Which dimensions are unresolved?”

“Why is this value invalid?”

“Which views share this width?”

“Show me all conflicts.”

“Which constraints control this circle?”

The assistant should query the semantic graph.

It should not infer answers from a screenshot when structured data exists.

This improves accuracy.

---

# Article 440 — Future Intelligent Dimension Placement

A future layout engine can help place dimension annotations cleanly.

It can avoid overlaps.

It can respect standard conventions.

It can rank readable positions.

It should not change the underlying dimension.

The user can drag the annotation.

The engine can reflow nearby labels.

Research on dimension layout suggests that readability matters.

---

# Article 441 — Future Pattern Recognition

The application may recognize repeated holes, repeated spacing, or symmetric structures.

It can suggest:

Equal.

Pattern.

Symmetry.

Same diameter.

These suggestions can reduce repetitive dimensioning.

The user confirms.

Version 1 does not need a general pattern engine.

The graph should be able to add such relationships later.

---

# Article 442 — Future Design Memory

Craft Loop can remember design intent within a document.

If the user repeatedly sets holes equal, a new hole can receive a suggestion.

If a view uses a consistent spacing rule, the system can propose it.

This is not autonomous design.

It is memory of the user’s established choices.

The memory should remain document-scoped by default.

---

# Article 443 — Future Personal Interaction Model

With consent, Craft Loop can learn personal interaction preferences.

Preferred command aliases.

Typical handwriting.

Frequently used units.

Toolbar ordering.

Common document templates.

This personalization should improve interaction.

It should not modify engineering rules.

The user can reset personalization.

---

# Article 444 — Future Machine-Learned Constraint Ranking

A learned model can rank likely constraints.

Input can include:

Geometry.

Existing constraints.

Stroke order.

Visual context.

Historical accepted relationships.

The model outputs candidates.

The deterministic solver verifies feasibility.

The user accepts.

This is a natural application of research such as AutoConstrain and SketchGraphs-derived models.

---

# Article 445 — Future Machine-Learned Cross-View Linking

Cross-view correspondence may benefit from learned ranking.

A model can consider:

Shape.

Alignment.

Dimensions.

Neighborhood.

View type.

Repeated design patterns.

The model proposes likely links.

The user confirms uncertain cases.

Confirmed links become deterministic graph edges.

---

# Article 446 — Future Natural Language Notes

A future model can extract optional engineering hints from notes.

For example:

“Make these holes equal.”

The application could propose an Equal constraint.

This should never happen silently.

Natural-language notes remain notes unless the user invokes an assistant or confirms the action.

This protects the notebook from over-interpretation.

---

# Article 447 — Future Voice Input

Voice can become an optional secondary input.

A user might say:

“Set this to thirty millimeters.”

The command system can route the voice request to the same domain operation as handwriting or toolbar entry.

Voice is not part of the Hand First principle’s core.

It is an accessibility and convenience extension.

The command architecture should allow it later.

---

# Article 448 — Future Camera Input

A user may photograph a paper sketch.

Future recognition could propose structured geometry.

This is a different input pipeline from live ink.

The system lacks stroke timing and order.

Confidence must therefore be treated differently.

The first product should not depend on camera-to-engineering conversion.

The architecture can support it later.

---

# Article 449 — Future Scanned Drawing Interpretation

Existing technical drawings contain complex line conventions and dimension sets.

Research shows this is a substantial document-understanding problem.

Craft Loop can eventually import and interpret selected elements.

The feature should not be confused with simple image import.

It requires dedicated recognition, standards interpretation, and validation.

---

# Article 450 — Future Orthographic-to-Three-Dimensional Integration

A later product phase may use the semantic orthographic graph as input to three-dimensional modeling.

The key advantage is that Craft Loop can provide more than images.

It can provide:

View identities.

Dimensions.

Constraints.

Correspondences.

Design intent.

Unresolved information.

This richer input can reduce ambiguity.

Nevertheless, three-dimensional generation is outside Version 1.

---

# Article 451 — Future Integration With Generative Design

Generative design may eventually use Craft Loop constraints as design intent.

A user could sketch allowable dimensions or profiles.

A generative system could propose alternatives.

The human remains the designer.

This future should not influence Version 1 enough to add premature complexity.

Craft Loop must succeed as a two-dimensional notebook independently.

---

# Article 452 — Future Manufacturing Handoff

A future release may create stronger manufacturing handoff.

Drawing Exchange Format.

Standard drawing sheets.

Revision metadata.

Geometric dimensioning and tolerancing.

Manufacturing notes.

Sign-off.

These features require domain validation.

Version 1 should build clean semantics that make future handoff possible.

---

# Article 453 — Future Team Libraries

Organizations may want:

Templates.

Standards profiles.

Approved symbols.

Drawing styles.

Command vocabularies.

Export presets.

These can become shared team libraries.

The individual creative workflow should remain simple.

Enterprise administration belongs outside the drawing surface.

---

# Article 454 — Future Compliance Modes

Professional sectors may require strict standards.

A future Compliance Mode could validate:

Required views.

Dimension completeness.

Line conventions.

Title blocks.

Revision information.

Tolerance notation.

The product should only claim such compliance after expert and legal review.

This is intentionally not a Version 1 promise.

---

# Article 455 — Future Scripting

Power users may eventually want automation.

A safe scripting interface could operate on semantic geometry.

This is far beyond the initial notebook experience.

If introduced, it should not become necessary for ordinary users.

The command system should remain human-readable.

---

# Article 456 — Future Plug-In System

A plug-in ecosystem may eventually extend specialized engineering domains.

Examples could include:

Architecture.

Electronics.

Mechanical fabrication.

Woodworking.

Robotics.

A plug-in system creates security and compatibility complexity.

Version 1 should not include it.

The core document architecture can remain extensible.

---

# Article 457 — Future Domain Profiles

Different engineering domains use different conventions.

A future profile can adapt:

Symbols.

Units.

Templates.

Annotations.

Common constraints.

Export.

The underlying Ink, Geometry, Dimension, and Orthographic engines can remain shared.

Profiles should not fragment the product into separate applications prematurely.

---

# Article 458 — Future Architecture and Construction Use

Architecture uses orthographic plans, elevations, sections, scales, and annotations.

Craft Loop’s semantic notebook may eventually support these workflows.

However, architectural drawing has domain-specific requirements.

Version 1 should focus on general engineering drawing concepts.

Domain expansion should follow user evidence.

---

# Article 459 — Future Robotics Use

Robotics combines mechanical, electrical, and system-level sketches.

A future Craft Loop notebook could host linked mechanical views, wiring notes, sensor layouts, and annotations.

This is strategically interesting.

It also risks broadening scope too early.

The first release should solve its core drawing problem before becoming a multidisciplinary engineering notebook.

---

# Article 460 — Future Craft Ecosystem Position

Craft Loop can become the two-dimensional creative entry point in a larger Craft product family.

Its role can be:

Think.

Sketch.

Formalize.

Dimension.

Communicate.

Another product can later:

Model.

Generate.

Simulate.

Manufacture.

This separation can keep each product focused while allowing shared semantic data in the future.


# Article 461 — Term: Ink

Ink is the raw or visually expressive result of pen input before it necessarily becomes formal engineering structure.

Ink can represent handwriting, sketch lines, commands, gestures, notes, or candidate geometry.

Craft Loop must preserve the distinction between ink and structured geometry.

Ink is a first-class document concept.

---

# Article 462 — Term: Digital Ink

Digital Ink is ink represented as sampled stylus data rather than as a static image.

It can include positions, timestamps, pressure, tilt, and other input properties.

Digital Ink provides richer evidence for recognition than a screenshot.

Craft Loop should retain this richness where practical.

---

# Article 463 — Term: Online Ink Recognition

Online Ink Recognition interprets strokes while preserving the sequence and timing of their creation.

This differs from recognizing a raster image after drawing is complete.

Craft Loop benefits from online recognition because timing, order, and motion help disambiguate intent.

---

# Article 464 — Term: Offline Ink Recognition

Offline Ink Recognition interprets an image of handwriting or drawing without access to original stroke timing.

This approach may be useful for imported scans.

It provides less interaction context than live Craft Loop ink.

---

# Article 465 — Term: Primitive

A Primitive is a basic structured geometric entity.

Examples include line, arc, circle, ellipse, and point.

Primitives are the building blocks of a structured sketch.

---

# Article 466 — Term: Parametric Geometry

Parametric Geometry is geometry controlled by explicit parameters and relationships.

A circle may be defined by center and radius.

A line may be defined by endpoints and constraints.

Craft Loop uses parametric ideas in two dimensions without requiring three-dimensional modeling.

---

# Article 467 — Term: Constraint

A Constraint is a rule that restricts how geometric entities can relate or move.

Examples include parallel, perpendicular, tangent, equal, coincident, and concentric.

Constraints encode design intent.

---

# Article 468 — Term: Geometric Constraint

A Geometric Constraint controls relationships between geometry rather than assigning a direct numeric value.

Parallel and tangent are examples.

Craft Loop should distinguish these from dimensional constraints internally.

---

# Article 469 — Term: Dimensional Constraint

A Dimensional Constraint establishes a numerical relationship.

Length, distance, angle, radius, and diameter are examples.

A dimensional constraint can drive geometry.

---

# Article 470 — Term: Driving Dimension

A Driving Dimension controls the geometry.

Changing the value changes the drawing according to constraints.

Craft Loop may hide this term from beginners while preserving the behavior.

---

# Article 471 — Term: Reference Dimension

A Reference Dimension reports a measurement without controlling geometry.

Its displayed value changes when the geometry changes.

Reference dimensions are useful for professional inspection and documentation.

---

# Article 472 — Term: Derived Dimension

A Derived Dimension is calculated from other confirmed relationships.

The user does not independently control it unless they change the constraint structure.

Craft Loop can display derived values with a subtle visual distinction.

---

# Article 473 — Term: Shared Dimension

A Shared Dimension represents one semantic quantity used by multiple entities or views.

The width shared by Front and Top is an example.

Multiple annotations can point to the same Shared Dimension.

---

# Article 474 — Term: Bounded Dimension

A Bounded Dimension is not uniquely known but is restricted to a valid range.

The third side of a triangle with two fixed sides can be bounded.

Craft Loop can use this state to explain invalid values intelligently.

---

# Article 475 — Term: Degree of Freedom

A Degree of Freedom is an independent way geometry can still move or change.

Traditional computer-aided design uses this concept to describe under-constrained sketches.

Craft Loop should use the concept internally while presenting simpler language to most users.

---

# Article 476 — Term: Under-Constrained

Under-Constrained geometry still has one or more degrees of freedom.

In Craft Loop this is often a normal exploratory state.

Under-constrained should not be framed as inherently wrong.

---

# Article 477 — Term: Fully Constrained

Fully Constrained geometry has enough relationships to determine its relevant position and shape.

This can provide predictable editing.

Craft Loop should not require full constraint for every notebook sketch.

---

# Article 478 — Term: Over-Constrained

Over-Constrained usually refers to relationships that redundantly or inconsistently attempt to control already determined geometry.

Craft Loop should translate this into plain-language conflict explanations.

---

# Article 479 — Term: Design Intent

Design Intent is the set of relationships the designer wants preserved when the drawing changes.

Symmetry, equal spacing, parallel edges, and shared diameters can express intent.

Craft Loop treats design intent as more important than merely matching a current picture.

---

# Article 480 — Term: Constraint Graph

A Constraint Graph represents geometric entities and the relationships between them as a graph.

SketchGraphs research uses this approach.

Craft Loop should use a related relational model for its structured drawing core.

---

# Article 481 — Term: Design Intent Graph

The Design Intent Graph is Craft Loop’s conceptual layer for user-confirmed structural relationships.

It distinguishes intentional relationships from accidental geometric coincidence.

It supports predictable edits.

---

# Article 482 — Term: Multiview Constraint Graph

The Multiview Constraint Graph connects geometry, dimensions, and correspondences across orthographic View Blocks.

It is the core internal representation for Orthographic Intelligence.

It does not require a three-dimensional solid.

---

# Article 483 — Term: Orthographic Projection

Orthographic Projection represents an object through views projected perpendicular to principal planes.

Front, Top, and Right are common views.

Craft Loop supports linked orthographic drawing without three-dimensional reconstruction in Version 1.

---

# Article 484 — Term: Principal View

A Principal View is one of the standard directional representations of an object.

Examples include Front, Top, Right, Left, Back, and Bottom.

Craft Loop initially focuses on the views most useful for the Version 1 workflow.

---

# Article 485 — Term: View Identity

View Identity is the semantic declaration that a View Block represents Front, Top, Right, Back, or another supported view.

Craft Loop requires explicit identity before it relies on view semantics.

---

# Article 486 — Term: View Block

A View Block is a structured two-dimensional engineering view placed on the notebook canvas.

It has geometry, identity, dimensions, annotations, and links.

It can move spatially without losing engineering meaning.

---

# Article 487 — Term: Orthographic Set

An Orthographic Set is a group of linked View Blocks representing one design state.

A notebook can contain multiple sets.

This supports alternatives and revisions on the same page.

---

# Article 488 — Term: Projection Convention

A Projection Convention defines how orthographic views are arranged and interpreted.

First-angle and third-angle projection are important examples.

Craft Loop should store the convention explicitly.

---

# Article 489 — Term: First-Angle Projection

First-Angle Projection is an established orthographic projection method.

Its view arrangement differs from third-angle projection.

Craft Loop should follow the selected standards profile rather than inventing its own arrangement.

---

# Article 490 — Term: Third-Angle Projection

Third-Angle Projection is another established orthographic projection method commonly associated with American engineering practice.

Craft Loop should support it as a document or orthographic-set setting.

---

# Article 491 — Term: Cross-View Correspondence

Cross-View Correspondence is the semantic relationship indicating that entities in different views represent the same feature or shared engineering quantity.

Correspondence may be deterministic or suggested.

Confirmed correspondence enables propagation.

---

# Article 492 — Term: Orthographic Intelligence

Orthographic Intelligence is Craft Loop’s combined capability for understanding view identity, shared dimensions, correspondences, unresolved information, conflicts, and projection relationships.

It is not a generative image model.

It is a relational engineering system.

---

# Article 493 — Term: Orthographic Relationship Engine

The Orthographic Relationship Engine is the deterministic domain engine that maintains links among views.

It reads the Multiview Constraint Graph.

It coordinates propagation and validation.

---

# Article 494 — Term: Orthographic Readiness

Orthographic Readiness describes whether a source view has sufficient structure and identity to participate in linked multiview work.

It does not require complete dimensioning.

Readiness can be partial.

---

# Article 495 — Term: Progressive Geometric Resolution

Progressive Geometric Resolution is the process by which unknown engineering information becomes known as the user adds dimensions, constraints, or correspondences.

A drawing can move from vague to precise without a single conversion event.

---

# Article 496 — Term: Progressive Engineering Formalization

Progressive Engineering Formalization is the broader process by which freehand thought becomes structured engineering representation.

It includes recognition, refinement, dimensioning, constraints, and view linking.

The user controls the pace.

---

# Article 497 — Term: Semantic Paper

Semantic Paper is Craft Loop’s document metaphor for a canvas containing different semantic object types while remaining visually notebook-like.

It allows notes, geometry, dimensions, commands, and views to coexist.

---

# Article 498 — Term: Semantic Dimension

A Semantic Dimension is the underlying engineering relationship and value.

It exists independently from whether a visible label is currently shown.

This enables clean multiview drawings.

---

# Article 499 — Term: Dimension Annotation

A Dimension Annotation is the visible graphical representation of a semantic dimension.

It can be hidden, moved, or duplicated without creating a new underlying engineering value.

---

# Article 500 — Term: Dimension Association

Dimension Association is the process of determining which geometric relationship a handwritten or typed value controls or describes.

It combines spatial, temporal, contextual, and semantic evidence.

---

# Article 501 — Term: Ink Intent Engine

The Ink Intent Engine classifies recent strokes into plausible semantic categories.

It protects the notebook from aggressive over-interpretation.

It routes strokes to specialized engines.

---

# Article 502 — Term: Engineering Handwriting Parser

The Engineering Handwriting Parser converts recognized text into structured engineering values such as length, angle, radius, diameter, or tolerance candidates.

It does not decide the geometric target.

---

# Article 503 — Term: Primitive Recognition Engine

The Primitive Recognition Engine interprets eligible strokes as geometric primitive candidates.

It can use deterministic fitting, machine learning, or a hybrid approach.

---

# Article 504 — Term: Beautification Engine

The Beautification Engine refines an accepted geometric interpretation into a clean representation.

It minimizes unnecessary displacement.

It supports Ink Morphing.

---

# Article 505 — Term: Ink Morphing

Ink Morphing is the visual transition from human-drawn ink to structured geometry or technical text.

It communicates continuity.

It reinforces that the system refines rather than replaces the user’s work.

---

# Article 506 — Term: Ink Command Language

The Ink Command Language allows users to invoke application actions by writing commands with the pen.

Commands remain secondary to visible tools for discoverability.

The language is contextual.

---

# Article 507 — Term: Confirmation Gesture

A Confirmation Gesture converts candidate command ink into an intentional command.

Craft Loop initially proposes an enclosing circle.

Context and timing prevent accidental execution.

---

# Article 508 — Term: Contextual Command Grammar

The Contextual Command Grammar defines the valid command vocabulary within the current interaction context.

It reduces ambiguity.

Notebook, Sketch, and Orthographic contexts can expose different command sets.

---

# Article 509 — Term: Command Namespace

A Command Namespace is the set of commands available in one context.

Shortcuts are resolved within the namespace.

This allows concise expert commands without a globally crowded vocabulary.

---

# Article 510 — Term: Shortest Unique Prefix

The Shortest Unique Prefix is the shortest command prefix that uniquely identifies one valid action in the active namespace.

Ambiguous prefixes never execute automatically.

---

# Article 511 — Term: Ephemeral Ink

Ephemeral Ink is pen input used for interaction rather than permanent document content.

Command words and confirmation marks may be ephemeral.

The event remains in history even if the ink disappears.

---

# Article 512 — Term: Temporal Disambiguation

Temporal Disambiguation uses stroke timing to help infer intent.

A circle drawn immediately around a new command can mean confirmation.

A circle around older content can mean selection.

Timing is combined with other context.

---

# Article 513 — Term: Pen-Native Interface

A Pen-Native Interface is designed around pen interaction from the beginning.

The pen is not merely a replacement pointer.

Craft Loop aims to be pen-native.

---

# Article 514 — Term: Hand First, Type Second

Hand First, Type Second means the pen is the primary language of Craft Loop.

The user can draw, dimension, annotate, label, and invoke tools by hand.

Typing remains available as a secondary path.

---

# Article 515 — Term: Human-in-the-Loop

Human-in-the-Loop means automated or learned systems assist while the user retains decision authority.

Craft Loop uses this principle for recognition, constraint suggestions, correspondence, and future machine intelligence.

---

# Article 516 — Term: Assistive Intelligence

Assistive Intelligence augments the user rather than replacing the user.

It interprets, suggests, ranks, and explains.

It does not silently own engineering intent.

---

# Article 517 — Term: Deterministic Automation

Deterministic Automation performs actions whose result follows from confirmed engineering rules.

Shared-width propagation is an example.

This is different from probabilistic suggestion.

---

# Article 518 — Term: Ambiguity Engine

The Ambiguity Engine tracks uncertainty and identifies what missing fact would resolve it.

It prevents unknown information from becoming fabricated certainty.

---

# Article 519 — Term: Ghost Geometry

Ghost Geometry is a visual representation of suggested, projected, previewed, or unresolved geometry.

It is not confirmed engineering geometry.

Its visual treatment communicates uncertainty.

---

# Article 520 — Term: Conflict Object

A Conflict Object is a structured representation of an engineering contradiction.

It records affected entities, values, explanation, severity, and resolution options.

This supports clear recovery.


# Article 521 — Term: Engineering Consistency

Engineering Consistency means the geometry, dimensions, constraints, units, and cross-view relationships can coexist without contradiction.

Consistency is broader than visual correctness.

Craft Loop continuously protects it.

---

# Article 522 — Term: Geometric Consistency Engine

The Geometric Consistency Engine validates semantic coherence across the document.

It coordinates local geometry, constraints, dimensions, and cross-view relationships.

It produces structured conflicts and warnings.

---

# Article 523 — Term: Constraint State Engine

The Constraint State Engine interprets how determined or unresolved geometry and dimensions are.

It supports visual state, command availability, and readiness logic.

---

# Article 524 — Term: Snap

Snap is temporary attraction or alignment during interaction.

Snap does not necessarily create a permanent constraint.

Craft Loop should preserve this distinction.

---

# Article 525 — Term: Smart Guide

A Smart Guide is temporary visual geometry that communicates alignment, equal spacing, projection, or another relationship during interaction.

It disappears when no longer relevant.

---

# Article 526 — Term: Construction Geometry

Construction Geometry is auxiliary geometry used to establish relationships without representing a visible final edge.

Centerlines are an important example.

Construction geometry can participate in constraints.

---

# Article 527 — Term: Centerline

A Centerline identifies an axis or center relation.

It is useful for symmetry, circles, alignment, and technical representation.

Centerlines have specific line conventions in formal drawings.

---

# Article 528 — Term: Hidden Line

A Hidden Line represents an edge or feature not directly visible from a particular view.

Craft Loop must preserve the engineering meaning of dashed line conventions.

Automatic hidden-line inference is outside the initial capability unless justified by sufficient geometry.

---

# Article 529 — Term: Leader Line

A Leader Line connects an annotation to the feature it describes.

Standards define conventions for leader and reference lines.

Craft Loop should treat leaders as semantic annotation connectors.

---

# Article 530 — Term: Witness Line

A Witness Line, often called an extension line in dimensioning contexts, helps define the geometry referenced by a dimension.

Craft Loop’s dimension system should understand it semantically rather than only graphically.

---

# Article 531 — Term: Dimension Line

A Dimension Line visually communicates the span or angle associated with a dimension.

It is part of presentation.

The semantic dimension exists independently.

---

# Article 532 — Term: Drawing Scale

Drawing Scale describes the relationship between represented size and physical output size.

It is not the same as screen zoom.

Craft Loop should preserve real semantic dimensions independently of display scale.

---

# Article 533 — Term: Document Unit

Document Unit is the default unit used when a numeric input omits an explicit unit.

Examples include millimeters or inches.

The internal representation should normalize quantities consistently.

---

# Article 534 — Term: Unit Normalization

Unit Normalization converts user input into a canonical internal unit without changing physical meaning.

It prevents unit mismatches.

Display can later use a different unit.

---

# Article 535 — Term: Local Coordinate Frame

A Local Coordinate Frame defines coordinates inside a View Block.

Moving the View Block on the page does not change these engineering coordinates.

This separates layout from design.

---

# Article 536 — Term: Page Coordinate System

The Page Coordinate System describes where objects appear on semantic paper.

It is a presentation space.

It is not the same as engineering coordinates inside a view.

---

# Article 537 — Term: Two-and-a-Half-Dimensional Intent Model

The Two-and-a-Half-Dimensional Intent Model describes linked two-dimensional views with shared extents and feature relationships but without a complete three-dimensional solid.

It is an internal conceptual term for Craft Loop Version 1.

---

# Article 538 — Term: Semantic Geometry

Semantic Geometry is structured geometry that carries meaning beyond appearance.

A line can know that it is part of a view.

A circle can know that it has a diameter dimension.

This distinguishes Craft Loop from generic vector drawing.

---

# Article 539 — Term: Visual Geometry

Visual Geometry is the rendered representation of semantic or raw entities.

Visual Geometry can change style without changing engineering meaning.

---

# Article 540 — Term: Provenance

Provenance records where a semantic result came from.

It can distinguish user-created, system-suggested, user-accepted, derived, and propagated state.

Provenance supports trust and debugging.

---

# Article 541 — Term: Transaction

A Transaction is an atomic engineering operation that may change multiple internal objects while appearing as one user action.

Undo and autosave should operate on stable transactions.

---

# Article 542 — Term: Atomic Commit

Atomic Commit means a transaction is either applied completely or not applied.

A cross-view dimension change should not leave one view updated and another stale.

---

# Article 543 — Term: Stale Result

A Stale Result is an asynchronous computation based on an older document state.

Craft Loop should reject stale recognition and intelligence results.

---

# Article 544 — Term: Revision Identity

Revision Identity identifies the semantic document state used by an asynchronous process.

Results can verify that the revision is still relevant before applying.

---

# Article 545 — Term: Local-First

Local-First means core work remains available and authoritative on the device.

Cloud services extend synchronization, sharing, backup, and optional intelligence.

They do not sit in the live pen path.

---

# Article 546 — Term: Offline-First Interaction

Offline-First Interaction means the user can begin and continue ordinary work without a network connection.

Craft Loop targets this behavior for core Version 1 drawing.

---

# Article 547 — Term: Schema

A Schema defines how Craft Loop semantic data is structured in storage.

The schema must be versioned.

Future application releases migrate old documents explicitly.

---

# Article 548 — Term: Serialization

Serialization converts the in-memory semantic model into durable data.

Serialization should be deterministic and safe.

---

# Article 549 — Term: Migration

Migration transforms an older document schema into a newer supported form.

It should preserve confirmed semantics.

Recognition should not be rerun automatically as migration.

---

# Article 550 — Term: Interoperability

Interoperability is the ability to exchange useful output with other systems.

Portable Document Format, Scalable Vector Graphics, and future Drawing Exchange Format support interoperability.

---

# Article 551 — Term: Native Document

The Native Document is the full Craft Loop representation.

It contains more semantic information than exported graphics.

It remains the authoritative editable source.

---

# Article 552 — Term: Portable Document Format

Portable Document Format is a primary sharing and printing format.

Craft Loop should produce vector technical output where practical.

The export does not contain the full native semantic graph.

---

# Article 553 — Term: Scalable Vector Graphics

Scalable Vector Graphics is a vector exchange format useful for creative and technical workflows.

It preserves graphical vectors but not all Craft Loop semantics.

---

# Article 554 — Term: Drawing Exchange Format

Drawing Exchange Format is a widely used engineering exchange format.

It is a strategic future bridge to traditional computer-aided design.

Craft Loop should validate mappings carefully before release.

---

# Article 555 — Term: Standards-Aware

Standards-Aware means Craft Loop understands selected technical-drawing conventions and can apply them.

It does not automatically mean formally certified compliance.

---

# Article 556 — Term: Standards-Compliant

Standards-Compliant is a stronger claim requiring the relevant implemented output to satisfy the applicable standard requirements.

Craft Loop should use this claim only after formal review.

---

# Article 557 — Term: Progressive Disclosure

Progressive Disclosure reveals advanced capabilities when they become relevant.

It supports beginners and professionals in one interface.

---

# Article 558 — Term: Contextual Toolbar

A Contextual Toolbar changes available tools according to current activity, selection, and valid operations.

It reduces persistent interface density.

---

# Article 559 — Term: Adaptive Interface

An Adaptive Interface changes presentation or available actions based on context.

Craft Loop uses adaptation to reduce irrelevant controls.

Engineering semantics remain stable.

---

# Article 560 — Term: Direct Manipulation

Direct Manipulation means users interact with the object or value itself rather than operating through distant configuration dialogs.

Dragging a dimension label or line endpoint is an example.

---

# Article 561 — Term: Inline Editing

Inline Editing changes values or content near their visual representation.

Craft Loop should prefer inline dimension editing to distant numeric property panels.

---

# Article 562 — Term: Muscle Memory

Muscle Memory is learned physical familiarity with repeated interaction.

Short ink commands and stable gesture locations can become faster over time.

Craft Loop should preserve shortcut compatibility where practical.

---

# Article 563 — Term: Discoverability

Discoverability is the ability to learn what actions exist without prior memorization.

Visible toolbars and contextual hints provide discoverability.

Ink commands accelerate work after discovery.

---

# Article 564 — Term: Cognitive Load

Cognitive Load is the mental effort required to operate the software in addition to thinking about the engineering problem.

Craft Loop aims to reduce avoidable operational cognitive load.

It does not remove the intellectual content of engineering.

---

# Article 565 — Term: Context Switching

Context Switching occurs when the user must shift attention between drawing, keyboard, menus, dialogs, and numerical panels.

Hand First, Type Second is partly designed to reduce context switching.

---

# Article 566 — Term: Interaction Flow

Interaction Flow is the continuity of thought and action while working.

Craft Loop prioritizes flow by keeping common interactions on the canvas.

---

# Article 567 — Term: Human Factors

Human Factors examines how people perceive, understand, and operate systems.

Craft Loop is as much a human-factors project as a computational-geometry project.

---

# Article 568 — Term: Affordance

An Affordance is a visual or interactive cue that suggests how an element can be used.

Handles, ghost guides, and command previews are Craft Loop affordances.

---

# Article 569 — Term: Progressive Certainty

Progressive Certainty is the transition from unknown or suggested information to confirmed engineering truth as the user provides evidence.

It is closely related to Progressive Geometric Resolution.

---

# Article 570 — Term: Confidence

Confidence is an internal estimate of how likely an interpretation is correct.

Confidence controls suggestion behavior.

It does not replace geometric validation.

---

# Article 571 — Term: Suggestion

A Suggestion is a system-proposed interpretation or relationship that has not yet become confirmed engineering truth.

Suggestions are reversible and rejectable.

---

# Article 572 — Term: Confirmation

Confirmation is an explicit user action or deterministic rule that promotes a proposal into accepted state.

Machine learning alone does not constitute confirmation for ambiguous design intent.

---

# Article 573 — Term: Conflict

A Conflict is an inconsistency between confirmed or proposed engineering statements.

Conflicts require explanation and resolution.

They should not be hidden.

---

# Article 574 — Term: Warning

A Warning identifies a potential problem that may not make the current geometry impossible.

Warnings can be nonblocking.

Craft Loop should distinguish warnings from conflicts.

---

# Article 575 — Term: Blocker

A Blocker prevents a specific operation because required information or valid state is missing.

Missing View Identity before semantic Orthographic entry is an example.

A blocker should explain the smallest action needed to continue.

---

# Article 576 — Term: Readiness

Readiness describes whether the current document state can support a requested workflow.

Orthographic Readiness is a specific example.

Readiness can be graded.

---

# Article 577 — Term: Human Override

Human Override is the user’s ability to reject, remove, or replace automated suggestions and interpretations.

Craft Loop treats this as a fundamental right of the designer.

---

# Article 578 — Term: Safe Degradation

Safe Degradation means optional capabilities can fail or become unavailable without damaging core drawing.

Offline handwriting fallback is an example.

---

# Article 579 — Term: Deterministic Core

The Deterministic Core contains geometry, constraints, units, validated relationships, and document semantics whose behavior must be reproducible.

Probabilistic intelligence sits around this core.

---

# Article 580 — Term: Probabilistic Assistance

Probabilistic Assistance uses uncertain models to interpret or rank human intent.

It must remain separate from deterministic engineering truth.


# Article 581 — Requirement Group: Notebook Entry

Craft Loop shall open to a notebook library rather than a technical project wizard.

A new document shall be creatable with one clear action.

The user shall be able to begin drawing without selecting a drawing standard first.

The user shall be able to draw before creating a Sketch context.

The application shall autosave newly created documents.

The application shall allow renaming at any time.

The application shall not require network connectivity to create a document.

The initial visual state shall prioritize the canvas.

The interface shall not present a three-dimensional viewport.

The product shall not use three-dimensional terminology as a prerequisite for ordinary Version 1 work.

---

# Article 582 — Requirement Group: Pen Input

Craft Loop shall support stylus input on supported iPad devices.

Craft Loop shall support stylus input on supported Android tablets.

Pen-down ink shall appear with low latency.

Palm contacts shall not routinely create unintended strokes.

Pressure shall be used when supported and when the selected pen style benefits.

Tilt shall be available as an optional expressive signal.

The semantic engine shall not require pressure or tilt to function.

Stroke data shall include timestamps.

Completed strokes shall be available to recognition engines.

The user shall be able to disable pressure-sensitive width if desired.

---

# Article 583 — Requirement Group: Navigation

The user shall be able to pan the canvas.

The user shall be able to zoom smoothly.

Pan and zoom shall not change engineering dimensions.

The user shall be able to fit a selected View Block.

The user shall be able to recover from becoming spatially lost on a large canvas.

Navigation shall not accidentally create ink while palm rejection is active.

Navigation shall remain responsive in large documents.

Rotation of the device shall not rotate engineering geometry unexpectedly.

---

# Article 584 — Requirement Group: Ink Preservation

Raw ink shall remain available until an interpretation transaction commits.

The user shall be able to undo refinement.

The system shall not force all ink into structured geometry.

Notes shall remain ordinary ink unless converted.

Command ink shall be distinguishable from permanent ink.

Original handwriting may be retained in history according to storage policy.

Recognition failure shall preserve the ink.

Recognition uncertainty shall not delete ink.

---

# Article 585 — Requirement Group: Primitive Recognition

Version 1 shall recognize line candidates.

Version 1 shall recognize circle candidates.

Version 1 shall recognize arc candidates.

Version 1 shall recognize ellipse candidates if quality is sufficient.

Version 1 shall recognize rectangle or quadrilateral candidates according to implementation.

The recognizer shall provide confidence.

The recognizer shall allow “keep as ink.”

The recognizer shall not apply permanent constraints merely by recognizing a shape.

The recognizer shall preserve document revision context.

Stale recognition results shall not apply.

---

# Article 586 — Requirement Group: Refinement

The user shall be able to refine a recognized primitive.

Draw-and-hold shall be evaluated as a primary refinement gesture.

Refinement shall visually maintain continuity with the original stroke.

Refinement shall be reversible.

Refinement shall not move geometry an unreasonable distance from the intended stroke.

The user shall be able to choose another candidate when recognition is ambiguous.

The engine shall not repeatedly suggest a rejected interpretation without changed evidence.

---

# Article 587 — Requirement Group: Structured Lines

A structured line shall have exact endpoints.

A line shall expose length as a measurable quantity.

A line shall participate in constraints.

A line shall support endpoint snapping.

A line shall support horizontal and vertical relationships.

A line shall support parallel and perpendicular relationships with other lines.

Line presentation shall be separate from line semantics.

---

# Article 588 — Requirement Group: Structured Circles

A structured circle shall have exact center and radius.

A circle shall support radius dimensions.

A circle shall support diameter dimensions.

A circle shall support concentric constraints.

A circle shall support tangent relationships where solver support exists.

The visible circle style shall not alter its geometry.

---

# Article 589 — Requirement Group: Arcs

An arc shall have a stable parametric representation.

An arc shall support radius or diameter where meaningful.

An arc shall support endpoint relationships.

An arc shall support tangency where solver support exists.

Arc selection shall distinguish the arc from a full circle.

---

# Article 590 — Requirement Group: Handwriting

Craft Loop shall accept handwritten numbers.

Craft Loop shall accept handwritten engineering labels.

Craft Loop shall accept ordinary handwritten notes.

The system shall preserve notes that are not converted.

The user shall be able to convert recognized handwriting to typed text when desired.

Handwriting recognition shall expose uncertainty internally.

Low-confidence recognition shall not modify engineering geometry.

---

# Article 591 — Requirement Group: Engineering Numeric Parsing

The parser shall support integer dimensions.

The parser shall support decimal dimensions.

The parser shall support negative values where the dimension type permits them.

The parser shall support angle notation.

The parser shall support radius notation.

The parser shall support diameter notation.

The parser shall apply document units when no unit is written.

The parser shall respect explicit units.

The parser shall reject malformed numeric input safely.

The parser shall not decide the target geometry.

---

# Article 592 — Requirement Group: Dimension Association

The system shall support association through explicit selection.

The system shall support association through a dimension gesture.

The system may support proximity-based association.

The system shall not rely on proximity alone when multiple candidates exist.

The system shall scope association to the active View Block.

The system shall use timing as contextual evidence.

The system shall allow the user to redirect an incorrectly proposed target.

The system shall not commit low-confidence association silently.

---

# Article 593 — Requirement Group: Linear Dimensions

The user shall be able to create a linear dimension.

The user shall be able to write its value by pen.

The user shall be able to type its value.

The dimension shall update geometry if driving.

The dimension shall update its display if reference.

The dimension shall participate in conflict detection.

The dimension annotation shall be movable without changing the semantic value.

---

# Article 594 — Requirement Group: Angular Dimensions

The user shall be able to dimension an angle between eligible geometry.

The user shall be able to write a degree value by hand.

The parser shall recognize degree notation.

The Constraint Engine shall validate the angle against existing relationships.

A conflicting perpendicular relation shall not be silently broken.

---

# Article 595 — Requirement Group: Radius and Diameter

The user shall be able to apply radius to circular geometry.

The user shall be able to apply diameter to circular geometry.

Handwritten `R` or diameter notation may be recognized.

The system shall validate the target type.

A diameter dimension shall not attach to a line.

The system shall provide a clear correction path.

---

# Article 596 — Requirement Group: Basic Constraints

Version 1 shall support Coincident where solver architecture permits.

Version 1 shall support Horizontal.

Version 1 shall support Vertical.

Version 1 shall support Parallel.

Version 1 shall support Perpendicular.

Version 1 shall support Equal for compatible entities.

Version 1 shall support Concentric.

Version 1 shall support Tangent where solver quality is sufficient.

Version 1 shall support Symmetry where implementation remains understandable.

The exact final Version 1 set shall be validated during solver integration.

---

# Article 597 — Requirement Group: Constraint Suggestions

Suggestions shall be visually distinct from confirmed constraints.

Suggestions shall be rejectable.

Rejected suggestions shall not repeatedly reappear without changed evidence.

Suggestions shall not override explicit user intent.

The user shall be able to disable automatic suggestions.

Machine-learned suggestions shall pass solver validation.

---

# Article 598 — Requirement Group: Geometric Conflict

The system shall detect impossible constraint combinations.

The system shall detect contradictory shared dimensions.

The system shall detect invalid dimension ranges when mathematically known.

The system shall provide an explanation.

The system shall preserve the last valid geometry.

The user shall select the resolution.

The system shall not silently alter confirmed dimensions to recover.

---

# Article 599 — Requirement Group: View Identity

A View Block shall support Front identity.

A View Block shall support Top identity.

A View Block shall support Right identity.

A View Block shall support Back identity.

Additional principal identities may be added.

View identity shall be semantic, not only a text label.

The user shall be able to assign identity by pen.

The user shall be able to assign identity through visible controls.

Changing identity shall revalidate relationships.

---

# Article 600 — Requirement Group: Orthographic Entry

The user shall be able to enter Orthographic mode from a labeled structured view.

Full dimensioning shall not be required.

Blocking local conflicts shall be explained.

The user shall be able to invoke Orthographic through the toolbar.

The user shall be able to invoke Orthographic through an Ink Command.

The transition shall be undoable.

The source View Block shall remain visible.

---

# Article 601 — Requirement Group: Orthographic Set

The application shall create or identify an Orthographic Set when linked views are created.

A set shall contain View Blocks.

A set shall store projection convention.

A set shall store shared dimensions.

A set shall store correspondences.

A notebook shall support more than one set.

Two Front views in different sets shall be allowed.

---

# Article 602 — Requirement Group: Shared Dimensions

A shared dimension shall have one semantic value.

Multiple view annotations may reference it.

Changing the value shall propagate to linked views.

Deleting one annotation shall not necessarily delete the shared dimension.

Conflicting new values shall create a conflict.

The user shall be able to replace the shared value intentionally.

---

# Article 603 — Requirement Group: Unresolved Values

Unknown depth shall remain unknown.

Unknown dimensions shall not be assigned arbitrary defaults as engineering truth.

The interface shall indicate unresolved state.

The user shall be able to resolve the value from another view.

Resolved values shall propagate where relationships require.

Unresolved state shall survive save and reopen.

---

# Article 604 — Requirement Group: Cross-View Correspondence

The system shall store confirmed correspondences.

The system may suggest correspondences.

Suggested correspondences shall be distinguishable from confirmed.

The user shall be able to reject a suggestion.

The user shall be able to unlink a confirmed correspondence.

Cross-view propagation shall rely only on confirmed or deterministic relationships.

---

# Article 605 — Requirement Group: Orthographic Layout

The system shall support a selected projection convention.

The default view layout shall follow the selected convention.

Moving a View Block shall not change engineering meaning.

The user shall be able to rearrange for presentation.

The system shall retain semantic identity after rearrangement.

Export may restore standard arrangement if the user requests a formal layout.

---

# Article 606 — Requirement Group: Ink Commands

Version 1 shall support a limited command vocabulary.

Full command words shall work when recognition is supported.

Unique prefixes may work.

Ambiguous prefixes shall not execute automatically.

The command shall require explicit confirmation according to the gesture model.

The command system shall use the shared Command Bus.

The system shall display recognition feedback.

The command shall be undoable where meaningful.

---

# Article 607 — Requirement Group: Command Safety

Plain notes shall not execute commands.

A known word shall not execute without the confirmation rule.

High-risk commands shall require stronger confirmation.

The user shall be able to disable Ink Commands.

Command misrecognition shall not corrupt document semantics.

Command behavior shall be consistent across platforms.

---

# Article 608 — Requirement Group: Toolbar

The main toolbar shall remain compact.

The active tool shall be clear.

The toolbar shall adapt to context.

The toolbar shall not duplicate separate logic from Ink Commands.

Essential tools shall remain discoverable without memorizing commands.

The toolbar shall respect handedness preferences.

---

# Article 609 — Requirement Group: Undo

Every user-visible engineering transaction shall be undoable unless explicitly documented otherwise.

Undo shall restore semantic relationships.

Undo shall restore view propagation.

Undo shall restore prior tool state for command errors when appropriate.

Undo shall not leave stale conflicts.

Redo shall reproduce the same deterministic semantic result.

---

# Article 610 — Requirement Group: Persistence

Documents shall autosave.

Confirmed semantics shall survive restart.

Raw ink shall survive according to document policy.

View identity shall survive.

Orthographic Sets shall survive.

Constraints shall survive.

Dimensions shall survive.

Projection convention shall survive.

Conflict state shall survive when unresolved.

---

# Article 611 — Requirement Group: Offline

Core drawing shall work offline.

Primitive recognition using deterministic methods shall work offline.

Constraint solving shall work offline.

Dimensions shall work offline.

Orthographic relationships shall work offline.

Native save shall work offline.

Cloud-only assistance shall degrade safely.

---

# Article 612 — Requirement Group: Export

Portable Document Format export shall be supported for a usable Version 1.

The output shall preserve visible engineering geometry.

The output shall preserve visible dimensions.

The output shall preserve notes selected for export.

The output shall preserve view labels.

The output shall respect units.

The output shall not include ephemeral command ink.

Vector export shall be prioritized.

---

# Article 613 — Requirement Group: Search

The notebook library shall support title search.

Recognized handwriting search is desirable where platform capability supports it.

Search shall not require cloud connectivity for local documents when local indexing is available.

Search shall not modify source ink.

---

# Article 614 — Requirement Group: Accessibility

Controls shall have accessible labels.

Critical state shall not depend only on color.

Reduced motion shall be supported.

Left-handed configuration shall be supported.

Right-handed configuration shall be supported.

Keyboard alternatives shall exist for important commands.

The product shall remain usable without haptic hardware.

---

# Article 615 — Requirement Group: Privacy

Core drawing shall not require uploading the notebook.

Model-training use of user drawings shall require explicit consent.

Users shall be able to delete cloud content.

The privacy model shall explain optional cloud intelligence.

Telemetry shall minimize content capture.

---

# Article 616 — Requirement Group: Reliability

The application shall recover from ordinary process termination without corrupting the document.

Atomic transactions shall prevent partial cross-view updates.

Stale asynchronous results shall be rejected.

Cancelled operations shall not commit later.

Export failure shall not modify the source.

---

# Article 617 — Requirement Group: Performance

Pen interaction shall remain responsive.

Autosave shall not block pen input.

Small constraint graphs shall solve interactively.

Orthographic dimension propagation shall feel immediate.

Recognition shall not freeze the canvas.

Large-document performance shall be measured.

---

# Article 618 — Requirement Group: Platform Consistency

iPad and Android shall share semantic behavior.

Dimensions shall solve equivalently.

Orthographic links shall propagate equivalently.

Command names may localize but semantic actions shall match.

Optional device features may differ.

Core capability shall not be exclusive to one platform without explicit product decision.

---

# Article 619 — Requirement Group: Product Boundaries

Version 1 shall not create a three-dimensional solid.

Version 1 shall not claim automatic complete hidden geometry.

Version 1 shall not include computer-aided manufacturing.

Version 1 shall not include simulation.

Version 1 shall not autonomously redesign user geometry.

Version 1 shall not require a feature tree.

Version 1 shall not require a keyboard.

---

# Article 620 — Requirement Group: Human Authority

User-created intent shall take priority over machine suggestions.

The system shall not silently overwrite confirmed dimensions.

The system shall not silently delete confirmed constraints.

The system shall not silently link ambiguous features.

The system shall expose a correction path.

The user shall remain the final decision-maker.


# Article 621 — Validation Scenario: Blank Page to First Line

Open a new document.

Draw one rough line.

Confirm that ink appears immediately.

Hold to refine.

Confirm that the result becomes a structured line.

Undo.

Confirm that the original ink state is restored.

Redo.

Confirm that the structured line returns.

Save and reopen.

Confirm that the structured line remains structured.

---

# Article 622 — Validation Scenario: Rough Circle

Draw a rough circle.

Do not hold.

Confirm it remains ink or a noncommitted candidate.

Draw another rough circle and hold.

Confirm a precise circle is proposed or committed according to interaction policy.

Undo.

Confirm the original stroke returns.

Verify that a handwritten letter O in a sentence is not converted into a circle.

---

# Article 623 — Validation Scenario: Rectangle Versus Trapezoid

Draw an intentionally imperfect rectangle.

Refine it.

Confirm the result matches intended rectangle behavior.

Draw an intentional trapezoid.

Verify that the system does not force rectangle interpretation when evidence is weak.

Reject a Rectangle suggestion.

Confirm the rejection is respected.

---

# Article 624 — Validation Scenario: Handwritten Integer

Select a line.

Invoke a dimension guide.

Write `30`.

Confirm recognition.

Confirm association to the selected line.

Confirm the line becomes 30 document units.

Confirm the visible annotation uses technical typography.

Undo and verify the prior length returns.

---

# Article 625 — Validation Scenario: Handwritten Decimal

Select a dimension.

Write `30.5`.

Confirm correct parse.

Verify physical geometry.

Change locale to one using comma.

Write `30,5`.

Confirm locale-aware interpretation according to supported policy.

---

# Article 626 — Validation Scenario: Explicit Unit

Set document unit to millimeters.

Write `3 cm` on a length dimension.

Verify internal conversion to 30 millimeters.

Verify the visible display follows the chosen display-unit policy.

Save and reopen.

Verify the physical length remains unchanged.

---

# Article 627 — Validation Scenario: Angle

Draw two lines.

Create an angular dimension.

Write `45°`.

Verify the angle becomes 45 degrees.

Add a Perpendicular constraint.

Attempt to retain 45 degrees.

Verify a conflict is reported.

Confirm no silent conversion to 90 degrees occurs.

---

# Article 628 — Validation Scenario: Diameter

Draw and refine a circle.

Create a diameter dimension.

Write `Ø20`.

Verify the circle diameter becomes 20 units.

Verify the radius reflects 10 internally.

Verify a line cannot accept the diameter dimension.

---

# Article 629 — Validation Scenario: Radius

Draw an arc.

Apply radius dimension.

Write `R10`.

Verify the arc radius becomes 10 units.

Verify the parser recognizes the prefix.

Verify undo restores the prior radius.

---

# Article 630 — Validation Scenario: Numeric Note

Write:

“Use 30 bolts.”

Place the note near a structured sketch.

Verify `30` remains part of the note.

Verify no dimension is created.

Move the note.

Verify geometry remains unchanged.

---

# Article 631 — Validation Scenario: Ambiguous Target

Create two parallel lines close together.

Write `30` between them without selecting a target.

Verify the system does not silently attach to the wrong line.

Confirm target candidates are shown or ink remains ordinary according to confidence policy.

Select one line.

Confirm association completes.

---

# Article 632 — Validation Scenario: Triangle Feasibility

Create a triangle.

Set one side to 30.

Set another side to 50.

Attempt to set the third side to 300.

Verify the dimension is rejected as geometrically impossible.

Verify the current valid triangle remains.

Verify the explanation identifies the relationship.

Enter a feasible value.

Verify the conflict resolves.

---

# Article 633 — Validation Scenario: Shared Width

Create a Front and Top view within one Orthographic Set.

Link width.

Set Front width to 100.

Verify Top width becomes 100.

Hide the width annotation in Top.

Verify the semantic value remains shared.

Change Front width to 120.

Verify Top geometry updates despite hidden annotation.

---

# Article 634 — Validation Scenario: Shared Width Conflict

With shared width set to 100, write 130 in Top.

Verify a conflict object appears.

Verify 130 does not become a second independent width.

Choose “replace with 130.”

Verify Front and Top update atomically.

Undo.

Verify both return to 100.

---

# Article 635 — Validation Scenario: Reference Dimension

Create geometry with a driving width.

Add a reference measurement of the same width.

Change the driving dimension.

Verify the reference updates.

Verify editing the reference does not change geometry unless converted to driving.

---

# Article 636 — Validation Scenario: Redundant Dimension

Create a line with a driving length.

Add another dimension to the same exact length relation.

Verify the system identifies redundancy.

Verify the user can create a visible duplicate annotation only if the product allows it.

Verify no duplicate driving variable is created.

---

# Article 637 — Validation Scenario: Parallel Constraint

Draw two nearly parallel lines.

Apply Parallel.

Move one line endpoint.

Verify the parallel relation remains.

Remove Parallel.

Move again.

Verify independent motion returns.

---

# Article 638 — Validation Scenario: Perpendicular Constraint

Draw two lines.

Apply Perpendicular.

Drag one line.

Verify the relationship remains at ninety degrees.

Attempt to apply an incompatible fixed angle.

Verify conflict explanation.

---

# Article 639 — Validation Scenario: Coincident Constraint

Draw two line endpoints near each other.

Apply Coincident.

Move one connected entity.

Verify the endpoints remain coincident.

Remove the constraint.

Verify separation becomes possible.

---

# Article 640 — Validation Scenario: Equal Circles

Draw two circles.

Apply Equal.

Change one radius.

Verify both radii update according to solver policy.

Attempt to apply conflicting radius dimensions.

Verify conflict.

---

# Article 641 — Validation Scenario: Concentric Circles

Draw two circles.

Apply Concentric.

Move the shared center.

Verify both move together.

Change radius of one.

Verify concentricity remains while radius can differ unless Equal is also applied.

---

# Article 642 — Validation Scenario: Tangent

Draw a line and a circle.

Apply Tangent.

Move the line.

Verify tangency remains if a valid solution exists.

Force an impossible combination with additional constraints.

Verify the system stops at valid state and explains the conflict.

---

# Article 643 — Validation Scenario: Symmetry

Create geometry on both sides of a centerline.

Apply Symmetry.

Change one side.

Verify the counterpart behaves according to the constraint.

Remove Symmetry.

Verify independent editing returns.

---

# Article 644 — Validation Scenario: Under-Constrained Sketch

Draw a line with no dimensions.

Enter Sketch context.

Verify the line can move.

Verify Craft Loop does not display an alarming error.

Verify the state can remain saved.

Add one dimension.

Verify remaining freedom is still allowed.

---

# Article 645 — Validation Scenario: Fully Determined Geometry

Create a constrained rectangle with enough dimensions and relationships to determine it.

Verify geometry remains stable.

Attempt to drag a fixed point.

Verify the interface communicates why movement is restricted.

Avoid generic solver language in the default message.

---

# Article 646 — Validation Scenario: Over-Constrained Attempt

Create fully determined geometry.

Attempt to add an incompatible dimension.

Verify the new transaction does not commit.

Verify the old state remains valid.

Verify resolution choices are offered.

---

# Article 647 — Validation Scenario: Command “Pen”

Write `Pen`.

Do not circle it.

Verify it remains ordinary handwriting.

Write `Pen` again and circle it immediately.

Verify Pen becomes active.

Verify command ink disappears according to policy.

Undo the command.

Verify prior tool state returns if defined.

---

# Article 648 — Validation Scenario: Command “Eraser”

Write `E`.

Circle it in a namespace where `E` uniquely means Eraser.

Verify Eraser becomes active.

Erase a stroke.

Return to Pen using toolbar.

Verify command and toolbar share one tool state.

---

# Article 649 — Validation Scenario: Command “Sketch”

Write `Sketch`.

Circle it.

Verify the toolbar adapts to Sketch context.

Verify the page remains visible.

Verify notes remain writable.

Exit Sketch.

Verify notebook tools return.

---

# Article 650 — Validation Scenario: Short Prefix

In Sketch context, write a prefix uniquely identifying Line.

Circle it.

Verify Line activates.

Add another command to the test vocabulary that creates ambiguity.

Verify the ambiguous prefix no longer executes automatically.

---

# Article 651 — Validation Scenario: Full Command During Ambiguity

Create an ambiguous short-prefix condition.

Write the full word `Line`.

Circle it.

Verify Line executes.

This confirms full names remain the stable fallback.

---

# Article 652 — Validation Scenario: Command in Sentence

Write:

“Use line as a guide.”

Circle the whole sentence for emphasis.

Verify the Line tool does not activate.

This scenario should remain part of command regression tests.

---

# Article 653 — Validation Scenario: Command Circle Versus Lasso

Write a valid command.

Wait beyond the configured command timing threshold.

Circle the old text.

Verify the system favors selection according to policy.

Write a new command and circle it immediately.

Verify command execution.

Test intermediate timing.

Verify ambiguity is handled safely.

---

# Article 654 — Validation Scenario: Geometry Circle Versus Confirmation Circle

Enter Circle tool.

Draw a circle around empty canvas.

Verify geometry is created.

Draw a circle around a recent valid command.

Verify command behavior.

Confirm context separates the two.

---

# Article 655 — Validation Scenario: Left-Handed Command

Set hand preference to left-handed.

Write a command.

Verify recognition choices and popovers appear away from the palm.

Verify the toolbar is accessible.

Repeat a dimension workflow.

Confirm no control is systematically obscured.

---

# Article 656 — Validation Scenario: Front View Identity

Create a structured sketch.

Write `Front` beneath it.

Verify View Identity becomes Front.

Hide the visible label.

Verify semantic identity remains.

Show the label again.

Verify no new view is created.

---

# Article 657 — Validation Scenario: Orthographic Without View Label

Create a structured sketch.

Invoke Orthographic without View Identity.

Verify transition does not proceed silently.

Verify the interface asks for View Identity.

Provide Front.

Verify the original Orthographic request can continue without requiring re-entry.

---

# Article 658 — Validation Scenario: Orthographic With No Dimensions

Create a Front sketch without dimensions.

Label Front.

Enter Orthographic.

Verify linked view blocks can appear.

Verify unknown depth remains unresolved.

Verify no arbitrary numeric depth is generated.

---

# Article 659 — Validation Scenario: Resolve Depth in Top

Use the prior no-dimension orthographic drawing.

Write a depth value in Top.

Verify the semantic depth is created.

Verify Right receives the same depth relationship.

Verify no three-dimensional model is created.

---

# Article 660 — Validation Scenario: Dimension in Right Propagates to Front

Create Front and Right views with shared height.

Write height in Right.

Verify Front updates.

This confirms no permanent Master View.

---

# Article 661 — Validation Scenario: Back View Uncertainty

Create Front with overall width and height.

Add Back.

Verify overall known extents can be shared according to relationship policy.

Verify unseen back-specific features are not invented.

Draw a back-specific feature manually.

Verify it remains Back-local until correspondence is confirmed.

---

# Article 662 — Validation Scenario: Multiple Orthographic Sets

Create Concept A Front.

Enter Orthographic.

Create Concept B Front elsewhere.

Enter Orthographic.

Verify the two sets remain independent.

Change width in Concept A.

Verify Concept B is unaffected.

---

# Article 663 — Validation Scenario: Duplicate Front Within Same Set

Attempt to assign a second Front identity within one linked set.

Verify the system identifies ambiguity or disallows it according to set policy.

Offer “Create new Orthographic Set” if appropriate.

---

# Article 664 — Validation Scenario: Move View Block

Move Top far across the page.

Verify semantic dimensions remain linked.

Change shared width.

Verify propagation continues.

Verify page movement does not alter local engineering coordinates.

---

# Article 665 — Validation Scenario: Duplicate View

Duplicate Front.

Verify the application asks or applies documented behavior for linked versus independent duplication.

Confirm no hidden second controller of the same semantic geometry is created accidentally.

---

# Article 666 — Validation Scenario: Projection Convention

Create an Orthographic Set.

Choose first-angle projection.

Verify default layout.

Switch to third-angle projection.

Verify layout changes according to implemented standard mapping.

Verify semantic dimensions do not change.

---

# Article 667 — Validation Scenario: Save Unresolved Orthographic Drawing

Create Front.

Enter Orthographic with unknown depth.

Save.

Close application.

Reopen.

Verify unresolved depth remains unresolved.

Verify the application does not generate a value during reopening.

---

# Article 668 — Validation Scenario: Save Conflict

Create a cross-view conflict.

Close document before resolving.

Reopen.

Verify the conflict object remains.

Verify the last valid geometry remains.

Verify the user can resume resolution.

---

# Article 669 — Validation Scenario: Stale Recognition

Draw ink.

Start an expensive recognition operation.

Erase the ink before recognition returns.

Allow the result to return.

Verify it is rejected as stale.

No ghost geometry should appear from deleted input.

---

# Article 670 — Validation Scenario: Stale Correspondence Suggestion

Create views.

Start asynchronous correspondence analysis.

Unlink or delete a source feature.

Allow analysis to return.

Verify the suggestion is discarded.

---

# Article 671 — Validation Scenario: Cancel Recognition

Begin a recognition operation.

Close the document.

Verify the operation cancels.

Reopen.

Verify no delayed result applies to the new session.

---

# Article 672 — Validation Scenario: Offline Start

Disable network connectivity.

Launch Craft Loop.

Create a notebook.

Draw.

Refine using deterministic recognition.

Add dimensions.

Use constraints.

Save.

Verify the workflow remains usable.

---

# Article 673 — Validation Scenario: Optional Cloud Intelligence Offline

Disable network connectivity.

Invoke a feature that requires cloud intelligence.

Verify the interface explains unavailability.

Verify existing drawing remains editable.

Verify no data is lost.

---

# Article 674 — Validation Scenario: Autosave Crash

Create a sequence of committed actions.

Force terminate the application.

Restart.

Verify committed actions are recovered.

Verify no partially committed transaction exists.

---

# Article 675 — Validation Scenario: Crash During Propagation

Change a shared dimension.

Force termination during persistence.

Restart.

Verify the document contains either the old complete state or the new complete state.

Verify no mixed Front and Top values exist.

---

# Article 676 — Validation Scenario: Device Rotation

Draw a line.

Rotate tablet.

Verify geometry does not change.

Verify canvas transform adapts.

Verify toolbar and handedness layout remain usable.

---

# Article 677 — Validation Scenario: Memory Stress

Create a long notebook with thousands of strokes.

Add multiple View Blocks.

Zoom repeatedly.

Undo and redo.

Observe memory.

Verify memory remains within defined budgets.

Verify inactive rendering resources can be released.

---

# Article 678 — Validation Scenario: Export

Create a dimensioned orthographic set.

Export to Portable Document Format.

Verify geometry.

Verify dimensions.

Verify line types.

Verify labels.

Verify notes.

Verify no command ink.

Measure a known dimension in the output where appropriate.

Confirm semantic value is preserved.

---

# Article 679 — Validation Scenario: Vector Export

Export structured geometry to Scalable Vector Graphics.

Open the file in representative vector software.

Verify lines remain vectors.

Verify text remains readable.

Verify scale policy is documented.

Verify semantic Craft Loop relationships are not falsely implied to survive the generic format.

---

# Article 680 — Validation Scenario: Document Migration

Create a document with an older test schema.

Open in a newer build.

Run migration.

Verify geometry.

Verify dimensions.

Verify constraints.

Verify View Identity.

Verify Orthographic Sets.

Verify no recognition is rerun on confirmed entities.

