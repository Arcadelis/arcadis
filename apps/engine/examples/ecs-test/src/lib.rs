#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Bytes, Env, Symbol, Vec};
use soroban_ecs::component::{Component, ComponentTrait};
use soroban_ecs::world::World;

// -- COMPONENT DEFINITIONS --

/// A component representing the position of an entity in 2D space.

#[contracttype]
#[derive(Clone, Debug, PartialEq)]

pub struct Position {
    pub x: u32,

    pub y: u32,
}

/// Implementation of the `ComponentTrait` allows `Position` to be used in the ECS.

impl ComponentTrait for Position {
    /// A unique identifier for the component type.

    fn component_type() -> Symbol {
        symbol_short!("pos")
    }

    /// Serializes the component data into bytes for storage.

    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);

        bytes.append(&Bytes::from_array(env, &self.x.to_be_bytes()));

        bytes.append(&Bytes::from_array(env, &self.y.to_be_bytes()));

        bytes
    }

    /// Deserializes bytes from storage back into a `Position` instance.

    fn deserialize(_env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 8 {
            return None;
        }

        let x = u32::from_be_bytes(data.slice(0..4).try_into().unwrap());

        let y = u32::from_be_bytes(data.slice(4..8).try_into().unwrap());

        Some(Self { x, y })
    }
}

/// A component representing the velocity of an entity.

#[contracttype]
#[derive(Clone, Debug, PartialEq)]

pub struct Velocity {
    pub dx: i32,

    pub dy: i32,
}

impl ComponentTrait for Velocity {
    fn component_type() -> Symbol {
        symbol_short!("vel")
    }

    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);

        bytes.append(&Bytes::from_array(env, &self.dx.to_be_bytes()));

        bytes.append(&Bytes::from_array(env, &self.dy.to_be_bytes()));

        bytes
    }

    fn deserialize(_env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 8 {
            return None;
        }

        let dx = i32::from_be_bytes(data.slice(0..4).try_into().unwrap());

        let dy = i32::from_be_bytes(data.slice(4..8).try_into().unwrap());

        Some(Self { dx, dy })
    }
}

// -- SYSTEM DEFINITION --

/// A system that updates the position of entities based on their velocity.

/// In this implementation, a system is a standalone function that operates on the `World`.

pub fn movement_system(env: &Env, world: &mut World) {
    // Query for all entities that have both a `Position` and a `Velocity` component.

    let query_component_types: [Symbol; 2] =
        [Position::component_type(), Velocity::component_type()];

    let entities_to_update = world.query_entities(&query_component_types);

    for entity_id in entities_to_update.iter() {
        // Retrieve current components.

        let pos_comp = world
            .get_component(entity_id, &Position::component_type())
            .unwrap();

        let vel_comp = world
            .get_component(entity_id, &Velocity::component_type())
            .unwrap();

        // Deserialize component data to perform logic.

        let mut pos = Position::deserialize(env, &pos_comp.data).unwrap();

        let vel = Velocity::deserialize(env, &vel_comp.data).unwrap();

        // Update position.

        pos.x = ((pos.x as i32).saturating_add(vel.dx)).max(0) as u32;
        pos.y = ((pos.y as i32).saturating_add(vel.dy)).max(0) as u32;

        // Serialize the updated component.

        let new_pos_comp = Component::new(Position::component_type(), pos.serialize(env));

        // The current API requires removing the old component and adding the new one.

        world.remove_component_from_entity(entity_id, &Position::component_type());

        world.add_component_to_entity(entity_id, new_pos_comp);
    }
}

// -- CONTRACT DEFINITION --

#[contract]

pub struct EcsTestContract;

#[contractimpl]
impl EcsTestContract {
    pub fn run(env: Env) -> (u32, u32) {
        // 1. Create a new World (it will create its own env)
        let mut world = World::new();

        // 2. Define the components for our entity using the contract's env
        let pos = Position { x: 10, y: 20 };
        let vel = Velocity { dx: 5, dy: -5 };

        // Use the world's internal env for component serialization
        let world_env = soroban_sdk::Env::default();
        let pos_component = Component::new(Position::component_type(), pos.serialize(&world_env));
        let vel_component = Component::new(Velocity::component_type(), vel.serialize(&world_env));

        let mut components = Vec::new(&world_env);
        components.push_back(pos_component);
        components.push_back(vel_component);

        // 3. Spawn an entity with the specified components
        let entity = world.spawn(components);

        // 4. Update position manually (as in your original code)
        let pos_comp = world
            .get_component(entity.id(), &Position::component_type())
            .unwrap();
        let vel_comp = world
            .get_component(entity.id(), &Velocity::component_type())
            .unwrap();

        let mut pos = Position::deserialize(&world_env, &pos_comp.data).unwrap();
        let vel = Velocity::deserialize(&world_env, &vel_comp.data).unwrap();

        pos.x = ((pos.x as i32).saturating_add(vel.dx)).max(0) as u32;
        pos.y = ((pos.y as i32).saturating_add(vel.dy)).max(0) as u32;

        let new_pos_comp = Component::new(Position::component_type(), pos.serialize(&world_env));
        world.remove_component_from_entity(entity.id(), &Position::component_type());
        world.add_component_to_entity(entity.id(), new_pos_comp);

        // 5. Retrieve the final position
        let final_pos_comp = world
            .get_component(entity.id(), &Position::component_type())
            .unwrap();
        let final_pos = Position::deserialize(&world_env, &final_pos_comp.data).unwrap();

        (final_pos.x, final_pos.y)
    }
}

#[cfg(test)]
mod test;
