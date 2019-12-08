#[derive(Debug, Clone, Copy)]
pub struct Body {
    x: f64,
    y: f64,
    z: f64,
    mass: f64,
}

fn average(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

fn average_with_mass(a: f64, b: f64, a_mass: f64, b_mass:f64) -> f64 {
    average(a * a_mass, b * b_mass) / (a_mass + b_mass)
}

fn merge_two_bodies(a: Body, b: Body) -> Body {
    Body {
        x: average_with_mass(a.x, b.x, a.mass, b.mass),
        y: average_with_mass(a.y, b.y, a.mass, b.mass),
        z: average_with_mass(a.z, b.z, a.mass, b.mass),
        mass: a.mass + b.mass,
    }
}

fn merge_all_bodies_iter(bodies: &[Body]) -> Body {
    let berycenter = bodies[0];
    bodies.iter().skip(1).fold(berycenter, |berycenter, body| {
        merge_two_bodies(berycenter, *body)
    })
}

fn main() {
}
