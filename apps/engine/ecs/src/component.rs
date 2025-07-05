use soroban_sdk::{Symbol, Bytes, Env, contracttype, symbol_short, Val, IntoVal, TryFromVal, FromVal};
use alloc::vec::Vec;

/// A unique identifier for a component type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentId {
    id: u32,
}

impl ComponentId {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
    pub fn id(&self) -> u32 {
        self.id
    }
}
// Soroban SDK trait implementations for ComponentId
impl IntoVal<Env, Val> for ComponentId {
    fn into_val(&self, env: &Env) -> Val {
        self.id.into_val(env)
    }
}

impl TryFromVal<Env, Val> for ComponentId {
    type Error = soroban_sdk::ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let id: u32 = TryFromVal::try_from_val(env, val)?;
        Ok(ComponentId::new(id))
    }
}

#[contracttype]
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ComponentStorage {
    Table = 0,
    Sparse = 1,
}
impl Default for ComponentStorage {
    fn default() -> Self {
        Self::Table
    }
}



#[contracttype]
#[derive(Debug, Clone)]
pub struct Component {
    pub component_type: Symbol,
    pub data: Bytes,
    pub storage: ComponentStorage,
}

impl Component {
    pub fn new(component_type: Symbol, data: Bytes) -> Self {
        Self {
            component_type,
            data,
            storage: ComponentStorage::default(),
        }
    }
    pub fn with_storage(component_type: Symbol, data: Bytes, storage: ComponentStorage) -> Self {
        Self {
            component_type,
            data,
            storage,
        }
    }
    pub fn component_type(&self) -> &Symbol {
        &self.component_type
    }
    pub fn data(&self) -> &Bytes {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut Bytes {
        &mut self.data
    }
    pub fn storage(&self) -> ComponentStorage {
        self.storage
    }
    pub fn set_storage(&mut self, storage: ComponentStorage) {
        self.storage = storage;
    }
}

/// Registry for managing component types
#[derive(Debug, Clone)]
pub struct ComponentRegistry {
    next_id: u32,
    components: Vec<(Symbol, ComponentId)>,
}

impl ComponentRegistry {
    /// Create a new component registry
    pub fn new() -> Self {
        Self {
            next_id: 1,
            components: Vec::new(),
        }
    }

    /// Register a new component type
    pub fn register_component(&mut self, component_type: Symbol) -> ComponentId {
        // Check if component type is already registered
        for (ctype, id) in &self.components {
            if ctype == &component_type {
                return *id;
            }
        }
        
        let id = ComponentId::new(self.next_id);
        self.next_id += 1;
        self.components.push((component_type, id));
        id
    }

    /// Get the component ID for a component type
    pub fn get_component_id(&self, component_type: &Symbol) -> Option<ComponentId> {
        for (ctype, id) in &self.components {
            if ctype == component_type {
                return Some(*id);
            }
        }
        None
    }

    /// Get the component type for a component ID
    pub fn get_component_type(&self, component_id: ComponentId) -> Option<Symbol> {
        for (ctype, id) in &self.components {
            if id == &component_id {
                return Some(ctype.clone());
            }
        }
        None
    }

    /// Get the number of registered component types
    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    /// Check if a component type is registered
    pub fn is_registered(&self, component_type: &Symbol) -> bool {
        for (ctype, _) in &self.components {
            if ctype == component_type {
                return true;
            }
        }
        false
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}



pub trait ComponentTrait {
    fn component_type() -> Symbol;
    fn serialize(&self, env: &Env) -> Bytes;
    fn deserialize(env: &Env, data: &Bytes) -> Option<Self>
    where
        Self: Sized;
    fn default_storage() -> ComponentStorage {
        ComponentStorage::Table
    }
}

#[contracttype]
#[derive(Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl ComponentTrait for Position {
    fn component_type() -> Symbol {
        symbol_short!("position")
    }
    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        let x_bytes = Bytes::from_array(env, &self.x.to_be_bytes());
        let y_bytes = Bytes::from_array(env, &self.y.to_be_bytes());
        bytes.append(&x_bytes);
        bytes.append(&y_bytes);
        bytes
    }
    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 8 {
            return None;
        }
        let x = i32::from_be_bytes([data.get(0).unwrap(), data.get(1).unwrap(), data.get(2).unwrap(), data.get(3).unwrap()]);
        let y = i32::from_be_bytes([data.get(4).unwrap(), data.get(5).unwrap(), data.get(6).unwrap(), data.get(7).unwrap()]);
        Some(Self { x, y })
    }
}

#[contracttype]
#[derive(Clone)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}
impl Velocity {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl ComponentTrait for Velocity {
    fn component_type() -> Symbol {
        symbol_short!("velocity")
    }
    fn serialize(&self, env: &Env) -> Bytes {
        let mut bytes = Bytes::new(env);
        let x_bytes = Bytes::from_array(env, &self.x.to_be_bytes());
        let y_bytes = Bytes::from_array(env, &self.y.to_be_bytes());
        bytes.append(&x_bytes);
        bytes.append(&y_bytes);
        bytes
    }
    fn deserialize(env: &Env, data: &Bytes) -> Option<Self> {
        if data.len() != 8 {
            return None;
        }
        let x = i32::from_be_bytes([data.get(0).unwrap(), data.get(1).unwrap(), data.get(2).unwrap(), data.get(3).unwrap()]);
        let y = i32::from_be_bytes([data.get(4).unwrap(), data.get(5).unwrap(), data.get(6).unwrap(), data.get(7).unwrap()]);
        Some(Self { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_component_id_creation() {
        let id = ComponentId::new(1);
        assert_eq!(id.id(), 1);
    }

    #[test]
    fn test_component_creation() {
        let env = Env::default();
        let component_type = symbol_short!("test");
        let data = Bytes::new(&env);
        data.append(&1u8);
        data.append(&2u8);
        data.append(&3u8);
        data.append(&4u8);
        let component = Component::new(component_type, data.clone());
        
        assert_eq!(component.component_type(), &symbol_short!("test"));
        assert_eq!(component.data(), &data);
        assert_eq!(component.storage(), ComponentStorage::Table);
    }

    #[test]
    fn test_component_registry() {
        let mut registry = ComponentRegistry::new();
        assert_eq!(registry.component_count(), 0);

        let component_type = symbol_short!("test");
        let id = registry.register_component(component_type);
        assert_eq!(registry.component_count(), 1);
        assert!(registry.is_registered(&component_type));

        let retrieved_id = registry.get_component_id(&component_type);
        assert_eq!(retrieved_id, Some(id));
    }

    #[test]
    fn test_position_component() {
        let env = Env::default();
        let position = Position::new(100, 200);
        let data = position.serialize(&env);
        let deserialized = Position::deserialize(&env, &data).unwrap();
        
        assert_eq!(position.x, deserialized.x);
        assert_eq!(position.y, deserialized.y);
    }
} 