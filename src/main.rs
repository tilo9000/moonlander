use crate::ml::*;
use bevy::prelude::*;

mod ml;

fn main() {
    App::build()
        .add_plugin(MLPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
