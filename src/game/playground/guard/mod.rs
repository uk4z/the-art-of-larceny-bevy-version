pub mod systems;
pub mod components;

use bevy::prelude::*;

use crate::game::playground::components::WorldPosition;
use crate::game::playground::guard::components::Patrol; 

use systems::*;


pub struct GuardPlugin;

impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_guard)
            .add_system(move_guard)
            .add_system(update_patrols_positions)
            .add_system(guard_motion_handler)
            .add_system(update_fov);
    }
}




pub fn get_guard_direction( 
    patrol: &mut Patrol,
    guard_position: &WorldPosition, 
) -> Vec3 {
    let direction = Vec3::from(patrol.get_position())-Vec3::from(*guard_position);
    direction.normalize_or_zero()
}