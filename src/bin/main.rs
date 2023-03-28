use fixed::traits::LossyInto;
use physics::{
    number, Circle, Number, PhysicsState, Vector2, TIME_STEP, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use raylib::prelude::*;

fn random_in_range(rng: &mut dyn rand::RngCore, range: std::ops::RangeInclusive<Number>) -> Number {
    use rand::Rng;

    let start = range.start().to_bits();
    let end = range.end().to_bits();
    let result = rng.gen_range(start..=end);
    Number::from_bits(result)
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Hello, World")
        .size(WINDOW_WIDTH as _, WINDOW_HEIGHT as _)
        .msaa_4x()
        .vsync()
        .build();

    let min_bounds = Vector2 {
        x: -Number::from_num(WINDOW_WIDTH) / number!(2),
        y: -Number::from_num(WINDOW_HEIGHT) / number!(2),
    };
    let max_bounds = Vector2 {
        x: Number::from_num(WINDOW_WIDTH) / number!(2),
        y: Number::from_num(WINDOW_HEIGHT) / number!(2),
    };

    let mut rng = <rand::rngs::StdRng as rand::SeedableRng>::from_seed(Default::default());
    let mut state = PhysicsState::new(
        Some(min_bounds),
        Some(max_bounds),
        std::iter::repeat_with(|| {
            let radius = random_in_range(&mut rng, number!(10.0)..=number!(20.0));
            Circle {
                position: Vector2 {
                    x: random_in_range(&mut rng, (min_bounds.x + radius)..=(max_bounds.x - radius)),
                    y: random_in_range(&mut rng, (min_bounds.y + radius)..=(max_bounds.x - radius)),
                },
                velocity: Vector2 {
                    x: random_in_range(&mut rng, number!(-50.0)..=number!(50.0)),
                    y: random_in_range(&mut rng, number!(-50.0)..=number!(50.0)),
                },
                mass: Number::PI * radius * radius,
                radius,
            }
        })
        .take(50),
    );
    let mut last_time = std::time::Instant::now();
    let mut fixed_time = number!(0);
    while !rl.window_should_close() {
        let time = std::time::Instant::now();
        let dt = time.duration_since(last_time);
        last_time = time;

        let dt = Number::from_num(dt.as_nanos()) * number!(0.000_000_001);
        fixed_time += dt;
        while fixed_time >= TIME_STEP {
            state.update(TIME_STEP);
            fixed_time -= TIME_STEP;
        }

        // Rendering
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color {
                r: 51,
                g: 51,
                b: 51,
                a: 255,
            });
            {
                let mut d = d.begin_mode2D(Camera2D {
                    offset: raylib::math::Vector2 {
                        x: WINDOW_WIDTH as f32 / 2.0,
                        y: WINDOW_HEIGHT as f32 / 2.0,
                    },
                    target: raylib::math::Vector2::zero(),
                    rotation: 0.0,
                    zoom: 1.0,
                });
                for circle in &state.circles {
                    d.draw_circle_v(
                        raylib::math::Vector2 {
                            x: circle.position.x.lossy_into(),
                            y: (-circle.position.y).lossy_into(),
                        },
                        circle.radius.lossy_into(),
                        Color::RED,
                    );
                }
            }
            d.draw_text(
                &format!("Frame Time: {:.3}ms", dt * number!(1000)),
                5,
                5,
                20,
                Color::WHITE,
            );
        }
    }
}
