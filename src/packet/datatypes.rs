use core::fmt;

#[derive(Debug)]
pub enum McFaultCode {
    None = 0,
    OverVoltage = 1,
    UnderVoltage = 2,
    Drv = 3,
    AbsOverCurrent = 4,
    OverTempFet = 5,
    OverTempMotor = 6,
}

#[derive(Debug)]
pub struct McValues {
    pub temp_mosfet: f32,
    pub temp_motor: f32,
    pub current_motor: f32,
    pub current_in: f32,
    pub id: f32,
    pub iq: f32,
    pub duty_now: f32,
    pub rpm: f32,
    pub v_in: f32,
    pub amp_hours: f32,
    pub amp_hours_charged: f32,
    pub watt_hours: f32,
    pub watt_hours_charged: f32,
    pub tachometer: i32,
    pub tachometer_abs: i32,
    pub fault_code: McFaultCode,
    pub pid_pos: f32,
    pub vesc_id: u8,
}

impl fmt::Display for McValues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "temp_mosfet: {}, temp_motor: {}, current_motor: {}, current_in: {}, id: {}, iq: {}, duty_now: {}, rpm: {}, v_in: {}, amp_hours: {}, amp_hours_charged: {}, watt_hours: {}, watt_hours_charged: {}, tachometer: {}, tachometer_abs: {}, fault_code: {:?}, pid_pos: {}, vesc_id: {}",
            self.temp_mosfet,
            self.temp_motor,
            self.current_motor,
            self.current_in,
            self.id,
            self.iq,
            self.duty_now,
            self.rpm,
            self.v_in,
            self.amp_hours,
            self.amp_hours_charged,
            self.watt_hours,
            self.watt_hours_charged,
            self.tachometer,
            self.tachometer_abs,
            self.fault_code,
            self.pid_pos,
            self.vesc_id
        )
    }
}

impl fmt::Display for McFaultCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
