use petgraph::prelude::NodeIndex;

#[derive(Default, Debug, Copy, Clone)]
pub struct Task {
    /// Number of bits
    pub data_size: u64,
    /// Average of necessary cycles per bit
    pub processing_density: f64,
    /// Percent of computing that can be parallelized
    pub parallel_fraction: f64,
    /// Node on topology this task is forced to be present
    pub pin: Option<NodeIndex>,
}

impl Task {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with_pin(mut self, node: NodeIndex) -> Self {
        self.pin = Some(node);
        self
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Dependency {
    /// Number of bits
    pub data_size: u64,
}

impl Dependency {
    pub fn new(data_size: u64) -> Self {
        Self { data_size }
    }
}

impl std::fmt::Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}b", self.data_size))
    }
}
