use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

// server statistics
pub struct Metrics(HashMap<&'static str, AtomicUsize>);

impl Metrics {
    pub fn new(names: &[&'static str]) -> Self {
        let mut metrics: HashMap<&'static str, AtomicUsize> = HashMap::new();
        for name in names.iter() {
            metrics.insert(name, AtomicUsize::new(0));
        }
        Self(metrics)
    }

    pub fn inc(&self, name: &'static str) {
        if let Some(m) = self.0.get(name) {
            m.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn add(&self, name: &'static str, val: usize) {
        if let Some(m) = self.0.get(name) {
            m.fetch_add(val, Ordering::Relaxed);
        }
    }

    pub fn dec(&self, name: &'static str) {
        if let Some(m) = self.0.get(name) {
            m.fetch_sub(1, Ordering::Relaxed);
        }
    }

    pub fn snapshot(&self) -> Vec<(&'static str, usize)> {
        self.0
            .iter()
            .map(|(k, v)| (*k, v.load(Ordering::Relaxed)))
            .collect()
    }

    pub fn get(&self, name: &'static str) -> Option<usize> {
        self.0.get(name).map(|v| v.load(Ordering::Relaxed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        pub(crate) static ref METRICS: Metrics = Metrics::new(&[
            "topics",
            "clients",
            "peers",
            "broadcasts",
            "servers",
            "states",
            "subscribers"
        ]);
    }

    #[test]
    fn it_works() {
        METRICS.inc("topics");
        METRICS.add("topics", 2);
        METRICS.dec("topics");

        assert_eq!(METRICS.get("topics"), Some(2));
    }
}
