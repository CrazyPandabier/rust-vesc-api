pub trait Command {
    fn get_data(&self) -> Vec<u8>;
}

pub struct SetDutyCycle {
    id: u8,
    duty_cycle: i32,
}

pub struct SetRpm {
    id: u8,
    rpm: i32,
}

pub struct SetCurrent {
    id: u8,
    current: i32,
}
pub struct SetCurrentBrake {
    id: u8,
    current: i32,
}

pub struct Alive {
    id: u8,
}

pub struct GetValues {
    id: u8,
}

impl SetDutyCycle {
    pub fn new(duty_cycle: i32) -> SetDutyCycle {
        SetDutyCycle { id: 5, duty_cycle }
    }
}

impl SetRpm {
    pub fn new(rpm: i32) -> SetRpm {
        SetRpm { id: 8, rpm: rpm }
    }
}

impl SetCurrent {
    pub fn new(current: i32) -> Self {
        SetCurrent {
            id: 6,
            current: current,
        }
    }
}

impl SetCurrentBrake {
    pub fn new(current: i32) -> Self {
        SetCurrentBrake {
            id: 7,
            current: current,
        }
    }
}

impl Default for Alive {
    fn default() -> Self {
        Alive { id: 30 }
    }
}

impl Default for GetValues {
    fn default() -> Self {
        GetValues { id: 4 }
    }
}

impl Command for SetDutyCycle {
    fn get_data(&self) -> Vec<u8> {
        let bytes = self.duty_cycle.to_be_bytes();
        vec![self.id, bytes[0], bytes[1], bytes[2], bytes[3]]
    }
}

impl Command for SetRpm {
    fn get_data(&self) -> Vec<u8> {
        let bytes = self.rpm.to_be_bytes();
        vec![self.id, bytes[0], bytes[1], bytes[2], bytes[3]]
    }
}

impl Command for SetCurrent {
    fn get_data(&self) -> Vec<u8> {
        let bytes = self.current.to_be_bytes();
        vec![self.id, bytes[0], bytes[1], bytes[2], bytes[3]]
    }
}

impl Command for SetCurrentBrake {
    fn get_data(&self) -> Vec<u8> {
        let bytes = self.current.to_be_bytes();
        vec![self.id, bytes[0], bytes[1], bytes[2], bytes[3]]
    }
}

impl Command for Alive {
    fn get_data(&self) -> Vec<u8> {
        vec![self.id]
    }
}

impl Command for GetValues {
    fn get_data(&self) -> Vec<u8> {
        vec![self.id]
    }
}
