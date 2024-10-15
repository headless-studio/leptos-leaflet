use super::Position;
use leaflet::LatLng;
use leaflet::LatLngBounds;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub ne_corner: Position,
    pub sw_corner: Position,
}

impl Bounds {
    pub fn new(ne_corner: Position, sw_corner: Position) -> Self {
        Self {
            ne_corner,
            sw_corner,
        }
    }

    pub fn get_center(&self) -> Position {
        Position {
            lat: (self.ne_corner.lat + self.sw_corner.lat) / 2.0,
            lng: (self.ne_corner.lng + self.sw_corner.lng) / 2.0,
        }
    }

    pub fn get_bottom_left(&self) -> Position {
        Position::new(self.sw_corner.lat, self.sw_corner.lng)
    }

    pub fn get_top_right(&self) -> Position {
        Position::new(self.ne_corner.lat, self.ne_corner.lng)
    }

    pub fn get_top_left(&self) -> Position {
        Position::new(self.ne_corner.lat, self.sw_corner.lng)
    }

    pub fn get_bottom_right(&self) -> Position {
        Position::new(self.sw_corner.lat, self.ne_corner.lng)
    }

    pub fn get_size(&self) -> Position {
        Position {
            lat: (self.ne_corner.lat - self.sw_corner.lat).abs(),
            lng: (self.ne_corner.lng - self.sw_corner.lng).abs(),
        }
    }

    pub fn contains(&self, position: Position) -> bool {
        self.sw_corner.lat <= position.lat
            && self.ne_corner.lat >= position.lat
            && self.sw_corner.lng <= position.lng
            && self.ne_corner.lng >= position.lng
    }

    // Returns true if the rectangle intersects the given bounds. Two bounds intersect if they have at least one point in common.
    pub fn intersects(&self, other: Bounds) -> bool {
        let lat_overlap =
            self.ne_corner.lat >= other.sw_corner.lat && self.sw_corner.lat <= other.ne_corner.lat;
        let lng_overlap =
            self.ne_corner.lng >= other.sw_corner.lng && self.sw_corner.lng <= other.ne_corner.lng;

        lat_overlap && lng_overlap
    }

    // Returns true if the rectangle overlaps the given bounds. Two bounds overlap if their intersection is an area.
    pub fn overlaps(&self, other: Bounds) -> bool {
        let lat_overlap =
            self.ne_corner.lat > other.sw_corner.lat && self.sw_corner.lat < other.ne_corner.lat;
        let lng_overlap =
            self.ne_corner.lng > other.sw_corner.lng && self.sw_corner.lng < other.ne_corner.lng;

        lat_overlap && lng_overlap
    }

    pub fn is_valid(&self) -> bool {
        self.ne_corner.lat <= 90.0
            && self.ne_corner.lat >= -90.0
            && self.sw_corner.lat <= 90.0
            && self.sw_corner.lat >= -90.0
            && self.ne_corner.lng <= 180.0
            && self.ne_corner.lng >= -180.0
            && self.sw_corner.lng <= 180.0
            && self.sw_corner.lng >= -180.0
            && self.ne_corner.lat >= self.sw_corner.lat
            && self.ne_corner.lng >= self.sw_corner.lng
    }

    pub fn pad(&self, buffer_ratio: f64) -> Bounds {
        let lat_diff = self.ne_corner.lat - self.sw_corner.lat;
        let lng_diff = self.ne_corner.lng - self.sw_corner.lng;
        let lat_pad = lat_diff * buffer_ratio;
        let lng_pad = lng_diff * buffer_ratio;
        Bounds {
            ne_corner: Position::new(self.ne_corner.lat + lat_pad, self.ne_corner.lng - lng_pad),
            sw_corner: Position::new(self.sw_corner.lat - lat_pad, self.sw_corner.lng + lng_pad),
        }
    }

    pub fn equals(&self, other: Bounds) -> bool {
        self.ne_corner == other.ne_corner && self.sw_corner == other.sw_corner
    }

    pub fn as_lat_lng_bounds(&self) -> LatLngBounds {
        LatLngBounds::new(
            &LatLng::new(self.ne_corner.lat, self.ne_corner.lng),
            &LatLng::new(self.sw_corner.lat, self.sw_corner.lng),
        )
    }
}

impl From<Bounds> for LatLngBounds {
    fn from(value: Bounds) -> Self {
        LatLngBounds::new(
            &LatLng::new(value.ne_corner.lat, value.ne_corner.lng),
            &LatLng::new(value.sw_corner.lat, value.sw_corner.lng),
        )
    }
}

impl From<&Bounds> for LatLngBounds {
    fn from(value: &Bounds) -> Self {
        LatLngBounds::new(
            &LatLng::new(value.ne_corner.lat, value.ne_corner.lng),
            &LatLng::new(value.sw_corner.lat, value.sw_corner.lng),
        )
    }
}
