use bevy::prelude::*;
use derive_more::{From, Into};
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

pub trait UIOptionProvider: Send + Sync + Any {
    fn get_option(&self, index: usize) -> Option<&dyn UIOptionValue>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait UIOptionValue: Any + Send + Sync + Debug + UIOptionString {
    fn as_any(&self) -> &dyn Any;
}

impl<T> UIOptionValue for T
where
    T: Any + Send + Sync + Debug + UIOptionString
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T: UIOptionValue + 'static> UIOptionProvider for Vec<T> {
    fn get_option(&self, index: usize) -> Option<&dyn UIOptionValue> {
        self.get(index).map(|val| val as &dyn UIOptionValue)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: UIOptionValue + 'static, const N: usize> UIOptionProvider for [T; N] {
    fn get_option(&self, index: usize) -> Option<&dyn UIOptionValue> {
        self.get(index).map(|val| val as &dyn UIOptionValue)
    }

    fn len(&self) -> usize {
        N
    }
}

impl<T: UIOptionValue + 'static> UIOptionProvider for [T] {
    fn get_option(&self, index: usize) -> Option<&dyn UIOptionValue> {
        self.get(index).map(|val| val as &dyn UIOptionValue)
    }

    fn len(&self) -> usize {
        self.len()
    }
}


#[derive(Component)]
pub struct Dropdown {
    pub selected: usize,
    pub options: Arc<dyn UIOptionProvider>,
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

#[derive(Component, From, Into)]
pub struct OptionSelector {
    pub selected: usize,
    pub options_provider: Arc<dyn UIOptionProvider>,
}

pub trait UIOptionString {
    fn push_ui_option_string(&self, string: &mut String);
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

    pub fn current<T: 'static>(&self) -> Option<&T> {
        self.options_provider
            .get_option(self.selected)?
            .as_any()
            .downcast_ref::<T>()
    }

    pub fn push_current_string(&self, string: &mut String) {
        if let Some(current) = self.options_provider.get_option(self.selected) {
            current.push_ui_option_string(string);
            return;
        }

        string.push_str("n/a");
    }

    pub fn next(&mut self) {
        if !self.options_provider.is_empty() {
            self.selected = (self.selected + 1) % self.options_provider.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.options_provider.is_empty() {
            self.selected = if self.selected == 0 {
                self.options_provider.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    pub fn set(&mut self, idx: usize) {
        if idx < self.options_provider.len() {
            self.selected = idx;
        }
    }
}