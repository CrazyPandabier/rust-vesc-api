extern crate serial;

use std::io::{Read, Write};

use packet::get_packet;
use serial::{prelude::*, SystemPort};

pub use packet::PacketData;
pub use serial::BaudRate;
pub use serial::Error;

mod packet;

pub struct Vesc {
    port: SystemPort,
}

impl Vesc {
    pub fn new(port_name: &str, baud_rate: serial::BaudRate) -> Result<Vesc, serial::Error> {
        let port = serial::open(port_name)?;
        let mut vesc = Vesc { port };
        vesc.init(baud_rate)?;
        Ok(vesc)
    }

    fn init(&mut self, baud_rate: serial::BaudRate) -> Result<(), serial::Error> {
        self.port.reconfigure(&|settings| {
            settings.set_baud_rate(baud_rate)?;
            settings.set_char_size(serial::Bits8);
            settings.set_parity(serial::ParityNone);
            settings.set_stop_bits(serial::Stop1);
            settings.set_flow_control(serial::FlowNone);
            Ok(())
        })?;

        self.port.set_timeout(std::time::Duration::from_secs(1))?;
        Ok(())
    }

    pub fn set_duty_cycle(&mut self, duty_cycle: f32) -> Result<(), Error> {
        let mut duty_cycle = duty_cycle;
        if duty_cycle > 1.0 {
            duty_cycle = 1.0;
        } else if duty_cycle < -1.0 {
            duty_cycle = -1.0;
        }

        let command = packet::commands::SetDutyCycle::new((duty_cycle * 100000.0) as i32);
        let packet = get_packet(command);
        self.port.write_all(&packet)?;
        Ok(())
    }

    pub fn get_values(&mut self) -> Result<PacketData, Error> {
        let command = packet::commands::GetValues::default();
        let packet = get_packet(command);
        self.port.write_all(&packet)?;
        std::thread::sleep(std::time::Duration::from_micros(10000));
        self.receive_packet()
    }

    pub fn send_alive(&mut self) -> Result<(), Error> {
        let command = packet::commands::Alive::default();
        let packet = get_packet(command);
        self.port.write_all(&packet)?;
        Ok(())
    }

    pub fn set_rpm(&mut self, rpm: i32) -> Result<(), Error> {
        let command = packet::commands::SetRpm::new(rpm);
        let packet = get_packet(command);
        self.port.write_all(&packet)?;
        Ok(())
    }

    pub fn set_current(&mut self, current: f32) -> Result<(), Error> {
        let command = packet::commands::SetCurrent::new((current * 1000.0) as i32);
        let packet = get_packet(command);
        self.port.write_all(&packet)?;
        Ok(())
    }

    pub fn set_current_brake(&mut self, current: f32) -> Result<(), Error> {
        let command = packet::commands::SetCurrentBrake::new((current * 1000.0) as i32);
        let packet = get_packet(command);
        self.port.write_all(&packet)?;
        Ok(())
    }

    fn receive_packet(&mut self) -> Result<PacketData, Error> {
        let mut buffer = [0; 256];
        let mut packet = Vec::new();

        let bytes_read = self.port.read(&mut buffer)?;
        packet.extend_from_slice(&buffer[..bytes_read]);

        Ok(packet::process_packet(packet)?)
    }
}
