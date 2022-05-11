use rppal::gpio;
use std::{thread, time::Duration};

fn main() {
    //let sensor1 = BeltSensor::new(0, 0);

    test_motor();

    //let push_motor;
    //println!("Hello, world!");
}

fn test_motor() {
    let gpio_instance = gpio::Gpio::new().unwrap();
    let mut gpio_pin = gpio_instance.get(17).unwrap().into_output();
    gpio_pin.set_high();
    thread::sleep(Duration::from_secs(1));
    gpio_pin.set_low();
}

struct Motor {
    id: i32,
    motor_type: MotorType,
    active: bool,
    power: i32,
    at_reset: bool,
}

pub enum MotorType {
    Push,
    Sort,
}

impl Motor {
    fn new(id: i32, motor_type: MotorType) -> Self {
        Self {
            id: id,
            motor_type: motor_type,
            active: false,
            power: 0,
            at_reset: false,
        }
    }

    fn push(&mut self, time: i32) {
        if self.active == true {
            // todo
        }
    }

    fn reset(&mut self) {
        if self.at_reset == false {
            // todo
            self.at_reset = true;
        }
    }

    fn is_reset(&self) -> bool {
        self.at_reset
    }
}

pub struct BeltSensor {
    id: i32,
    location: i32,
    reading: Signal,
    time_of_update: i32,
    active: bool,
}

impl BeltSensor {
    fn new(id: i32, location: i32) -> Self {
        BeltSensor {
            location: location,
            id: id,
            reading: Signal::None,
            time_of_update: 0,
            active: false,
        }
    }

    fn sense(&mut self) {
        // TODO get sensor output
        // set self.reading
    }

    fn read(&self) -> &Signal {
        &self.reading
    }
}

enum Signal {
    White,
    Black,
    Other,
    None,
}

struct StateMachine {
    state: States,
    motors: Vec<Motor>,
    sensors: Vec<BeltSensor>,
}

impl StateMachine {
    fn new() -> Self {
        Self {
            state: States::Initial,
            motors: Vec::new(),
            sensors: Vec::new(),
        }
    }

    fn belt_reset(&mut self) {
        for sensor in &mut self.sensors {
            sensor.active = true;
        }

        for motor in &mut self.motors {
            motor.reset();
            motor.active = true;
        }
    }

    fn initialise(&mut self) {
        // todo calibrate

        self.belt_reset();

        self.state = States::Detecting;
    }

    fn detect(&mut self) {
        // turn off sorting part
        self.belt_reset();

        loop {
            for sensor in &mut self.sensors {
                sensor.sense();

                // if sensor 2 output then
                self.state = States::Push;
            }
        }
    }

    fn push(&mut self) {
        // todo push
        // todo turn off sensors
        self.state = States::Sort;
    }

    fn sort(&mut self) {
        // TODO: read color of sensor 1
        // TODO: check if the disk has missed then fail
        // TODO: adjust rods to the appropriate column
        // TODO: reset flicking motor
        self.state = States::Detecting;
    }
}

enum States {
    Initial,
    Detecting,
    Push,
    Sort,
}
