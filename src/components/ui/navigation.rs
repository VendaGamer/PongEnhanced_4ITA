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