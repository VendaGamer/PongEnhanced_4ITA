use bevy::prelude::{Component, Entity, Text};

#[derive(Component, Clone, Copy)]
pub struct UINavSlot {
    pub row: i32,
    pub column: i32,
}

impl UINavSlot {
    pub fn new(row: i32, column: i32) -> UINavSlot {
        UINavSlot { row, column }
    }

    pub fn row(row: i32) -> UINavSlot {
        UINavSlot { row, column: 0 }
    }

    pub fn column(column: i32) -> UINavSlot {
        UINavSlot { row: 0, column }
    }
}

#[derive(Component)]
pub struct OptionSelector {
    pub options: Vec<String>,
    pub selected: usize,
    pub label: String,
}

impl OptionSelector {
    pub fn cycle_next(&mut self) {
        self.selected = (self.selected + 1) % self.options.len();
    }

    pub fn cycle_prev(&mut self) {
        if self.selected == 0 {
            self.selected = self.options.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    pub fn get_current(&self) -> &str {
        &self.options[self.selected]
    }
}

#[derive(Component)]
pub struct SelectorText{
    pub selector_entity: Entity
}