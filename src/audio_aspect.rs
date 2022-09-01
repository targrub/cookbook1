use bevy::app::{App, Plugin};
use bevy::ecs::component::Component;

#[derive(Component)]
struct AudioAspectComponent;

// Public types from this plugin
#[derive(Default)]
pub struct AudioAspectType;

// Private types used by this plugin
// ...


#[derive(Default)]
pub struct CreateAudioAspectEvent {
    pub entity_id: u32,
    pub audiotype: AudioAspectType,
}

pub struct AudioAspect;

impl Plugin for AudioAspect {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CreateAudioAspectEvent>()
            // systems of this plugin
            ;
    }
}


// Systems for listening to events for this plugin will use EventReader< [event from this plugin] >

// Systems querying strictly for entities related to this system will use Query<Entity, With< [component from this plugin] >>
