//! [OpenPhys] is a simple physics engine library written in Rust.
//!
//! [OpenPhys]: https://github.com/CyntexMore/OpenPhys

pub mod modules {
    /// `heat_transfer` is used to simulate heat transfer between two objects based on the
    /// objects' mass, temperature and specific heat capacity.
    pub mod heat_transfer;
}

pub mod utils {
    /// The `Object` struct represents a physical object with basic properties.
    pub mod object;
}

pub use modules::heat_transfer::{simulate_heat_transfer, HeatTransferResult};
pub use utils::object::Object;
