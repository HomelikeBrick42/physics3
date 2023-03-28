use fixed::traits::LossyInto;
use physics::{
    number, Circle, Number, PhysicsState, Vector2, TIME_STEP, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Hello, World")
        .size(WINDOW_WIDTH as _, WINDOW_HEIGHT as _)
        .msaa_4x()
        .build();

    let mut state = PhysicsState::new(
        Some(Vector2 {
            x: -Number::from_num(WINDOW_WIDTH) / number!(2),
            y: -Number::from_num(WINDOW_HEIGHT) / number!(2),
        }),
        Some(Vector2 {
            x: Number::from_num(WINDOW_WIDTH) / number!(2),
            y: Number::from_num(WINDOW_HEIGHT) / number!(2),
        }),
        [
            Circle {
                position: Vector2 {
                    x: number!(-100),
                    y: number!(0),
                },
                velocity: Vector2 {
                    x: number!(50),
                    y: number!(0),
                },
                mass: number!(2),
                radius: number!(50),
            },
            Circle {
                position: Vector2 {
                    x: number!(200),
                    y: number!(-30),
                },
                velocity: Vector2 {
                    x: number!(-50),
                    y: number!(0),
                },
                mass: number!(1),
                radius: number!(50),
            },
        ],
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
