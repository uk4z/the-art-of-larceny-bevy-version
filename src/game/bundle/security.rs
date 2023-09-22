use crate::game::{playground::{security::components::{Active, SecurityBundle}, components::{ReachDistance, WorldPosition}}, components::Level};


pub fn get_security_bundle(level: &Level) -> Option<SecurityBundle> {
    match level {
        Level::Factory => {
            Some(
                SecurityBundle {
                    position: WorldPosition {x: 2534.0, y: 1411.0}, 
                    active: Active(true),
                    reach: ReachDistance(40.0),
                },
            )
        },
        Level::Tutorial => {
            Some(
                SecurityBundle {
                    position: WorldPosition {x: 1199.0, y: 767.0}, 
                    active: Active(true),
                    reach: ReachDistance(40.0),
                },
            )
        },
    }
}