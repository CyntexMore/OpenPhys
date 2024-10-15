use openphys::modules::sandbox::start_sandbox;
use openphys::utils::{object::Object, environment::Environment};

#[test]
fn test_sandbox_start() {
    let mut objects = vec![
        Object { mass: 1.0, position: [10.4, 2.4, 3.8], ..Object::default() },
        Object { mass: 1.0, position: [3.4, 1.0, 10.4], ..Object::default() },
    ];

    let mut env = Environment::default();

    let time_step: f64 = 0.1;

    start_sandbox(&mut env, &mut objects, time_step);
}