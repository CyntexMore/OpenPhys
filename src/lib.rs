// Declare the modules directory
pub mod modules {
    pub mod heat_transfer;
}

// Re-exports
pub use modules::heat_transfer::{simulate_heat_transfer, HeatTransferResult, Object};
