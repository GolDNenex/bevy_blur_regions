// Demonstrates how to use the `BlurRegionsCamera::blur` immediate blurring api.
//   cargo run --example immediate

#[path = "./utils.rs"]
mod utils;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy_blur_regions::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BlurRegionsPlugin::default())
        .add_systems(Startup, (setup, utils::spawn_example_scene_3d))
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DefaultBlurRegionsCamera::default(),
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn update(windows: Query<&Window>, mut blur_region_cameras: Query<&mut DefaultBlurRegionsCamera>) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok(mut blur_regions) = blur_region_cameras.get_single_mut() else {
        return;
    };

    let screen_size = Vec2::new(
        window.resolution.physical_width() as f32,
        window.resolution.physical_height() as f32,
    );
    blur_regions.blur(Rect::from_center_size(
        vec2(0.25, 0.5) * screen_size,
        vec2(0.3, 0.5) * screen_size,
    ));
    blur_regions.blur(Rect::from_center_size(
        vec2(0.75, 0.5) * screen_size,
        vec2(0.3, 0.5) * screen_size,
    ));
}
