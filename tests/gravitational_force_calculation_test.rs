use openphys::modules::gravitational_force::calculate_gravitational_force;
use openphys::utils::object::Object;

#[test]
fn test_gravitational_force_calculation() {
    let distance = 6000.0;

    let obj1 = Object {
        mass: 420.0,
        ..Object::default()
    };
    let obj2 = Object {
        mass: 844.5,
        ..Object::default()
    };

    calculate_gravitational_force(&obj1, &obj2, distance);
}

#[test]
fn test2_gravitational_force_calculation() {
    let distance = 9210.4;

    let obj1 = Object {
        mass: 595.0,
        ..Object::default()
    };
    let obj2 = Object {
        mass: 160.5,
        ..Object::default()
    };

    calculate_gravitational_force(&obj1, &obj2, distance);
}

#[test]
fn test3_gravitational_force_calculation() {
    let distance = 1590.4;

    let obj1 = Object {
        mass: 1.0,
        ..Object::default()
    };
    let obj2 = Object {
        mass: 1.0,
        ..Object::default()
    };

    calculate_gravitational_force(&obj1, &obj2, distance);
}
