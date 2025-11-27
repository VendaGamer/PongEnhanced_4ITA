use bevy::prelude::Text;

pub struct UIOption<T>
    where T: Copy + 'static + Send + Sync,
{
    pub text: Text,
    pub item: T,
}
