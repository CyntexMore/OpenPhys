// Declare the modules directory
pub mod modules;
pub mod utils;

// Re-exports
pub use modules::heat_transfer::{simulate_heat_transfer, HeatTransferResult};
pub use utils::object::Object;
