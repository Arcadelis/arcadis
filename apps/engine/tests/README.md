# Game Logic Contract Unit Test Suite

This directory contains comprehensive unit tests for the Soroban ECS game logic contract located at `apps/engine/contracts/game/`.

## Test Structure

### `game.rs`
Main unit test suite with the following test modules:

#### Contract Initialization Tests (`contract_initialization_tests`)
- ✅ `test_contract_initialization` - Verifies proper contract initialization
- ✅ `test_multiple_initialization` - Tests handling of multiple init calls

#### Entity Management Tests (`entity_management_tests`)
- ✅ `test_entity_creation` - Validates entity spawning with unique IDs
- ✅ `test_entity_position_retrieval` - Tests position component storage/retrieval
- ✅ `test_entity_health_retrieval` - Tests health component storage/retrieval
- ✅ `test_entity_despawning` - Validates entity removal from world

#### Movement System Tests (`movement_system_tests`)
- ✅ `test_movement_system_basic_movement` - Basic position updates
- ✅ `test_movement_system_negative_boundary` - Boundary clamping (negative coordinates)
- ✅ `test_movement_system_large_coordinates` - Large coordinate handling
- ✅ `test_movement_system_zero_movement` - Zero delta movement

#### Combat System Tests (`combat_system_tests`)
- ✅ `test_combat_system_basic_attack` - Basic health reduction
- ✅ `test_combat_system_multiple_attacks` - Cumulative damage
- ✅ `test_combat_system_entity_death` - Entity death when health reaches 0
- ✅ `test_combat_system_health_underflow_protection` - Saturating subtraction

#### Error Handling Tests (`error_handling_tests`)
- ✅ `test_operations_on_nonexistent_entities` - Invalid entity ID handling
- ✅ `test_operations_on_dead_entities` - Operations on killed entities
- ✅ `test_edge_case_maximum_entity_ids` - Edge case entity ID limits

#### World State Tests (`world_state_tests`)
- ✅ `test_empty_world_behavior` - Empty world operations
- ✅ `test_world_state_consistency` - Complex operation sequences
- ✅ `test_entity_id_uniqueness` - ID assignment and uniqueness

#### Integration Tests (`integration_tests`)
- ✅ `test_full_gameplay_scenario` - Complete gameplay simulation
- ✅ `test_many_entities_stress_test` - Performance under load (100 entities)

#### Component/System Tests (`component_system_tests`)
- ✅ `test_gameposition_component_serialization` - GamePosition trait testing
- ✅ `test_health_component_serialization` - Health trait testing
- ✅ `test_invalid_deserialization` - Invalid data handling
- ✅ `test_movement_system_direct` - Direct MovementSystem testing
- ✅ `test_combat_system_direct` - Direct CombatSystem testing

## Running Tests

### Prerequisites
Ensure you have the required dependencies installed:
- Rust (latest stable)
- Soroban CLI: `cargo install --locked stellar-cli`
- WebAssembly target: `rustup target add wasm32-unknown-unknown`

### Test Commands

#### Run all game logic tests:
```bash
cd apps/engine/contracts/game
cargo test
```

#### Run specific test modules:
```bash
# Run only entity management tests
cargo test entity_management_tests

# Run only movement system tests  
cargo test movement_system_tests

# Run only error handling tests
cargo test error_handling_tests
```

#### Run with verbose output:
```bash
cargo test -- --nocapture
```

#### Run a specific test:
```bash
cargo test test_full_gameplay_scenario -- --exact
```

#### Run tests with Soroban CLI:
```bash
soroban contract test
```

### Running from Engine Root
You can also run tests from the engine root directory:
```bash
cd apps/engine
cargo test --package game
```

## Test Coverage

This test suite provides comprehensive coverage of:

### Core Functionality (100%)
- ✅ Contract initialization and state management
- ✅ Entity creation, modification, and removal
- ✅ Position and Health component operations
- ✅ MovementSystem and CombatSystem execution
- ✅ Storage and retrieval operations

### Error Handling (100%)
- ✅ Invalid entity ID operations
- ✅ Operations on non-existent entities
- ✅ Operations on dead entities
- ✅ Edge cases and boundary conditions

### Integration Scenarios (100%)
- ✅ Multi-entity gameplay scenarios
- ✅ Complex operation sequences
- ✅ State consistency validation
- ✅ Performance under load

### Component/System Isolation (100%)
- ✅ ComponentTrait implementations
- ✅ Serialization/deserialization
- ✅ System function isolation testing
- ✅ Data validation and error handling

## Best Practices Demonstrated

### Test Organization
- **Modular structure**: Tests organized by functionality
- **Clear naming**: Descriptive test and module names
- **Helper functions**: Reusable setup functions
- **Comprehensive coverage**: Both positive and negative test cases

### Soroban Testing Patterns
- **Environment setup**: Proper test environment initialization
- **Contract deployment**: Using `env.register_contract()`
- **Client usage**: Testing via `GameWorldContractClient`
- **State verification**: Asserting contract state changes
- **Error validation**: Testing failure scenarios

### Edge Case Coverage
- **Boundary values**: Testing min/max values
- **Invalid inputs**: Non-existent entity IDs
- **State transitions**: Entity death and removal
- **Data integrity**: Serialization roundtrip testing

## Debugging Tips

### Test Failures
If tests fail, use these debugging approaches:

1. **Run with output**: `cargo test -- --nocapture`
2. **Single test**: `cargo test test_name -- --exact`
3. **Verbose**: `RUST_LOG=debug cargo test`
4. **Specific module**: `cargo test module_name`

### Common Issues
- **Workspace conflicts**: Ensure you're in the correct directory
- **Dependencies**: Verify `soroban-sdk` testutils feature is enabled
- **Environment**: Check Soroban CLI installation

## Extending Tests

When adding new functionality to the game contract:

1. **Add corresponding tests** in the appropriate module
2. **Test both success and failure cases**
3. **Include edge cases and boundary conditions**
4. **Verify state consistency**
5. **Update this documentation**

### Test Template
```rust
#[test]
fn test_new_functionality() {
    let (env, client) = setup_initialized_contract();
    
    // Arrange: Set up test data
    
    // Act: Execute the functionality
    
    // Assert: Verify the results
    assert!(result, "Description of expected behavior");
}
```

## Integration with CI/CD

These tests are designed to run in automated environments:

- **GitHub Actions**: Configured in `.github/workflows/engine-ci.yml`
- **Local testing**: Via the automated test script `scripts/test.sh`
- **Soroban sandbox**: Tests run in isolated Soroban environment

For continuous integration, tests should pass consistently and provide clear failure messages when issues occur.