/// The `Environment` struct is used to set the properties of a sandbox environment.
pub struct Environment {
    /// The gravitational force of the environment (m/s^2).
    pub gravity: f64,
    /// The temperature of the environment (globally, *for now*) in kelvin (K).
    pub temperature: f64,
    /// The air pressure of the environment (globally, *for now*) in bar (bar).
    pub pressure: f64,
    /// The duration of each simulation step in seconds (s).
    pub time_step: f64,
    /// The boundaries of the environment in a three dimensional space (*x* * *y* * *z*).
    pub boundaries: [f64; 3],
}

impl Environment {
    /// You can create a new environment with every property's value set to zero with the `Environment::new()` function.
    ///
    /// # Examples
    /// ```
    /// use openphys::utils::environment::Environment;
    ///
    /// let env = Environment::new();
    /// ```
    pub fn new() -> Self {
        Self {
            gravity: 0.0,
            temperature: 0.0,
            pressure: 0.0,
            time_step: 0.0,
            boundaries: [0.0, 0.0, 0.0],
        }
    }

    /// You can create a new environment with some sane defaults (relatively close to Earth's properties) with the `Environment::default()` function.
    ///
    /// # Examples
    /// ```
    /// use openphys::utils::environment::Environment;
    ///
    /// let env = Environment::default();
    /// ```
    ///
    /// You can also create a new environment by modifying some of its values while keeping others at their defaults.
    ///
    /// ```
    /// use openphys::utils::environment::Environment;
    ///
    /// let env = Environment {
    ///     boundaries: [100.0, 20.0, 100.0],
    ///     ..Environment::default()
    /// };
    /// ```
    pub fn default() -> Self {
        Self {
            gravity: 9.81,
            temperature: 288.15,
            pressure: 1.013,
            time_step: 0.1,
            boundaries: [10.0, 10.0, 10.0],
        }
    }
}
