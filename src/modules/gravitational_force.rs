use crate::utils::object::Object;

const G: f64 = 6.67430e-11; // Gravitational constant

/// The 'calculate_gravitational_force' function is used to calculate the gravitational force
/// between two objects.
pub fn calculate_gravitational_force(obj1: &Object, obj2: &Object, distance: f64) {
    let mass_product = obj1.mass * obj2.mass;
    let _result: f64 = G * mass_product / (distance * distance);
}
