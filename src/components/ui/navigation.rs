use std::hash::Hash;
use crate::models::ui::option::UIOption;
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct UINavSlot {
    pub row: u32,
    pub column: u32,
}

impl UINavSlot {
    pub fn new(row: u32, column: u32) -> UINavSlot {
        UINavSlot { row, column }
    }

    pub fn row(row: u32) -> UINavSlot {
        UINavSlot { row, column: 0 }
    }

    pub fn column(column: u32) -> UINavSlot {
        UINavSlot { row: 0, column }
    }
}

#[derive(Component)]
pub struct OptionSelector {
    pub options: Vec<UIOption>,
    pub selected: usize,
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
        self.options[self.selected].text
    }

    pub fn new(options: Vec<UIOption>) -> OptionSelector{
        OptionSelector{
            options,
            selected: 0
        }
    }

    pub fn new_selected(options: Vec<UIOption>, selected: usize) -> OptionSelector{
        OptionSelector{
            options,
            selected
        }
    }
}

#[derive(Component)]
pub struct SelectorText{
    pub selector_entity: Entity
}