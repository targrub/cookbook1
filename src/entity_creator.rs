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
use bevy::prelude::ParallelSystemDescriptorCoercion;

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

pub struct RemoveEntityCreatorComponentEvent {
    entity: Entity,
}

#[derive(Debug)]
struct DespawnEntitiesEvent {
    entities: Vec<Entity>,
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
            .add_system(remove_entity_creator_components.after(mysystem))
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
//        let event = DespawnEntitiesEvent { entities, }
//        ev_despawn_entities_writer.send(event);
    }
}

fn create_entity_and_aspects(
    commands: &mut Commands,
    writers: &mut Writers,
)
{
    let e = create_entity(commands, writers);
    add_audio_aspect(commands, writers, e, AudioAspectType::default());
    add_graphics_aspect(commands, writers, e, Color::BLUE, "bee".to_string());
    add_graphics_aspect(commands, writers, e, Color::ALICE_BLUE, "cee".to_string());
    //remove_entity_creator_component(e, commands, writers);
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
    entity: Entity,
    commands: &mut Commands,
    writers: &mut Writers,
) {
    writers.ev_remove_entity_creator_component_writer.send(RemoveEntityCreatorComponentEvent {entity});
}

fn add_audio_aspect(
    commands: &mut Commands,
    writers: &mut Writers,
    entity: Entity,
    audiotype: AudioAspectType,
)
{
    writers.ev_createaudio_writer.send(CreateAudioAspectEvent {entity: Some(entity), audiotype});
}

fn add_graphics_aspect(
    commands: &mut Commands,
    writers: &mut Writers,
    entity: Entity,
    color: Color,
    name: String,
)
{
    writers.ev_creategraphics_writer.send(CreateGraphicsAspectEvent {entity: Some(entity), name});
}

fn despawn_entities(
    mut commands: Commands,
    mut ev_despawn_entities_reader: EventReader<DespawnEntitiesEvent>,
    q: Query<Entity>,
) {
    for ev in ev_despawn_entities_reader.iter() {
        for de in ev.entities.iter() {
            if q.contains(*de) {
                commands.entity(*de).despawn();
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
        if q.contains(ev.entity) {
            commands.entity(ev.entity).remove::<EntityCreatorComponent>();
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
    app.world.resource_mut::<Input<KeyCode>>().clear();

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
        //assert_eq!(app.world.query::<&EntityCreatorComponent>().iter(&app.world).len(), 1);
    }

    app.update();

    {
        assert_eq!(app.world.entities().len(), 1);
    }
}

/*
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

    assert_eq!(app.world.entities().len(), 0);

    let mut input = Input::<KeyCode>::default();
    input.press(KeyCode::C);
    app.insert_resource(input);
    app.update();
    app.world.resource_mut::<Input<KeyCode>>().clear();

    {
        // Should this be 0 or 1?
        assert_eq!(app.world.entities().len(), 1);
        let e = app.world.query::<Entity>().iter(&app.world).next();    // clumsy.  better incantation?

        let despawn_event = DespawnEntitiesEvent { entities: vec![e.unwrap()]};

        app.world.send_event(despawn_event);

// this doesn't change the outcome, and is more verbose
//        let mut despawn_entities_events = app.world.resource_mut::<Events<DespawnEntitiesEvent>>();
//        despawn_entities_events.send(despawn_event);
    }

    app.update();
    {
        // should this be 0 or 1?
        assert_eq!(app.world.entities().len(), 0);
    }

    app.update();

    {
        // should this be 0 or 1?  WHAT? 2???
        assert_eq!(app.world.entities().len(), 0);
    }
}
*/
fn commands_test_spawn_minimal(
    mut commands: Commands,
) {
    commands.spawn();
}

#[test]
fn test_bevy_spawn_entity_results_in_1_entity_after_each_update()
{
    let mut app = App::new();

    assert_eq!(app.world.entities().len(), 0);

    app.update();

    assert_eq!(app.world.entities().len(), 0);

    app.add_system(commands_test_spawn_minimal);

    app.update();

    assert_eq!(app.world.entities().len(), 1);

    app.update();

    assert_eq!(app.world.entities().len(), 2);
}

#[derive(Default)]
struct ShouldSpawn(bool);

fn commands_test_spawn_minimal_with_resource_guard(
    resource: Res<ShouldSpawn>,
    mut commands: Commands,
) {
    if resource.0 {
        commands.spawn();
    }
}

#[test]
fn test_bevy_spawn_additional_entity_each_update_when_resource_is_true()
{
    let mut app = App::new();

    assert_eq!(app.world.entities().len(), 0);

    app.update();

    assert_eq!(app.world.entities().len(), 0);

    app.insert_resource(ShouldSpawn(false));
    app.add_system(commands_test_spawn_minimal_with_resource_guard);

    app.update();

    assert_eq!(app.world.entities().len(), 0);

    app.insert_resource(ShouldSpawn(true));
    app.update();

    assert_eq!(app.world.entities().len(), 1);

    app.update();

    assert_eq!(app.world.entities().len(), 2);
    app.insert_resource(ShouldSpawn(false));

    app.update();

    assert_eq!(app.world.entities().len(), 2);
}


#[test]
fn test_despawning_through_despawnentitiesevent()
{
    let mut app = App::new();

    app.add_event::<DespawnEntitiesEvent>();
    app.add_system(despawn_entities);

    assert_eq!(app.world.entities().len(), 0);

    app.insert_resource(ShouldSpawn(true));
    app.add_system(commands_test_spawn_minimal_with_resource_guard);

    app.update();

    assert_eq!(app.world.entities().len(), 1);

    app.insert_resource(ShouldSpawn(false));

    app.update();

    assert_eq!(app.world.entities().len(), 1);

    let e = app.world.query::<Entity>().iter(&app.world).next();    // clumsy.  better incantation?

    let despawn_event = DespawnEntitiesEvent { entities: vec![e.unwrap()]};
    app.world.send_event(despawn_event);

    app.update();

    assert_eq!(app.world.entities().len(), 0);

    app.update();

    assert_eq!(app.world.entities().len(), 0);
}

#[test]
fn test_despawning_through_despawnentitiesevent_using_plugin()
{
    let mut app = App::new();

    app.add_plugin(AssetPlugin::default());
    app.add_asset::<Mesh>();
    app.add_asset::<StandardMaterial>();

    app.add_plugin(crate::audio_aspect::AudioAspect);
    app.add_plugin(crate::graphics_aspect::GraphicsAspect);
    app.add_plugin(EntityCreator);

    let mut input = Input::<KeyCode>::default();
    app.insert_resource(input);

    assert_eq!(app.world.entities().len(), 0);

    app.insert_resource(ShouldSpawn(true));
    app.add_system(commands_test_spawn_minimal_with_resource_guard);

    app.update();

    assert_eq!(app.world.entities().len(), 1);

    app.insert_resource(ShouldSpawn(false));

    app.update();

    assert_eq!(app.world.entities().len(), 1);

    let e = app.world.query::<Entity>().iter(&app.world).next();    // clumsy.  better incantation?

    let despawn_event = DespawnEntitiesEvent { entities: vec![e.unwrap()]};
    app.world.send_event(despawn_event);

    app.update();

    assert_eq!(app.world.entities().len(), 0);

    app.update();

    assert_eq!(app.world.entities().len(), 0);
}


#[test]
fn test_despawning_after_spawning_using_plugin()
{
    let mut app = App::new();

    let mut input = Input::<KeyCode>::default();
    input.press(KeyCode::C);
    app.insert_resource(input);

    app.add_plugin(AssetPlugin::default());
    app.add_asset::<Mesh>();
    app.add_asset::<StandardMaterial>();

    app.add_plugin(crate::audio_aspect::AudioAspect);
    app.add_plugin(crate::graphics_aspect::GraphicsAspect);
    app.add_plugin(EntityCreator);

    app.update();

    app.world.resource_mut::<Input<KeyCode>>().clear();

    assert_eq!(app.world.entities().len(), 1);

    app.update();

    assert_eq!(app.world.entities().len(), 1);

    let e = app.world.query::<Entity>().iter(&app.world).next();    // clumsy.  better incantation?

    let despawn_event = DespawnEntitiesEvent { entities: vec![e.unwrap()]};
    app.world.send_event(despawn_event);

    app.update();

    assert_eq!(app.world.entities().len(), 0);

    app.update();

    assert_eq!(app.world.entities().len(), 0);
}

#[test]
fn test_despawn_many_times()
{
    for _ in 0..1000 {
        test_despawning_after_spawning_using_plugin();
    }
}

#[test]
fn test_entity_creation_many_times()
{
    for _ in 0..1000 {
        test_entity_creation();
    }
}

#[test]
fn test_1_after_each_update_many_times()
{
    for _ in 0..1000 {
        test_bevy_spawn_entity_results_in_1_entity_after_each_update();
    }
}

#[test]

fn test_despawning_through_despawnentitiesevent_using_plugin_many_times()
{
    for _ in 0..1000 {
        test_despawning_through_despawnentitiesevent_using_plugin();
    }
}

#[test]
fn test_despawning_through_despawnentitiesevent_many_times()
{
    for _ in 0..1000 {
        test_despawning_through_despawnentitiesevent();
    }
}

#[test]
fn test_add_entity_only_when_resource_true_many_times()
{
    for _ in 0..1000 {
        test_bevy_spawn_additional_entity_each_update_when_resource_is_true();
    }
}
}
