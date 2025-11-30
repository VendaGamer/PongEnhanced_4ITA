use leafwing_input_manager::prelude::ActionState;
use bevy::prelude::*;
use crate::components::ui::navigation::UINavSlot;
use crate::resources::*;
use crate::resources::navigation::UISelection;

pub fn ui_navigation(
    input: Query<&ActionState<MenuAction>>,
    mut sel: ResMut<UISelection>,
    slots: Query<&UINavSlot>,
) {
    for input_state in &input {
        if input_state.just_pressed(&MenuAction::Navigate) {

            let data = input_state.axis_pair(&MenuAction::Navigate);
            let axis = data.xy();

            dbg!(axis);

            if axis.x.abs() > 0.5 {
                if axis.x > 0.0 {
                    sel.column += 1;
                    dbg!("Moved right");
                } else {
                    sel.column -= 1;
                    dbg!("Moved left");
                }
            }

            if axis.y.abs() > 0.5 {
                if axis.y > 0.0 {
                    sel.row -= 1;
                    dbg!("Moved up");
                } else {
                    sel.row += 1;
                    dbg!("Moved down");
                }
            }

            clamp_to_existing_slots(&mut sel, &slots);
        }
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


    let cols_in_row: Vec<i32> =
        existing.iter().filter(|s| s.row == sel.row).map(|s| s.column).collect();

    if !cols_in_row.is_empty() {
        let min_col = cols_in_row.iter().min().unwrap();
        let max_col = cols_in_row.iter().max().unwrap();
        sel.column = sel.column.clamp(*min_col, *max_col);
    }
}
