# soroban-ecs Module Documentation

## Overview

The `soroban-ecs` module provides an Entity Component System (ECS) implementation specifically designed for Soroban smart contracts. This module allows developers to create complex game logic and state management within the constraints of the Soroban blockchain environment.

### What is ECS?

Entity Component System (ECS) is an architectural pattern that separates data (components) from logic (systems) and uses entities as containers for components. This pattern is particularly well-suited for game development and complex state management.

### Key Features

- **no_std Compatible**: Designed to work within Soroban's WebAssembly environment
- **Memory Efficient**: Optimized for blockchain constraints
- **Type Safe**: Leverages Rust's type system for compile-time safety
- **Soroban Integration**: Seamlessly integrates with Soroban SDK
- **Event System**: Built-in event handling for reactive programming
- **Resource Management**: Global state management capabilities

## Architecture

### Core Concepts

#### Entities
Entities are unique identifiers that act as containers for components. They don't contain data themselves but serve as the glue between components and systems.

```rust
use soroban_ecs::{Entity, EntityId};

// EntityId represents a unique entity identifier
let entity_id = EntityId::new(1, 0);

// Entity contains metadata about the entity
let entity = Entity::new(entity_id);
```

#### Components
Components are data structures that define the properties and state of entities. They are pure data containers without behavior.

```rust
use soroban_ecs::{Component, ComponentTrait};
use soroban_sdk::{symbol_short, Env, Bytes};

// Define a custom component
#[derive(Clone)]
pub struct Health {
    pub current: u32,
    pub maximum: u32,
}

impl ComponentTrait for Health {
    fn component_type() -> Symbol {
        symbol_short!("health")
    }

    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        bytes.append(&self.current.to_be_bytes());
        bytes.append(&self.maximum.to_be_bytes());
        bytes
    }

    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 8 {
            return None;
        }
        let current = u32::from_be_bytes([
            data.get(0).unwrap(), data.get(1).unwrap(),
            data.get(2).unwrap(), data.get(3).unwrap()
        ]);
        let maximum = u32::from_be_bytes([
            data.get(4).unwrap(), data.get(5).unwrap(),
            data.get(6).unwrap(), data.get(7).unwrap()
        ]);
        Some(Self { current, maximum })
    }
}
```

#### Systems
Systems contain the logic that operates on entities with specific components. They define how the game world evolves over time.

```rust
use soroban_ecs::{System, SystemParam};

pub struct HealthSystem;

impl System for HealthSystem {
    type Fetch = SystemParam<Health>;

    fn fetch(world: &World) -> Self::Fetch {
        // Fetch entities with Health components
        SystemParam::new(world)
    }

    fn run(&self, health_components: &[Health]) {
        for health in health_components {
            // Apply health regeneration logic
            if health.current < health.maximum {
                health.current = health.current.saturating_add(1);
            }
        }
    }
}
```

#### World
The World is the central container that manages all entities, components, systems, and resources.

```rust
use soroban_ecs::World;

let mut world = World::new();
```

### Storage Architecture

The module provides two storage strategies:

#### Table Storage
- **Use Case**: Components that are present on most entities
- **Performance**: Fast iteration, memory efficient
- **Example**: Position, Health, Velocity

#### Sparse Storage
- **Use Case**: Components that are present on few entities
- **Performance**: Memory efficient for rare components
- **Example**: Special abilities, temporary effects

## Setup and Usage

### Adding to Your Project

1. **Add Dependency**
```toml
[dependencies]
soroban-ecs = { path = "apps/engine/ecs" }
```

2. **Import the Module**
```rust
use soroban_ecs::prelude::*;
```

### Basic Usage Pattern

```rust
use soroban_sdk::{contract, contractimpl, Env};
use soroban_ecs::{World, Position, Velocity};

#[contract]
pub struct GameContract;

#[contractimpl]
impl GameContract {
    pub fn init_game(env: &Env) -> World {
        let mut world = World::new();

        // Spawn a player entity
        let player_components = vec![
            Component::new(symbol_short!("pos"), Position::new(0, 0).serialize(env)),
            Component::new(symbol_short!("vel"), Velocity::new(1, 0).serialize(env)),
        ];

        world.spawn(player_components);
        world
    }

    pub fn move_player(env: &Env, world: &mut World, entity_id: u32) {
        // Get player position and velocity
        if let (Some(pos), Some(vel)) = (
            world.get_component(EntityId::new(entity_id, 0), &symbol_short!("pos")),
            world.get_component(EntityId::new(entity_id, 0), &symbol_short!("vel"))
        ) {
            // Update position based on velocity
            let new_pos = Position::new(
                pos.x + vel.x,
                pos.y + vel.y
            );

            // Update the component
            world.add_component_to_entity(
                EntityId::new(entity_id, 0),
                Component::new(symbol_short!("pos"), new_pos.serialize(env))
            );
        }
    }
}
```

## Integration with Soroban Contracts

### Contract Structure

```rust
use soroban_sdk::{contract, contractimpl, symbol_short, Env, IntoVal, TryFromVal};
use soroban_ecs::{World, Position, MovementSystem};

#[contract]
pub struct GameWorldContract;

#[derive(Clone, Debug)]
pub struct GameWorldData {
    pub is_initialized: bool,
    pub entity_count: u32,
    pub world_data: World,
}

// Implement Soroban traits for contract data
impl IntoVal<Env, soroban_sdk::Val> for GameWorldData {
    fn into_val(&self, env: &Env) -> soroban_sdk::Val {
        (self.is_initialized, self.entity_count).into_val(env)
    }
}

impl TryFromVal<Env, soroban_sdk::Val> for GameWorldData {
    type Error = soroban_sdk::ConversionError;

    fn try_from_val(env: &Env, val: &soroban_sdk::Val) -> Result<Self, Self::Error> {
        let (is_initialized, entity_count): (bool, u32) = TryFromVal::try_from_val(env, val)?;
        Ok(GameWorldData {
            is_initialized,
            entity_count,
            world_data: World::new(),
        })
    }
}
```

### Entity Management

```rust
#[contractimpl]
impl GameWorldContract {
    pub fn spawn_entity(env: &Env, x: u32, y: u32) -> u32 {
        let mut contract_data = Self::get_contract_data(env);

        // Create position component
        let position = Position::new(x as i32, y as i32);
        let entity_id = contract_data.entity_count;

        // Store entity data in contract storage
        let entity_key = symbol_short!("entity");
        let entity_data: (u32, u32, u32) = (entity_id, position.x as u32, position.y as u32);
        env.storage().instance().set(&entity_key, &entity_data.into_val(env));

        // Update entity count
        contract_data.entity_count += 1;
        Self::save_contract_data(env, &contract_data);

        entity_id
    }

    pub fn move_entity(env: &Env, entity_id: u32, dx: i32, dy: i32) -> bool {
        let entity_key = symbol_short!("entity");

        if let Some(entity_data) = env.storage().instance().get::<Symbol, Val>(&entity_key) {
            if let Ok((id, x, y)) = <(u32, u32, u32)>::try_from_val(env, &entity_data) {
                if id == entity_id {
                    // Use MovementSystem for position updates
                    let current_position = Position::new(x as i32, y as i32);
                    let new_position = MovementSystem::update(&current_position, dx, dy);

                    // Store updated position
                    let updated_data: (u32, u32, u32) = (id, new_position.x as u32, new_position.y as u32);
                    env.storage().instance().set(&entity_key, &updated_data.into_val(env));
                    return true;
                }
            }
        }
        false
    }
}
```

## Built-in Components

### Position Component
```rust
use soroban_ecs::Position;

let position = Position::new(10, 20);
assert_eq!(position.x, 10);
assert_eq!(position.y, 20);
```

### Velocity Component
```rust
use soroban_ecs::Velocity;

let velocity = Velocity::new(5, -3);
assert_eq!(velocity.x, 5);
assert_eq!(velocity.y, -3);
```

## Built-in Systems

### MovementSystem
```rust
use soroban_ecs::{Position, MovementSystem};

let position = Position::new(10, 20);
let new_position = MovementSystem::update(&position, 5, -3);
assert_eq!(new_position.x, 15);
assert_eq!(new_position.y, 17);
```

## Event System

The ECS module includes a built-in event system for reactive programming:

```rust
use soroban_ecs::{World, Event};
use soroban_sdk::symbol_short;

let mut world = World::new();

// Send an event
let event = Event::new(symbol_short!("player_moved"), event_data);
world.send_event(event);

// Get events of a specific type
let events = world.get_events(&symbol_short!("player_moved"));
for event in events {
    // Handle the event
}

// Clear all events
world.clear_events();
```

## Resource Management

Resources represent global state that can be accessed by all systems:

```rust
use soroban_ecs::{World, Resource};
use soroban_sdk::symbol_short;

let mut world = World::new();

// Add a global resource
let game_time = Resource::new(symbol_short!("game_time"), time_data);
world.add_resource(game_time);

// Access the resource
if let Some(time_resource) = world.get_resource(&symbol_short!("game_time")) {
    // Use the resource
}

// Remove the resource
world.remove_resource(&symbol_short!("game_time"));
```

## Querying Entities

Query entities that have specific components:

```rust
use soroban_ecs::World;
use soroban_sdk::symbol_short;

let world = World::new();

// Query entities with both Position and Velocity components
let component_types = vec![
    symbol_short!("position"),
    symbol_short!("velocity")
];

let entities = world.query_entities(&component_types);
for entity_id in entities {
    // Process entities with both components
}
```

## Limitations and no_std Considerations

### Memory Constraints
- **WASM Memory**: Limited by Soroban's WebAssembly environment
- **Allocation Strategy**: Uses `wee_alloc` for memory management
- **Component Size**: Keep components small to avoid memory issues

### Performance Considerations
- **Iteration**: Linear iteration over entities and components
- **Storage**: Choose appropriate storage strategy (Table vs Sparse)
- **Caching**: Minimize repeated queries and component access

### Soroban SDK Limitations
- **Mutable References**: Limited support for mutable references in vectors
- **Serialization**: All data must be serializable for storage
- **Environment**: All operations require Soroban environment context



## Example Integration

See the complete example in `apps/engine/examples/ecs-test/src/lib.rs` for a full implementation of a game world contract using the soroban-ecs module.

### Key Example Features:
- Entity spawning and despawning
- Component management
- System integration
- Contract storage integration
- Event handling
- Resource management


## Conclusion

The `soroban-ecs` module provides a powerful foundation for building complex game logic and state management within Soroban smart contracts. By following the ECS pattern and understanding the constraints of the blockchain environment, developers can create efficient and maintainable game contracts.

For more examples and advanced usage patterns, refer to the test files in the module source code and the example contract in `apps/engine/examples/ecs-test/`.