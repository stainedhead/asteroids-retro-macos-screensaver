# Death Loop Protection System

## Problem

In classic Asteroids, it's possible to get stuck in a "death loop" where:
1. The player dies and respawns at the center
2. Too many asteroids are nearby
3. The player immediately dies again upon respawning
4. This cycle repeats, making the game unplayable

This is especially problematic in screensaver mode where the AI can't strategically wait for a safe moment.

## Solution

We've implemented a multi-layered protection system:

### 1. Death Tracking

The game tracks recent player deaths:

```rust
pub struct GameState {
    deaths_in_short_time: u32,      // Counter for deaths
    time_since_last_death: f32,     // Time tracker
    // ...
}
```

**Logic:**
- If player dies within 3 seconds of previous death → increment counter
- If more than 3 seconds have passed → reset counter to 1
- This distinguishes between "bad luck" and "death loop"

### 2. Automatic Field Clearing

When 3+ deaths occur in quick succession (within 3-second windows):
- **All asteroids are cleared** from the field
- **All bullets are cleared** (reduces chaos)
- **5-second grace period** before new asteroids spawn
- Death counter resets to prevent repeated clearing

```rust
fn handle_player_death(&mut self) {
    if self.time_since_last_death < 3.0 {
        self.deaths_in_short_time += 1;
    } else {
        self.deaths_in_short_time = 1;
    }

    if self.deaths_in_short_time >= 3 {
        self.clear_asteroid_field();  // Clear everything!
        self.deaths_in_short_time = 0;
    }

    self.player_ship.respawn();
}
```

### 3. Maximum Asteroid Limit

To prevent overwhelming asteroid density:

```rust
max_asteroids: 12  // Hard limit on total asteroids
```

**Asteroid Spawning:**
- Only spawns if `asteroids.len() < max_asteroids`
- Prevents exponential growth from asteroid splitting
- Maintains playable difficulty

```rust
if self.time_since_asteroid_spawn > 3.0
   && self.asteroids.len() < self.max_asteroids {
    self.spawn_asteroid();
}
```

## How It Works

### Normal Gameplay
```
Time: 0s  - Player dies (counter = 1)
Time: 5s  - Playing normally
Time: 12s - Player dies (counter = 1, reset because >3s)
Time: 15s - Playing normally
```
No field clearing, normal difficulty progression.

### Death Loop Detected
```
Time: 0s  - Player dies (counter = 1)
Time: 1s  - Player dies again! (counter = 2)
Time: 2s  - Player dies again! (counter = 3)
          → FIELD CLEARED!
          → 5-second grace period starts
          → Counter resets to 0
Time: 7s  - First new asteroid can spawn
```

The player gets breathing room to recover.

## Benefits

### For Screensaver Mode
- AI doesn't get stuck in impossible situations
- Keeps the screensaver running smoothly
- Prevents frustrating repetitive failures

### For Gameplay
- Fair respawn mechanic
- Prevents unfair deaths from overwhelming asteroid fields
- Maintains challenge without being punishing

### For Performance
- Asteroid limit prevents infinite growth
- Field clearing prevents bullet/asteroid buildup
- Keeps frame rate stable

## Parameters (Tunable)

```rust
const DEATH_WINDOW: f32 = 3.0;        // Seconds between deaths to count
const DEATH_THRESHOLD: u32 = 3;       // Deaths needed to trigger clearing
const GRACE_PERIOD: f32 = 5.0;        // Seconds before respawning asteroids
const MAX_ASTEROIDS: usize = 12;      // Maximum total asteroids
```

These can be adjusted for different difficulty levels:
- **Easier**: Increase thresholds, longer grace period
- **Harder**: Decrease thresholds, shorter grace period, more asteroids

## Implementation Details

### Death Detection Points

Death is tracked at all collision types:
1. **Player ↔ Asteroid**: Most common death
2. **Player ↔ Saucer**: Physical collision
3. **Player ↔ Bullet**: Shot by saucer

All call `handle_player_death()` instead of direct `respawn()`.

### Early Return Pattern

When a death is detected, we immediately return from collision checking:
```rust
if player_died {
    self.handle_player_death();
    return;  // Don't check more collisions this frame
}
```

This prevents:
- Multiple death triggers in one frame
- Score counting after death
- Unnecessary processing

### Field Clearing Details

The `clear_asteroid_field()` method:
```rust
fn clear_asteroid_field(&mut self) {
    self.asteroids.clear();           // Remove all asteroids
    self.bullets.clear();             // Remove all bullets
    self.time_since_asteroid_spawn = -5.0;  // Negative = grace period
}
```

The negative spawn timer creates a countdown:
- -5.0 → -4.0 → ... → 0.0 → spawn enabled
- Prevents immediate respawning
- Gives player time to position

## Testing

To test the death loop protection:
1. Wait for many asteroids to spawn
2. Let the ship die 3 times quickly
3. Observe the field clearing
4. Note the 5-second grace period before new asteroids

The system should prevent infinite death loops while maintaining gameplay challenge.

## Visual Feedback

Currently, field clearing is instant. Future improvements could add:
- Flash effect when field clears
- Warning indicator when death counter is high
- Score penalty for field clearing
- Visual countdown during grace period

## Statistics

With default settings:
- **12 asteroids max**: 12 large + potential splits = manageable
- **3-second window**: Enough to detect loops, not too sensitive
- **5-second grace**: Time to orient and prepare
- **3 death threshold**: Balance between protection and challenge

The system ensures smooth screensaver operation while maintaining the classic Asteroids challenge!
