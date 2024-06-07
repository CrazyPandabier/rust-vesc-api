use std::io::Error;

const PACKET_MAX_LENGTH: usize = 512;
const SMALL_PACKET_MAX_LENGTH: usize = 256;

mod buffer;
pub mod commands;
mod crc;
pub mod datatypes;

use datatypes::McValues;

pub fn get_packet<T: commands::Command>(command: T) -> Vec<u8> {
    let data = command.get_data();

    if data.len() > PACKET_MAX_LENGTH {
        panic!("Data too long");
    }

    let mut packet = Vec::new();

    if data.len() >= SMALL_PACKET_MAX_LENGTH {
        packet.push(0x03);
        packet.push((data.len() >> 8) as u8);
        packet.push((data.len() & 0xFF) as u8);
    } else {
        packet.push(0x02);
        packet.push(data.len() as u8);
    }

    packet.extend(data.clone());

    let crc = crc::crc16(&data);
    packet.push((crc >> 8) as u8);
    packet.push((crc & 0xFF) as u8);

    packet.push(0x03);
    packet
}

pub enum PacketData {
    Values(McValues),
}

pub fn process_packet(buffer: Vec<u8>) -> Result<PacketData, Error> {
    let mut payload_length = 0;
    let mut payload_index = 0;
    let mut payload: Vec<u8> = Vec::new();

    for (index, byte) in buffer.iter().enumerate() {
        if *byte == 0x02 {
            payload_length = buffer[index + 1] as usize;
            payload_index = index + 2;
            break;
        }

        if *byte == 0x03 {
            payload_length = (buffer[index + 2] as usize) << 8 | buffer[index + 1] as usize;
            payload_index = index + 3;
            break;
        }
    }

    if payload_length == 0 {
        return Err(Error::new(std::io::ErrorKind::Other, "Invalid packet"));
    }

    for i in 0..payload_length {
        payload.push(buffer[payload_index + i]);
    }

    let crc = crc::crc16(&payload);
    let crc_received = (buffer[payload_index + payload_length] as u16) << 8
        | buffer[payload_index + payload_length + 1] as u16;

    if crc != crc_received {
        return Err(Error::new(std::io::ErrorKind::Other, "CRC error"));
    }

    Ok(extract_data(payload))
}

fn extract_data(buffer: Vec<u8>) -> PacketData {
    let id = buffer[0];
    match id {
        4 => {
            let mut values = McValues {
                temp_mosfet: 0.0,
                temp_motor: 0.0,
                current_motor: 0.0,
                current_in: 0.0,
                id: 0.0,
                iq: 0.0,
                duty_now: 0.0,
                rpm: 0.0,
                v_in: 0.0,
                amp_hours: 0.0,
                amp_hours_charged: 0.0,
                watt_hours: 0.0,
                watt_hours_charged: 0.0,
                tachometer: 0,
                tachometer_abs: 0,
                fault_code: datatypes::McFaultCode::None,
                pid_pos: 0.0,
                vesc_id: 0,
            };

            //print buffer
            println!("{:?}", buffer);

            let mut index = 1;
            values.temp_mosfet = buffer::get_float16(&buffer, 10.0, &mut index);
            values.temp_motor = buffer::get_float16(&buffer, 10.0, &mut index);
            values.current_motor = buffer::get_float32(&buffer, 100.0, &mut index);
            values.current_in = buffer::get_float32(&buffer, 100.0, &mut index);
            values.id = buffer::get_float32(&buffer, 100.0, &mut index);
            values.iq = buffer::get_float32(&buffer, 100.0, &mut index);
            values.duty_now = buffer::get_float16(&buffer, 1000.0, &mut index);
            values.rpm = buffer::get_float32(&buffer, 1.0, &mut index);
            values.v_in = buffer::get_float16(&buffer, 10.0, &mut index);
            values.amp_hours = buffer::get_float32(&buffer, 10000.0, &mut index);
            values.amp_hours_charged = buffer::get_float32(&buffer, 10000.0, &mut index);
            values.watt_hours = buffer::get_float32(&buffer, 10000.0, &mut index);
            values.watt_hours_charged = buffer::get_float32(&buffer, 10000.0, &mut index);
            values.tachometer = buffer::get_int32(&buffer, &mut index);
            values.tachometer_abs = buffer::get_int32(&buffer, &mut index);

            PacketData::Values(values)
        }
        _ => panic!("Unknown packet id"),
    }
}
