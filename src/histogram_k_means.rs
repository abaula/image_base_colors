use crate::histogram::{Histogram, HistogramEntry};

pub struct HistogramKMeans {}

impl HistogramKMeans {
    pub fn cluster(histogram: &Histogram, _number_of_clusters: i32, _max_try_count: i32) -> Vec<HistogramEntry> {
        let vec = histogram.get_vec();
        vec
    }
}