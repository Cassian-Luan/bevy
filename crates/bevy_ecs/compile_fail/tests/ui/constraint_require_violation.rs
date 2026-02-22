//@compile-flags: -Cdebug-assertions
use bevy_ecs::prelude::*;

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
#[constraint(require(Health))]
struct Player;

#[derive(Bundle)]
//~^ E0080
struct BadBundle {
    player: Player,
}

fn main() {}
