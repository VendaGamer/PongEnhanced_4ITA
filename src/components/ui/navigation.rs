use bevy::prelude::Component;

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
