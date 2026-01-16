use bevy::{
    prelude::*
};
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum LocationState {
    #[default]
    Moving,
    Location(usize),
}
mod location;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins((
        DefaultPlugins,
    ))
    .add_plugins(location::LocationPlugin)
    .add_systems(Startup, init)
    .run()
    ;
}

// ---

fn init (
    mut cmd: Commands,
) {
    cmd.spawn((
        Transform::from_xyz(15., 15., 15.).looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        AmbientLight{brightness: 500., ..default()}
    ));
    cmd.spawn((
        DirectionalLight{
            illuminance: 1_0000.,
            ..default()
        },
        Transform::IDENTITY.looking_at(Vec3::ZERO, Vec3::Y)
    ));
    cmd.spawn((
        Node {
            padding: UiRect::all(Val::Px(10.)),
            ..default()  
        },
        children![
            Text::new("Press Space for next model")
        ]
        
    ));

}

