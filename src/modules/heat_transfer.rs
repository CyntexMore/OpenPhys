use crate::utils::object::Object;

#[derive(Debug, Clone)]
pub struct HeatTransferResult {
    pub time_steps: usize,
    pub total_time: f64,
    pub total_heat_transferred: f64,
    pub final_temp_obj1: f64,
    pub final_temp_obj2: f64,
}

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
