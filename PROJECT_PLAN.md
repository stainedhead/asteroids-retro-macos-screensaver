# Asteroids Retro macOS Screensaver - Project Plan

## Project Overview

A high-performance macOS screensaver written in Rust that recreates the classic 1979 Atari Asteroids arcade game as an ambient screensaver. Built for Apple Silicon with native multi-monitor support.

## Feature Specification

### Core Features

#### 1. Classic Asteroids Gameplay (Ambient Mode)
- **Autonomous gameplay**: No player input, asteroids drift and occasionally get destroyed by AI spaceships
- **Vector-based rendering**: True to the original 1979 arcade aesthetic with clean line graphics
- **Wrap-around physics**: Objects exiting one screen edge reappear on the opposite side
- **Asteroid mechanics**:
  - Multiple sizes (large, medium, small)
  - Rotation animation for each asteroid
  - Random shapes generated from 12-point vector definitions
  - Asteroids split into smaller pieces when hit (visual variety)
  - Continuous spawning to maintain screen activity

#### 2. AI Spaceship Attackers
- **Flying saucers**: Periodically traverse the asteroid field
- **AI behavior**:
  - Random or tracking movement patterns
  - Shoot at asteroids (not at non-existent player)
  - Two types: large saucer (random shots) and small saucer (more accurate)
- **Configurable count**: 0-5 simultaneous spaceships
- **Spawn timing**: Random intervals to create visual interest

#### 3. Visual Customization

##### Color Schemes
- **Default**: Neon green (#00FF00) - retro arcade aesthetic
- **Classic**: White (#FFFFFF) - original Asteroids look
- **Additional colors**:
  - Cyan (#00FFFF)
  - Magenta (#FF00FF)
  - Amber (#FFBF00)
  - Red (#FF0000)
  - Blue (#0000FF)
  - Purple (#8000FF)
  - Orange (#FF8000)
  - Yellow (#FFFF00)
  - Teal (#00FF80)
- **Total**: 11 color schemes
- **Implementation**: Single color for all vector graphics per session

##### Optional CRT Effects
- **Scanlines**: Horizontal lines for authentic CRT look
- **Phosphor glow**: Soft bloom around vector lines
- **Screen curvature**: Subtle barrel distortion
- **Flicker**: Occasional brightness variation
- **All optional**: Can be toggled individually

#### 4. Speed Control
- **5 speed settings**:
  - Very Slow: 0.5x base speed
  - Slow: 0.75x base speed
  - Medium (default): 1.0x base speed
  - Fast: 1.5x base speed
  - Very Fast: 2.0x base speed
- **Affects**:
  - Asteroid drift velocity
  - Rotation speed
  - Spaceship movement
  - Bullet travel speed
  - Spawn rates

#### 5. Zoom/Scale Control
- **Purpose**: Adjust visual density for different screen sizes
- **Range**: 0.5x to 2.0x (50% to 200%)
- **Default**: 1.0x (auto-adjusted for screen resolution)
- **Behavior**:
  - Scales all game objects proportionally
  - Maintains aspect ratios
  - Adjusts spawn density to match zoom level

#### 6. Spaceship Count
- **Range**: 0-5 simultaneous AI spaceships
- **Default**: 2 spaceships
- **0 spaceships**: Pure asteroid field (meditative mode)
- **1-2 spaceships**: Occasional action
- **3-5 spaceships**: High activity mode

#### 7. Multi-Monitor Support
- **Native support**: Each monitor runs independent instance
- **Synchronized option**: Same game state across all monitors
- **Independent option**: Different random seeds per monitor
- **Configuration**: User can select which monitors to enable
- **Performance**: Efficient rendering for multiple displays

#### 8. Apple Silicon Optimization
- **Native ARM64 compilation**: aarch64-apple-darwin target
- **Metal API**: Hardware-accelerated vector rendering
- **Universal binary**: Support both Apple Silicon and Intel (optional)
- **Low power consumption**: Optimized for efficiency
- **60 FPS target**: Smooth animation on all supported hardware

### Technical Architecture

#### Rendering Stack
1. **Graphics Library**: **wgpu** (recommended)
   - Cross-platform GPU API (Metal on macOS)
   - Modern, safe Rust bindings
   - Excellent performance on Apple Silicon
   - Supports vector rendering via Lyon

2. **Vector Rendering**: **Lyon**
   - GPU-based 2D vector graphics
   - Built on wgpu
   - Perfect for Asteroids-style line graphics
   - Hardware-accelerated tessellation

3. **Alternative**: **flo_draw** (backup option)
   - Simpler API
   - OpenGL-based
   - Good for rapid prototyping

#### macOS Integration
- **Framework**: Objective-C wrapper for NSScreenSaverView
- **FFI**: Rust core with minimal Objective-C bridge
- **Bundle**: .saver bundle for macOS System Preferences
- **Configuration**: Native macOS preference pane UI

#### Project Structure
```
asteroids-retro-macos-screensaver/
├── src/
│   ├── lib.rs              # Core library entry point
│   ├── game/
│   │   ├── mod.rs          # Game engine module
│   │   ├── asteroid.rs     # Asteroid entity and physics
│   │   ├── spaceship.rs    # AI spaceship logic
│   │   ├── bullet.rs       # Projectile physics
│   │   ├── physics.rs      # Movement, collision, wrap-around
│   │   └── spawner.rs      # Entity spawning logic
│   ├── rendering/
│   │   ├── mod.rs          # Rendering module
│   │   ├── vector.rs       # Vector shape definitions
│   │   ├── colors.rs       # Color scheme management
│   │   ├── effects.rs      # CRT effects (scanlines, glow)
│   │   └── renderer.rs     # Main rendering pipeline
│   ├── config/
│   │   ├── mod.rs          # Configuration module
│   │   ├── settings.rs     # User settings structure
│   │   └── persistence.rs  # macOS UserDefaults integration
│   └── macos/
│       ├── mod.rs          # macOS-specific module
│       ├── screensaver.m   # Objective-C NSScreenSaverView
│       ├── bridge.rs       # FFI bridge
│       └── preferences.m   # Preference pane UI
├── tests/
│   ├── game_tests.rs       # Game logic tests
│   ├── physics_tests.rs    # Physics simulation tests
│   └── rendering_tests.rs  # Rendering tests (headless)
├── examples/
│   ├── demo.rs             # Standalone demo app
│   └── benchmark.rs        # Performance benchmarking
├── documents/
│   ├── product-summary.md
│   ├── product-details.md
│   └── technical-details.md
├── Cargo.toml
├── AGENTS.md
└── README.md
```

## Implementation Phases

### Phase 1: Foundation (Weeks 1-2)
**Goal**: Establish project structure and basic rendering

#### Deliverables:
1. **Project Setup**
   - [x] Copy .claude directory and AGENTS.md
   - [ ] Initialize Cargo project with dependencies
   - [ ] Set up test framework
   - [ ] Configure CI/CD for Apple Silicon

2. **Basic Rendering**
   - [ ] Integrate wgpu + Lyon
   - [ ] Create window/canvas rendering
   - [ ] Implement vector line drawing
   - [ ] Test on Apple Silicon hardware

3. **Core Data Structures**
   - [ ] Define `Asteroid` struct with position, velocity, rotation
   - [ ] Define `Spaceship` struct with AI state
   - [ ] Define `Bullet` struct
   - [ ] Implement `Vector2D` math utilities

4. **Configuration System**
   - [ ] Create `ScreenSaverConfig` struct
   - [ ] Implement default settings
   - [ ] Add JSON serialization/deserialization
   - [ ] Write configuration tests

**Success Criteria**:
- Can draw vector lines on screen
- Basic shapes render smoothly at 60 FPS
- Configuration loads and saves correctly
- All tests pass

---

### Phase 2: Game Physics (Weeks 3-4)
**Goal**: Implement core Asteroids physics and mechanics

#### Deliverables:
1. **Movement System**
   - [ ] Implement inertia-based movement
   - [ ] Add rotation physics
   - [ ] Create wrap-around screen logic
   - [ ] Write movement tests (TDD)

2. **Asteroid Logic**
   - [ ] Generate random asteroid shapes (12-point vectors)
   - [ ] Implement asteroid spawning
   - [ ] Add rotation animation
   - [ ] Create size variations (large/medium/small)
   - [ ] Test asteroid generation

3. **Collision Detection**
   - [ ] Implement circle-based collision (simple)
   - [ ] Add bullet-asteroid collision
   - [ ] Implement asteroid splitting
   - [ ] Test collision edge cases

4. **Demo Application**
   - [ ] Create `examples/demo.rs`
   - [ ] Interactive window with asteroids
   - [ ] Keyboard controls for testing
   - [ ] Performance monitoring

**Success Criteria**:
- Asteroids drift and rotate realistically
- Wrap-around works on all edges
- Collisions detected accurately
- Demo runs at stable 60 FPS

---

### Phase 3: AI Spaceships (Weeks 5-6)
**Goal**: Add autonomous spaceship attackers

#### Deliverables:
1. **Spaceship Entity**
   - [ ] Design spaceship vector shape
   - [ ] Implement movement patterns
   - [ ] Add spawn/despawn logic
   - [ ] Test spaceship behavior

2. **AI Behavior**
   - [ ] Random traversal pattern
   - [ ] Shooting logic (aim at asteroids)
   - [ ] Large saucer (random shots)
   - [ ] Small saucer (accurate shots)
   - [ ] Test AI decision-making

3. **Bullet System**
   - [ ] Bullet spawning from spaceships
   - [ ] Bullet-asteroid collision
   - [ ] Bullet lifetime management
   - [ ] Visual effects for hits

4. **Game Balance**
   - [ ] Tune spaceship spawn rates
   - [ ] Adjust shooting frequency
   - [ ] Balance asteroid destruction rate
   - [ ] Ensure continuous visual activity

**Success Criteria**:
- Spaceships spawn and move naturally
- AI shoots at asteroids convincingly
- Asteroids break apart when hit
- Game remains visually interesting

---

### Phase 4: Visual Customization (Weeks 7-8)
**Goal**: Implement color schemes and visual effects

#### Deliverables:
1. **Color System**
   - [ ] Implement 11 color schemes
   - [ ] Create color configuration
   - [ ] Add color blending for fading effects
   - [ ] Test all color options

2. **CRT Effects (Optional)**
   - [ ] Scanline overlay shader
   - [ ] Phosphor glow post-processing
   - [ ] Screen curvature distortion
   - [ ] Flicker animation
   - [ ] Toggle options for each effect

3. **Speed Control**
   - [ ] Implement 5 speed settings
   - [ ] Scale all game timings
   - [ ] Test at all speed levels
   - [ ] Ensure stability at extreme speeds

4. **Zoom/Scale**
   - [ ] Implement zoom functionality
   - [ ] Adjust spawn density
   - [ ] Scale all game objects
   - [ ] Test on different resolutions

**Success Criteria**:
- All 11 colors render correctly
- CRT effects look authentic
- Speed settings work smoothly
- Zoom maintains visual quality

---

### Phase 5: macOS Integration (Weeks 9-10)
**Goal**: Create native macOS screensaver bundle

#### Deliverables:
1. **Objective-C Bridge**
   - [ ] Create NSScreenSaverView subclass
   - [ ] Implement FFI to Rust core
   - [ ] Handle lifecycle (start/stop/configure)
   - [ ] Test memory management

2. **Preference Pane**
   - [ ] Design macOS preference UI
   - [ ] Add controls for all settings:
     - Color scheme dropdown
     - Speed slider (5 levels)
     - Zoom slider (0.5x - 2.0x)
     - Spaceship count slider (0-5)
     - CRT effects checkboxes
   - [ ] Implement UserDefaults persistence
   - [ ] Test preference changes

3. **Screensaver Bundle**
   - [ ] Create .saver bundle structure
   - [ ] Configure Info.plist
   - [ ] Package Rust library
   - [ ] Test installation in ~/Library/Screen Savers

4. **Multi-Monitor Support**
   - [ ] Detect multiple displays
   - [ ] Create renderer instance per monitor
   - [ ] Add synchronized vs. independent mode
   - [ ] Test on dual/triple monitor setups

**Success Criteria**:
- Screensaver installs in System Preferences
- All settings save and load correctly
- Works on multiple monitors
- Runs stably for extended periods

---

### Phase 6: Optimization & Polish (Weeks 11-12)
**Goal**: Performance optimization and final refinements

#### Deliverables:
1. **Performance Optimization**
   - [ ] Profile CPU usage
   - [ ] Optimize rendering pipeline
   - [ ] Reduce memory allocations
   - [ ] Test on older Apple Silicon (M1)
   - [ ] Ensure low power consumption

2. **Testing & QA**
   - [ ] Comprehensive unit test coverage
   - [ ] Integration tests
   - [ ] Manual testing on multiple Macs
   - [ ] Extended stability testing (24+ hours)
   - [ ] Multi-monitor stress testing

3. **Documentation**
   - [ ] Update README.md
   - [ ] Write installation guide
   - [ ] Create user manual
   - [ ] Document all settings
   - [ ] Add troubleshooting section

4. **Final Polish**
   - [ ] Fine-tune animations
   - [ ] Adjust default settings
   - [ ] Add Easter eggs (optional)
   - [ ] Create demo video
   - [ ] Prepare for release

**Success Criteria**:
- CPU usage < 5% on M1/M2/M3
- Memory stable (no leaks)
- 60 FPS on all supported hardware
- All tests pass
- Documentation complete

---

## Technology Stack

### Core Dependencies (Cargo.toml)

```toml
[package]
name = "asteroids-retro-macos-screensaver"
version = "0.1.0"
edition = "2021"

[dependencies]
# Graphics
wgpu = "0.19"           # Modern GPU API (Metal on macOS)
lyon = "1.0"            # Vector graphics tessellation
winit = "0.29"          # Window management (for examples)

# Math
glam = "0.25"           # Fast vector math

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Random
rand = "0.8"

# macOS FFI
objc = "0.2"
cocoa = "0.25"
core-graphics = "0.23"

[dev-dependencies]
criterion = "0.5"       # Benchmarking

[build-dependencies]
cc = "1.0"              # For compiling Objective-C

[lib]
crate-type = ["cdylib", "rlib"]

[profile.release]
opt-level = 3
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization
```

### Platform Requirements
- **macOS**: 12.0 (Monterey) or later
- **Architecture**: Apple Silicon (ARM64) primary, Intel (x86_64) optional
- **Graphics**: Metal-capable GPU (all modern Macs)

---

## Development Roadmap

### Timeline Overview
| Phase | Duration | Key Milestone |
|-------|----------|---------------|
| Phase 1 | 2 weeks | Vector rendering works |
| Phase 2 | 2 weeks | Asteroids drift and collide |
| Phase 3 | 2 weeks | AI spaceships active |
| Phase 4 | 2 weeks | Full visual customization |
| Phase 5 | 2 weeks | macOS screensaver bundle |
| Phase 6 | 2 weeks | Production ready |
| **Total** | **12 weeks** | **v1.0 Release** |

### Milestones

#### M1: Basic Rendering (End of Phase 1)
- Vector graphics render on screen
- Configuration system works
- Project structure established

#### M2: Interactive Demo (End of Phase 2)
- Demo app shows drifting asteroids
- Physics feels authentic
- Collisions work

#### M3: Full Gameplay (End of Phase 3)
- AI spaceships shoot asteroids
- Continuous visual activity
- Game balance achieved

#### M4: Customizable (End of Phase 4)
- All 11 colors available
- Speed and zoom controls work
- CRT effects implemented

#### M5: macOS Native (End of Phase 5)
- Screensaver installs correctly
- Preferences pane functional
- Multi-monitor support

#### M6: Production Release (End of Phase 6)
- Optimized and tested
- Documentation complete
- Ready for public release

---

## Risk Assessment

### Technical Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| wgpu/Lyon integration complexity | Medium | High | Start with simple shapes, prototype early |
| macOS screensaver API changes | Low | High | Test on multiple macOS versions |
| Multi-monitor rendering issues | Medium | Medium | Implement single-monitor first, add multi later |
| Performance on older M1 | Low | Medium | Profile early, optimize incrementally |
| Objective-C FFI bugs | Medium | Medium | Minimize FFI surface, thorough testing |

### Schedule Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| wgpu learning curve | Medium | Low | Allocate buffer time in Phase 1 |
| CRT effects complexity | Medium | Low | Mark as optional, can defer to v1.1 |
| macOS integration issues | Medium | High | Start Phase 5 early if possible |

---

## Success Metrics

### Performance Targets
- **Frame Rate**: Consistent 60 FPS
- **CPU Usage**: < 5% on M2/M3, < 10% on M1
- **Memory**: < 100 MB stable (no leaks)
- **Power**: Minimal GPU power state activation
- **Startup**: < 1 second to first frame

### Quality Targets
- **Test Coverage**: > 80% line coverage
- **Clippy**: Zero warnings
- **Documentation**: All public APIs documented
- **User Testing**: Positive feedback from 5+ beta testers

---

## Future Enhancements (Post-v1.0)

### v1.1 Features
- [ ] Sound effects (optional, off by default)
- [ ] Particle effects for explosions
- [ ] More spaceship types
- [ ] Achievement system (time-based milestones)

### v1.2 Features
- [ ] Custom color schemes (user-defined RGB)
- [ ] Advanced physics options
- [ ] Alternative asteroid shape sets
- [ ] Performance statistics overlay

### v2.0 Features
- [ ] 3D mode (perspective view)
- [ ] Multiple game modes (original Asteroids, Deluxe, etc.)
- [ ] Cross-platform (Windows/Linux screensavers)
- [ ] Integration with music visualization

---

## Questions for Review

Before beginning development, please review and provide feedback on:

1. **Graphics Library Choice**: Is wgpu + Lyon the right choice, or would you prefer a simpler approach (e.g., flo_draw or OpenGL)?

2. **Feature Priority**: Are all Phase 4 features (CRT effects, zoom, etc.) necessary for v1.0, or should some be deferred?

3. **Multi-Monitor Strategy**: Should we target "synchronized" mode first, or "independent" instances per monitor?

4. **Color Palette**: Are 11 color schemes sufficient, or would you like more/fewer options?

5. **Timeline**: Does the 12-week timeline seem reasonable, or should we adjust scope?

6. **AI Behavior**: Should spaceships only shoot asteroids, or should they also shoot each other occasionally?

7. **Spawn Density**: Should asteroid/spaceship density adapt automatically to screen size, or remain user-controlled?

8. **Testing Priority**: Should we focus on Apple Silicon exclusively, or also test on Intel Macs?

---

## Next Steps

Once you approve this plan:

1. I'll create the initial Cargo project structure
2. Set up the test framework following TDD principles
3. Create the basic wgpu + Lyon rendering prototype
4. Begin Phase 1 development with daily test-driven increments

Please review and let me know if you'd like any adjustments to the scope, timeline, or technical approach!
