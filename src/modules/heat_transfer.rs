#[derive(Debug, Clone)]
pub struct Object {
    pub mass: f64,                   // Mass in kilograms (kg)
    pub temperature: f64,            // Temperature in Kelvin (K)
    pub specific_heat_capacity: f64, // Specific heat capacity in J/(kg*K)
}

impl Object {
    pub fn new(mass: f64, temperature: f64, specific_heat_capacity: f64) -> Self {
        Self {
            mass,
            temperature,
            specific_heat_capacity,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeatTransferResult {
    pub time_steps: usize,
    pub total_time: f64,
    pub total_heat_transferred: f64,
    pub final_temp_obj1: f64,
    pub final_temp_obj2: f64,
}

impl HeatTransferResult {
    pub fn time_steps(&self) -> usize {
        self.time_steps
    }

    pub fn total_time(&self) -> f64 {
        self.total_time
    }

    pub fn total_heat_transferred(&self) -> f64 {
        self.total_heat_transferred
    }

    pub fn final_temperatures(&self) -> (f64, f64) {
        (self.final_temp_obj1, self.final_temp_obj2)
    }
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

    loop {
        let initial_temp_diff = (obj1.temperature - obj2.temperature).abs();

        if initial_temp_diff < equilibrium_threshold {
            break;
        }

        let heat_capacity1 = obj1.mass * obj1.specific_heat_capacity;
        let heat_capacity2 = obj2.mass * obj2.specific_heat_capacity;

        let heat_transfer_rate =
            initial_temp_diff / (1.0 / heat_capacity1 + 1.0 / heat_capacity2) * time_step;

        let obj1_delta_t = heat_transfer_rate / heat_capacity1;
        let obj2_delta_t = heat_transfer_rate / heat_capacity2;

        if obj1.temperature > obj2.temperature {
            obj1.temperature -= obj1_delta_t;
            obj2.temperature += obj2_delta_t;
        } else {
            obj1.temperature += obj1_delta_t;
            obj2.temperature -= obj2_delta_t;
        }

        total_heat_transferred += heat_transfer_rate.abs();
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
