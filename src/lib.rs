//! [OpenPhys] is a simple physics engine library written in Rust.
//!
//! [OpenPhys]: https://github.com/CyntexMore/OpenPhys

pub mod modules {
    /// The `gravitational_force` function is used to calculate the gravitational force between two
    /// objects.
    pub mod gravitational_force;
    /// The `heat_transfer` function is used to simulate heat transfer between two objects.
    pub mod heat_transfer;
    /// The `sandbox` function is used to simulate a sandbox environment.
    pub mod sandbox;
}

pub mod utils {
    /// The `Object` struct represents a physical object with basic properties.
    pub mod object;
    /// The `Environment` struct represents a physical environment with basic properties.
    pub mod environment;
}

pub use utils::object::Object;
