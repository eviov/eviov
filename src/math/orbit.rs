use super::*;

#[derive(Debug)]
pub struct Orbit {
    /// Gravitational constant times mass
    mu: f64,
    /// Semi-major axis
    a: f64,
    /// Eccentricity
    e: f64,
    /// True anomaly at epoch
    omega: f64,
    /// mean anomaly at epoch
    m0: f64,
}

impl Orbit {
    pub fn from_mass_eci(mass: Mass, eci: Eci) -> Self {
        // Reference: https://space.stackexchange.com/a/2590/29538
        // TODO: perform analysis on numerical issues

        let mu = f64::from(f32::from(mass));

        // Increase precision first
        let Vector(rx, ry) = eci.position();
        let rx = f64::from(rx.0);
        let ry = f64::from(ry.0);
        let rm = (rx * rx + ry * ry).sqrt();

        let Vector(vx, vy) = eci.velocity();
        let vx = f64::from(vx.0);
        let vy = f64::from(vy.0);

        let h = rx * vy - ry * vx;

        let ev = [vy * h / mu - rx / rm, -vx * h / mu - ry / rm];
        let e2 = ev.iter().map(|c| c * c).sum::<f64>();
        let e = e2.sqrt();

        let a = h * h / mu / (1.0 - e2);

        let omega = ev[1].atan2(ev[0]);
        let m0 = unimplemented!();

        Self {
            mu,
            a,
            e,
            omega,
            m0,
        }
    }

    pub fn instant(&self, t: Time) -> OrbitInstant<'_> {
        let Time(t) = t;

        OrbitInstant {
            orbit: self,
            m: f64::from(t) * (self.mu / self.a / self.a / self.a).sqrt() + self.m0,
        }
    }
}

pub struct OrbitInstant<'t> {
    orbit: &'t Orbit,
    m: f64,
}
