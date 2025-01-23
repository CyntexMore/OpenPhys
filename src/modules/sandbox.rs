use crate::utils::environment::Environment;
use crate::utils::object::Object;

/// You can use the `print_sandbox_info` function to print the details of a sandbox environment.
///
/// This function is still pretty experimental.
///
/// # Examples
/// ```
/// use openphys::utils::environment::Environment;
/// use openphys::utils::object::Object;
/// use openphys::modules::sandbox::init_sandbox;
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
///     let result = print_sandbox_info(&mut env, &mut objects, time_step);
/// }
/// ```
pub fn print_sandbox_info(environment: &mut Environment, objects: &mut [Object], time_step: f64) {
    println!("Initializing sandbox environment...\n");

    println!("Sandbox Environment Variables:");
    println!("Gravity: {} m/s^2", environment.gravity);
    println!("Temperature: {} K", environment.temperature);
    println!("Air Pressure: {} bar", environment.pressure);
    println!(
        "Environment Boundaries: x={}, y={}, z={}\n",
        environment.boundaries[0], environment.boundaries[1], environment.boundaries[2]
    );
}
