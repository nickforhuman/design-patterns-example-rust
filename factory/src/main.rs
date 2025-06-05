use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum ToyType {
    Doll,
    Robot,
    Car,
}

#[derive(Debug, Serialize, Deserialize)]
struct Factory;

trait Toy {
    fn play(&self);
}

struct Doll;
struct Robot;
struct Car;

impl Toy for Robot {
    fn play(&self) {
        println!("i am a robot");
    }
}

impl Toy for Doll {
    fn play(&self) {
        println!("i am a doll");
    }
}

impl Toy for Car {
    fn play(&self) {
        println!("i am a car")
    }
}

impl Factory {
    fn build_toy(toy_type: ToyType) -> Box<dyn Toy> {
        match toy_type {
            ToyType::Robot => Box::new(Robot),
            ToyType::Car => Box::new(Car),
            ToyType::Doll => Box::new(Doll),
        }
    }
}

fn main() {
    let robot = Factory::build_toy(ToyType::Robot);
    let doll = Factory::build_toy(ToyType::Doll);
    let car = Factory::build_toy(ToyType::Car);

    robot.play();
    car.play();
    doll.play();
}
