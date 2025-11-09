# AGENTS.md - AI Development Guidelines

This file provides guidance to AI coding assistants when working with this repository.

## Project Overview

Asteroids Retro macOS Screensaver is a high-performance, customizable screensaver written in Rust that recreates the classic arcade game Asteroids as a retro screensaver. The project emphasizes type safety, comprehensive testing, and clean architecture.

**Important**: This is the ONLY AI rules file for this repository. All CLAUDE.md references should point to this file. If a CLAUDE.md file is created, it must be renamed to AGENTS.md immediately.

## Core Requirements

### Visual Themes
- Default: Classic vector-based white-on-black arcade style
- Additional themes: Neon colors (green, cyan, magenta, amber)
- Retro CRT effects: scanlines, phosphor glow, screen curvature options
- Themes must be configurable at runtime

### Color Schemes
- Default: Classic white vector graphics (#FFFFFF on black)
- Additional colors: neon green, cyan, magenta, amber, and 7 other configurable options (total 11 color schemes)
- Colors should be user-selectable through screensaver settings

### Animation Speed
- 5 speed settings: 2 slower, 1 medium (default), 2 faster
- Speed affects asteroid movement, rotation, and spawning rate

### Technical Constraints
- **Rust only**: Use core Rust packages when possible
- **Package approval required**: Any additional dependencies must be approved before use
- **macOS screensaver framework**: Must integrate with macOS screensaver system

## Development Commands

### Building
```bash
cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Linting
```bash
cargo clippy -- -D warnings
```

### Formatting
```bash
cargo fmt
```

## Development Practices

### Test-Driven Development (TDD)

**TDD is mandatory for all changes.** Follow this workflow:

1. **Write the test first**: Define expected behavior in test form
2. **Run the test**: Verify it fails (red)
3. **Write minimal code**: Make the test pass (green)
4. **Refactor**: Improve code while keeping tests green
5. **Repeat**: Continue until feature is complete

**Test Requirements**:
- Unit tests for all public functions
- Integration tests for module interactions
- Property tests for invariants (where applicable)
- Mock implementations for external dependencies
- 100% test pass rate before committing

**Test Organization**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_behavior() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected_value);
    }
}
```

### Documentation Requirements

**MANDATORY**: Documentation files MUST be kept updated with every significant change. These files serve as context for AI agents and developers.

#### Required Documentation Files

1. **product-summary.md** (Root level)
   - High-level product vision and key features
   - Target audience and use cases
   - Current project status
   - Must be concise (1-2 pages max)
   - Update when: Adding major features, changing project direction

2. **product-details.md** (Root level)
   - Detailed feature specifications
   - User-facing behavior and configuration
   - Visual design and aesthetics
   - Gameplay mechanics and interactions
   - Update when: Modifying features, adding settings, changing UX

3. **technical-details.md** (Root level)
   - System architecture and design decisions
   - Implementation details and algorithms
   - Performance characteristics
   - Dependencies and their purposes
   - Update when: Changing architecture, optimizing code, adding dependencies

4. **README.md** (Root level)
   - Professional user-facing documentation
   - Installation and building instructions
   - Quick start guide
   - Project structure overview
   - Update when: Any user-visible change

5. **AGENTS.md** (This file, Root level)
   - Development practices and guidelines
   - AI agent instructions
   - Update when: Changing development processes

#### Documentation Update Rules

**When to Update Documentation**:
- ✅ **Always**: After completing any feature or significant code change
- ✅ **Before pushing**: Verify all relevant docs are updated
- ✅ **During PR review**: Check documentation completeness
- ✅ **After refactoring**: Update technical-details.md with new architecture

**What Must Be Updated**:
- If you change **game mechanics**: Update product-details.md
- If you change **architecture/design**: Update technical-details.md
- If you add **user-facing features**: Update README.md and product-details.md
- If you change **build process**: Update README.md
- If you add **new dependencies**: Update technical-details.md
- If you change **project goals**: Update product-summary.md

**How AI Agents Must Handle Documentation**:
1. **Before making changes**: Read product-summary.md, product-details.md, and technical-details.md for context
2. **After making changes**: Update ALL affected documentation files in the same session
3. **Never skip**: Documentation is not optional - it's required for every significant change
4. **Keep consistent**: Ensure documentation matches actual implementation

**Code Documentation**:
```rust
/// Brief description of function
///
/// Longer explanation of behavior, edge cases, etc.
///
/// # Arguments
/// * `param1` - Description of parameter
///
/// # Returns
/// Description of return value
///
/// # Examples
/// ```
/// let result = function(42);
/// assert_eq!(result, expected);
/// ```
pub fn function(param1: Type) -> ReturnType {
    // implementation
}
```

### Code Quality Standards

**Rust Best Practices**:
- Follow Rust API Guidelines: https://rust-lang.github.io/api-guidelines/
- Use `Result<T, E>` for recoverable errors
- Use `Option<T>` for optional values
- Avoid `.unwrap()` and `.expect()` in library code
- Prefer `if let` and `match` for pattern matching
- Use descriptive variable and function names

**Error Handling**:
```rust
// Good
pub fn parse_config(json: &str) -> Result<Config, ConfigError> {
    serde_json::from_str(json)
        .map_err(ConfigError::ParseError)
}

// Bad
pub fn parse_config(json: &str) -> Config {
    serde_json::from_str(json).unwrap()  // Never do this!
}
```

**Performance Considerations**:
- Pre-allocate vectors with `Vec::with_capacity()`
- Use references to avoid unnecessary clones
- Profile before optimizing
- Document performance characteristics in comments

**Code Organization**:
- Keep functions under 50 lines when possible
- One responsibility per function
- Use modules to group related functionality
- Export only what needs to be public

## Architecture Notes

### macOS Screensaver Integration
The screensaver must integrate with macOS's ScreenSaver framework. This typically requires:
- Creating a screensaver bundle (.saver)
- Implementing the ScreenSaver view interface
- Providing configuration UI for user settings

### Configuration System
User preferences (visual theme, color scheme, speed, CRT effects) must persist between screensaver sessions and be accessible through the macOS screensaver preferences pane.

### Game Rendering Pipeline
The asteroids effect requires:
- Vector-based rendering for classic arcade look
- Asteroid generation with varied shapes and sizes
- Rotation and movement physics for asteroids
- Collision detection (optional for visual variety)
- Efficient rendering to avoid performance issues on high-resolution displays
- Proper memory management for game state

## Repository Management

### Version Control
- Public repository on GitHub: `asteroids-retro-macos-screensaver`
- Use `.gitignore` to exclude binary files (standard Rust settings)
- Commit message format should be clear and descriptive

### Collaboration
- The README.md is written to invite new developers
- Keep documentation accessible to encourage contributions
- Maintain clear issue templates and contribution guidelines

## Debugging and Development

### Running Tests with Debug Output

```bash
# Run all tests with output
cargo test -- --nocapture

# Run specific test with output
cargo test test_name -- --nocapture

# Run tests with debug logging
RUST_LOG=debug cargo test

# Show all test output including passed tests
cargo test -- --nocapture --test-threads=1
```

### CLI Development and Debugging

Currently, the project is a library crate. To create a CLI tool for debugging:

**Option 1: Create a development binary**

1. Add a `src/bin` directory:
```bash
mkdir -p src/bin
```

2. Create `src/bin/demo.rs`:
```rust
use asteroids_retro_core::*;

fn main() {
    // Create configuration
    let config = ScreenSaverConfig::default();

    println!("Configuration: {:#?}", config);
    println!("Visual theme: {:?}", config.visual_theme);

    // Test engine creation
    let mut asteroids = AsteroidsGame::new(config);

    println!("Created game engine with {} asteroids",
             asteroids.total_asteroids());

    // Simulate updates
    for frame in 0..10 {
        asteroids.update();
        println!("Frame {}: {} active asteroids",
                 frame, asteroids.active_asteroids());
    }
}
```

3. Run with:
```bash
cargo run --bin demo
```

**Option 2: Use examples directory**

1. Create `examples/visualize.rs`:
```rust
use asteroids_retro_core::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let config = if args.len() > 1 {
        // Parse command-line arguments
        ScreenSaverConfig::from_json(&args[1]).unwrap()
    } else {
        ScreenSaverConfig::default()
    };

    println!("Asteroids Retro Visualization");
    println!("==============================");
    println!("Configuration: {:#?}", config);

    // Your debugging code here
}
```

2. Run with:
```bash
cargo run --example visualize

# With JSON config
cargo run --example visualize '{"visual_theme":"NeonGreen","color_scheme":"Cyan","speed":"Fast","screen_width":1920,"screen_height":1080}'
```

### Profiling and Performance

```bash
# Install flamegraph
cargo install flamegraph

# Profile the code
cargo flamegraph --bin demo

# Benchmark (requires criterion setup)
cargo bench

# Check binary size
cargo build --release
ls -lh target/release/libasteroids_retro_core.dylib
```

### Debug Logging

Add debug logging to your code:

```rust
#[cfg(debug_assertions)]
eprintln!("Debug: asteroid {} at position ({}, {})", asteroid.id, asteroid.x, asteroid.y);
```

### Memory Debugging

```bash
# Install cargo-valgrind (if available on macOS)
cargo install cargo-valgrind

# Run with valgrind
cargo valgrind run --bin demo

# Or use Instruments on macOS
cargo build --release
# Then profile with Instruments app
```

### Integration Testing

For testing rendering without a full macOS screensaver:

```rust
// In tests/integration_test.rs
use asteroids_retro_core::*;

struct TestRenderer {
    width: u32,
    height: u32,
}

impl Renderer for TestRenderer {
    // Implement trait methods for testing
}

#[test]
fn test_full_render_cycle() {
    let config = ScreenSaverConfig::default();
    let mut asteroids = AsteroidsGame::new(config);
    let mut renderer = TestRenderer::new(1920, 1080);

    for _ in 0..100 {
        asteroids.update();
        asteroids.render(&mut renderer);
    }

    // Assert expectations
}
```

## Future Development Considerations

When implementing new features:

1. **Dependency Management**:
   - Check if additional crates are needed
   - Get approval before adding dependencies
   - Prefer well-maintained, popular crates
   - Check license compatibility (MIT/Apache-2.0)

2. **TDD Workflow**:
   - Write tests first
   - Ensure tests fail initially
   - Implement minimal code to pass
   - Refactor while keeping tests green

3. **Documentation Updates**:
   - Update README.md for user-facing changes
   - Update technical-details.md for architecture changes
   - Update product-details.md for feature specifications
   - Update this file (AGENTS.md) for process changes

4. **Backwards Compatibility**:
   - Ensure saved user preferences still load
   - Provide migration path for breaking changes
   - Version configuration format if needed

5. **Platform Testing**:
   - Test on multiple macOS versions if possible
   - Consider different screen resolutions
   - Verify on both Intel and Apple Silicon

6. **Performance Validation**:
   - Benchmark before and after changes
   - Profile CPU usage on reference hardware
   - Check memory usage for leaks
   - Verify frame rate stability

## Commit Guidelines

**Commit Message Format**:
```
<type>: <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Maintenance tasks

**Examples**:
```
feat: Add neon cyan color scheme support

Implements vibrant cyan color option for retro arcade aesthetic.
Includes unit tests for color rendering and CRT glow effects.

Closes #42
```

```
fix: Correct asteroid rotation calculation

Rotation angles were incorrectly normalized, causing visual jitter.
Updated formula in update_rotation method.

Added regression test to prevent future issues.
```

## Pre-Commit Checklist

Before committing code, ensure:

- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Documentation is updated
- [ ] New code has tests
- [ ] Performance impact is acceptable
- [ ] No debug print statements left in code
- [ ] Commit message follows guidelines

## Pre-Push Checklist

**CRITICAL**: Before pushing changes to the repository, always run the full CI linting suite locally to prevent CI failures:

```bash
# Run both formatting check and clippy with all targets
cargo fmt --all -- --check && cargo clippy --all-targets --all-features -- -D warnings
```

If either command fails:
1. **Formatting issues**: Run `cargo fmt --all` to auto-fix
2. **Clippy warnings**: Fix each warning according to clippy's suggestions
3. **Re-run the checks** to ensure everything passes
4. Only then push your changes

**Why this matters**:
- CI failures block merges and waste time
- Local checks are faster than waiting for CI
- Prevents "fix linting" commits that clutter history
- Ensures code quality before it reaches the repository

**Tip**: You can create a git pre-push hook to automate this:
```bash
#!/bin/bash
# .git/hooks/pre-push
cargo fmt --all -- --check && cargo clippy --all-targets --all-features -- -D warnings
```
