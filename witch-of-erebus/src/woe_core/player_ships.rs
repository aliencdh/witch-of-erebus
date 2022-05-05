//! Defines the starship and boarding ship.

use raylib::math::Vector2;

use super::CORE_ASSET_FOLDER;
use crate::*;

pub fn module<'et>(
    rl: &mut raylib::RaylibHandle,
    thread: &raylib::RaylibThread,
) -> anyhow::Result<Module<'et>> {
    info!("Initializing `[Core] Player Ships` module.");
    let boarding_ship = EntityType {
        label: String::from("[Core] Boarding Ship"),
        texture: rl
            .load_texture(
                thread,
                &format!("{}{}", CORE_ASSET_FOLDER, "core_boarding_ship.png"),
            )
            .map_err(anyhow::Error::msg)?,
    };

    let starship = EntityType {
        label: String::from("[Core] Starship"),
        texture: rl
            .load_texture(
                thread,
                &format!("{}{}", CORE_ASSET_FOLDER, "core_starship.png"),
            )
            .map_err(anyhow::Error::msg)?,
    };

    Ok(Module::Core(CoreModule {
        name: String::from("[Core] Player Ships"),
        entities: vec![boarding_ship, starship],
        update: Box::new(update),
        init: Box::new(init),
    }))
}

fn update<'et>(_state: &State, _module: &'et CoreModule) -> Vec<Change<'et>> {
    todo!()
}

fn init<'et>(state: &State, module: &'et CoreModule) -> Vec<Change<'et>> {
    // add boarding ship
    // entities.insert(
    // 0,
    // Box::new(
    // modules[0].request_entity_by_id(&RequestEntityByIDDescriptor {
    // id: 0,
    // translation: Vector2::new((window_width / 2) as f32, (window_height / 2) as f32),
    // rotation: 0.0,
    // scale: 1.0,
    // })?,
    // ),
    // );

    Vec::from([Change::AddEntity(Entity {
        entity_type: &module.entities[0], // boarding ship
        translation: Vector2::new(
            (state.window_width / 2) as f32,
            (state.window_height / 2) as f32,
        ),
        rotation: 0.0,
        scale: 1.0,
    })])
}
