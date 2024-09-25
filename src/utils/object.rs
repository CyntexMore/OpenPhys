#[derive(Debug, Clone)]
pub struct Object {
    pub mass: f64,                   // Mass in kilograms (kg)
    pub temperature: f64,            // Temperature in Kelvin (K)
    pub specific_heat_capacity: f64, // Specific heat capacity in J/(kg*K)
}

impl Object {
    pub fn new(mass: f64, temperature: f64, specific_heat_capacity: f64) -> Self {
        Self {
            mass,
            temperature,
            specific_heat_capacity,
        }
    }
}
