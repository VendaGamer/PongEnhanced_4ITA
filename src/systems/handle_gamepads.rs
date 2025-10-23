use bevy::input::gamepad::GamepadConnectionEvent;
use bevy::prelude::EventReader;

pub fn check_connection(mut events: EventReader<GamepadConnectionEvent>){
    for ev in events.read(){
        
    }
}