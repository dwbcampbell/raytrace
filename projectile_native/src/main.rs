use nalgebra_glm as glm;
struct Projectile {
    position: glm::Vec4,
    velocity: glm::Vec4,
}

struct Environment {
    gravity: glm::Vec4,
    wind: glm::Vec4,
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
    let v = glm::Vec4::new(1.0, 1.8, 0.0, 0.0).normalize() * 11.25;

    let mut p = Projectile {
        position: glm::Vec4::new(0.0, 1.0, 0.0, 1.0),
        velocity: v,
    };

    let e = Environment {
        gravity: glm::Vec4::new(0.0, -0.1, 0.0, 0.0),
        wind: glm::Vec4::new(-0.01, 0.0, 0.0, 0.0),
    };

    while p.position.y > 0.0 {
        p = tick(&e, &p);
        println!("{}f32 {}f32", p.position.x, p.position.y);
    }
}
