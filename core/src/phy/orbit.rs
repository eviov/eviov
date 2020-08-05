use std::cmp::Ordering;

use crate::units::{self, LengthExt};

/// Represents a Keplerian orbit.
///
/// This struct represents the trajectory of a Keplerian orbit in a star system.
/// This is isomorphic to an `OrbitalState` given a specific mass and time frame.
///
/// Extra data are stored in this struct for efficient computation.
#[derive(Debug)]
pub struct Orbit {
    /// orbit eccentricity, a value between 0 and 1
    eccentricity: f64,
    /// Mean length of the apsides
    semimajor: units::Length,
    /// Argument of periapsis, an angle value
    periapsis: units::Bearing,
}

impl Orbit {
    /// Computes the `Orbit` parameters at a specific `OrbitalState`.
    pub fn from_states(state: OrbitalState, t: units::GameInstant, m: units::Mass) -> Self {
        // TODO optimize float precision
        // TODO optimize parameters

        let mu = m.0; // TODO tune the modifier for gravity coefficient

        // conversion from 4 d.f. to 4 d.f.
        // source: https://web.archive.org/web/20160418175843/https://ccar.colorado.edu/asen5070/handouts/cart2kep2002.pdf

        let r = (state.position - units::Position::origin()).0;
        let r_norm = r.norm();
        let v = state.velocity.unit.0;

        // 1. Angular momentum (h). Dimension: M L^2 T^-1
        let ang_momentum: f64 = r[0] * v[1] - r[1] * v[0];
        // 3. Specific energy (E_s). Dimension: L^2 T^-2
        let energy: f64 = v.norm_squared() / 2. - mu / r_norm;
        // 4. Semimajor axis (a). Dimension: L
        let semimajor: units::Length = -mu / 2. / energy;
        // 5. Eccentricity (e). Dimension: 1
        let eccentricity: f64 = (1. - ang_momentum.powi(2) / (semimajor * mu)).sqrt();
        // 8. Argument of latitude (u). Dimension: 1 (angle)
        let latitude: units::Bearing = units::Bearing(todo!());
        // 9a. Semi-latus rectum (p). Dimension: L
        let slr: units::Length = semimajor * (1. - eccentricity.powi(2));
        // 9b. True anomaly (nu). Dimension: 1 (angle)
        let true_anomaly: units::Theta = (slr - r_norm).arccos(r_norm * eccentricity);
        // 10. Argument of periapsis (omega). Dimension: 1 (angle)
        let periapsis: units::Bearing = latitude - true_anomaly;

        // Average sweep (n). Dimension: T^-1
        let average_sweep = units::Omega::of(units::Theta((mu / semimajor.powi(3)).sqrt()));

        // Eccentricity anomaly (E). Dimension: 1 (angle)
        let ecc_anomaly = r[0].arccos(semimajor);

        // Mean anomaly (M). Dimension: 1 (angle)
        let mean_anomaly = ecc_anomaly - units::Theta(eccentricity * ecc_anomaly.sin());

        let epoch_mean_anomaly = mean_anomaly - average_sweep.after(t - units::GameInstant::EPOCH);

        Self {
            eccentricity,
            semimajor,
            periapsis,
        }
    }

    /// Approximate the position of the orbit at time `t`.
    ///
    /// The tolerance is measured on each component,
    /// and the modulus would have sqrt(2) times of this tolerance.
    pub fn approx_position(
        &self,
        t: units::GameInstant,
        tolerance: units::Length,
    ) -> units::Position {
        todo!()
    }

    /// Approximate the velocity of the orbit at time `t`.
    ///
    /// The tolerance is measured on each component,
    /// and the modulus would have sqrt(2) times of this tolerance.
    pub fn approx_velocity(
        &self,
        t: units::GameInstant,
        m: units::Mass,
        tolerance: units::Length,
    ) -> units::Velocity {
        todo!()
    }

    /// Approximate the bearing of the orbit at time `t`.
    pub fn approx_bearing(
        &self,
        t: units::GameInstant,
        m: units::Mass,
        tolerance: units::Theta,
    ) -> units::Bearing {
        todo!()
    }

    /// Approximate the distance of the orbit from origin at time `t`.
    pub fn approx_radius(
        &self,
        t: units::GameInstant,
        m: units::Mass,
        tolerance: units::Length,
    ) -> units::Length {
        todo!()
    }

    /// Compare which orbit has radius.
    ///
    /// `Ordering::Greater` implies that `other` has a shorter arc to `self` along the clockwise
    /// direction than along the counterclockwise direction.
    ///
    /// If the difference between the two radii is less than floating point precision in internal
    /// operations, `Ordering::Equal` may be returned.
    pub fn compare_radius(
        &self,
        other: &Self,
        t: units::GameInstant,
        tolerance: units::Length,
    ) -> Ordering {
        todo!()
    }

    /// Compare which orbit has greater bearing along the minor arc.
    ///
    /// `Ordering::Greater` implies that `self` has a shorter arc to `other` along the clockwise
    /// direction than along the counterclockwise direction.
    ///
    /// If the difference between the two arcs (in the mod-2pi field) is less than floating point
    /// precision in internal operations, `Ordering::Equal` may be returned.
    pub fn compare_bearing(
        &self,
        other: &Self,
        t: units::GameInstant,
        tolerance: units::Length,
    ) -> Ordering {
        todo!()
    }

    /// Tests whether the radius is in the range (low, high) at time `t`.
    pub fn radius_in_range(
        &self,
        low: units::Length,
        high: units::Length,
        t: units::GameInstant,
    ) -> units::Length {
        todo!()
    }

    /// Tests whether the bearing is in the arc starting from `low`, extending counterclockwise until `high`.
    pub fn bearing_in_range(
        &self,
        low: units::Length,
        high: units::Length,
        t: units::GameInstant,
    ) -> units::Length {
        todo!()
    }

    /// Computes the time (starting from `after`) when this orbit intersects with the circle of
    /// radius `radius` (either moving into the circle or out of it).
    ///
    /// This method only computes a lower bound, i.e. from `after` up to the returned value, the
    /// orbit must not cross the `radius` circle. This method is not guaranteed to return the ideal
    /// lower bound, but is designed to converge towards the intersection efficiently.
    ///
    /// This method returns `None` if the orbit can never, or not in computationally relevant time,
    /// intersect with `radius`.
    pub fn when_intersect_radius(
        &self,
        radius: units::Length,
        after: units::GameInstant,
    ) -> Option<units::GameInstant> {
        todo!()
    }

    /// Computes the time (starting from `after`) when the bearings of the two orbits have less
    /// than `delta` apart.
    ///
    /// This method only computes a lower bound. It returns `Some(after)` if the two bearings are
    /// already less than `delta` apart. It returns `None` if the orbit can never, or not in
    /// computationally relevant time, reach less than `delta` bearing difference.
    pub fn when_intersect_bearing(
        &self,
        other: &Self,
        delta: units::Theta,
        after: units::GameInstant,
    ) -> Option<units::GameInstant> {
        todo!()
    }
}

/// Represents the ECI position and velocity of an orbit at time `t`.
#[derive(Debug, Clone, Copy)]
pub struct OrbitalState {
    position: units::Position,
    velocity: units::Velocity,
}
