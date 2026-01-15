use bevy::{
    prelude::*,
    color::palettes::css
};

use crate::LocationState;
pub struct LocationPlugin;
impl Plugin for LocationPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<LocationNextIdx>()
        .init_resource::<LocationHandles>()
        .init_state::<LocationState>()
        .add_computed_state::<Location>()
        .add_systems(OnEnter(LocationState::Moving), moving)
        .add_systems(OnEnter(Location), enter)
        .add_systems(Update, loaded.run_if(on_message::<AssetEvent<Scene>>))
        ;
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Location;

impl ComputedStates for Location {
    type SourceStates = LocationState;
    fn compute(sources: LocationState) -> Option<Self> {
        match sources {
            LocationState::Location (..) => Some(Self),
            _ => None,
        }
    }
}

#[derive(Resource, Default)]
pub struct LocationHandles([Option<Handle<Scene>>; 4]);

#[derive(Component)]
struct LocationIdx(usize);

#[derive(Resource, Default)]
struct LocationNextIdx(usize);

// ---

fn exit(
    tr: On<Pointer<Click>>,
    mut next: ResMut<NextState<LocationState>>,
    mut next_index: ResMut<LocationNextIdx>,
    portal_q: Query<&LocationIdx>
) {
    if let Ok(LocationIdx(loc_idx)) = portal_q.get(tr.entity)  {
        next_index.0 = *loc_idx;
        next.set(LocationState::Moving);
    }
}

// ---

fn moving( 
    mut next: ResMut<NextState<LocationState>>,
    next_index: Res<LocationNextIdx>
) {
    next.set(LocationState::Location(next_index.0));
}

// ---

fn enter(
    mut cmd: Commands,
    assets: ResMut<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    location: Res<State<LocationState>>,
    mut room_handles: ResMut<LocationHandles>

) {
    let LocationState::Location(loc_index) = *location.get() else {
        return;
    };

    if let Some(handle) = &room_handles.0[loc_index]  {
        cmd.run_system_cached_with(spawn, handle.clone());
    } else {
        let sh = assets.load(GltfAssetLabel::Scene(0).from_asset(format!("models/locations/location-{}.glb", loc_index + 1)));
        room_handles.0[loc_index] = Some(sh.clone());
    }


    let mesh = meshes.add(Cuboid::from_size(vec3(1., 0.1, 1.)));
    let mat = materials.add(Color::from(css::ALICE_BLUE));


    let exits = vec![
        (if loc_index == 0 {3} else {loc_index - 1}, 0. ),
        ((loc_index + 1) % 4 , 2.)
    ];

    for e in exits {
        cmd.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(mat.clone()),
            DespawnOnExit(Location),
            LocationIdx(e.0),
            Transform::from_xyz(e.1, 0., 0.)
        ))
        .observe(exit)
        ;
    }
}

// ---

fn loaded(
    mut reader: MessageReader<AssetEvent<Scene>>,
    room_handles: Res<LocationHandles>,
    mut cmd: Commands,
    location: Res<State<LocationState>>,
) {
    let LocationState::Location(loc_index) = *location.get() else {
        return;
    };

    let Some(handle) = &room_handles.0[loc_index] else {
        return;
    };
    
    for r in reader.read() {
        if let AssetEvent::Added { id } = r {
            if *id  == handle.id() {
                cmd.run_system_cached_with(spawn, handle.clone());
            }
        }
    }
}

// ---

fn spawn(
    In(handle): In<Handle<Scene>>,
    mut cmd: Commands,
) {
    cmd.spawn((
        SceneRoot(handle),
        DespawnOnExit(Location)
    ));
}