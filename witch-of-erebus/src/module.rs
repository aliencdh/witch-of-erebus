use raylib::math::Vector2;

use crate::*;

pub enum Module {
    Core(CoreModule),
    User(UserModule),
}
impl Module {
    /// Returns an entity of the specified `EntityType` centered at the translation specified in the descriptor.
    pub fn request_entity_by_id(
        &self,
        descriptor: &RequestEntityByIDDescriptor,
    ) -> anyhow::Result<Entity> {
        match self {
            Module::Core(module) => module.request_entity_by_id(descriptor),
            Module::User(module) => module.request_entity_by_id(descriptor),
        }
    }

    pub fn entity_types(&self) -> &[EntityType] {
        match self {
            Module::Core(module) => module.entity_types(),
            Module::User(module) => module.entity_types(),
        }
    }
}

pub struct RequestEntityByIDDescriptor {
    pub id: usize,
    pub translation: Vector2,
    pub rotation: f32,
    pub scale: f32,
}

/// A module that's part of the core game.
pub struct CoreModule {
    pub name: String,
    pub entities: Vec<EntityType>,
    pub update: Box<dyn Fn(&mut State) -> ()>,
    pub init: Box<dyn Fn(&mut State) -> ()>,
}
impl CoreModule {
    fn request_entity_by_id(
        &self,
        descriptor: &RequestEntityByIDDescriptor,
    ) -> anyhow::Result<Entity> {
        Ok(
            self.entities[descriptor.id].create_entity(&CreateEntityDescriptor {
                translation: descriptor.translation,
                rotation: descriptor.rotation,
                scale: descriptor.scale,
            }),
        )
    }

    fn entity_types(&self) -> &[EntityType] {
        &self.entities
    }
}

/// A module installed by a user, aka a mod.
pub struct UserModule {
    pub name: String,
    pub entities: Vec<Box<EntityType>>,
    pub update:
        Box<libloading::Symbol<'static, for<'r> unsafe extern "C" fn(&'r State) -> &'r [Change]>>,
    pub init:
        Box<libloading::Symbol<'static, for<'r> unsafe extern "C" fn(&'r State) -> &'r [Change]>>,
}
impl UserModule {
    fn request_entity_by_id(
        &self,
        _descriptor: &RequestEntityByIDDescriptor,
    ) -> anyhow::Result<Entity> {
        todo!()
    }

    fn entity_types(&self) -> &[EntityType] {
        todo!()
    }
}
