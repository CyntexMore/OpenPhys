//! [OpenPhys] is a simple physics engine library written in Rust.
//!
//! [OpenPhys]: https://github.com/CyntexMore/OpenPhys

pub mod modules {
    /// The `gravitational_force` function is used to calculate the gravitational force between two
    /// objects.
    pub mod gravitational_force;
    /// The `heat_transfer` function is used to simulate heat transfer between two objects.
    pub mod heat_transfer;
}

pub mod utils {
    /// The `Object` struct represents a physical object with basic properties.
    pub mod object;
}

pub use modules::gravitational_force::calculate_gravitational_force;
pub use modules::heat_transfer::{simulate_heat_transfer, HeatTransferResult};
pub use utils::object::Object;
