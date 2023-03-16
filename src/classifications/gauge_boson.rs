
mod photon;
mod wplus;
mod wminus;
mod gluon;

enum MediatedForce {
    Electromagnetic,
    Weak,
    Strong
}

mod gauge_boson {
    pub trait GaugeBoson : Boson {
        pub const fn mediated_force(&self) -> MediatedForce;
    }
}