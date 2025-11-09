use super::bullet::Bullet;
use super::ship::Ship;
use crate::renderer::Color;
use rand::Rng;

pub fn update_ship_ai(ship: &mut Ship, targets: &[(f32, f32)], delta_time: f32) {
    // Update ship physics
    ship.update(delta_time);

    if targets.is_empty() {
        return;
    }

    // Find nearest target and check if it's dangerously close
    let mut nearest_target = targets[0];
    let mut nearest_dist_sq = f32::MAX;

    for &(tx, ty) in targets {
        let dx = tx - ship.x;
        let dy = ty - ship.y;
        let dist_sq = dx * dx + dy * dy;

        if dist_sq < nearest_dist_sq {
            nearest_dist_sq = dist_sq;
            nearest_target = (tx, ty);
        }
    }

    let (tx, ty) = nearest_target;
    let nearest_dist = nearest_dist_sq.sqrt();

    // Calculate angle to target
    let dx = tx - ship.x;
    let dy = ty - ship.y;
    // atan2(dy, dx) gives angle where 0=right (+X), π/2=up (+Y)
    // Ship angle 0 means nose up (+Y in local), which is world angle π/2
    // So ship_angle = world_angle - π/2
    let target_angle = dy.atan2(dx) - std::f32::consts::FRAC_PI_2;

    // Normalize angle difference
    let mut angle_diff = target_angle - ship.angle;
    while angle_diff > std::f32::consts::PI {
        angle_diff -= std::f32::consts::PI * 2.0;
    }
    while angle_diff < -std::f32::consts::PI {
        angle_diff += std::f32::consts::PI * 2.0;
    }

    // DANGER ZONE: Aggressive asteroid avoidance
    let danger_distance = 0.25; // Increased from 0.15
    let warning_distance = 0.35; // Early warning zone
    let is_ahead = angle_diff.abs() < std::f32::consts::PI * 0.66; // Within 120° ahead (wider arc)

    if nearest_dist < danger_distance && is_ahead {
        // IMMEDIATE EVASIVE MANEUVER: Turn hard perpendicular to threat
        let evade_direction = if angle_diff > 0.0 { -1.0 } else { 1.0 }; // Turn AWAY
        ship.rotate(evade_direction, delta_time);
        ship.angular_velocity = 0.0;
        // Don't thrust toward danger
        return;
    }

    // WARNING ZONE: Slow down and prepare to evade
    if nearest_dist < warning_distance && is_ahead {
        // Don't thrust toward approaching threats
        ship.angular_velocity = 0.0;
        return;
    }

    // Normal behavior: aim and shoot
    // Rotate towards target
    if angle_diff.abs() > 0.05 {
        // Tighter tolerance for alignment
        let direction = if angle_diff > 0.0 { 1.0 } else { -1.0 };
        ship.rotate(direction, delta_time);
        // Stop angular velocity so ship stops rotating
        ship.angular_velocity = 0.0;
    } else {
        // Facing target - only thrust if not too close
        ship.angular_velocity = 0.0; // Ensure no residual rotation

        if nearest_dist > 0.1 {
            // Keep safe distance
            ship.thrust(delta_time);
        }

        // Random shooting to make it interesting
        let mut rng = rand::thread_rng();
        if ship.can_shoot() && rng.gen_bool(0.3) {
            ship.shoot();
        }
    }
}

pub fn create_bullet_from_ship(ship: &Ship, color: Color) -> Bullet {
    let (nose_x, nose_y) = ship.get_nose_position();
    Bullet::new(nose_x, nose_y, ship.angle, ship.id, color)
}
