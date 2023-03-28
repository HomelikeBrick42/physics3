use crate::{number, Circle, Number, Vector2};
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
        self.circles
            .par_iter_mut()
            .for_each(|circle| circle.velocity.y -= number!(200) * ts);

        let solved = AtomicBool::new(false);
        for _ in 0..1000 {
            if solved.load(Relaxed) {
                break;
            }
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
                                    circle.velocity.y = circle.velocity.y.abs();
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
                                    circle.velocity.x = circle.velocity.x.abs();
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
                                    circle.velocity.y = -circle.velocity.y.abs();
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
                                    circle.velocity.x = -circle.velocity.x.abs();
                                    // step back foward through time
                                    circle.position += circle.velocity * time;

                                    break 'collision_checks;
                                }
                            }

                            for (other_index, other_circle) in self.old_circles.iter().enumerate() {
                                if index != other_index {
                                    let relative_velocity = circle.velocity - other_circle.velocity;
                                    let relative_position = circle.position - other_circle.position;
                                    let sqr_distance = relative_position.sqr_length();
                                    let combined_radius = circle.radius + other_circle.radius;
                                    if (sqr_distance < combined_radius * combined_radius)
                                        && (relative_position.dot(-relative_velocity)
                                            > Number::ZERO)
                                    {
                                        // TODO: find collision time so position can be adjusted

                                        circle.velocity -= ((number!(2) * other_circle.mass)
                                            / (circle.mass + other_circle.mass))
                                            * (relative_velocity.dot(relative_position)
                                                / sqr_distance)
                                            * relative_position;

                                        break 'collision_checks;
                                    }
                                }
                            }

                            return circle;
                        }

                        solved.store(false, Relaxed);
                        circle
                    },
                ));
        }
        self.circles
            .par_iter_mut()
            .for_each(|circle| circle.position += circle.velocity * ts);
    }
}
