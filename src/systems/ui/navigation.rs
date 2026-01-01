use leafwing_input_manager::prelude::ActionState;
use bevy::prelude::*;
use bevy::ui::Checked;
use bevy::ui_widgets::{Slider, SliderRange, SliderThumb, SliderValue, TrackClick};
use crate::components::ui::navigation::UINavSlot;
use crate::resources::*;
use crate::resources::navigation::{NavigationState, UISelection};

pub fn ui_navigation(
    time: Res<Time>,
    input: Query<&ActionState<MenuAction>>,
    mut sel: ResMut<UISelection>,
    mut nav_state: ResMut<NavigationState>,
    slots: Query<&UINavSlot>,
) {
    // Update cooldown
    nav_state.cooldown = (nav_state.cooldown - time.delta_secs()).max(0.0);

    for input_state in &input {
        let axis = input_state.axis_pair(&MenuAction::Navigate).xy();

        // Only process if cooldown is done and axis has changed significantly
        if nav_state.cooldown <= 0.0 {
            let deadzone = 0.5;
            let moved_right = axis.x > deadzone && nav_state.last_axis.x <= deadzone;
            let moved_left = axis.x < -deadzone && nav_state.last_axis.x >= -deadzone;
            let moved_down = axis.y < -deadzone && nav_state.last_axis.y >= -deadzone;
            let moved_up = axis.y > deadzone && nav_state.last_axis.y <= deadzone;

            let mut moved = false;

            if moved_right {
                sel.column += 1;
                moved = true;
                dbg!("Moved right");
            } else if moved_left {
                sel.column = sel.column.saturating_sub(1);
                moved = true;
                dbg!("Moved left");
            }

            if moved_down {
                sel.row += 1;
                moved = true;
                dbg!("Moved down");
            } else if moved_up {
                sel.row = sel.row.saturating_sub(1);
                moved = true;
                dbg!("Moved up");
            }

            if moved {
                clamp_to_existing_slots(&mut sel, &slots);
                nav_state.cooldown = 0.15; // 150ms cooldown between inputs
            }
        }

        nav_state.last_axis = axis;
    }
}

pub fn sync_selection_to_ui(
    sel: Res<UISelection>,
    mut q: Query<(&UINavSlot, &mut Interaction), With<Button>>,
) {
    for (slot, mut interaction) in &mut q {
        if slot.row == sel.row && slot.column == sel.column {
            *interaction = Interaction::Hovered;
        } else {
            *interaction = Interaction::None;
        }
    }
}

fn clamp_to_existing_slots(sel: &mut UISelection, slots: &Query<&UINavSlot>) {
    let existing: Vec<_> = slots.iter().collect();
    if existing.is_empty() { return; }

    let min_row = existing.iter().map(|slot| slot.row).min().unwrap();
    let max_row = existing.iter().map(|slot| slot.row).max().unwrap();
    sel.row = sel.row.clamp(min_row, max_row);

    let cols_in_row: Vec<u32> =
        existing.iter().filter(|s| s.row == sel.row).map(|s| s.column).collect();

    if !cols_in_row.is_empty() {
        let min_col = *cols_in_row.iter().min().unwrap();
        let max_col = *cols_in_row.iter().max().unwrap();
        sel.column = sel.column.clamp(min_col, max_col);
    }

}

pub fn update_slider_visuals(
    sliders: Query<(Entity, &SliderValue, &SliderRange)>,
    children: Query<&Children>,
    mut thumbs: Query<&mut Node, With<SliderThumb>>,
) {
    for (slider_entity, value, range) in sliders.iter() {
        for child in children.iter_descendants(slider_entity) {
            if let Ok(mut thumb_node) = thumbs.get_mut(child) {
                let percent = ((value.0 - range.start()) / (range.end() - range.start()) * 100.0)
                    .clamp(0.0, 100.0);

                thumb_node.left = Val::Percent(percent);
                println!("Left: {}", percent);
            }
        }
    }
}