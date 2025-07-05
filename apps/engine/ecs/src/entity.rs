use soroban_sdk::{Symbol, Vec, Env, Val, IntoVal, TryFromVal, FromVal};
use core::fmt;

/// A unique identifier for an entity in the ECS world
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId {
    id: u64,
    generation: u32,
}

impl EntityId {
    /// Create a new entity ID
    pub fn new(id: u64, generation: u32) -> Self {
        Self { id, generation }
    }

    /// Get the numeric ID
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Get the generation
    pub fn generation(&self) -> u32 {
        self.generation
    }

    /// Check if this entity ID is valid
    pub fn is_valid(&self) -> bool {
        self.id != 0
    }
}

// Soroban SDK trait implementations for EntityId
impl IntoVal<Env, Val> for EntityId {
    fn into_val(&self, env: &Env) -> Val {
        (self.id, self.generation).into_val(env)
    }
}

impl TryFromVal<Env, Val> for EntityId {
    type Error = soroban_sdk::ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let (id, generation): (u64, u32) = TryFromVal::try_from_val(env, val)?;
        Ok(EntityId::new(id, generation))
    }
}

/// An entity in the ECS world
#[derive(Debug, Clone)]
pub struct Entity {
    id: EntityId,
    component_types: Vec<Symbol>,
}

impl Entity {
    /// Create a new entity
    pub fn new(id: EntityId) -> Self {
        let env = soroban_sdk::Env::default();
        Self {
            id,
            component_types: Vec::new(&env),
        }
    }

    /// Get the entity ID
    pub fn id(&self) -> EntityId {
        self.id
    }

    /// Add a component type to this entity
    pub fn add_component_type(&mut self, component_type: Symbol) {
        self.component_types.push_back(component_type);
    }

    /// Remove a component type from this entity
    pub fn remove_component_type(&mut self, component_type: &Symbol) -> bool {
        let mut found = false;
        let mut new_components = Vec::new(&soroban_sdk::Env::default());
        
        for i in 0..self.component_types.len() {
            let ctype = self.component_types.get(i).unwrap();
            if ctype == *component_type {
                found = true;
            } else {
                new_components.push_back(ctype.clone());
            }
        }
        
        if found {
            self.component_types = new_components;
        }
        found
    }

    /// Check if this entity has a specific component type
    pub fn has_component(&self, component_type: &Symbol) -> bool {
        for i in 0..self.component_types.len() {
            let ctype = self.component_types.get(i).unwrap();
            if ctype == *component_type {
                return true;
            }
        }
        false
    }

    /// Get all component types for this entity
    pub fn component_types(&self) -> &Vec<Symbol> {
        &self.component_types
    }

    /// Get the number of components
    pub fn component_count(&self) -> usize {
        self.component_types.len().try_into().unwrap()
    }
}

// Soroban SDK trait implementations for Entity
impl IntoVal<Env, Val> for Entity {
    fn into_val(&self, env: &Env) -> Val {
        // Serialize as a simple structure that Soroban can handle
        let id_val: Val = self.id.into_val(env);
        let types_val: Val = self.component_types.clone().into_val(env);
        (id_val, types_val).into_val(env)
    }
}

impl TryFromVal<Env, Val> for Entity {
    type Error = soroban_sdk::ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let (id_val, types_val): (Val, Val) = TryFromVal::try_from_val(env, val)?;
        let id: EntityId = TryFromVal::try_from_val(env, &id_val)?;
        let component_types: Vec<Symbol> = TryFromVal::try_from_val(env, &types_val)?;
        Ok(Entity { id, component_types })
    }
}

/// Manager for handling entity lifecycle
#[derive(Debug, Clone)]
pub struct EntityManager {
    next_id: u64,
    entities: Vec<Entity>,
    free_list: Vec<u64>,
}

impl EntityManager {
    /// Create a new entity manager
    pub fn new() -> Self {
        let env = soroban_sdk::Env::default();
        Self {
            next_id: 1,
            entities: Vec::new(&env),
            free_list: Vec::new(&env),
        }
    }

    /// Spawn a new entity
    pub fn spawn(&mut self) -> EntityId {
        let id = if self.free_list.len() > 0 {
            let freed_id = self.free_list.get(self.free_list.len() - 1).unwrap();
            self.free_list.remove(self.free_list.len() - 1);
            freed_id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            id
        };
        
        let entity_id = EntityId::new(id, 0);
        let entity = Entity::new(entity_id);
        self.entities.push_back(entity);
        entity_id
    }

    /// Despawn an entity
    pub fn despawn(&mut self, entity_id: EntityId) -> bool {
        for i in 0..self.entities.len() {
            let entity = self.entities.get(i).unwrap();
            if entity.id() == entity_id {
                self.entities.remove(i);
                self.free_list.push_back(entity_id.id());
                return true;
            }
        }
        false
    }

    /// Get an entity by ID
    pub fn get_entity(&self, entity_id: EntityId) -> Option<Entity> {
        for i in 0..self.entities.len() {
            let entity = self.entities.get(i).unwrap();
            if entity.id() == entity_id {
                return Some(entity.clone());
            }
        }
        None
    }

    /// Get a mutable reference to an entity by ID
    pub fn get_entity_mut(&mut self, entity_id: EntityId) -> Option<Entity> {
        // Since soroban_sdk::Vec doesn't have get_mut, we'll need to restructure this
        // For now, return a clone - this is a limitation of the Soroban SDK
        self.get_entity(entity_id)
    }

    /// Get the total number of entities
    pub fn entity_count(&self) -> usize {
        self.entities.len().try_into().unwrap()
    }

    /// Check if an entity exists
    pub fn exists(&self, entity_id: EntityId) -> bool {
        for i in 0..self.entities.len() {
            let entity = self.entities.get(i).unwrap();
            if entity.id() == entity_id {
                return true;
            }
        }
        false
    }

    /// Iterate over all entities
    pub fn iter_entities(&self) -> EntityIterator {
        EntityIterator {
            entities: &self.entities,
            index: 0,
        }
    }

    /// Iterate over all entities mutably
    pub fn iter_entities_mut(&mut self) -> EntityIteratorMut {
        EntityIteratorMut {
            entities: &mut self.entities,
            index: 0,
        }
    }
}

impl Default for EntityManager {
    fn default() -> Self {
        Self::new()
    }
}

// Soroban SDK trait implementations for EntityManager
impl IntoVal<Env, Val> for EntityManager {
    fn into_val(&self, env: &Env) -> Val {
        (self.next_id, self.entities.clone(), self.free_list.clone()).into_val(env)
    }
}

impl TryFromVal<Env, Val> for EntityManager {
    type Error = soroban_sdk::ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let (next_id, entities, free_list): (u64, Vec<Entity>, Vec<u64>) = TryFromVal::try_from_val(env, val)?;
        Ok(EntityManager { next_id, entities, free_list })
    }
}

/// Iterator over entities
pub struct EntityIterator<'a> {
    entities: &'a Vec<Entity>,
    index: u32,
}

impl<'a> Iterator for EntityIterator<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entities.len() {
            let entity = self.entities.get(self.index).unwrap();
            self.index += 1;
            Some(entity.clone())
        } else {
            None
        }
    }
}

/// Mutable iterator over entities
pub struct EntityIteratorMut<'a> {
    entities: &'a mut Vec<Entity>,
    index: u32,
}

impl<'a> Iterator for EntityIteratorMut<'a> {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entities.len() {
            let entity = self.entities.get(self.index).unwrap();
            self.index += 1;
            Some(entity.clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_entity_id_creation() {
        let entity_id = EntityId::new(1, 0);
        assert_eq!(entity_id.id(), 1);
        assert_eq!(entity_id.generation(), 0);
        assert!(entity_id.is_valid());
    }

    #[test]
    fn test_entity_creation() {
        let env = Env::default();
        let entity_id = EntityId::new(1, 0);
        let entity = Entity::new(entity_id);
        assert_eq!(entity.id(), entity_id);
        assert!(entity.is_empty());
    }

    #[test]
    fn test_entity_manager() {
        let mut manager = EntityManager::new();
        assert_eq!(manager.entity_count(), 0);

        let entity_id = manager.spawn();
        assert_eq!(manager.entity_count(), 1);
        assert!(manager.exists(entity_id));

        assert!(manager.despawn(entity_id));
        assert_eq!(manager.entity_count(), 0);
        assert!(!manager.exists(entity_id));
    }
} 