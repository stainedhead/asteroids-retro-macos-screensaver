# Asteroids Retro macOS Screensaver

[![Latest Release](https://img.shields.io/github/v/release/stainedhead/asteroids-retro-macos-screensaver?label=Download&style=for-the-badge)](https://github.com/stainedhead/asteroids-retro-macos-screensaver/releases/latest)

A faithful recreation of the classic 1979 Asteroids arcade game, reimagined as an autonomous macOS screensaver. Built with Rust and wgpu, optimized for Apple Silicon Macs.

## Overview

Watch a self-piloting spaceship navigate through an asteroid field, autonomously avoiding threats and shooting asteroids and enemy saucers. This screensaver delivers the nostalgic arcade experience with contemporary performance and visual polish, running independently without requiring user interaction.

## Installation

### Download the DMG (Recommended)

1. **Download**: Visit the [latest release](https://github.com/stainedhead/asteroids-retro-macos-screensaver/releases/latest) and download the `Asteroids-Retro-Screensaver.dmg` file

2. **Open the DMG**: Double-click the downloaded DMG file to mount it

3. **Install**: Drag the application to your Applications folder

4. **Launch**: Navigate to your Applications folder and double-click to run

**System Requirements**:
- macOS 12.0 (Monterey) or later
- Apple Silicon Mac (M1, M2, M3, or M4)

**Note**: On first launch, you may need to right-click the app and select "Open" to bypass Gatekeeper security, as the app is not currently notarized.

## Features

### Autonomous Gameplay
- **AI-Controlled Ship**: Single intelligent ship that automatically navigates, avoids dangers, and engages threats
- **Collision Avoidance System**: Danger and warning zones prevent crashes into asteroids
- **Strategic Targeting**: AI prioritizes nearest threats (asteroids and saucers)
- **Energy Management**: Power system balances thrusting, shooting, and recharging
- **Death Loop Protection**: Clears asteroid field after 3 rapid deaths to prevent frustrating respawn cycles

### Authentic Arcade Experience
- **Classic Vector Graphics**: Clean line-based rendering true to the original 1979 arcade game
- **Arcade Green Aesthetic**: Default retro green color scheme (#00FF33) reminiscent of classic arcade monitors
- **Original Game Elements**: Triangular player ship, jagged asteroids, flying saucers
- **Newtonian Physics**: Authentic momentum-based movement with inertia and screen wrapping
- **Flying Saucers**: Two enemy types (large and small) with different behaviors and point values

### Professional HUD Display
- **Direction Indicator**: Circular compass with rotating triangular cursor showing ship heading (0-359 degrees)
  - Rectangular box frame with circular compass inside
  - Center crosshair with rotating cursor pointer
  - Real-time numeric heading readout
- **Thrust Meter**: Bar graph displaying current velocity (0-9 scale)
- **Power Gauge**: Battery-style indicator showing remaining energy percentage (0-100%)
- **Score Display**: Vector-based 7-segment numbers tracking points in top-left corner
- **Grey HUD Elements**: Distinct instrument cluster color (#999999) separate from game objects

### Visual Customization
- **Configurable Colors**: Independent color settings for game objects and HUD elements
  - `game_color`: Controls ship, asteroids, bullets, saucers, and thrust flame (default: arcade green)
  - `hud_color`: Controls all HUD indicators and labels (default: grey)
- **Retro Aesthetic**: Default colors evoke classic arcade monitor appearance

## Visual Design

### Color Scheme
- **Game Objects**: Arcade green (#00FF33) by default - ship, asteroids, bullets, saucers, thrust flame
- **HUD Elements**: Professional grey (#999999) by default - score, direction indicator, thrust meter, power gauge, all labels
- **Background**: Pure black for authentic vector display appearance
- **Configurable**: Both game and HUD colors can be customized via code

### Graphics Style
- **Vector-based rendering**: All objects drawn as connected line segments
- **No textures or filled polygons**: Authentic CRT vector monitor aesthetic
- **Clean lines**: Smooth, crisp rendering using modern GPU acceleration
- **Dynamic elements**: Thrust flame scales with velocity, asteroids rotate, indicators update in real-time

### HUD Layout
- **Top-Left**: Score display (7-segment style digits)
- **Top-Right**: Three-indicator cluster
  1. DIRECT: Direction indicator with compass and rotating cursor
  2. THRUST: Horizontal bar showing current velocity
  3. POWER: Battery indicator showing remaining energy

## Game Mechanics

### AI Ship Behavior
The autonomous ship demonstrates intelligent gameplay:
- **Threat Assessment**: Scans all asteroids and saucers, selects nearest threat
- **Collision Avoidance**:
  - Danger zone (0.25 units): Immediate evasive maneuver
  - Warning zone (0.35 units): Stop thrusting, prepare to evade
  - Forward-only detection (120-degree arc)
- **Combat Strategy**: Rotates to face targets, shoots when aligned
- **Resource Management**: Balances energy between movement and weapons

### Energy System
- **Capacity**: 100% (1.0) full energy
- **Drain Rates**:
  - Thrusting: 30% per second
  - Shooting: 10% per shot (3-shot burst = 30%)
- **Recharge Rate**: 150% per second when idle (not thrusting or shooting)
- **Full Recharge Time**: ~0.67 seconds from empty
- **Minimum to Shoot**: 10% energy required

### Burst Fire Weapon
- **3-Shot Bursts**: Each trigger fires three bullets
- **Timing**: 80ms between shots in burst (shot 1: 0ms, shot 2: +80ms, shot 3: +160ms)
- **Cooldown**: 1 second between bursts
- **Energy Cost**: 0.1 (10%) per shot, 0.3 (30%) per burst

### Death Loop Protection
Prevents AI from getting stuck in unwinnable situations:
- **Detection**: Tracks time between deaths (3 deaths within 3-second windows)
- **Response**: Clears all asteroids and bullets, applies 5-second grace period
- **Prevention**: 12 asteroid maximum prevents overwhelming density

### Flying Saucers
- **Large Saucer**
  - Spawn probability: 70%
  - Points: 200
  - Shoots randomly (inaccurate)
  - Easier to hit

- **Small Saucer**
  - Spawn probability: 30%
  - Points: 1000
  - Aims at player (70% accuracy)
  - Faster movement, harder to hit

- **Spawn Rate**: Every 10 seconds (maximum 2 simultaneous saucers)

### Asteroids
- **Three Sizes**:
  - Large: 20 points (0.15 radius)
  - Medium: 50 points (0.10 radius)
  - Small: 100 points (0.05 radius)
- **Behavior**:
  - Spawn from screen edges every 3 seconds
  - Break into 2 smaller pieces when destroyed
  - Rotate as they move
  - Random jagged shapes for variety
  - Maximum 12 simultaneous asteroids

### Scoring System
- Large asteroid: 20 points
- Medium asteroid: 50 points
- Small asteroid: 100 points
- Large saucer: 200 points
- Small saucer: 1000 points

## Controls

This is a screensaver, so it runs autonomously.

- **ESC**: Exit screensaver

## Technical

### Platform
- **Graphics API**: wgpu with Metal backend
- **Target Platform**: Apple Silicon only (ARM64)
- **Optimized Build**: Link-time optimization (LTO), single codegen unit, native CPU targeting
- **Rendering**: Smooth 60+ FPS, delta-time based physics for consistent motion
- **Multi-Monitor Support**: Automatically adapts to display resolution

### Performance
- **Frame Rate**: 60+ FPS consistently on Apple Silicon
- **CPU Usage**: Less than 5% on M1/M2/M3/M4
- **Memory**: ~50 MB resident (stable)
- **GPU Usage**: Minimal (efficient 2D vector graphics)

## Building from Source

### Requirements

- macOS 12.0 (Monterey) or later
- Apple Silicon (M1/M2/M3/M4)
- Metal-compatible GPU (all Apple Silicon Macs)
- Rust 1.70 or later

### Build Instructions

```bash
cargo build --release
```

The project is configured to build only for Apple Silicon (aarch64-apple-darwin).

### Running from Source

```bash
cargo run --release
```

Or run the binary directly:
```bash
./target/aarch64-apple-darwin/release/asteroids_screensaver
```

## Project Structure

```
src/
├── lib.rs              # Library entry point
├── main.rs             # Binary entry point (winit event loop)
├── game/
│   ├── mod.rs         # Game state management and collision detection
│   ├── ship.rs        # Player ship with AI control
│   ├── saucer.rs      # Flying saucer enemies (large/small)
│   ├── asteroid.rs    # Asteroid spawning and behavior
│   ├── bullet.rs      # Bullet physics and lifetime
│   ├── ai.rs          # AI collision avoidance and targeting
│   └── physics.rs     # Physics utilities (rotation, wrapping)
├── renderer/
│   ├── mod.rs         # Renderer setup and color definitions
│   ├── pipeline.rs    # wgpu rendering pipeline
│   ├── vertex.rs      # Vertex definitions
│   ├── text.rs        # Vector-based 7-segment text rendering
│   └── shader.wgsl    # WGSL vertex/fragment shaders
└── macos/
    └── mod.rs         # Future: macOS screensaver bundle bridge
```

## Documentation

For more detailed information, see:

- **[product-summary.md](/Users/iggybdda/Code/stainedhead/Rust/asteroids-retro-macos-screensaver/product-summary.md)** - Quick overview and project vision
- **[product-details.md](/Users/iggybdda/Code/stainedhead/Rust/asteroids-retro-macos-screensaver/product-details.md)** - Complete feature specifications
- **[technical-details.md](/Users/iggybdda/Code/stainedhead/Rust/asteroids-retro-macos-screensaver/technical-details.md)** - Architecture and implementation details

## Differences from Original Asteroids

This screensaver maintains visual authenticity while adding modern enhancements:

### Authentic Elements
- Classic triangular ship design with thrust flame
- Vector-based graphics (lines only, no fills)
- Jagged asteroid shapes that break apart
- Flying saucer enemies
- Screen wrapping physics
- Original scoring system

### Modern Additions
- **Autonomous Operation**: AI-controlled ship for screensaver mode (not player-controlled)
- **Professional HUD**: Direction indicator, thrust meter, power gauge with retro styling
- **Energy Management**: Power system adds tactical depth
- **Burst Fire**: Three-shot bursts for better hit probability
- **Death Loop Protection**: Prevents frustrating AI failure states
- **Configurable Colors**: Independent game and HUD color settings
- **Smooth GPU Rendering**: Higher resolution vector graphics using Metal
- **Multi-Monitor Support**: Automatic display detection and adaptation

## Current Status

**Version**: 0.1.0 (Beta)

**Fully Implemented**:
- Single AI-controlled spaceship with autonomous behavior
- Flying saucer enemies (large and small variants)
- Asteroid spawning, rotation, and breakup mechanics
- Complete scoring system
- Professional three-indicator HUD (direction, thrust, power)
- Collision detection for all object types
- Death loop protection system
- Energy management with fast recharge
- Burst fire weapon system
- Configurable game and HUD colors

**Platform Support**: Apple Silicon (ARM64) only - macOS 12.0+

## Future Enhancements

### macOS Integration
- Full macOS ScreenSaver bundle (.saver) packaging
- System Preferences integration
- Configuration panel for color schemes and difficulty settings
- Multi-monitor independent instances

### Gameplay Features
- Optional user controls (keyboard/gamepad) for interactive mode
- Multiple color scheme presets
- Adjustable difficulty levels (asteroid density, saucer spawn rate)
- High score persistence

### Audio and Effects
- Sound effects (optional, toggleable)
- Particle effects for explosions
- Enhanced visual feedback

### Additional Features
- Multiple lives system with lives display
- Hyperspace feature (emergency escape)
- Extra ship awards at score milestones

## Credits

Based on the original Asteroids arcade game by Atari (1979), designed by Lyle Rains and Ed Logg.

Built with Rust and wgpu, optimized for Apple Silicon.

## License

MIT
