use crate::{number, Circle, Number, Vector2, MAX_PHYSICS_ITERATIONS};
use fixed_sqrt::FixedSqrt;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

#[derive(Default)]
pub struct PhysicsState {
    pub min_bounds: Option<Vector2>,
    pub max_bounds: Option<Vector2>,
    pub circles: Vec<Circle>,
    old_circles: Vec<Circle>,
}

impl PhysicsState {
    pub fn new(
        min_bounds: Option<Vector2>,
        max_bounds: Option<Vector2>,
        circles: impl IntoIterator<Item = Circle>,
    ) -> Self {
        Self {
            min_bounds,
            max_bounds,
            circles: circles.into_iter().collect(),
            old_circles: vec![],
        }
    }

    pub fn add_circle(&mut self, circle: Circle) {
        self.circles.push(circle);
    }

    pub fn update(&mut self, ts: Number) {
        std::mem::swap(&mut self.circles, &mut self.old_circles);
        self.circles.clear();
        self.circles
            .par_extend(self.old_circles.par_iter().copied().enumerate().map(
                |(index, mut circle)| {
                    let force: Vector2 = self
                        .old_circles
                        .par_iter()
                        .copied()
                        .enumerate()
                        .map(|(other_index, other_circle)| {
                            if index != other_index {
                                const G: Number = number!(5);
                                let relative_pos = other_circle.position - circle.position;
                                let sqr_distance =
                                    (circle.position - other_circle.position).sqr_length();
                                G * (circle.mass * other_circle.mass) / sqr_distance * relative_pos
                            } else {
                                Vector2::ZERO
                            }
                        })
                        .sum();
                    circle.velocity += force / circle.mass * ts;
                    circle
                },
            ));

        let solved = AtomicBool::new(false);
        let mut iterations = 0;
        while !solved.load(Relaxed) && iterations < MAX_PHYSICS_ITERATIONS {
            solved.store(true, Relaxed);

            std::mem::swap(&mut self.circles, &mut self.old_circles);
            self.circles.clear();
            self.circles
                .par_extend(self.old_circles.par_iter().copied().enumerate().map(
                    |(index, mut circle)| {
                        'collision_checks: {
                            if let Some(min_bounds) = self.min_bounds {
                                if (circle.velocity.y < Number::ZERO)
                                    && (circle.position.y - circle.radius < min_bounds.y)
                                {
                                    let depth = (circle.position.y - circle.radius) - min_bounds.y;
                                    let time = depth / circle.velocity.y;

                                    // rewind time
                                    circle.position -= circle.velocity * time;
                                    // adjust velocity
                                    circle.velocity.y = -circle.velocity.y;
                                    // step back foward through time
                                    circle.position += circle.velocity * time;

                                    break 'collision_checks;
                                }

                                if (circle.velocity.x < Number::ZERO)
                                    && (circle.position.x - circle.radius < min_bounds.x)
                                {
                                    let depth = (circle.position.x - circle.radius) - min_bounds.x;
                                    let time = depth / circle.velocity.x;

                                    // rewind time
                                    circle.position -= circle.velocity * time;
                                    // adjust velocity
                                    circle.velocity.x = -circle.velocity.x;
                                    // step back foward through time
                                    circle.position += circle.velocity * time;

                                    break 'collision_checks;
                                }
                            }

                            if let Some(max_bounds) = self.max_bounds {
                                if (circle.velocity.y > Number::ZERO)
                                    && (circle.position.y + circle.radius > max_bounds.y)
                                {
                                    let depth = (circle.position.y + circle.radius) - max_bounds.y;
                                    let time = depth / circle.velocity.y;

                                    // rewind time
                                    circle.position -= circle.velocity * time;
                                    // adjust velocity
                                    circle.velocity.y = -circle.velocity.y;
                                    // step back foward through time
                                    circle.position += circle.velocity * time;

                                    break 'collision_checks;
                                }

                                if (circle.velocity.x > Number::ZERO)
                                    && (circle.position.x + circle.radius > max_bounds.x)
                                {
                                    let depth = (circle.position.x + circle.radius) - max_bounds.x;
                                    let time = depth / circle.velocity.x;

                                    // rewind time
                                    circle.position -= circle.velocity * time;
                                    // adjust velocity
                                    circle.velocity.x = -circle.velocity.x;
                                    // step back foward through time
                                    circle.position += circle.velocity * time;

                                    break 'collision_checks;
                                }
                            }

                            if let Some(other_circle) = self
                                .old_circles
                                .par_iter()
                                .enumerate()
                                .find_map_first(|(other_index, other_circle)| {
                                    if index != other_index {
                                        let relative_velocity =
                                            circle.velocity - other_circle.velocity;
                                        let relative_position =
                                            circle.position - other_circle.position;
                                        let sqr_distance = relative_position.sqr_length();
                                        let combined_radius = circle.radius + other_circle.radius;
                                        if (sqr_distance < combined_radius * combined_radius)
                                            && (relative_position.dot(-relative_velocity)
                                                > Number::ZERO)
                                        {
                                            Some(other_circle)
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                            {
                                let relative_velocity = circle.velocity - other_circle.velocity;
                                let relative_position = circle.position - other_circle.position;
                                let sqr_distance = relative_position.sqr_length();
                                let combined_radius = circle.radius + other_circle.radius;

                                // TODO: find collision time so position can be adjusted accurately
                                // this is a temporary solution to make it "sorta correct"
                                circle.position += relative_position.normalized()
                                    * (combined_radius - sqr_distance.sqrt());

                                circle.velocity -= ((number!(2) * other_circle.mass)
                                    / (circle.mass + other_circle.mass))
                                    * (relative_velocity.dot(relative_position) / sqr_distance)
                                    * relative_position;

                                break 'collision_checks;
                            }

                            return circle;
                        }

                        solved.store(false, Relaxed);
                        circle
                    },
                ));

            iterations += 1;
        }

        if iterations == MAX_PHYSICS_ITERATIONS {
            println!("Max iterations reached, the simulation may become unstable");
        }

        self.circles
            .par_iter_mut()
            .for_each(|circle| circle.position += circle.velocity * ts);
    }
}
