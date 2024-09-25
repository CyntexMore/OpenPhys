#[derive(Debug, Clone)]
pub struct Object {
    /// The object's mass in kilograms (kg).
    pub mass: f64,
    /// The object's temperature in kelvin (K).
    pub temperature: f64,
    /// The object's specific heat capacity in [joule per kilogram kelvin](https://metricsystem.net/derived-units/units-whose-names-include-special-names/joule-per-kilogram-kelvin/) (J/kg K).
    pub specific_heat_capacity: f64,
    /// The object's density in kig per m^3 (kg/m^3).
    pub density: f64,
}

impl Object {
    /// You can create a new object using the `Object::new()` function. An object can have a
    /// `mass`, a `temperature` and a `specific_heat_capacity` value.
    /// # Examples
    /// ```
    /// let obj1 = Object::new(1.0, 273.15, 4980.0, 1.0);
    /// ````
    /// The variable must be mutable if you want to use it in a function that changes one or more
    /// of it's values.
    pub fn new(mass: f64, temperature: f64, specific_heat_capacity: f64, density: f64) -> Self {
        Self {
            mass,
            temperature,
            specific_heat_capacity,
            density,
        }
    }
}
