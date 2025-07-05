use soroban_sdk::{Symbol, Vec, Bytes, contracttype, Env};
use crate::entity::EntityId;
use crate::component::Component;

#[contracttype]
#[derive(Debug, Clone)]
pub struct Storage {
    pub entity_ids: Vec<u64>,
    pub component_types: Vec<Symbol>,
    pub component_data: Vec<Bytes>,
}

impl Storage {
    pub fn new() -> Self {
        let env = soroban_sdk::Env::default();
        Self {
            entity_ids: Vec::new(&env),
            component_types: Vec::new(&env),
            component_data: Vec::new(&env),
        }
    }

    /// Add a component to storage
    pub fn add_component(&mut self, entity_id: EntityId, component: Component) {
        self.remove_component(entity_id, component.component_type().clone());
        self.entity_ids.push_back(entity_id.id());
        self.component_types.push_back(component.component_type().clone());
        self.component_data.push_back(component.data().clone());
    }

    /// Remove a component from storage
    pub fn remove_component(&mut self, entity_id: EntityId, component_type: Symbol) -> bool {
        let mut found = false;
        let mut new_entity_ids = Vec::new(&soroban_sdk::Env::default());
        let mut new_component_types = Vec::new(&soroban_sdk::Env::default());
        let mut new_component_data = Vec::new(&soroban_sdk::Env::default());
        for i in 0..self.entity_ids.len() {
            let eid = self.entity_ids.get(i).unwrap();
            let ctype = self.component_types.get(i).unwrap();
            let cdata = self.component_data.get(i).unwrap();
            if eid == entity_id.id() && ctype == component_type {
                found = true;
            } else {
                new_entity_ids.push_back(eid);
                new_component_types.push_back(ctype.clone());
                new_component_data.push_back(cdata.clone());
            }
        }
        if found {
            self.entity_ids = new_entity_ids;
            self.component_types = new_component_types;
            self.component_data = new_component_data;
        }
        found
    }

    /// Get a component from storage
    pub fn get_component(&self, entity_id: EntityId, component_type: Symbol) -> Option<Component> {
        for i in 0..self.entity_ids.len() {
            let eid = self.entity_ids.get(i).unwrap();
            let ctype = self.component_types.get(i).unwrap();
            let cdata = self.component_data.get(i).unwrap();
            if eid == entity_id.id() && ctype == component_type {
                return Some(Component::new(ctype.clone(), cdata.clone()));
            }
        }
        None
    }

    /// Check if a component exists in storage
    pub fn has_component(&self, entity_id: EntityId, component_type: Symbol) -> bool {
        for i in 0..self.entity_ids.len() {
            let eid = self.entity_ids.get(i).unwrap();
            let ctype = self.component_types.get(i).unwrap();
            if eid == entity_id.id() && ctype == component_type {
                return true;
            }
        }
        false
    }

    /// Get all components for an entity
    pub fn get_entity_components(&self, entity_id: EntityId) -> Vec<Component> {
        let env = soroban_sdk::Env::default();
        let mut components = Vec::new(&env);
        for i in 0..self.entity_ids.len() {
            let eid = self.entity_ids.get(i).unwrap();
            let ctype = self.component_types.get(i).unwrap();
            let cdata = self.component_data.get(i).unwrap();
            if eid == entity_id.id() {
                components.push_back(Component::new(ctype.clone(), cdata.clone()));
            }
        }
        components
    }

    pub fn clear(&mut self) {
        let env = soroban_sdk::Env::default();
        self.entity_ids = Vec::new(&env);
        self.component_types = Vec::new(&env);
        self.component_data = Vec::new(&env);
    }

    pub fn len(&self) -> usize {
        self.entity_ids.len().try_into().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.entity_ids.is_empty()
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

pub type TableStorage = Storage;
pub type SparseStorage = Storage; 