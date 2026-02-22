//@compile-flags: -Cdebug-assertions
use bevy_ecs::prelude::*;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
#[constraint(forbid(Enemy))]
struct Player;

#[derive(Bundle)]
//~^ E0080
struct BadBundle {
    player: Player,
    enemy: Enemy,
}

fn main() {}
