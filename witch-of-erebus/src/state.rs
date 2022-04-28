use std::collections::HashMap;

use raylib::prelude::*;

use crate::entity::{CreateEntityDescriptor, EntityType};
use crate::module::{Module, RequestEntityByIDDescriptor};
use crate::Entity;

pub struct GlobalState<'lt> {
    pub clear_color: Color,
    pub modules: &'static Vec<Module>,
    pub loaded_entities: HashMap<usize, Box<Entity<'lt>>>,
    pub entities_by_name: HashMap<String, &'lt EntityType>,
}
impl<'lt> GlobalState<'lt> {
    pub fn new(
        modules: &'static Vec<Module>,
        window_width: usize,
        window_height: usize,
    ) -> anyhow::Result<Self> {
        let mut entities_by_name = HashMap::new();
        for module in modules {
            for entity_type in module.entity_types() {
                entities_by_name.insert(entity_type.label.clone(), entity_type);
            }
        }

        let mut entities = HashMap::new();
        // add boarding ship
        entities.insert(
            0,
            Box::new(
                modules[0].request_entity_by_id(&RequestEntityByIDDescriptor {
                    id: 0,
                    translation: Vector2::new(
                        (window_width / 2) as f32,
                        (window_height / 2) as f32,
                    ),
                    rotation: 0.0,
                    scale: 1.0,
                })?,
            ),
        );

        Ok(Self {
            clear_color: Color::BLACK,
            modules,
            loaded_entities: entities,
            entities_by_name,
        })
    }

    pub fn update(&mut self) {
        let mut state = State::new(
            (
                self.clear_color.r,
                self.clear_color.g,
                self.clear_color.b,
                self.clear_color.a,
            ),
            &self.loaded_entities,
        );
        let mut all_changes = vec![];
        for module_wrapper in self.modules {
            match module_wrapper {
                Module::Core(module) => module.update.call((&mut state,)),
                Module::User(module) => unsafe {
                    let changes = module.update.as_ref()(&state);
                    all_changes.append(&mut Vec::from(changes));
                },
            }
        }
        self.resolve_changes(&all_changes);
    }

    pub fn draw(
        &self,
        rl: &mut raylib::RaylibHandle,
        thread: &raylib::RaylibThread,
    ) -> anyhow::Result<()> {
        let mut draw_handle = rl.begin_drawing(thread);

        draw_handle.clear_background(self.clear_color);

        for (_, entity) in &self.loaded_entities {
            entity.draw(&mut draw_handle)?;
        }
        Ok(())
    }

    fn resolve_changes(&mut self, changes: &[Change]) {
        for change in changes {
            match change {
                &Change::ClearColor(r, g, b, a) => self.clear_color = Color::new(r, g, b, a),
                &Change::RequestEntityByID(id, translation, rotation, scale) => todo!(),
                Change::RequestEntityByLabel(label, translation, rotation, scale) => {
                    let max_id = self.loaded_entities.keys().max().unwrap();
                    self.loaded_entities.insert(
                        max_id + 1,
                        Box::new(self.entities_by_name.get(label).unwrap().create_entity(
                            &CreateEntityDescriptor {
                                translation: Vector2::from(translation.clone()),
                                rotation: rotation.clone(),
                                scale: scale.clone(),
                            },
                        )),
                    );
                }
                &Change::RmEntity(id) => {
                    self.loaded_entities.remove(&id);
                }
            }
        }
    }
}

/// Interface for modules to interact with the game state.
#[repr(C)]
pub struct State<'own, 'entity> {
    pub clear_color: (u8, u8, u8, u8),
    pub entities: &'entity HashMap<usize, Box<Entity<'entity>>>,
    phantom_data: &'own (),
}
impl<'own, 'entity> State<'own, 'entity> {
    pub fn new(
        clear_color: (u8, u8, u8, u8),
        entities: &'entity HashMap<usize, Box<Entity<'entity>>>,
    ) -> Self {
        Self {
            clear_color,
            entities,
            phantom_data: &(),
        }
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub enum Change {
    ClearColor(u8, u8, u8, u8),
    /// # UNSUPPORTED
    /// Instruction to request an entity given its ID, translation, rotation and scale.
    RequestEntityByID(usize, (f32, f32), f32, f32),
    /// Instruction to request an entity given its label, translation, rotation and scale.
    RequestEntityByLabel(String, (f32, f32), f32, f32),
    /// Instruction to remove an entity given its ID.
    RmEntity(usize),
}
