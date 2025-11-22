use raylib::prelude::*;
use std::f32::consts::PI;

/// Solve Kepler's equation: M = E - e*sin(E)
/// Uses Newton-Raphson iteration to find eccentric anomaly E from mean anomaly M
///
/// M: Mean anomaly (radians)
/// e: Eccentricity (0 <= e < 1)
///
/// Returns: Eccentric anomaly E (radians)
fn solve_kepler_equation(mean_anomaly: f32, eccentricity: f32) -> f32 {
    // Normalize mean anomaly to [0, 2π]
    let m = mean_anomaly % (2.0 * PI);
    let m = if m < 0.0 { m + 2.0 * PI } else { m };

    // Initial guess for eccentric anomaly
    // For e < 0.8, M is a good starting point
    let mut eccentric_anomaly = if eccentricity < 0.8 {
        m
    } else {
        // For higher eccentricity, use π as starting guess
        PI
    };

    // Newton-Raphson iteration
    // Typically converges in 3-5 iterations
    const MAX_ITERATIONS: i32 = 10;
    const TOLERANCE: f32 = 1e-6;

    for _ in 0..MAX_ITERATIONS {
        // f(E) = E - e*sin(E) - M
        let f = eccentric_anomaly - eccentricity * eccentric_anomaly.sin() - m;

        // f'(E) = 1 - e*cos(E)
        let f_prime = 1.0 - eccentricity * eccentric_anomaly.cos();

        // Newton-Raphson step: E_new = E_old - f/f'
        let delta = f / f_prime;
        eccentric_anomaly -= delta;

        // Check convergence
        if delta.abs() < TOLERANCE {
            break;
        }
    }

    eccentric_anomaly
}

/// Convert eccentric anomaly to true anomaly
///
/// E: Eccentric anomaly (radians)
/// e: Eccentricity
///
/// Returns: True anomaly (radians)
fn eccentric_to_true_anomaly(eccentric_anomaly: f32, eccentricity: f32) -> f32 {
    // Formula: tan(ν/2) = sqrt((1+e)/(1-e)) * tan(E/2)
    let sqrt_factor = ((1.0 + eccentricity) / (1.0 - eccentricity)).sqrt();
    let true_anomaly = 2.0 * (sqrt_factor * (eccentric_anomaly / 2.0).tan()).atan();

    true_anomaly
}

/// Convert mean anomaly (what increases linearly with time) to true anomaly (actual position angle)
/// This is the KEY function that solves Kepler's equation!
///
/// mean_anomaly: The angle if the orbit were circular (increases at constant rate)
/// eccentricity: Orbital eccentricity (0 = circle, approaching 1 = very elliptical)
///
/// Returns: True anomaly - the actual angle from periapsis
pub fn mean_anomaly_to_true_anomaly(mean_anomaly: f32, eccentricity: f32) -> f32 {
    // Special case: circular orbit (no need to solve Kepler's equation)
    if eccentricity < 1e-6 {
        return mean_anomaly;
    }

    // Step 1: Solve Kepler's equation to get eccentric anomaly
    let eccentric_anomaly = solve_kepler_equation(mean_anomaly, eccentricity);

    // Step 2: Convert eccentric anomaly to true anomaly
    let true_anomaly = eccentric_to_true_anomaly(eccentric_anomaly, eccentricity);

    true_anomaly
}

/// Calculate position on an elliptical orbit using TRUE ANOMALY
/// This is the SINGLE source of truth for orbital mechanics
///
/// Now properly handles Keplerian motion!
pub fn calculate_orbital_position(
    center: Vector3,
    semi_major_axis: f32,
    eccentricity: f32,
    inclination: f32,
    mean_anomaly: f32, // Changed from 'angle' - this is what increases linearly!
) -> Vector3 {
    // Convert mean anomaly to true anomaly (the actual position angle)
    let true_anomaly = mean_anomaly_to_true_anomaly(mean_anomaly, eccentricity);

    let a = semi_major_axis;
    let e = eccentricity;

    // Ellipse equation: r = a(1-e²) / (1 + e*cos(ν))
    // where ν is the TRUE anomaly
    let r = a * (1.0 - e * e) / (1.0 + e * true_anomaly.cos());

    // Position in orbital plane (XZ, Y=0)
    let x_orbit = r * true_anomaly.cos();
    let z_orbit = -r * true_anomaly.sin(); // Negative for correct direction

    // Apply inclination (rotate around X-axis)
    let x = x_orbit;
    let y = z_orbit * inclination.sin();
    let z = z_orbit * inclination.cos();

    Vector3::new(center.x + x, center.y + y, center.z + z)
}

/// Generate a complete orbital path as a series of points
/// Uses true anomaly sampling for accurate ellipse visualization
pub fn generate_orbital_path(
    center: Vector3,
    semi_major_axis: f32,
    eccentricity: f32,
    inclination: f32,
    num_points: usize,
) -> Vec<Vector3> {
    let mut points = Vec::with_capacity(num_points);

    for i in 0..num_points {
        // Sample MEAN anomaly uniformly (this gives uniform time sampling)
        let mean_anomaly = (i as f32 / num_points as f32) * 2.0 * PI;

        // Calculate position using the same Keplerian mechanics
        let pos = calculate_orbital_position(
            center,
            semi_major_axis,
            eccentricity,
            inclination,
            mean_anomaly,
        );
        points.push(pos);
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_orbit() {
        // For circular orbits, mean anomaly = true anomaly
        let mean = PI / 4.0;
        let e = 0.0;
        let true_anom = mean_anomaly_to_true_anomaly(mean, e);
        assert!((true_anom - mean).abs() < 1e-5);
    }

    #[test]
    fn test_kepler_equation_convergence() {
        // Test that Kepler's equation converges for various eccentricities
        let mean = PI / 3.0;

        for e in [0.1, 0.3, 0.5, 0.7, 0.9] {
            let eccentric = solve_kepler_equation(mean, e);
            // Verify: M = E - e*sin(E)
            let calculated_mean = eccentric - e * eccentric.sin();
            assert!((calculated_mean - mean).abs() < 1e-5);
        }
    }
}
