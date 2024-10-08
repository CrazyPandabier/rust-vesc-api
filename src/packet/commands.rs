pub trait Command {
    fn get_data(&self) -> Vec<u8>;
}

pub struct SetDutyCycle {
    id: u8,
    duty_cycle: i32,
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
