use fixed::traits::LossyInto;
use physics::{number, Circle, Number, PhysicsState, Vector2, TIME_STEP};
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .msaa_4x()
        .title("Hello, World")
        .build();

    let mut state = PhysicsState::from_iter([
        Circle {
            position: Vector2 {
                x: number!(200),
                y: number!(300),
            },
            velocity: Vector2 {
                x: number!(50),
                y: number!(0),
            },
            mass: number!(1),
            radius: number!(50),
        },
        Circle {
            position: Vector2 {
                x: number!(400),
                y: number!(270),
            },
            velocity: Vector2 {
                x: number!(-50),
                y: number!(0),
            },
            mass: number!(1),
            radius: number!(50),
        },
    ]);
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

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            for circle in &state {
                d.draw_circle_v(
                    raylib::math::Vector2 {
                        x: circle.position.x.lossy_into(),
                        y: circle.position.y.lossy_into(),
                    },
                    circle.radius.lossy_into(),
                    Color::RED,
                );
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
