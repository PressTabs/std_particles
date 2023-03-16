
mod gauge_boson;
mod scalar_boson;

mod boson {
    pub trait Boson : ElementaryParticle {
        pub const fn spin(&self) -> i32;
    }
}