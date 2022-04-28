//! Defines the starship and boarding ship.

use super::CORE_ASSET_FOLDER;
use crate::*;

pub fn module(
    rl: &mut raylib::RaylibHandle,
    thread: &raylib::RaylibThread,
) -> anyhow::Result<Module> {
    info!("Initializing `[Core] Player Ships` module.");
    let boarding_ship = EntityType {
        label: String::from("[Core] Boarding Ship"),
        texture: rl
            .load_texture(
                thread,
                &format!("{}{}", CORE_ASSET_FOLDER, "core_boarding_ship.png"),
            )
            .unwrap(),
    };

    let starship = EntityType {
        label: String::from("[Core] Starship"),
        texture: rl
            .load_texture(
                thread,
                &format!("{}{}", CORE_ASSET_FOLDER, "core_starship.png"),
            )
            .unwrap(),
    };

    Ok(Module::Core(CoreModule {
        name: String::from("[Core] Player Ships"),
        entities: vec![boarding_ship, starship],
        update: Box::new(update),
    }))
}

fn update(_state: &mut State) {}
