# Product Details

## Complete Feature Specifications

### Game Objects

#### Player Ship
- **Design**: Classic triangular spaceship with "A" shape profile
- **Visual Elements**:
  - Nose tip with small diamond window detail
  - Straight sides with crossbar (forms capital "A" shape)
  - Double overhang lines at base (wing flaps)
  - Dynamic thrust flame (3 levels based on velocity)
- **Size**: 0.024 units (24% smaller than original design for better balance)
- **Color**: Uses game_color (default: arcade green #00FF33)
- **Physics**: Newtonian momentum with velocity limiting (max 1.0 units/second)
- **Friction**: 5% velocity decay per frame for natural deceleration

#### Flying Saucers (Enemies)
Two distinct enemy types that spawn periodically:

**Large Saucer**:
- Spawn probability: 70%
- Points awarded: 200
- Shooting behavior: Random direction (inaccurate)
- Size: 0.06 units
- Appearance: Classic dome with flat base and center line
- Movement: Horizontal traversal with small random vertical drift

**Small Saucer**:
- Spawn probability: 30%
- Points awarded: 1000
- Shooting behavior: 70% accuracy aimed at player
- Size: 0.04 units
- Appearance: Smaller dome design, faster movement
- Movement: Faster horizontal speed, more challenging target

**Spawn Rate**: Every 10 seconds (maximum 2 simultaneous saucers)
**Color**: Uses game_color

#### Asteroids
- **Three Size Levels**:
  - Large (level 3): 0.15 radius, 20 points
  - Medium (level 2): 0.10 radius, 50 points
  - Small (level 1): 0.05 radius, 100 points
- **Behavior**:
  - Random jagged polygon shapes (12-point generation)
  - Rotation animation (unique speed per asteroid)
  - Spawn from screen edges every 3 seconds
  - Split into 2 smaller pieces when destroyed
  - Maximum 12 simultaneous asteroids (prevents overwhelming density)
- **Color**: Uses game_color

#### Bullets
- **Design**: Small square projectiles (0.008 units)
- **Speed**: 2.0 units/second
- **Lifetime**: 2 seconds before auto-cleanup
- **Firing**: 3-shot burst mechanic with 80ms between shots
- **Cooldown**: 1 second between bursts
- **Color**: Uses game_color
- **Owner Tracking**: Prevents friendly fire

### Heads-Up Display (HUD)

All HUD elements use hud_color (default: grey #999999) for clear distinction from gameplay objects.

#### 1. Score Display (Top-Left)
- **Position**: (-0.95, 0.9) in normalized device coordinates
- **Font**: Vector-based 7-segment style digits
- **Size**: 0.06 units
- **Updates**: Real-time as points are earned
- **Color**: HUD color

#### 2. Direction Indicator (Top-Right, Primary)
- **Label**: "DIRECT" (left-aligned at x=0.55)
- **Visual Components**:
  - Rectangular box frame (matches screen aspect ratio)
  - Circular compass inside box (aspect-corrected)
  - Center point marked with small cross
  - Triangular rotating cursor pointing in heading direction
  - Numeric readout (0-359 degrees, right-aligned)
- **Position**: y=0.90, indicator width 0.12 units
- **Behavior**: Cursor rotates in real-time matching ship angle
- **Purpose**: Shows current heading for navigation awareness
- **Color**: HUD color

#### 3. Thrust Indicator (Top-Right, Middle)
- **Label**: "THRUST" (left-aligned at x=0.55)
- **Visual Components**:
  - Rectangular bar outline (0.12 wide x 0.025 high)
  - Filled bar proportional to current speed
  - Three horizontal fill lines for depth
  - Numeric readout (0-9 scale, right-aligned)
- **Position**: y=0.78 (0.12 below direction indicator)
- **Behavior**: Fills left-to-right based on velocity magnitude
- **Purpose**: Shows current movement speed
- **Color**: HUD color

#### 4. Power Indicator (Top-Right, Bottom)
- **Label**: "POWER" (left-aligned at x=0.55)
- **Visual Components**:
  - Rectangular battery outline (0.12 wide x 0.025 high)
  - Battery tip (arrow pointing right)
  - Filled bar proportional to energy level
  - Four horizontal fill lines for depth
  - Numeric readout (0-100 percentage, right-aligned)
- **Position**: y=0.66 (0.12 below thrust indicator)
- **Behavior**: Drains during thrusting/shooting, recharges during idle
- **Purpose**: Shows remaining energy for thrust and weapons
- **Color**: HUD color

### Game Mechanics

#### AI Behavior System

**Collision Avoidance**:
- **Danger Zone**: 0.25 units radius - immediate evasive maneuver (hard turn perpendicular)
- **Warning Zone**: 0.35 units radius - stop thrusting, prepare to evade
- **Detection Arc**: 120 degrees forward (66% of PI) - only threats ahead trigger avoidance
- **Evasion Strategy**: Turn away from threat, prevent head-on collisions

**Target Acquisition**:
- Scans all asteroids and saucers
- Selects nearest threat
- Calculates optimal intercept angle
- Rotates to face target (0.05 radian tolerance)

**Combat Behavior**:
- Shoots when aligned with target (30% probability per frame when ready)
- Maintains safe distance (>0.1 units)
- Thrusts toward target when safely aligned
- Uses burst fire (3 shots) for higher hit probability

**Energy Management**:
- Thrusting drains 0.3 energy per second
- Each shot costs 0.1 energy
- Fast recharge: 1.5 energy per second when idle (not thrusting or shooting)
- Full energy: 1.0 (100%)
- Minimum to shoot: 0.1 (10%)
- System prevents shooting/thrusting at zero energy

#### Death Loop Protection

Prevents frustrating respawn cycles:

**Detection**:
- Tracks time between player deaths
- If 3+ deaths occur within 3-second windows: death loop detected

**Response**:
- Clears all asteroids from field
- Removes all bullets
- Applies 5-second grace period before new asteroid spawns
- Resets death counter

**Purpose**: Ensures AI doesn't get stuck in unwinnable situations

#### Scoring System

**Point Values**:
- Large asteroid destroyed: 20 points
- Medium asteroid destroyed: 50 points
- Small asteroid destroyed: 100 points
- Large saucer destroyed: 200 points
- Small saucer destroyed: 1000 points

**Score Updates**: Instantaneous when bullet hits target or ship collides with saucer

### Visual Design

#### Color System

**Two Independent Color Channels**:

1. **Game Color** (game_color field):
   - Default: ARCADE_GREEN (RGB: 0.0, 1.0, 0.33)
   - Applied to: Ship, asteroids, bullets, saucers, thrust flame
   - Purpose: Unified retro aesthetic for game objects

2. **HUD Color** (hud_color field):
   - Default: GREY (RGB: 0.6, 0.6, 0.6)
   - Applied to: Score, direction indicator, thrust meter, power gauge, all labels
   - Purpose: Clear instrument readability separate from gameplay

**Available Color Constants** (in Color struct):
- BLACK (0, 0, 0)
- WHITE (1, 1, 1)
- RED (1, 0, 0)
- GREEN (0, 1, 0)
- BLUE (0, 0.5, 1)
- YELLOW (1, 1, 0)
- CYAN (0, 1, 1)
- MAGENTA (1, 0, 1)
- ARCADE_GREEN (0, 1, 0.33) - Matrix/arcade monitor green
- GREY (0.6, 0.6, 0.6) - Professional instrument grey

**Implementation**: All rendering functions accept Color parameters, enabling runtime customization without recompilation.

#### Vector Graphics Style

**Line Rendering**:
- All objects drawn as connected line segments
- No filled polygons or textures
- Authentic vector display aesthetic
- Black background (RGB: 0, 0, 0)

**Text Rendering**:
- Custom 7-segment style digits (0-9)
- Capital letter support for labels
- Vector line-based (no bitmaps)
- Consistent thickness across all text

#### Visual Feedback

**Dynamic Elements**:
- Thrust flame scales with velocity (3 levels)
- Ship rotates smoothly toward targets
- Asteroids rotate continuously
- Bullets leave visible trails
- Score updates immediately

**Screen Wrapping**:
- Objects exiting one edge reappear on opposite edge
- Seamless wraparound (no pop-in)
- Maintains original Asteroids behavior

### Physics System

#### Movement
- **Position Update**: position += velocity * delta_time
- **Velocity Limiting**: Maximum speed clamped to 1.0 units/second
- **Friction**: 5% per frame (0.95 multiplier)
- **Thrust Speed**: 0.6 units/second when active
- **Bullet Speed**: 2.0 units/second

#### Rotation
- **Angular Velocity**: 3.0 radians/second when turning
- **Angle Update**: angle += angular_velocity * delta_time
- **AI Rotation Control**: Sets angular velocity to zero after reaching target angle

#### Collision Detection
- **Circle-based detection**: Simplified physics using radius checks
- **Asteroid Collision Radius**: Uses asteroid.size field
- **Saucer Collision Radius**: Calculated from saucer size
- **Ship Collision**: 0.01 units effective radius
- **Bullet Collision**: 0.01 units effective radius

### Spawn Systems

#### Asteroid Spawning
- **Rate**: Every 3 seconds
- **Condition**: Only if current count < 12 (max_asteroids)
- **Position**: Random X (-1.0 to 1.0), fixed Y (±1.2, off-screen)
- **Initial Size**: Always large (level 3)
- **Grace Period**: 5 seconds after death loop clear

#### Saucer Spawning
- **Rate**: Every 10 seconds
- **Condition**: Only if current count < 2
- **Type Selection**: 70% large, 30% small
- **Position**: Left or right screen edge (random)
- **Movement**: Horizontal traversal across screen
- **Lifetime**: Self-destructs when leaving screen bounds

### User Controls

**Current Implementation**:
- **ESC Key**: Exit screensaver immediately
- **No Manual Controls**: Ship is fully autonomous (screensaver mode)

**Future Configuration Options**:
- Color scheme selection (game and HUD colors)
- Asteroid density adjustment
- Saucer spawn rate
- AI aggressiveness levels
- Sound effects toggle

### Performance Characteristics

**Frame Rate**:
- Target: 60 FPS
- Achieved: 60+ FPS consistently on Apple Silicon
- V-Sync: Enabled (PresentMode::Fifo)

**Memory Usage**:
- Dynamic vertex buffer allocation per frame
- Efficient vector-based data structures
- No memory leaks (Rust memory safety)
- Minimal heap allocations

**CPU Usage**:
- Low overhead (< 5% on M1/M2/M3)
- Delta-time based physics (frame-rate independent)
- Single-threaded rendering (sufficient for 2D)

### Platform Requirements

**Minimum**:
- macOS 12.0 (Monterey) or later
- Apple Silicon (M1 or newer)
- Metal-compatible GPU (all Apple Silicon Macs)

**Recommended**:
- macOS 13.0 (Ventura) or later
- M2/M3/M4 for best performance
- 1920x1080 or higher resolution display

**Multi-Monitor**:
- Automatically detects display size
- Adapts aspect ratio for HUD indicators
- Independent instances per monitor (when configured as screensaver)
