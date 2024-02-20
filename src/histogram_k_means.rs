use rand::prelude::*;
use crate::{histogram::{Histogram, HistogramEntry}, rgba_color::RgbaColor};

pub struct HistogramKMeans {}

impl HistogramKMeans {
    pub fn new() -> Self {
        Self {}
    }

    pub fn cluster(&self, histogram: &Histogram, number_of_clusters: u32, max_try_count: u32) -> Vec<HistogramEntry> {

        let vec = histogram.get_vec();
        //vec.sort_by(|a, b| b.weight.cmp(&a.weight));
        //vec.into_iter().take(number_of_clusters as usize).collect::<Vec<_>>()
        let data_size: u32 = vec.len().try_into().expect("Failed convert usize to u32.");
        let num_dimentions = RgbaColor::get_size();

        let (mut data, mut weights) = self.initialize(&vec);
        let mut clustering = self.init_clustering(data_size, number_of_clusters);
        let mut means = self.allocate_means(number_of_clusters, num_dimentions);

        let mut changed = true;
        let mut success = true;
        let mut counter: u32 = 0;

        while changed && success && counter < max_try_count {
            success = self.update_means(&mut data, &mut weights, &mut clustering, &mut means);
            changed = self.update_clustering(&mut data, &mut clustering, &mut means);
            counter += 1;
        }

        let mut centers: Vec<HistogramEntry> = Vec::new();

        for i in 0..number_of_clusters {
            let cluster_count = clustering.iter().filter(|&n| *n == i).count();
            let cluster_index: usize = i.try_into().expect("Failed convert u32 to usize.");
            let mean = &means[cluster_index];
            let color = RgbaColor::new(mean[0], mean[1], mean[2], mean[3]);
            let weight = cluster_count as f32 / data_size as f32;
            centers.push(HistogramEntry::new(color, weight));
        }

        centers
    }

    fn update_means(&self, data: &mut Vec<Vec<f32>>, weights: &mut Vec<f32>, clustering: &mut Vec<u32>, means: &mut Vec<Vec<f32>>) -> bool {
        true
    }

    fn update_clustering(&self, data: &mut Vec<Vec<f32>>, clustering: &mut Vec<u32>, means: &mut Vec<Vec<f32>>) -> bool {
        true
    }

    fn initialize(&self, histogram: &Vec<HistogramEntry>) -> (Vec<Vec<f32>>, Vec<f32>) {

        let mut data: Vec<Vec<f32>> = Vec::new();
        let mut weights: Vec<f32> = Vec::new();

        for entry in histogram {
            data.push(entry.color.to_vec());
            weights.push(entry.weight);
        }

        (data, weights)
    }

    fn init_clustering(&self, size: u32, number_of_clusters: u32) -> Vec<u32> {

        let mut rng = rand::thread_rng();

        let clustering: Vec<u32> = (0..size)
            .map(|x| {
                if x < number_of_clusters {
                    x
                }
                else {
                    rng.gen_range(0..number_of_clusters)
                }
            })
            .collect();

        clustering
    }

    fn allocate_means(&self, num_clusters: u32, num_dimentions: u32) -> Vec<Vec<f32>> {
        vec![vec![0.0f32; num_dimentions as usize]; num_clusters as usize]
    }
}