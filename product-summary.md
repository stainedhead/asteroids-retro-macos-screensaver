# Product Summary

## Vision

Asteroids Retro macOS Screensaver is a faithful recreation of the classic 1979 Atari Asteroids arcade game, reimagined as a modern macOS screensaver. Built with Rust and optimized for Apple Silicon, it delivers the nostalgic arcade experience with contemporary performance and visual polish.

## Core Concept

A self-piloting spaceship navigates through an asteroid field, autonomously avoiding threats and shooting asteroids and enemy saucers. The screensaver runs independently, providing ambient entertainment that captures the essence of retro arcade gaming without requiring user interaction.

## Key Features

### Authentic Arcade Experience
- **Classic Vector Graphics**: Clean line-based rendering true to the original 1979 arcade game
- **Arcade Green Aesthetic**: Default retro green color scheme reminiscent of classic arcade monitors
- **Original Game Elements**: Triangular player ship, jagged asteroids, flying saucers, and vector-based HUD
- **Newtonian Physics**: Authentic momentum-based movement with inertia and screen wrapping

### Intelligent Gameplay
- **Autonomous AI Pilot**: Ship automatically navigates, avoids dangers, and engages threats
- **Collision Avoidance System**: Danger and warning zones prevent the ship from crashing into asteroids
- **Strategic Targeting**: AI prioritizes nearest threats (asteroids and saucers)
- **Burst Fire Weapon**: Three-shot burst mechanic for tactical shooting
- **Energy Management**: Power system balances thrusting, shooting, and recharging

### Professional HUD Display
- **Direction Indicator**: Circular compass with rotating cursor showing ship heading in degrees
- **Thrust Meter**: Visual bar graph displaying current velocity (0-9 scale)
- **Power Gauge**: Battery-style indicator showing remaining energy percentage
- **Score Display**: Vector-based 7-segment numbers tracking points in top-left corner
- **Grey HUD Elements**: Distinct instrument cluster color separate from game objects

### Visual Customization
- **Configurable Colors**: Independent color settings for game objects (ship, asteroids, bullets, saucers) and HUD elements
- **Arcade Green Default**: Retro #00FF33 green for game elements
- **Grey Instruments**: Professional #999999 grey for HUD readability
- **Direction Indicator Design**: Rectangular frame with circular compass and triangular rotating cursor

### Technical Excellence
- **Apple Silicon Optimized**: Native ARM64 compilation for M1/M2/M3/M4 Macs
- **Metal Backend**: Hardware-accelerated rendering using wgpu with Metal API
- **High Performance**: Link-time optimization (LTO) and single codegen unit for maximum efficiency
- **Multi-Monitor Support**: Automatic display detection and adaptation

## Current Status

### Fully Implemented
- Single AI-controlled spaceship with autonomous behavior
- Flying saucer enemies (large and small variants)
- Asteroid spawning, rotation, and breakup mechanics
- Complete scoring system (20/50/100 points for asteroids, 200/1000 for saucers)
- Professional three-indicator HUD (direction, thrust, power)
- Collision detection (ship-asteroid, bullet-asteroid, ship-saucer, bullet-saucer)
- Death loop protection system
- Energy management with fast recharge
- Burst fire weapon system
- Configurable game and HUD colors

### Working Features
- Smooth 60+ FPS rendering
- Screen wrapping for all objects
- Dynamic thrust visualization
- Respawn mechanics
- Automatic difficulty balancing (12 asteroid maximum)

### Controls
- **ESC**: Exit screensaver

## Distribution

### Installation

The screensaver is distributed as a DMG package for easy installation on Apple Silicon Macs:

- **Pre-built Release**: Available on GitHub Releases as `Asteroids-Retro-Screensaver.dmg`
- **Platform**: Apple Silicon only (M1, M2, M3, M4 processors)
- **macOS Version**: Requires macOS 12.0 (Monterey) or later
- **Installation**: Simple drag-and-drop to Applications folder
- **Source Code**: Available for users who prefer to build from source

### Getting Started

Users can download the latest DMG from the [GitHub Releases page](https://github.com/stainedhead/asteroids-retro-macos-screensaver/releases), mount the disk image, and drag the application to their Applications folder. No additional setup or configuration is required.

## Target Audience

### Primary Users
- **macOS Users**: Owners of Apple Silicon Macs (M1/M2/M3/M4) seeking quality screensavers
- **Retro Gaming Enthusiasts**: Fans of classic arcade games who appreciate authentic recreations
- **Minimalists**: Users who prefer clean, non-distracting ambient displays

### Use Cases
- **Screensaver**: Primary use as an automated screensaver during computer idle time
- **Ambient Display**: Background entertainment for desk setups or shared screens
- **Nostalgia**: Evoking memories of classic arcade gaming
- **Demonstration**: Showcasing Rust performance and Metal rendering on Apple Silicon

## Differentiators

### Authentic Yet Modern
Unlike generic screensavers or crude arcade clones, this project balances historical accuracy with modern technology:
- Faithful to 1979 Asteroids visual design
- Smooth GPU-accelerated rendering
- Apple Silicon native performance
- Professional HUD with real-time telemetry

### Intelligent Automation
The AI pilot isn't just random movement - it demonstrates strategic behavior:
- Threat assessment and prioritization
- Collision avoidance with danger zones
- Resource management (energy/shooting)
- Natural-looking autonomous gameplay

### Technical Quality
Built with professional engineering practices:
- Memory-safe Rust implementation
- Modern graphics API (wgpu + Metal)
- Optimized release builds
- Clean architecture with separated concerns

## Project Status

**Current Version**: 0.1.0 (Beta)

**Development Stage**: Functional standalone application, ready for macOS screensaver bundle integration

**Platform Support**: Apple Silicon (ARM64) only - macOS 12.0+

**Next Steps**:
1. macOS ScreenSaver bundle (.saver) creation
2. System preferences integration
3. User configuration panel
4. Optional sound effects
5. Additional color scheme presets
