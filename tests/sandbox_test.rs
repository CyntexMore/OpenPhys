use openphys::modules::sandbox::init_sandbox;
use openphys::utils::{environment::Environment, object::Object};

#[test]
fn test_sandbox_start() {
    let mut objects = vec![
        Object {
            mass: 1.0,
            position: [10.4, 2.4, 3.8],
            ..Object::default()
        },
        Object {
            mass: 1.0,
            position: [3.4, 1.0, 10.4],
            ..Object::default()
        },
    ];

    let mut env = Environment::default();

    let time_step: f64 = 0.1;

    init_sandbox(&mut env, &mut objects, time_step);
}

#[test]
fn test_sandbox_obj_move() {
    let mut obj1 = Object {
        position: [0.0, 0.0, 0.0],
        ..Object::default()
    };
    println!("obj1's position: {:?}", obj1.position);

    obj1.mv_to(1.0, 1.0, 1.0);
    println!("obj1's position: {:?}", obj1.position);

    obj1.mv_x(-1.0);
    println!("obj1's position: {:?}", obj1.position);

    obj1.mv_y(-1.0);
    println!("obj1's position: {:?}", obj1.position);

    obj1.mv_z(-1.0);
    println!("obj1's position: {:?}", obj1.position);
}
