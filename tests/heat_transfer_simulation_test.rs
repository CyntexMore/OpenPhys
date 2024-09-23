use openphys::{simulate_heat_transfer, HeatTransferResult, Object};

#[test]
fn test_heat_transfer_simulation() {
    let mut obj1 = Object::new(1.0, 373.15, 4186.0); // 1kg of water at its boiling point
    let mut obj2 = Object::new(1.0, 273.15, 4186.0); // 1kg of water at its freezing point
    let time_step = 0.1; // 0.1 seconds
    let equilibrium_threshold = 1e-6; // The equilibrium threshold is 10^-6 because
                                      // trying to check if two f64 values are equal
                                      // would cause an infinite (or close to
                                      // infinite) loop

    let result = simulate_heat_transfer(&mut obj1, &mut obj2, time_step, equilibrium_threshold);

    println!("Time steps: {}", result.time_steps);
    println!("Total time elapsed: {:.2}", result.total_time);
    println!(
        "Total heat transferred: {:.2}",
        result.total_heat_transferred
    );
    println!("Final temp of obj1: {:.2}", result.final_temp_obj1);
    println!("Final temp of obj2: {:.2}", result.final_temp_obj2);
}
