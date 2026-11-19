//! The wheel over a Blend Mode dropdown steps the mode without opening the list, on the Blending
//! Options page and on an effect's page alike, as the other dropdowns in Layer Style do.

use egui::accesskit::Role;
use egui::vec2;
use egui_kittest::{Harness, kittest::Queryable};

use super::tests::{DialogId, EffectKind, StyleValue, app_with_layer, draft, first_of, param};
use super::*;
use crate::PhotocraftApp;

fn harness(app: Box<PhotocraftApp>) -> Harness<'static, Box<PhotocraftApp>> {
    let mut h = Harness::builder().with_size(vec2(1500.0, 1000.0)).with_context(app.ctx().clone()).build_ui_state(
        |ui, app: &mut Box<PhotocraftApp>| {
            app.framework.dialogs.run(ui);
            take(app, ui.ctx());
        },
        app,
    );
    h.run_steps(4);
    h
}

/// One wheel notch of `lines` (positive is up) over the dialog's Blend Mode dropdown.
fn wheel_blend(h: &mut Harness<'_, Box<PhotocraftApp>>, lines: f32) {
    let at = h.get_by_role_and_label(Role::ComboBox, "Blend Mode").rect().center();
    h.hover_at(at);
    h.run_steps(1);
    h.event(egui::Event::MouseWheel {
        source: egui::MouseWheelSource::Unknown,
        unit: egui::MouseWheelUnit::Line,
        delta: vec2(0.0, lines),
        phase: egui::TouchPhase::Move,
        modifiers: egui::Modifiers::NONE,
    });
    h.run_steps(6);
}

fn layer_blend(h: &Harness<'_, Box<PhotocraftApp>>, id: DialogId) -> String {
    draft(h.state(), id).style.blending.blend.clone()
}

fn effect_blend(h: &Harness<'_, Box<PhotocraftApp>>, id: DialogId) -> Option<StyleValue> {
    let shadow = first_of(h.state(), id, EffectKind::DropShadow);
    param(&draft(h.state(), id).style, shadow, "blend")
}

#[test]
fn the_wheel_steps_the_layers_blend_mode() {
    let mut app = app_with_layer(32);
    let id = open(&mut app, None).unwrap();
    let mut h = harness(app);
    assert_eq!(layer_blend(&h, id), "Normal");
    wheel_blend(&mut h, -1.0);
    let stepped = layer_blend(&h, id);
    assert_ne!(stepped, "Normal", "a notch down steps to the next mode");
    assert!(h.query_by_role(Role::ListBox).is_none(), "the list stays closed");
    wheel_blend(&mut h, 1.0);
    assert_eq!(layer_blend(&h, id), "Normal", "a notch up steps back");
}

#[test]
fn the_wheel_steps_an_effects_blend_mode() {
    let mut app = app_with_layer(32);
    let id = open(&mut app, Some(EffectKind::DropShadow.key())).unwrap();
    let mut h = harness(app);
    let before = effect_blend(&h, id);
    wheel_blend(&mut h, -1.0);
    let after = effect_blend(&h, id);
    assert_ne!(after, before, "a notch down steps the shadow's mode");
    wheel_blend(&mut h, 1.0);
    assert_eq!(effect_blend(&h, id), before, "a notch up steps back");
}
