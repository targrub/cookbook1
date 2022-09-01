#![allow(dead_code, unused,)]
// Bevy app/ecs types
use bevy::app::{App, Plugin};
use bevy::ecs::{
    component::Component,
    entity::Entity,
    event::EventWriter,
    system::{Commands, Res, SystemParam}
};

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

// Struct with SystemParam trait allows us to pass multiple EventWriters, EventReaders, etc., to subsystems
#[derive(SystemParam)]
struct Writers<'w, 's> {
    ev_createaudio_writer: EventWriter<'w, 's, CreateAudioAspectEvent>,
    ev_creategraphics_writer: EventWriter<'w, 's, CreateGraphicsAspectEvent>,
}

// This plugin
pub struct EntityCreator;

impl Plugin for EntityCreator {
    fn build(&self, app: &mut App) {
        app
            .add_system(mysystem)
            ;
    }        
}

// Systems for this plugin
fn mysystem(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut writers: Writers,
  ) {
    if input.just_pressed(KeyCode::C) {
        create_entity_and_aspects(&mut commands, &mut writers);
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
