use std::fmt::Debug;
use bevy::prelude::*;
use std::sync::Arc;

#[derive(Component)]
pub struct Dropdown {
    pub selected: usize,
    pub options: Arc<Vec<Box<dyn OptionValue>>>,
}

#[derive(Component)]
pub struct ConditionalVisibility {
    pub depends_on: SettingsSelector,
    pub required_values: Vec<String>,
}

#[derive(Component)]
#[require(Button)]
pub struct SelectorButton(pub bool);

#[derive(Component)]
#[require(Text)]
pub struct SelectorText;

#[derive(Component)]
pub struct OptionSelector {
    pub selected: usize,
    pub options: Arc<Vec<Box<dyn OptionValue>>>,
}

pub trait OptionValue: Send + Sync + Debug + UIOptionString { }


pub trait UIOptionString {
    fn fill_ui_option_string(&self, string: &mut String);
}

#[derive(Component)]
pub enum SettingsSelector {
    WindowMode,
    Monitor,
    Resolution,
    RefreshRate,
    ShowFPS,
}

impl OptionSelector {

    pub fn new(options: Vec<Box<dyn OptionValue>>) -> Self {
        Self {
            selected: 0,
            options: Arc::new(options),
        }
    }

    pub fn with_selected(options: Vec<Box<dyn OptionValue>>, selected: usize) -> Self {
        Self {
            selected: selected.min(options.len().saturating_sub(1)),
            options: Arc::new(options),
        }
    }

    pub fn current(&self) -> Option<&dyn OptionValue> {
        self.options.get(self.selected).map(|b| &**b)
    }

    pub fn current_string(&self) -> &str {
        self.current()
            .map(|v| v.fill_ui_option_string())
            .unwrap_or_else(|| "None")
    }

    pub fn next(&mut self) {
        if !self.options.is_empty() {
            self.selected = (self.selected + 1) % self.options.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.options.is_empty() {
            self.selected = if self.selected == 0 {
                self.options.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    pub fn set(&mut self, idx: usize) {
        if idx < self.options.len() {
            self.selected = idx;
        }
    }
}