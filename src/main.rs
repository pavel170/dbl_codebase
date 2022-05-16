use rppal::{
    gpio::{self, Gpio},
    i2c, pwm,
    pwm::Channel,
};
use std::{
    thread::{self, Thread},
    time::Duration,
};

fn main() {
    run_tests();

    //let sensor1 = BeltSensor::new(0, 0);

    //let push_motor;
    //println!("Hello, world!");
}

fn run_tests() {
    pwm_test()
    //test_motor();
    //test_input();
    //gpio_test();
}

fn pwm_test() {
    let pwm0 =
        pwm::Pwm::with_frequency(Channel::Pwm1, 200.0, 1.0, pwm::Polarity::Normal, false).unwrap();
    pwm0.enable().ok();
}

fn gpio_test() {
    let gpio_inst = Gpio::new().unwrap();

    let mut pin = gpio_inst.get(17).unwrap();

    let mut outpin = pin.into_output();
    loop {
        outpin.set_high();
        thread::sleep(Duration::from_millis(1000));
        outpin.set_low();
        thread::sleep(Duration::from_millis(1000));
    }
}

fn test_motor() {
    const MOTOR1_A: usize = 2;
    const MOTOR1_B: usize = 3;
    const MOTOR2_A: usize = 1;
    const MOTOR2_B: usize = 4;
    const MOTOR4_A: usize = 0;
    const MOTOR4_B: usize = 6;
    const MOTOR3_A: usize = 5;
    const MOTOR3_B: usize = 7;

    let gpio_instance: Gpio = Gpio::new().unwrap();
    let mut motor_latch_pin = gpio_instance.get(11).unwrap().into_output();
    let mut motor_clk_pin = gpio_instance.get(13).unwrap().into_output();
    let mut motor_data_pin = gpio_instance.get(15).unwrap().into_output();

    let mut latch_state = 0;

    latch_state |= 1 << MOTOR1_A;
    latch_state &= !(1 << MOTOR1_B);

    motor_latch_pin.set_low();
    motor_data_pin.set_low();
    for i in 0..8 {
        motor_clk_pin.set_low();
        if latch_state & (1 << (7 - i)) > 0 {
            motor_data_pin.set_high();
        } else {
            motor_data_pin.set_low();
        }
        motor_clk_pin.set_high();
    }

    motor_latch_pin.set_high();

    let pwm0 =
        pwm::Pwm::with_frequency(Channel::Pwm1, 200.0, 1.0, pwm::Polarity::Normal, false).unwrap();
    pwm0.enable().ok();
}

fn test_input() {
    let mut i2c_inst = i2c::I2c::new().unwrap();
    const ADDR: u16 = 0x23;
    const REG: u8 = 0x10;
    let mut reg = [0u8; 2];
    loop {
        i2c_inst.set_slave_address(ADDR).ok();
        i2c_inst.block_read(REG, &mut reg);
        println!("{}", (reg[1] as f64 + (256.0 * (reg[0] as f64))) / 1.2);
        thread::sleep(Duration::from_millis(200));
    }
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
