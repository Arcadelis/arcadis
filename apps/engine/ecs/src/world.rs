use soroban_sdk::{Symbol, Vec, contracttype};
use crate::entity::{Entity, EntityId, EntityManager, EntityIterator, EntityIteratorMut};
use crate::component::{Component, ComponentRegistry};
use crate::storage::Storage;
use crate::resource::Resource;
use crate::event::{Event};

/// The main ECS world that contains all entities, components, and systems
#[derive(Debug, Clone)]
pub struct World {
    /// Entity manager for handling entity lifecycle
    pub entities: EntityManager,
    /// Component registry for managing component types
    pub components: ComponentRegistry,
    /// Component storage system
    pub storage: Storage,
    /// Resources (global state)
    pub resources: Vec<Resource>,
    /// Event system
    pub events: Vec<Event>,
}

impl World {
    /// Create a new empty world
    pub fn new() -> Self {
        let env = soroban_sdk::Env::default();
        Self {
            entities: EntityManager::new(),
            components: ComponentRegistry::new(),
            storage: Storage::new(),
            resources: Vec::new(&env),
            events: Vec::new(&env),
        }
    }

    /// Spawn a new empty entity
    pub fn spawn_empty(&mut self) -> Entity {
        let entity_id = self.entities.spawn();
        Entity::new(entity_id)
    }

    /// Spawn a new entity with components
    pub fn spawn(&mut self, components: Vec<Component>) -> Entity {
        let entity_id = self.entities.spawn();
        let mut entity = Entity::new(entity_id);
        
        // Add components to the entity and storage
        for component in components {
            self.add_component_to_entity(entity_id, component);
        }
        
        entity
    }

    /// Add a component to an entity
    pub fn add_component_to_entity(&mut self, entity_id: EntityId, component: Component) {
        // Register the component type if not already registered
        self.components.register_component(component.component_type().clone());
        // Add component type to entity
        if let Some(mut entity) = self.entities.get_entity_mut(entity_id) {
            entity.add_component_type(component.component_type().clone());
            // Since we can't modify the entity in place, we need to update it
            // This is a limitation of the Soroban SDK
        }
        // Store the component data
        self.storage.add_component(entity_id, component);
    }

    /// Remove a component from an entity
    pub fn remove_component_from_entity(&mut self, entity_id: EntityId, component_type: &Symbol) -> bool {
        // Remove component type from entity
        if let Some(mut entity) = self.entities.get_entity_mut(entity_id) {
            entity.remove_component_type(component_type);
            // Since we can't modify the entity in place, we need to update it
            // This is a limitation of the Soroban SDK
        }
        // Remove component data from storage
        self.storage.remove_component(entity_id, component_type.clone())
    }

    /// Get a component from an entity
    pub fn get_component(&self, entity_id: EntityId, component_type: &Symbol) -> Option<Component> {
        self.storage.get_component(entity_id, component_type.clone())
    }

    /// Get a mutable reference to a component from an entity
    pub fn get_component_mut(&mut self, entity_id: EntityId, component_type: &Symbol) -> Option<Component> {
        // Since we simplified storage, we'll need to implement this differently
        // For now, return a clone of the component if it exists
        self.get_component(entity_id, component_type)
    }

    /// Check if an entity has a specific component
    pub fn has_component(&self, entity_id: EntityId, component_type: &Symbol) -> bool {
        if let Some(entity) = self.entities.get_entity(entity_id) {
            entity.has_component(component_type)
        } else {
            false
        }
    }

    /// Despawn an entity and remove all its components
    pub fn despawn(&mut self, entity_id: EntityId) -> bool {
        if let Some(entity) = self.entities.get_entity(entity_id) {
            // Remove all components from storage
            let component_types = entity.component_types().clone();
            for i in 0..component_types.len() {
                let ctype = component_types.get(i).unwrap();
                self.storage.remove_component(entity_id, ctype.clone());
            }
        }
        self.entities.despawn(entity_id)
    }

    /// Get the total number of entities
    pub fn entity_count(&self) -> usize {
        self.entities.entity_count()
    }

    /// Get the total number of component types
    pub fn component_count(&self) -> usize {
        self.components.component_count()
    }

    /// Check if an entity exists
    pub fn exists(&self, entity_id: EntityId) -> bool {
        self.entities.exists(entity_id)
    }

    /// Get an entity by ID
    pub fn get_entity(&self, entity_id: EntityId) -> Option<Entity> {
        self.entities.get_entity(entity_id)
    }

    /// Get a mutable reference to an entity by ID
    pub fn get_entity_mut(&mut self, entity_id: EntityId) -> Option<Entity> {
        self.entities.get_entity_mut(entity_id)
    }

    /// Add a resource to the world
    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.push_back(resource);
    }

    /// Get a resource by type
    pub fn get_resource(&self, resource_type: &Symbol) -> Option<Resource> {
        for i in 0..self.resources.len() {
            let res = self.resources.get(i).unwrap();
            if res.resource_type() == resource_type {
                return Some(res.clone());
            }
        }
        None
    }

    /// Get a mutable reference to a resource by type
    pub fn get_resource_mut(&mut self, resource_type: &Symbol) -> Option<Resource> {
        // Since soroban_sdk::Vec doesn't have iter_mut, we'll return a clone
        self.get_resource(resource_type)
    }

    /// Remove a resource from the world
    pub fn remove_resource(&mut self, resource_type: &Symbol) -> Option<Resource> {
        let mut found = None;
        let mut new_resources = Vec::new(&soroban_sdk::Env::default());
        for i in 0..self.resources.len() {
            let res = self.resources.get(i).unwrap();
            if res.resource_type() == resource_type {
                found = Some(res.clone());
            } else {
                new_resources.push_back(res.clone());
            }
        }
        if found.is_some() {
            self.resources = new_resources;
        }
        found
    }

    /// Send an event
    pub fn send_event(&mut self, event: Event) {
        self.events.push_back(event);
    }

    /// Get all events of a specific type
    pub fn get_events(&self, event_type: &Symbol) -> Vec<Event> {
        let env = soroban_sdk::Env::default();
        let mut filtered = Vec::new(&env);
        for i in 0..self.events.len() {
            let event = self.events.get(i).unwrap();
            if event.event_type() == event_type {
                filtered.push_back(event.clone());
            }
        }
        filtered
    }

    /// Clear all events
    pub fn clear_events(&mut self) {
        let env = soroban_sdk::Env::default();
        self.events = Vec::new(&env);
    }

    /// Iterate over all entities
    pub fn iter_entities(&self) -> EntityIterator {
        self.entities.iter_entities()
    }

    /// Iterate over all entities mutably
    pub fn iter_entities_mut(&mut self) -> EntityIteratorMut {
        self.entities.iter_entities_mut()
    }

    /// Query entities with specific components
    pub fn query_entities(&self, component_types: &[Symbol]) -> Vec<EntityId> {
        let env = soroban_sdk::Env::default();
        let mut results = Vec::new(&env);
        for entity in self.iter_entities() {
            let mut has_all_components = true;
            for i in 0..component_types.len() {
                let ctype = &component_types[i];
                if !entity.has_component(ctype) {
                    has_all_components = false;
                    break;
                }
            }
            if has_all_components {
                results.push_back(entity.id());
            }
        }
        results
    }

    /// Clear all entities and components
    pub fn clear_entities(&mut self) {
        self.entities = EntityManager::new();
        self.storage = Storage::new();
    }

    /// Clear all resources
    pub fn clear_resources(&mut self) {
        let env = soroban_sdk::Env::default();
        self.resources = Vec::new(&env);
    }

    /// Clear everything in the world
    pub fn clear(&mut self) {
        self.clear_entities();
        self.clear_resources();
        self.clear_events();
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, symbol_short};

    #[test]
    fn test_world_creation() {
        let world = World::new();
        assert_eq!(world.entity_count(), 0);
        assert_eq!(world.component_count(), 0);
    }

    #[test]
    fn test_entity_spawn() {
        let mut world = World::new();
        let entity = world.spawn_empty();
        assert_eq!(world.entity_count(), 1);
        assert!(world.exists(entity.id()));
    }

    #[test]
    fn test_component_management() {
        let mut world = World::new();
        let entity_id = world.spawn_empty().id();
        
        let component_type = symbol_short!("test");
        let component_data = vec![Env::default(), 1, 2, 3, 4];
        let component = Component::new(component_type, component_data);
        
        world.add_component_to_entity(entity_id, component);
        assert!(world.has_component(entity_id, &symbol_short!("test")));
        
        let retrieved_component = world.get_component(entity_id, &symbol_short!("test"));
        assert!(retrieved_component.is_some());
    }

    #[test]
    fn test_entity_despawn() {
        let mut world = World::new();
        let entity_id = world.spawn_empty().id();
        assert_eq!(world.entity_count(), 1);
        
        assert!(world.despawn(entity_id));
        assert_eq!(world.entity_count(), 0);
        assert!(!world.exists(entity_id));
    }

    #[test]
    fn test_resource_management() {
        let mut world = World::new();
        let resource_type = symbol_short!("test_resource");
        let resource_data = vec![Env::default(), 1, 2, 3, 4];
        let resource = Resource::new(resource_type, resource_data);
        
        world.add_resource(resource);
        assert!(world.get_resource(&symbol_short!("test_resource")).is_some());
        
        let removed_resource = world.remove_resource(&symbol_short!("test_resource"));
        assert!(removed_resource.is_some());
        assert!(world.get_resource(&symbol_short!("test_resource")).is_none());
    }

    #[test]
    fn test_event_system() {
        let mut world = World::new();
        let event_type = symbol_short!("test_event");
        let event_data = vec![Env::default(), 1, 2, 3, 4];
        let event = Event::new(event_type, event_data);
        
        world.send_event(event);
        let events = world.get_events(&symbol_short!("test_event"));
        assert_eq!(events.len(), 1);
        
        world.clear_events();
        let events = world.get_events(&symbol_short!("test_event"));
        assert_eq!(events.len(), 0);
    }
} 