use openphys::{simulate_heat_transfer, HeatTransferResult, Object};

#[test]
fn test_heat_transfer_simulation() {
    let mut obj1 = Object::new(1.0, 373.15, 4186.0); // 1kg of water at its boiling point
    let mut obj2 = Object::new(1.0, 273.15, 4186.0); // 1kg of water at its freezing point
    let time_step = 0.1; // 0.1 seconds
    let equilibrium_threshold = 0.000001; // The equilibrium threshold is 10^-6 because
                                          // trying to check if two f64 values are equal
                                          // would cause an infinite (or close to
                                          // infinite) loop

    let result = simulate_heat_transfer(&mut obj1, &mut obj2, time_step, equilibrium_threshold);

    println!("Simulation completed in {} steps", result.time_steps());
    println!("Total time: {} seconds", result.total_time());
    println!(
        "Total heat transferred: {}J",
        result.total_heat_transferred()
    );
    println!("Final temperatures: {:?}", result.final_temperatures());

    // Add assertions to verify the results
    assert!(
        result.time_steps() > 0,
        "Simulation should take at least one step"
    );
    assert!(result.total_time() > 0.0, "Total time should be positive");
    assert!(
        result.total_heat_transferred() > 0.0,
        "Heat should be transferred"
    );

    let (final_temp1, final_temp2) = result.final_temperatures();
    assert!(
        (final_temp1 - final_temp2).abs() < equilibrium_threshold,
        "Final temperatures should be within the equilibrium threshold"
    );
}
