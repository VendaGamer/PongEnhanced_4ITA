use bevy::input::gamepad::GamepadConnectionEvent;
use bevy::prelude::{Color, MessageReader};

pub fn check_connection(mut events: MessageReader<GamepadConnectionEvent>){
    for ev in events.read(){
        
    }
}