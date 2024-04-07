use para_graph::model::{Device, Task};

#[derive(Debug, Default, Copy, Clone)]
pub struct Matching {
    pub finish_time: f64,
    pub node: usize,
}

impl std::fmt::Display for Matching {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Node {:2} Finish Time {:8.5}",
            self.node, self.finish_time
        ))
    }
}

pub(crate) fn computing_time(d: &Device, t: &Task) -> f64 {
    t.processing_density
        * t.data_size as f64
        * (1. - t.parallel_fraction + t.parallel_fraction / d.number_of_cores as f64)
        / (d.cpu_frequency * 100_000_000.)
}
