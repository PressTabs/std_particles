
mod photon {
    pub trait Photon : GaugeBoson {
        pub const fn wavelength() -> f64;

        pub const fn frequency() -> f64;

        pub const fn energy() -> f64;

        pub const fn momentum() -> f64;
    }
}