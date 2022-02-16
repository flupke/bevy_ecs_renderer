use bevy_ecs::prelude::*;
use std::collections::HashMap;

// Layer components
#[derive(Component)]
struct Layer(String);

#[derive(Component)]
struct Parent(Option<Entity>);

#[derive(Component)]
struct ZIndex(u64);

#[derive(Component)]
struct Rotation(f64);

// Operation components
#[derive(Component)]
struct Rotate(f64);

#[derive(Component)]
struct LayerRef(Entity);

fn main() {
    let mut world = World::new();

    // Add some layers
    //let parent = add_layer(&mut world, String::from("parent"), None);
    let parent = world
        .spawn()
        .insert(Layer(String::from("parent")))
        .insert(Parent(None))
        .insert(Rotation(0.0))
        .insert(ZIndex(0))
        .id();
    world
        .spawn()
        .insert(Layer(String::from("first_child")))
        .insert(Parent(Some(parent)))
        .insert(Rotation(0.0))
        .insert(ZIndex(0));
    world
        .spawn()
        .insert(Layer(String::from("second_child")))
        .insert(Parent(Some(parent)))
        .insert(Rotation(0.0))
        .insert(ZIndex(0));

    // Add some operations
    world.spawn().insert(Rotate(0.5)).insert(LayerRef(parent));

    let mut schedule = Schedule::default();
    schedule
        .add_stage(
            "update",
            SystemStage::parallel().with_system_set(
                SystemSet::new()
                    .with_system(compute_z_index)
                    .with_system(update_rotations),
            ),
        )
        .add_stage("render", SystemStage::single_threaded().with_system(render));

    schedule.run(&mut world);
    schedule.run(&mut world);
}

fn compute_z_index(mut query: Query<(&Parent, &mut ZIndex)>) {
    let mut root_counter = 0_u64;
    let mut parents_counters = HashMap::new();
    for (parent, mut z_index) in query.iter_mut() {
        let computed_z_index = match parent.0 {
            Some(parent_entity) => {
                parents_counters
                    .entry(parent_entity)
                    .and_modify(|z_index| *z_index += 1)
                    .or_insert(0_u64);
                parents_counters[&parent_entity]
            }
            None => {
                root_counter += 1;
                root_counter
            }
        };
        z_index.0 = computed_z_index;
    }
}

fn update_rotations(
    rotations_query: Query<(&LayerRef, &Rotate)>,
    mut layers_query: Query<&mut Rotation>,
) {
    for (layer_ref, rotate) in rotations_query.iter() {
        let mut rotation = layers_query.get_mut(layer_ref.0).unwrap();
        rotation.0 += rotate.0;
    }
}

fn render(query: Query<(&Layer, &ZIndex, &Rotation)>) {
    for (layer, z_index, rotation) in query.iter() {
        println!("{} z_index={} rotation={}", layer.0, z_index.0, rotation.0);
    }
    println!();
}
