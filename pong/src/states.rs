use bevy::{
    prelude::*,
    time::FixedTimestep,
};

use crate::physics::*;
use crate::paddle::*;
use crate::ball::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(paddle_setup)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_run_criteria(FixedTimestep::step(super::FPS as f64))
                    .with_system(check_for_collisions)
                    .with_system(reset_ball)
                    .with_system(paddle_movement.before(check_for_collisions))
                    .with_system(apply_velocity.before(check_for_collisions))
            );
    }
}