use soroban_sdk::{Symbol, Vec};
use alloc::boxed::Box;
use crate::world::World;
use crate::entity::EntityId;

/// A query for entities with specific components
#[derive(Debug, Clone)]
pub struct Query {
    /// Required component types
    pub required_components: Vec<Symbol>,
    /// Excluded component types
    pub excluded_components: Vec<Symbol>,
}

impl Query {
    /// Create a new query
    pub fn new() -> Self {
        let env = soroban_sdk::Env::default();
        Self {
            required_components: Vec::new(&env),
            excluded_components: Vec::new(&env),
        }
    }

    /// Add a required component type
    pub fn with_component(mut self, component_type: Symbol) -> Self {
        self.required_components.push_back(component_type);
        self
    }

    /// Add an excluded component type
    pub fn without_component(mut self, component_type: Symbol) -> Self {
        self.excluded_components.push_back(component_type);
        self
    }

    /// Execute the query on a world
    pub fn execute(&self, world: &World) -> Vec<EntityId> {
        let env = soroban_sdk::Env::default();
        let mut results = Vec::new(&env);
        
        for entity in world.iter_entities() {
            // Check if entity has all required components
            let has_required = self.required_components.iter()
                .all(|component_type| entity.has_component(&component_type));
            
            // Check if entity has none of the excluded components
            let has_excluded = self.excluded_components.iter()
                .any(|component_type| entity.has_component(&component_type));
            
            if has_required && !has_excluded {
                results.push_back(entity.id());
            }
        }
        
        results
    }

    /// Check if the query is empty (no requirements)
    pub fn is_empty(&self) -> bool {
        self.required_components.is_empty() && self.excluded_components.is_empty()
    }
}

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}

/// Query state for tracking query results
#[derive(Debug, Clone)]
pub struct QueryState {
    query: Query,
    last_results: Vec<EntityId>,
    last_execution_time: u64,
}

impl QueryState {
    /// Create a new query state
    pub fn new(query: Query) -> Self {
        let env = soroban_sdk::Env::default();
        Self {
            query,
            last_results: Vec::new(&env),
            last_execution_time: 0,
        }
    }

    /// Execute the query and update state
    pub fn execute(&mut self, world: &World) -> &Vec<EntityId> {
        self.last_results = self.query.execute(world);
        self.last_execution_time = 0; // In a real implementation, this would be the current time
        &self.last_results
    }

    /// Get the last query results
    pub fn results(&self) -> &Vec<EntityId> {
        &self.last_results
    }

    /// Check if the query has any results
    pub fn is_empty(&self) -> bool {
        self.last_results.is_empty()
    }

    /// Get the number of results
    pub fn len(&self) -> usize {
        self.last_results.len().try_into().unwrap()
    }

    /// Get the last execution time
    pub fn last_execution_time(&self) -> u64 {
        self.last_execution_time
    }

    /// Check if the query needs to be re-executed
    pub fn needs_update(&self, current_time: u64) -> bool {
        // In a real implementation, you might check if the world has changed
        // For now, we'll just return true to always re-execute
        true
    }
}

/// Query builder for constructing complex queries
pub struct QueryBuilder {
    query: Query,
}

impl QueryBuilder {
    /// Create a new query builder
    pub fn new() -> Self {
        Self {
            query: Query::new(),
        }
    }

    /// Add a required component type
    pub fn with_component(mut self, component_type: Symbol) -> Self {
        self.query = self.query.with_component(component_type);
        self
    }

    /// Add an excluded component type
    pub fn without_component(mut self, component_type: Symbol) -> Self {
        self.query = self.query.without_component(component_type);
        self
    }

    /// Add multiple required component types
    pub fn with_components(mut self, component_types: Vec<Symbol>) -> Self {
        for component_type in component_types {
            self.query = self.query.with_component(component_type);
        }
        self
    }

    /// Add multiple excluded component types
    pub fn without_components(mut self, component_types: Vec<Symbol>) -> Self {
        for component_type in component_types {
            self.query = self.query.without_component(component_type);
        }
        self
    }

    /// Build the final query
    pub fn build(self) -> Query {
        self.query
    }

    /// Build the final query state
    pub fn build_state(self) -> QueryState {
        QueryState::new(self.query)
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Query filter for more complex querying
pub trait QueryFilter {
    /// Check if an entity matches this filter
    fn matches(&self, world: &World, entity_id: EntityId) -> bool;
}

/// Filter for entities with a specific component
pub struct WithComponent {
    component_type: Symbol,
}

impl WithComponent {
    /// Create a new filter
    pub fn new(component_type: Symbol) -> Self {
        Self { component_type }
    }
}

impl QueryFilter for WithComponent {
    fn matches(&self, world: &World, entity_id: EntityId) -> bool {
        world.has_component(entity_id, &self.component_type)
    }
}

/// Filter for entities without a specific component
pub struct WithoutComponent {
    component_type: Symbol,
}

impl WithoutComponent {
    /// Create a new filter
    pub fn new(component_type: Symbol) -> Self {
        Self { component_type }
    }
}

impl QueryFilter for WithoutComponent {
    fn matches(&self, world: &World, entity_id: EntityId) -> bool {
        !world.has_component(entity_id, &self.component_type)
    }
}

/// Combined filter that requires all sub-filters to match
pub struct AllFilters {
    filters: Vec<Symbol>, // Simplified to just store component types
}

impl AllFilters {
    /// Create a new combined filter
    pub fn new() -> Self {
        let env = soroban_sdk::Env::default();
        Self {
            filters: Vec::new(&env),
        }
    }

    /// Add a filter
    pub fn add_filter(mut self, component_type: Symbol) -> Self {
        self.filters.push_back(component_type);
        self
    }
}

impl QueryFilter for AllFilters {
    fn matches(&self, world: &World, entity_id: EntityId) -> bool {
        if let Some(entity) = world.get_entity(entity_id) {
            for i in 0..self.filters.len() {
                let ctype = self.filters.get(i).unwrap();
                if !entity.has_component(&ctype) {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

/// Combined filter that requires any sub-filter to match
pub struct AnyFilter {
    filters: Vec<Symbol>, // Simplified to just store component types
}

impl AnyFilter {
    /// Create a new combined filter
    pub fn new() -> Self {
        let env = soroban_sdk::Env::default();
        Self {
            filters: Vec::new(&env),
        }
    }

    /// Add a filter
    pub fn add_filter(mut self, component_type: Symbol) -> Self {
        self.filters.push_back(component_type);
        self
    }
}

impl QueryFilter for AnyFilter {
    fn matches(&self, world: &World, entity_id: EntityId) -> bool {
        if let Some(entity) = world.get_entity(entity_id) {
            for i in 0..self.filters.len() {
                let ctype = self.filters.get(i).unwrap();
                if entity.has_component(&ctype) {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }
}

/// Execute a query with a custom filter
pub fn query_with_filter(world: &World, filter: &dyn QueryFilter) -> Vec<EntityId> {
    let env = soroban_sdk::Env::default();
    let mut results = Vec::new(&env);
    
    for entity in world.iter_entities() {
        if filter.matches(world, entity.id()) {
            results.push_back(entity.id());
        }
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_query_creation() {
        let query = Query::new();
        assert!(query.is_empty());
    }

    #[test]
    fn test_query_with_component() {
        let query = Query::new()
            .with_component(symbol_short!("position"))
            .with_component(symbol_short!("velocity"));
        
        assert!(!query.is_empty());
        assert_eq!(query.required_components.len(), 2);
    }

    #[test]
    fn test_query_builder() {
        let query = QueryBuilder::new()
            .with_component(symbol_short!("position"))
            .without_component(symbol_short!("dead"))
            .build();
        
        assert!(!query.is_empty());
        assert_eq!(query.required_components.len(), 1);
        assert_eq!(query.excluded_components.len(), 1);
    }

    #[test]
    fn test_query_state() {
        let query = Query::new().with_component(symbol_short!("position"));
        let mut query_state = QueryState::new(query);
        
        let world = World::new();
        let results = query_state.execute(&world);
        assert_eq!(results.len(), 0);
        assert!(query_state.is_empty());
    }

    #[test]
    fn test_with_component_filter() {
        let filter = WithComponent::new(symbol_short!("position"));
        let world = World::new();
        
        // Since we have no entities with position components, this should return false
        let entity_id = EntityId::new(1, 0);
        assert!(!filter.matches(&world, entity_id));
    }

    #[test]
    fn test_without_component_filter() {
        let filter = WithoutComponent::new(symbol_short!("position"));
        let world = World::new();
        
        // Since we have no entities with position components, this should return true
        let entity_id = EntityId::new(1, 0);
        assert!(filter.matches(&world, entity_id));
    }

    #[test]
    fn test_all_filters() {
        let filter = AllFilters::new()
            .add_filter(WithComponent::new(symbol_short!("position")))
            .add_filter(WithoutComponent::new(symbol_short!("dead")));
        
        let world = World::new();
        let entity_id = EntityId::new(1, 0);
        
        // Should return false because no entity has position component
        assert!(!filter.matches(&world, entity_id));
    }

    #[test]
    fn test_any_filter() {
        let filter = AnyFilter::new()
            .add_filter(WithComponent::new(symbol_short!("position")))
            .add_filter(WithComponent::new(symbol_short!("velocity")));
        
        let world = World::new();
        let entity_id = EntityId::new(1, 0);
        
        // Should return false because no entity has either component
        assert!(!filter.matches(&world, entity_id));
    }

    #[test]
    fn test_query_with_filter() {
        let filter = WithComponent::new(symbol_short!("position"));
        let world = World::new();
        
        let results = query_with_filter(&world, &filter);
        assert_eq!(results.len(), 0);
    }
} 