use bevy::prelude::*;

use mars::Pos;

fn main() {
    App::build().add_plugins(DefaultPlugins).register_component::<Pos>().run();
}
