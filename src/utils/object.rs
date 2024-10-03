#[derive(Debug, Clone)]
pub struct Object {
    /// The object's mass in kilograms (kg).
    pub mass: f64,
    /// The object's temperature in kelvin (K).
    pub temperature: f64,
    /// The object's specific heat capacity in [joule per kilogram kelvin](https://metricsystem.net/derived-units/units-whose-names-include-special-names/joule-per-kilogram-kelvin/) (J/kg K).
    pub specific_heat_capacity: f64,
    /// The object's density in kilogram per m^3 (kg/m^3).
    pub density: f64,
    /// The object's velocity in meter per second (m/s).
    pub velocity: f64,
    /// The object's kinetic energy in joules (J).
    pub kinetic_energy: f64,
    /// The object's position in a three dimensional space (x, y, z).
    pub position: [f64; 3],
}

/// You can create a new object inside of a variable.
///
/// # Usage
/// ```
/// use openphys::utils::object::Object;
///
/// let obj1 = Object {
///     mass: 14.4,
///     ..Object::default()
/// };
/// ```
///
/// The variable must be mutable if you want to use it in a function that changes one or more
/// of it's values.
impl Object {
    /// You can create a new empty object using the `Object::new()` function.
    ///
    /// # Usage
    /// ```
    /// use openphys::utils::object::Object;
    ///
    /// let obj1 = Object::new();
    /// ```
    ///
    /// The variable must be mutable if you want to use it in a function that changes one or more
    /// of it's values.
    pub fn new() -> Self {
        Self {
            mass: 0.0,
            temperature: 0.0,
            specific_heat_capacity: 0.0,
            density: 0.0,
            velocity: 0.0,
            kinetic_energy: 0.0,
            position: [0.0, 0.0, 0.0],
        }
    }

    /// You can create an object with some reasonable defaults with the `Object::default()`
    /// function.
    ///
    /// # Usage
    /// ```
    /// use openphys::utils::object::Object;
    ///
    /// let obj1 = Object::default();
    /// ```
    /// Also, you can create an object with some changed values and some default values, like this:
    /// ```
    /// use openphys::utils::object::Object;
    ///
    /// let obj1 = Object {
    ///     mass: 1.0,
    ///     temperature: 293.15,
    ///     ..Object::default()
    /// };
    /// ```
    ///
    /// The variable must be mutable if you want to use it in a function that changes one or more
    /// of it's values.
    pub fn default() -> Self {
        Self {
            mass: 1.0,
            temperature: 293.15,
            specific_heat_capacity: 4186.0,
            density: 1000.0,
            velocity: 0.0,
            kinetic_energy: 0.0,
            position: [0.0, 0.0, 0.0],
        }
    }
}
