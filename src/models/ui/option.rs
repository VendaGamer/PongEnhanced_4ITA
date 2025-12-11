use std::any::Any;

pub trait OptionValue: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn clone_box(&self) -> Box<dyn OptionValue>;
}

impl<T: 'static + Clone + Send + Sync> OptionValue for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn clone_box(&self) -> Box<dyn OptionValue> {
        Box::new(self.clone())
    }
}

pub struct UIOption {
    pub text: &'static str,
    value: Box<dyn OptionValue>,
}


impl UIOption {
    pub fn new<T: OptionValue>(text: &'static str, value: T) -> Self {
        Self {
            text,
            value: Box::new(value),
        }
    }

    pub fn get_value<T: 'static>(&self) -> Option<&T> {
        self.value.as_any().downcast_ref::<T>()
    }
}

impl Clone for UIOption {
    fn clone(&self) -> Self {
        Self {
            text: self.text,
            value: self.value.clone_box(),
        }
    }
}