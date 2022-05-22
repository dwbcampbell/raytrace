use raytracelib::{self, canvas::Canvas, color::Color, Tuple};
use std::io;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;

    Projectile {
        position: position,
        velocity: velocity,
    }
}

fn main() {
    let v = Tuple::normalize(&Tuple::vector(1.0, 1.8, 0.0)) * 11.25;

    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: v,
    };

    let e = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut canvas: Canvas = Canvas::new(900, 550);
    while p.position.y > 0.0 {
        p = tick(&e, &p);
        canvas.write_pixel(
            p.position.x.round() as usize,
            550 - (p.position.y.round() as usize),
            Color {
                red: 1.0,
                green: 0.0,
                blue: 0.0,
            },
        );
    }

    canvas.write_to_ppm(&mut io::stdout());
}
