#![allow(dead_code, unused,)]
use bevy::MinimalPlugins;
// Bevy app/ecs types
use bevy::app::{App, Plugin};
use bevy::ecs::system::Despawn;
use bevy::ecs::{
    component::Component,
    entity::Entity,
    event::EventWriter,
    query::With,
    system::{Commands, Query, Res, SystemParam}
};

use bevy::prelude::{StandardMaterial, EventReader};
// Necessary component for spawned entity
use bevy::render::prelude::SpatialBundle;
use bevy::utils::default;

// Bevy non-app/ecs types
use bevy::input::{keyboard::KeyCode, Input};

// Events used by other plugins to add their components to entities this plugin spawns
use crate::audio_aspect::CreateAudioAspectEvent;
use crate::graphics_aspect::CreateGraphicsAspectEvent;

// Some types used in audioplugin passed to CreateAudioEvent must be made public
// but that's the only exposure to the internals.
use bevy::render::color::Color;
use crate::audio_aspect::AudioAspectType;

// Public components for this plugin
#[derive(Component)]
pub struct EntityCreatorComponent; // used as marker component to limit which entities are examined when Create [this plugin's] Events are received

struct RemoveEntityCreatorComponentEvent {
    entity_id: u32,
}

struct DespawnEntitiesEvent {
    entity_ids: Vec<u32>,
}

// Struct with SystemParam trait allows us to pass multiple EventWriters, EventReaders, etc., to subsystems
#[derive(SystemParam)]
struct Writers<'w, 's> {
    ev_remove_entity_creator_component_writer: EventWriter<'w, 's, RemoveEntityCreatorComponentEvent>,
    ev_createaudio_writer: EventWriter<'w, 's, CreateAudioAspectEvent>,
    ev_creategraphics_writer: EventWriter<'w, 's, CreateGraphicsAspectEvent>,
}

// This plugin
pub struct EntityCreator;

impl Plugin for EntityCreator {
    fn build(&self, app: &mut App) {
        app
            .add_system(mysystem)
            .add_event::<RemoveEntityCreatorComponentEvent>()
            .add_system(remove_entity_creator_components)
            .add_event::<DespawnEntitiesEvent>()
            .add_system(despawn_entities)
            ;
    }        
}

// Systems for this plugin
fn mysystem(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut writers: Writers,
    mut ev_despawn_entities_writer: EventWriter<DespawnEntitiesEvent>,
  ) {
    if input.just_pressed(KeyCode::C) {
        create_entity_and_aspects(&mut commands, &mut writers);
    }
    if input.just_pressed(KeyCode::D) {
//        let e_ids = ;
//        let event = DespawnEntitiesEvent { entity_ids: e_ids, }
//        ev_despawn_entities_writer.send(event);
    }
}

fn create_entity_and_aspects(
    commands: &mut Commands,
    writers: &mut Writers,
)
{
    let e = create_entity(commands, writers);
    add_audio_aspect(commands, writers, &e, AudioAspectType::default());
    add_graphics_aspect(commands, writers, &e, Color::BLUE, "bee".to_string());
    add_graphics_aspect(commands, writers, &e, Color::ALICE_BLUE, "cee".to_string());
    remove_entity_creator_component(&e, commands, writers);
}

fn create_entity(
    commands: &mut Commands,
    writers: &mut Writers,
) -> Entity {
    let e = commands.spawn()
        .insert_bundle(SpatialBundle::default())
        .insert(EntityCreatorComponent)
        .id();
    e
}

fn remove_entity_creator_component(
    entity: &Entity,
    commands: &mut Commands,
    writers: &mut Writers,
) {
    writers.ev_remove_entity_creator_component_writer.send(RemoveEntityCreatorComponentEvent {entity_id: entity.id()});
}

fn add_audio_aspect(
    commands: &mut Commands,
    writers: &mut Writers,
    entity: &Entity,
    audiotype: AudioAspectType,
)
{
    writers.ev_createaudio_writer.send(CreateAudioAspectEvent {entity_id: entity.id(), audiotype});
}

fn add_graphics_aspect(
    commands: &mut Commands,
    writers: &mut Writers,
    entity: &Entity,
    color: Color,
    name: String,
)
{
    writers.ev_creategraphics_writer.send(CreateGraphicsAspectEvent {entity_id: entity.id(), name});
}

fn despawn_entities(
    mut commands: Commands,
    mut ev_despawn_entities_reader: EventReader<DespawnEntitiesEvent>,
    q: Query<Entity>,
) {
    for ev in ev_despawn_entities_reader.iter() {
        for e in q.iter() {
            for de in ev.entity_ids.iter() {
                if *de == e.id() {
                    commands.entity(e).despawn();
                }
            }
        }
    }
}

fn remove_entity_creator_components(
    mut commands: Commands,
    mut ev_remove_entity_creator_component_reader: EventReader<RemoveEntityCreatorComponentEvent>,
    q: Query<Entity, With<EntityCreatorComponent>>,
) {
    for ev in ev_remove_entity_creator_component_reader.iter() {
        for e in q.iter() {
            if ev.entity_id == e.id() {
                commands.entity(e).remove::<EntityCreatorComponent>();
            }
        }
    }
}


#[cfg(test)]
mod tests {

use bevy::prelude::*;
use super::*;
use bevy::asset::AssetPlugin;

#[test]
fn test_entity_creation() {
    let mut app = App::new();

    app.add_plugin(AssetPlugin::default());
    app.add_asset::<Mesh>();
    app.add_asset::<StandardMaterial>();

    app.add_plugin(crate::audio_aspect::AudioAspect);
    app.add_plugin(crate::graphics_aspect::GraphicsAspect);
    app.add_plugin(EntityCreator);

    let mut input = Input::<KeyCode>::default();
    input.press(KeyCode::C);
    app.insert_resource(input);
    app.update();

    {
        // Created entity with component
        assert_eq!(app.world.query::<&EntityCreatorComponent>().iter(&app.world).len(), 1);

        // Sent 1 create audio event
        let audio_created_events = app.world.resource::<Events<CreateAudioAspectEvent>>();
        let mut create_audio_reader = audio_created_events.get_reader();
        assert_eq!(create_audio_reader.iter(audio_created_events).len(), 1);

        // Sent 2 create graphics events
        let graphics_created_events = app.world.resource::<Events<CreateGraphicsAspectEvent>>();
        let mut create_graphics_reader = graphics_created_events.get_reader();
        assert_eq!(create_graphics_reader.iter(graphics_created_events).len(), 2);

        // Should this be 0 at this point?
        // When does remove_entity_creator_components get run,
        // and is the event at the EventReader?
        // When is the component actually removed?
        // What data is this query run on?
        assert_eq!(app.world.query::<&EntityCreatorComponent>().iter(&app.world).len(), 1);
    }

    app.world.resource_mut::<Input<KeyCode>>().clear();
    app.update();

    {
        // Same questions here
        assert_eq!(app.world.query::<&EntityCreatorComponent>().iter(&app.world).len(), 0);

        // Check that no more events are forthcoming
        let audio_created_events = app.world.resource::<Events<CreateAudioAspectEvent>>();
        let mut create_audio_reader = audio_created_events.get_reader();
        assert_eq!(create_audio_reader.iter(audio_created_events).len(), 1);

        let graphics_created_events = app.world.resource::<Events<CreateGraphicsAspectEvent>>();
        let mut create_graphics_reader = graphics_created_events.get_reader();
        assert_eq!(create_graphics_reader.iter(graphics_created_events).len(), 2);
    }
}

#[test]
fn test_despawn_entities()
{
    let mut app = App::new();

    app.add_plugin(AssetPlugin::default());
    app.add_asset::<Mesh>();
    app.add_asset::<StandardMaterial>();

    app.add_plugin(crate::audio_aspect::AudioAspect);
    app.add_plugin(crate::graphics_aspect::GraphicsAspect);
    app.add_plugin(EntityCreator);

    assert_eq!(app.world.query::<Entity>().iter(&app.world).len(), 0);

    let mut input = Input::<KeyCode>::default();
    input.press(KeyCode::C);
    app.insert_resource(input);
    app.update();

    {
        // Should this be 0 or 1?
        assert_eq!(app.world.query::<Entity>().iter(&app.world).len(), 1);
        let e = app.world.query::<Entity>().iter(&app.world).next();
        println!("created {:?}", e);

        let mut despawn_entities_events = app.world.resource_mut::<Events<DespawnEntitiesEvent>>();
        let despawn_event = DespawnEntitiesEvent { entity_ids: {let mut eids = Vec::new(); eids.push(e.unwrap().id()); eids}};
        despawn_entities_events.send(despawn_event);
    }

    app.update();
    {
        // should this be 0 or 1?
        assert_eq!(app.world.query::<Entity>().iter(&app.world).len(), 1);
    }

    app.update();

    {
        // should this be 0 or 1?  WHAT? 2???
        assert_eq!(app.world.query::<Entity>().iter(&app.world).len(), 0);
    }
}

}
