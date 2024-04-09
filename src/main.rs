mod simulation;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_silk::prelude::*;
// use simulation::{Params, Simulation};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(ClothPlugin)
        .run();
}
