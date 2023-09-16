#[derive(Debug, Copy, Clone)]
pub struct Device {
    pub number_of_cores: u32,
    /// In Ghz
    pub cpu_frequency: f64,
}

impl Device {
    pub fn new(number_of_cores: u32, cpu_frequency: f64) -> Self {
        Self {
            number_of_cores,
            cpu_frequency,
        }
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Transmission {
    /// In Gbps
    pub transmission_rate: f64,
}

impl Transmission {
    pub fn new(transmission_rate: f64) -> Self {
        Self { transmission_rate }
    }
}

impl std::fmt::Display for Transmission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}gbps", self.transmission_rate))
    }
}
