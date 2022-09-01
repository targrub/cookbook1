use bevy::app::App;
use bevy::DefaultPlugins;

mod entity_creator;
mod audio_aspect;
mod graphics_aspect;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(entity_creator::EntityCreator)
    .add_plugin(audio_aspect::AudioAspect)
    .add_plugin(graphics_aspect::GraphicsAspect)
    .run();
}
