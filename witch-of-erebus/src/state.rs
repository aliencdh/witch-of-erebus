use std::collections::HashMap;

use raylib::math::{Quaternion, Transform, Vector3};
use raylib::prelude::*;

use crate::core;
use crate::module::{Module, RequestEntityByIDDescriptor};
use crate::Entity;

pub struct GlobalState<'lt> {
    pub clear_color: Color,
    pub modules: &'static Vec<Module>,
    pub loaded_entities: Vec<Box<Entity<'lt>>>,
}
impl<'lt> GlobalState<'lt> {
    pub fn new(
        modules: &'static Vec<Module>,
        window_width: usize,
        window_height: usize,
    ) -> anyhow::Result<Self> {
        let mut entities = vec![];
        // add boarding ship
        entities.push(Box::new(modules[0].request_entity_by_id(
            &RequestEntityByIDDescriptor {
                id: 0,
                translation: Vector2::new((window_width / 2) as f32, (window_height / 2) as f32),
                rotation: 0.0,
                scale: 1.0,
            },
        )?));

        Ok(Self {
            clear_color: Color::BLACK,
            modules,
            loaded_entities: entities,
        })
    }

    pub fn update(&mut self) {
        let mut state = State {
            clear_color: (
                self.clear_color.r,
                self.clear_color.g,
                self.clear_color.b,
                self.clear_color.a,
            ),
        };
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

    pub fn draw(&self, rl: &mut raylib::RaylibHandle, thread: &raylib::RaylibThread) {
        let mut draw_handle = rl.begin_drawing(thread);

        draw_handle.clear_background(self.clear_color);

        for entity in &self.loaded_entities {
            entity.draw(&mut draw_handle);
        }
    }

    fn resolve_changes(&mut self, changes: &[Change]) {
        for change in changes {
            match change {
                &Change::ClearColor(r, g, b, a) => self.clear_color = Color::new(r, g, b, a),
            }
        }
    }
}

#[repr(C)]
pub struct State {
    pub clear_color: (u8, u8, u8, u8),
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Change {
    ClearColor(u8, u8, u8, u8),
}
