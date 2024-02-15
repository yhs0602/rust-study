enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// use TrafficLight::{Red, Yellow};
use TrafficLight::{*};


fn main() {
    println!("Hello, world!");
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
