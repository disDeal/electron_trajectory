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

#[derive(PartialEq)]
enum Model {
    Classic,
    Quantum,
}

const CUTOFF: f64 = 5.0e-4;
const TMAX: f64 = 2.;
const dt: f64 = 0.001;
const RMAX: f64 = 10.;
const dx: f64 = 0.001;

const N_TJR: usize = 30;
const MASS: f64 = 1.;
const hbar: f64 = 1.;
const Q_el: f64 = 1.6e-19;

fn u_c(r: Vec3d, r0: Vec3d) -> Vec3d {
    let dist = r.distance(r0);
    if { dist >= CUTOFF } {
        Vec3d::one() * -1. / dist
    } else {
        Vec3d::zero()
    }
}

fn u_q(r: Vec3d, r0: Vec3d) -> Vec3d {
    let dist = r.distance(r0);
    Vec3d::one() * (-0.5 * hbar / MASS) * -2.0 * (-dist).exp() / dist
}

fn calc_force(func: impl Fn(Vec3d, Vec3d) -> Vec3d, r0: Vec3d) -> impl Fn(Vec3d) -> Vec3d {
    move |r0| ROOT_PARTICLES.iter().map(|&r| func(r0, r)).sum()
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
    let center = Vec3d::new(0., 0., 0.);

    let mut rng = rand::thread_rng();
    let roots_size = ROOT_PARTICLES.len();

    for i in 0..N_TJR {
        let name = match model {
            Model::Quantum => format!("out2/bmd_{:05}.trj", i),
            Model::Classic => format!("out2/cmd_{:05}.trj", i),
        };
        let mut file = std::io::BufWriter::new(std::fs::File::create(name).unwrap());
        let mut r = random_spec_sphere(10.) + ROOT_PARTICLES[rng.gen_range(0, roots_size)];
        let mut rprev = r + random_spec_sphere(dx);
        let mut t = 0.;
        while t <= TMAX && r.distance(center) <= RMAX {
            let mut force = -gradient(calc_force(u_c, center), r);
            if model == Model::Quantum {
                force -= gradient(calc_force(u_q, r), r);
            }
            let rnew = r * 2. - rprev + (force / MASS) * dt * dt;
            rprev = r;
            r = rnew;
            file.write(&r.to_string().as_bytes()).unwrap();

            t += dt;
        }
    }
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
