use leaflet::LatLng;
use leaflet::LatLngBounds;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Bbox {
    pub ne_lat: f64,
    pub ne_lng: f64,
    pub sw_lat: f64,
    pub sw_lng: f64,
}

impl Bbox {
    pub fn new(ne_lat: f64, ne_lng: f64, sw_lat: f64, sw_lng: f64) -> Self {
        Self {
            ne_lat,
            ne_lng,
            sw_lat,
            sw_lng,
        }
    }
}

impl From<Bbox> for LatLngBounds {
    fn from(value: Bbox) -> Self {
        LatLngBounds::new(
            &LatLng::new(value.ne_lat, value.ne_lng),
            &LatLng::new(value.sw_lat, value.sw_lng),
        )
    }
}

impl From<&Bbox> for LatLngBounds {
    fn from(value: &Bbox) -> Self {
        LatLngBounds::new(
            &LatLng::new(value.ne_lat, value.ne_lng),
            &LatLng::new(value.sw_lat, value.sw_lng),
        )
    }
}
