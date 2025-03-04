#[derive(Debug, Clone)]
pub struct Object {
    /// The object's mass in kilograms (kg).
    pub mass: f64,
    /// The object's temperature in kelvin (K).
    pub temperature: f64,
    /// Specifies if the object's temperature is in Celsius or not (˚C).
    pub is_celsius: bool,
    /// The object's specific heat capacity in joule per kilogram kelvin (J/kg K).
    pub specific_heat_capacity: f64,
    /// The object's density in grams per milliliter (g/ml).
    pub density: f64,
    /// The object's kinetic energy in joules (J).
    pub kinetic_energy: f64,
    /// The object's velocity in a three-dimensional in meter per second (m/s).
    pub velocity: [f64; 3],
    /// The object's acceleration in a three-dimensional space in meter per second squared (m/s^2).
    pub acceleration: [f64; 3],
    /// The position of the object's center in a three-dimensional space (x, y, z).
    pub position: [f64; 3],
}

/// You can create a new object inside a variable.
///
/// # Examples
/// ```
/// use openphys::utils::object::Object;
///
/// let obj1 = Object {
///     mass: 14.4,
///     ..Object::default()
/// };
/// ```
impl Object {
    /// You can create a new empty object using the `Object::new()` function.
    ///
    /// # Examples
    /// ```
    /// use openphys::utils::object::Object;
    ///
    /// let obj1 = Object::new();
    /// ```
    pub fn new() -> Self {
        Self {
            mass: 0.0,
            temperature: 0.0,
            is_celsius: false,
            specific_heat_capacity: 0.0,
            density: 0.0,
            kinetic_energy: 0.0,
            velocity: [0.0, 0.0, 0.0],
            acceleration: [0.0, 0.0, 0.0],
            position: [0.0, 0.0, 0.0],
        }
    }

    /// You can create an object with some reasonable defaults with the `Object::default()`
    /// function.
    ///
    /// # Examples
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
    ///     temperature: 20.0,
    ///     is_celsius: true,
    ///     ..Object::default()
    /// };
    /// ```
    pub fn default() -> Self {
        Self {
            mass: 1.0,
            temperature: 293.15,
            is_celsius: false,
            specific_heat_capacity: 4186.0,
            density: 1.0,
            kinetic_energy: 0.0,
            velocity: [0.0, 0.0, 0.0],
            acceleration: [0.0, 0.0, 0.0],
            position: [0.0, 0.0, 0.0],
        }
    }

    /// You can move an object to a specific location with the `Object::mv_to()` function.
    ///
    /// # Examples
    /// ```
    /// use openphys::utils::object::Object;
    ///
    /// let mut obj1 = Object::default();
    /// obj1.mv_to(1.0, 1.0, 1.0);
    /// ```
    pub fn mv_to(&mut self, x: f64, y: f64, z: f64) {
        self.position = [x, y, z];
    }

    /// You can move an object by a specific distance on the *x* axis with the
    /// `Object::mv_x()` function.
    ///
    /// # Examples
    /// ```
    /// use openphys::utils::object::Object;
    ///
    /// let mut obj1 = Object::default();
    /// obj1.mv_x(1.0);
    /// ```
    pub fn mv_x(&mut self, distance: f64) {
        self.position[0] += distance;
    }

    /// You can move an object by a specific distance on the *y* axis with the
    /// `Object::mv_y()` function.
    ///
    /// # Examples
    /// ```
    /// use openphys::utils::object::Object;
    ///
    /// let mut obj1 = Object::default();
    /// obj1.mv_y(1.0);
    /// ```
    pub fn mv_y(&mut self, distance: f64) {
        self.position[1] += distance;
    }

    /// You can move an object by a specific distance on the *z* axis with the
    /// `Object::mv_z()` function.
    ///
    /// # Examples
    /// ```
    /// use openphys::utils::object::Object;
    ///
    /// let mut obj1 = Object::default();
    /// obj1.mv_z(1.0);
    /// ```
    pub fn mv_z(&mut self, distance: f64) {
        self.position[2] += distance;
    }
}
