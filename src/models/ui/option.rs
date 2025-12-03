use std::any::Any;

pub struct UIOption {
    pub text: &'static str,
    pub value: Box<dyn Any + Send + Sync>,
}

impl UIOption {
    pub fn new<T: 'static + Clone + Send + Sync>(text: &'static str, value: T) -> Self {
        Self {
            text,
            value: Box::new(value),
        }
    }
}