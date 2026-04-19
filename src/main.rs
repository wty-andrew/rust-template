use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            #[cfg(target_arch = "wasm32")]
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, quit)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_image(asset_server.load("sprites/bevy.png")));
}

fn quit(keys: Res<ButtonInput<KeyCode>>, mut writer: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        writer.write(AppExit::Success);
    }
}
