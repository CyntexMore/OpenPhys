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
    println!("Information about the current sandbox environment:\n");
    println!("Time step value: {:?}", time_step);
    println!("Objects: {:?}\n", objects);
    println!("Environment gravity: {:?}\n", environment.gravity);
    println!("Environment temperature: {:?}\n", environment.temperature);
    println!("Environment air pressure: {:?}\n", environment.pressure);
    println!("Exiting sandbox environment.\n");
}
