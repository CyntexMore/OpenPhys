# Heat transfer

The heat transfer function is designed to simulate heat transfer between two objects. It takes the mass, temperature and specific heat capacity of the objects into account. *\*Note that later the list of factors being taken into account will be expanded.*

## Files related to the heat transfer function

The heat transfer function's main code is located in the `src/modules/heat_transfer.rs` source file. You can also find a unit test of the function at `tests/heat_transfer_simulation_test.rs`.

## Usage of the heat transfer function

### Parts of the heat transfer function

You can import these parts of the heat transfer function:

  - `simulate_heat_transfer` — The function that does most of the work and processes the calculations
  - `HeatTransferResult` — The part of the heat transfer function that gives you the ability to return specific values
  - `Object` — The part of the heat transfer function that gives you the ability to create new objects

You can import all of those simply with:
```rs
use openphys::{simulate_heat_transfer, HeatTransferResult, Object};
```

### Returnable values from the heat transfer function

These are the returnable values from the `heat_transfer` function:

  - `time_steps` — The current amount of time steps, counting from `0`
  - `total_time` — The total amount of time elapsed since the start of the heat transfer
  - `total_heat_transferred` — Total heat transferred between the two objects in *Jules*
  - `final_temp_of_obj1` — The final temperature of `obj1`
  - `final_temp_of_obj2` — The final temperature of `obj2`

Here's an example of how you could return one of these values:
```rs
println!("Total time: {:.2} seconds", result.total_time);
```

### Creating new objects

You can create new objects with `Object::new(mass, temperature, specific_heat_capacity)`.

  - `mass` — The mass of the object in kilograms (*kg*)
  - `temperature` — The temperature of the object in Kelvin (*K*)
  - `specific_heat_capacity` — The amount of energy required to change the temperature of the object by 1K (*J/kg\*K*)

An example of a new object is:
```rs
let mut obj1 = Object::new(1.0, 273.15, 4186.0); // 1kg of water at its freezing point
```
The variable is mutable because the `simulate_heat_transfer` function changes the temperature of the objects (`obj1` in this case).

### Setting up some other variables

Before we can begin using the `simulate_heat_transfer` function we need to specify some variables.

  - `time_step` — The frequency we want the `simulate_heat_transfer` to update the temperature of an objects (1 `time_step` = 1 second) - adjust based on the accuracy you want, the *recommended* value is `0.1`
  - `equilibrium_threshold` — Once the difference between two objects' temperatures fall into this threshold, no further changes are made to the objects' temperatures, thus assuming balance between the two objects' temperatures - the temperature difference between the two objects doesn't have to be exactly 0, just low enough that they don't affect each other anymore - adjust based on the accuracy you want, the *recommended* value is *10^-6* but anything lower than *10^-2* will do

An example of specifying these two variable is:
```rs
let time_step = 0.1; // 0.1 seconds
let equilibrium_threshold = 1e-6; // 10^-6
```

### Using the `simulate_heat_transfer` function

We can pass the two objects and the other variables we made into the `simulate_heat_transfer` as parameters like this: `simulate_heat_transfer(&mut obj1, &mut obj2, time_step, equilibrium_threshold)`.

Here's an example of using the `simulate_heat_transfer` function with the variables we specified:
```rs
let result = simulate_heat_transfer(&mut obj1, &mut obj2, time_step, equilibrium_threshold);
```
