use crate::{FillRule, LineCap, LineJoin};
use leaflet::LatLng;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Position {
    pub lat: f64,
    pub lng: f64,
}

impl Position {
    pub fn new(lat: f64, lng: f64) -> Self {
        Self { lat, lng }
    }

    /// Determines the distance between two positions using the Haversine formula.
    ///
    /// The result is in meters
    pub fn distance_haversine(&self, other: &Self) -> f64 {
        const R: f64 = 6371e3; // Earth's radius in meters
        let phi1 = self.lat.to_radians();
        let phi2 = other.lat.to_radians();
        let delta_phi = (other.lat - self.lat).to_radians();
        let delta_lambda = (other.lng - self.lng).to_radians();

        let a = (delta_phi / 2.0).sin().powi(2)
            + phi1.cos() * phi2.cos() * (delta_lambda / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        R * c
    }

    /// Checks if the position is inside a circle
    ///
    /// # Arguments
    ///
    /// * `center`: Center of the circle
    /// * `radius`: Radius of the circle in meters
    ///
    /// returns: bool
    #[inline]
    pub fn inside_circle(&self, center: &Position, radius: f64) -> bool {
        self.distance_haversine(center) < radius
    }

    #[inline]
    /// Check if the position is inside a polygon
    pub fn inside_polygon(&self, polygon: &[Position]) -> bool {
        winding_number(polygon, self) > 0
    }

    /// Distance between two positions using pytagore theorem
    pub fn distance(&self, other: &Self) -> f64 {
        ((self.lat - other.lat).powi(2) + (self.lng - other.lng).powi(2)).sqrt()
    }
}

/// Winding number of a polygon
fn winding_number(poly: &[Position], point: &Position) -> i32 {
    let mut wn = 0;
    let n = poly.len();
    poly.iter().enumerate().for_each(|(i, p0)| {
        let p1 = &poly[(i + 1) % n];
        if p0.lng <= point.lng {
            if p1.lng > point.lng && is_left(p0, p1, point) > 0.0 {
                wn += 1;
            }
        } else if p1.lng <= point.lng && is_left(p0, p1, point) < 0.0 {
            wn -= 1;
        }
    });
    wn
}

/// Check if the point is on the left of the line
fn is_left(p0: &Position, p1: &Position, p2: &Position) -> f64 {
    (p1.lat - p0.lat) * (p2.lng - p0.lng) - (p2.lat - p0.lat) * (p1.lng - p0.lng)
}

impl From<Position> for LatLng {
    fn from(value: Position) -> Self {
        LatLng::new(value.lat, value.lng)
    }
}

impl From<Position> for (f64, f64) {
    fn from(value: Position) -> Self {
        (value.lat, value.lng)
    }
}

impl From<Position> for [f64; 2] {
    fn from(value: Position) -> Self {
        [value.lat, value.lng]
    }
}
