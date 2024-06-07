use vesc_api::{BaudRate, Vesc};

fn main() {
    let mut vesc = Vesc::new("/dev/ttyUSB0", BaudRate::Baud115200).unwrap();
    let max_duty = 0.5;
    let min_duty = -0.5;
    let mut duty = 0.0;

    let mut increase: bool = true;

    loop {
        vesc.set_duty_cycle(duty).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));

        if increase {
            duty += 0.1;
        } else {
            duty -= 0.1;
        }

        if duty >= max_duty {
            increase = false;
        } else if duty <= min_duty {
            increase = true;
        }
    }
}
