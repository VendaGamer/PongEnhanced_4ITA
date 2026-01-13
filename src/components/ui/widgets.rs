use crate::models::ui::option::UIOption;
use bevy::prelude::*;
use std::marker::PhantomData;
use bevy::reflect::{Enum, Reflect};

#[derive(Component)]
pub struct Dropdown {
    pub options: Vec<UIOption>,
    pub selected: usize,
}

#[derive(Component)]
#[require(Button)]
pub struct SelectorButton;

#[derive(Component)]
pub struct OptionSelector {
    pub options: Vec<UIOption>,
    pub selected: usize,
}

#[derive(Component)]
pub struct ResolutionSelector;

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
pub struct SelectorText {
    pub selector_entity: Entity
}