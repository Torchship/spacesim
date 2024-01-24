use serde_derive::{Deserialize, Serialize};

use crate::provider::Provider;

const G: f64 = 6.67430e-11; // gravitational constant

#[derive(Deserialize, Serialize)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Deserialize, Serialize)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f64,
}

impl Body {
    fn new(position: Vec2, velocity: Vec2, mass: f64) -> Body {
        Body { position, velocity, mass }
    }

    fn gravitational_force(&self, other: &Body) -> Vec2 {
        let dx = other.position.x - self.position.x;
        let dy = other.position.y - self.position.y;
        let distance = (dx*dx + dy*dy).sqrt();
        let force_magnitude = G * self.mass * other.mass / (distance * distance);
        Vec2 {
            x: force_magnitude * dx / distance,
            y: force_magnitude * dy / distance,
        }
    }
}

fn leapfrog_step(body: &mut Body, force: Vec2, dt: f64) {
    // Update velocity
    body.velocity.x += force.x / body.mass * dt;
    body.velocity.y += force.y / body.mass * dt;

    // Update position
    body.position.x += body.velocity.x * dt;
    body.position.y += body.velocity.y * dt;
}

pub trait BodyProviderExt {
    fn add_body(&mut self, body: Body);
}

impl BodyProviderExt for Provider<Body> {
    fn add_body(&mut self, body: Body) {
        self.items.push(body);
    }
}