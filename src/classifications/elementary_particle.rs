
mod chirality;
mod fermion;
mod boson;

pub trait ElementaryParticle<Chirality: chirality::Chirality> {
    pub const fn mass(&self) -> f64;

    pub const fn charge(&self) -> i32;
}