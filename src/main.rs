use rand::{Rng, SeedableRng};
use rand::distributions::{Distribution, Uniform};

const N_PARTICLES: usize = 20;
const N_ITERATIONS: usize = 1000;
const DIMENSIONS: usize = 2;

fn main() {
    let seed = [42; 16];
    let mut rng = rand::rngs::SmallRng::from_seed(seed);

    let c1 = 2.0;
    let c2 = 2.0;
    let w = 0.7;

    // Initialize particles with random positions and velocities
    let mut positions = [[0.0; DIMENSIONS]; N_PARTICLES];
    let mut velocities = [[0.0; DIMENSIONS]; N_PARTICLES];
    let mut personal_best_positions = [[0.0; DIMENSIONS]; N_PARTICLES];
    let mut personal_best_scores = [0.0; N_PARTICLES];
    let mut global_best_position = [0.0; DIMENSIONS];
    let mut global_best_score = f64::INFINITY;

    let uniform = Uniform::new(-5.0, 5.0);

    for i in 0..N_PARTICLES {
        for j in 0..DIMENSIONS {
            positions[i][j] = uniform.sample(&mut rng);
            velocities[i][j] = uniform.sample(&mut rng);
        }
        personal_best_positions[i] = positions[i];
        personal_best_scores[i] = objective_function(positions[i]);
        if personal_best_scores[i] < global_best_score {
            global_best_score = personal_best_scores[i];
            global_best_position = personal_best_positions[i];
        }
    }

    // Main loop
    for _ in 0..N_ITERATIONS {
        for i in 0..N_PARTICLES {
            for j in 0..DIMENSIONS {
                velocities[i][j] = w * velocities[i][j]
                    + c1 * rng.gen::<f64>() * (personal_best_positions[i][j] - positions[i][j])
                    + c2 * rng.gen::<f64>() * (global_best_position[j] - positions[i][j]);
                positions[i][j] += velocities[i][j];
            }

            let score = objective_function(positions[i]);
            if score < personal_best_scores[i] {
                personal_best_scores[i] = score;
                personal_best_positions[i] = positions[i];
                if score < global_best_score {
                    global_best_score = score;
                    global_best_position = personal_best_positions[i];
                }
            }
        }
    }

    println!("Best score: {}", global_best_score);
    println!("Best position: {:?}", global_best_position);
}

fn objective_function(x: [f64; DIMENSIONS]) -> f64 {
    // Function to be optimized
    -(x[0].powi(2) + x[1].powi(2)).exp() * (0.1 * (x[0].powi(2) + x[1].powi(2))).sin()
}
