# Functions

Welcome to **OpenPhys**'s "documentation", this specific Markdown file contains documentation about the available functions in **OpenPhys**.

## Heat transfer

The `heat_transfer` function serves the purpose of simulating heat transfer between two objects.

### Files related to the `heat_transfer` function

The `heat_transfer` function's source file is located at `src/modules/heat_transfer.rs`. You can also find a test program of it at `tests/heat_transfer_simulation_test.rs`, it also serves as an example program, if you look at it that way.

### Usage of the `heat_transfer` function

To start using the `heat_transfer` function you have to import it with:
```rs
use openphys::simulate_heat_transfer;
```
If you actually want to create objects you also have to import `Object` and if you want to return/output anything you also have to import `HeatTransferResult`, you can do all of that with:
```rs
use openphys::{simulate_heat_transfer, HeatTransferResult, Object};
```

You can create a new object with:
```rs
Object::new(mass, temperature, specific_heat_capacity)
```
To be specific you can create a new object using that method in `let` variable, like this:
```rs
let mut obj1 = Object::new(1.0, 273.15, 4186.0); // 1kg of water at its freezing point
```

The `simulate_heat_transfer` function takes in 4 parameters by default — the objects being passed should be mutable — those are `obj1`, `obj2`, `time_step` and `equilibrium_threshold`, implementing this inside of a `let` variable would look like this:
```rs
let result = simulate_heat_transfer(&mut obj1, &mut obj2, time_step, equilibrium_threshold);
```
The `time_step` should be specified inside a `let` variable like this (1 `time_step` is 1 second):
```rs
let time_step = 0.1;
```
The `equilibrium_threshold` should be specified inside of a `let` variable, the value that I *recommend* is *10^-6* but it can really be anything below *10^-2*. *Currently* the program doesn't check if both `f64` values are equal because it would cause an infinite loop (or a near infinite loop, of course the computer would crash after some time). Here's an example of how you could specify an `equilibrium_threshold`:
```rs
let equilibrium_threshold = 0.00001; // 10^-6
```

If you also named the `let` variable `result` for the `simulate_heat_transfer` function then you can return values with `result.VALUE_TO_RETURN()`, here's an example:
```rs
println!("Total time: {:.2} seconds", result.total_time());
```
> ![TIP]
> The best way to print out values from the results is using the `{:.2}` formatting, *for clarity*.

### Returnable values of the `heat_transfer` function

These are the returnable values from the `heat_transfer` function:
  - `time_steps` — The current amount of time steps, counting from `0`
  - `total_time` — The total amount of time elapsed since the start of the heat transfer
  - `total_heat_transferred` — Total heat transferred between the two objects in *Jules*
  - `final_temp_of_obj1` — The final temperature of `obj1`
  - `final_temp_of_obj2` — The final temperature of `obj2`
  - `final_temperatures` — The final temperatures of `obj1` and `obj2`, basically "the value" when thermal equilibrium is reached
