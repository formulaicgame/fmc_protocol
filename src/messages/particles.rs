use bevy::{
    math::{DVec3, Vec3},
    prelude::*,
};
use fmc_protocol_derive::ClientBound;
use serde::{Deserialize, Serialize};

#[derive(ClientBound, Event, Serialize, Deserialize, Debug, Clone)]
pub enum ParticleEffect {
    //Spawn {
    //    /// If the particle effect is long lasting and the server wants to despawn it at a later
    //    /// time, this should hold a unique id.
    //    id: Option<usize>,
    //    /// If the particle effect uses a texture, this needs to hold its' path relative to the
    //    /// "textures" directory of the assets.
    //    texture: Option<String>,
    //},
    //Delete {
    //    /// Id of particle effect
    //    id: usize,
    //},
    Explosion {
        /// Spawn location
        position: DVec3,
        /// Maximum offset a particle can be spawned at
        spawn_offset: Vec3,
        /// Min and max length of mesh quad
        size_range: (f32, f32),
        /// Minimum initial velocity
        min_velocity: Vec3,
        /// Maximum initial velocity
        max_velocity: Vec3,
        /// Texture used by mesh
        texture: String,
        /// Lifetime of each particle
        lifetime: f32,
        /// How many particles should be spawned
        count: u32,
    },
}
