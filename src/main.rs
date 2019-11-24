#![allow(non_upper_case_globals)]
#[macro_use]
extern crate lazy_static;

use rand::Rng;
use std::io::Write;

mod vec3d;
use vec3d::Vec3d;

lazy_static! {
    static ref ROOT_PARTICLES: Vec<Vec3d> = {
        let mut acc = Vec::new();
        acc.push(Vec3d::new(-1., 0., 0.));
        acc.push(Vec3d::new(1., 0., 0.));
        acc
    };
}

enum Model {
    Classic,
    Quantum,
}

const CUTOFF: f64 = 5.0e-4;
const dt: f64 = 0.001;
const dx: f64 = 0.001;

const N_TJR: usize = 30;
const MASS: f64 = 1.;
const hbar: f64 = 1.;
const Q_el: f64 = 1.6e-19;

fn u_c(r: Vec3d, r0: Vec3d) -> f64 {
    let dist = r.distance(r0);
    if { dist >= CUTOFF } {
        -1. / dist
    } else {
        0.
    }
}

fn u_q(r: Vec3d, r0: Vec3d) -> f64 {
    let dist = r.distance(r0);
    (-0.5 * hbar / MASS) * -2.0 * (-dist).exp() / dist
}

fn calc_force(func: impl Fn(Vec3d, Vec3d) -> Vec3d, r0: Vec3d) -> Vec3d {
    ROOT_PARTICLES.iter().map(|&r| func(r0, r)).sum()
}

fn gradient(func: impl Fn(Vec3d) -> Vec3d, r: Vec3d) -> Vec3d {
    let mut grad = [0f64; 3];
    for i in 0..grad.len() {
        let mut dr = [0f64; 3];
        dr[i] = dx;
        let dr = Vec3d::from(dr);

        grad += (func(r + dr) - func(r - dr)) / (2. * dx);
    }

    grad.into()
}

fn main() {
    let model = Model::Quantum;

    let name = match model {
        Model::Quantum => format!("out2/bmd_{}.trj", 1),
        Model::Classic => format!("out2/cmd_{}.trj", 1),
    };

    let mut file = std::io::BufWriter::new(std::fs::File::create(name).unwrap());

    let point = random_spec_sphere(10.);
    file.write(&point.to_string().as_bytes()).unwrap();
}

impl std::fmt::Display for Vec3d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}\n", self.x, self.y, self.z)
    }
}

fn random_spec_sphere(r: f64) -> Vec3d {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(CUTOFF, r);
    let phi = rng.gen_range(0., 2. * std::f64::consts::PI);
    let theta = rng.gen_range(0., std::f64::consts::PI);
    let x = r * (theta).sin() * (phi).cos();
    let y = r * (theta).sin() * (phi).sin();
    let z = r * (theta).cos();
    Vec3d::new(x, y, z)
}
