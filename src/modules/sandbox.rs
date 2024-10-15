use crate::utils::environment::Environment;
use crate::utils::object::Object;

/// You can use the `start_sandbox` function to start the simulation of a sandbox environment.
///
/// \*It's still pretty experimental.
///
/// # Examples
/// ```
/// use openphys::utils::environment::Environment;
/// use openphys::utils::object::Object;
/// use openphys::modules::sandbox::start_sandbox;
///
/// fn main() {
///     let mut objects = vec![
///         Object { mass: 1.0, position: [0.0, 1.0, 0.0], ..Object::default() },
///         Object { mass: 10.0, position: [0.0, 2.0, 0.0], ..Object::default() },
///     ];
///
///     let mut env = Environment::default();
///
///     let time_step: f64 = 0.1;
///
///     let result = start_sandbox(&mut env, &mut objects, time_step);
/// }
/// ```
pub fn start_sandbox(environment: &mut Environment, objects: &mut [Object], time_step: f64) {
    println!("Entering sandbox environment.\n");

    // Print basic information about the environment
    println!(
        "Environment gravity: {:?}\n\
        Environment temperature: {:?}\n\
        Environment pressure: {:?}\n\
        Environment boundaries: {:?}\n",
        environment.gravity, environment.temperature, environment.pressure, environment.boundaries
    );

    println!("Exiting sandbox environment.\n");
}
