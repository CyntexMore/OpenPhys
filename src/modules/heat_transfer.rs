use crate::utils::object::Object;

/// The `HeatTransferResult` struct is used to collect the results from the heat transfer simulation.
#[derive(Debug, Clone)]
pub struct HeatTransferResult {
    /// The current of number time steps elapsed since the start of the simulation.
    pub time_steps: usize,
    /// The current time elapsed since the start of the simulation.
    pub total_time: f64,
    /// The total amount of heat transferred between the objects in joules.
    pub total_heat_transferred: f64,
    /// The final temperature of obj1 after the simulation finishes.
    pub final_temp_obj1: f64,
    /// The final temperature of obj2 after the simulation finishes.
    pub final_temp_obj2: f64,
}

/// The `simulate_heat_transfer` function is used to simulate heat transfer between two objects.
/// # Usage
/// ```
/// use openphys::modules::heat_transfer::simulate_heat_transfer;
/// use openphys::utils::object::Object;
///
/// let mut obj1 = Object::new(1.0, 375.15, 4186.0, 1.0);
/// let mut obj2 = Object::new(1.0, 273.15, 4186.0, 1.0);
/// let time_step = 0.1;
/// let equilibrium_threshold = 1e-6;
///
/// let result = simulate_heat_transfer(&mut obj1,  &mut obj2, time_step, equilibrium_threshold);
/// ```
/// \*Note: The objects must be made in mutable variables since their temperatures will get changed. The `time_step` and `equilibrium_threshold` variables must be specified by the user.
///
/// The `time_step` variable specifies the duration of each simulation step in seconds.
///
/// The `equilibrium_threshold` variable specifies the temperature difference below which the objects are considered to be in thermal equilibrium. I *recommend* to use *10^-6* as a value but anything below *10^-2* will do.
pub fn simulate_heat_transfer(
    obj1: &mut Object,
    obj2: &mut Object,
    time_step: f64,
    equilibrium_threshold: f64,
) -> HeatTransferResult {
    let mut time_steps = 0;
    let mut total_time = 0.0;
    let mut total_heat_transferred = 0.0;

    let heat_capacity1 = obj1.mass * obj1.specific_heat_capacity;
    let heat_capacity2 = obj2.mass * obj2.specific_heat_capacity;
    let heat_capacity_factor = 1.0 / (1.0 / heat_capacity1 + 1.0 / heat_capacity2);

    while (obj1.temperature - obj2.temperature).abs() >= equilibrium_threshold {
        let heat_transfer_rate =
            (obj1.temperature - obj2.temperature).abs() * heat_capacity_factor * time_step;

        let obj1_delta_t = heat_transfer_rate / heat_capacity1;
        let obj2_delta_t = heat_transfer_rate / heat_capacity2;

        if obj1.temperature > obj2.temperature {
            obj1.temperature -= obj1_delta_t;
            obj2.temperature += obj2_delta_t;
        } else {
            obj1.temperature += obj1_delta_t;
            obj2.temperature -= obj2_delta_t;
        }

        total_heat_transferred += heat_transfer_rate;
        time_steps += 1;
        total_time += time_step;
    }

    HeatTransferResult {
        time_steps,
        total_time,
        total_heat_transferred,
        final_temp_obj1: obj1.temperature,
        final_temp_obj2: obj2.temperature,
    }
}
