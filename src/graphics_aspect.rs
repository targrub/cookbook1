#![allow(dead_code, unused,)]
// Bevy app/ecs types
use bevy::app::{App, Plugin};
use bevy::ecs::{
    component::Component,
    query::{With},
    system::{Query, Local, Commands, ResMut},
    entity::Entity,
    event::EventReader,
};
use bevy::prelude::ParallelSystemDescriptorCoercion;

// Bevy non-app/ecs types
use bevy::pbr::StandardMaterial;

// Public types from this plugin
// ...

// Private types used by this plugin
use bevy::asset::Assets;
use bevy::render::mesh::Mesh;

// Elements from elsewhere in the crate
use crate::entity_creator::EntityCreatorComponent;

// Private components for this plugin
#[derive(Component)]
struct GraphicsAspectComponent;

// Public events for this plugin
pub struct CreateGraphicsAspectEvent {
    pub entity: Option<Entity>,
    pub name: String,
}

impl Default for CreateGraphicsAspectEvent {
    fn default() -> Self {
        CreateGraphicsAspectEvent { entity: None, name: String::new(), }
    }
}


#[derive(Default)]
pub struct OtherGraphicsAspectEvent {
    pub shimmer_value: f32,
}

// settings for this plugin, kept in a Local
#[derive(Default)]
struct GraphicsAspectSettings;

// this plugin
pub struct GraphicsAspect;

impl Plugin for GraphicsAspect {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CreateGraphicsAspectEvent>()
            .add_event::<OtherGraphicsAspectEvent>()
            .add_system(graphicsaspect_system)
            .add_system(create_graphicsaspect_responder)
            .add_system(other_graphicsaspect_event_responder)
            ;
    }
}

// systems for this plugin
fn graphicsaspect_system(
    settings: Local<GraphicsAspectSettings>,
    q: Query<Entity, With<GraphicsAspectComponent>>,
) {

}


// Systems for listening to events for this plugin will use EventReader< [event from this plugin] >

// Systems for listening to events for this plugin

// ?? only for adding a component to listen for other graphics-related events?

// This system is only necessary when creating entities that listen for CreateGraphicsAspectEvents.
// Is this only run if !EventReader<CreateGraphicsAspectEvents>.is_empty()?  No.
// So, after creation, since the only reason to keep around the EntityCreatorComponent is allow despawning
// (any other reason?), if that can just be done recursively by the entity, we have no reason not to delete
// the EntityCreatorComponent from this entity after adding its aspects.
fn create_graphicsaspect_responder(
    settings: Local<GraphicsAspectSettings>,
    mut creategraphics_ev_reader: EventReader<CreateGraphicsAspectEvent>,
    q: Query<Entity, With<EntityCreatorComponent>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // have EntityId
    for ev in creategraphics_ev_reader.iter() {
        if q.contains(ev.entity.unwrap()) {
            // can use/modify GraphicsPluginSettings
            // ...

            // can add graphics-related components to an entity
            commands
                .entity(ev.entity.unwrap())
                .insert(GraphicsAspectComponent)
//                   .with_children(|builder| { ...
                ;
        }
    }
}

// Systems querying strictly for entities related to this system will use Query<Entity, With< [component from this plugin] >>

fn other_graphicsaspect_event_responder(
    settings: Local<GraphicsAspectSettings>,
    mut othergraphics_ev_reader: EventReader<OtherGraphicsAspectEvent>,
// may or may not be interested in entities with a component from this plugin
//    q: Query<Entity, With<GraphicsAspectComponent>>,
) {

}
