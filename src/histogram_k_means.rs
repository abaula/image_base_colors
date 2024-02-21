use itertools::Itertools;
use rand::prelude::*;
use crate::{histogram::{Histogram, HistogramEntry}, rgba_color::RgbaColor};

pub struct HistogramKMeans {}

impl HistogramKMeans {
    pub fn new() -> Self {
        Self {}
    }

    pub fn cluster(&self, histogram: &Histogram, number_of_clusters: u32, max_try_count: u32) -> Vec<(RgbaColor, f32)> {

        let vec = histogram.get_vec();

        if vec.len() < 1 {
            return Vec::new();
        }

        let data_size: u32 = vec.len().try_into().expect("Failed convert usize to u32.");
        let num_dimentions = RgbaColor::get_size();

        let (data, weights) = self.initialize(&vec);

        let mut clustering = self.init_clustering(data_size, number_of_clusters);
        let mut means = self.allocate_means(number_of_clusters, num_dimentions);
        let mut counter: u32 = 0;

        while counter < max_try_count {

            if let Some(new_means) = self.update_means(&means, &data, &weights, &clustering) {
                means = new_means;
            }
            else {
                break;
            }

            if let Some(new_clustering) = self.update_clustering(&clustering, &data,  &means) {
                clustering = new_clustering;
            }
            else {
                break;
            }

            counter += 1;
        }

        let mut centers: Vec<(RgbaColor, f32)> = Vec::new();

        for i in 0..number_of_clusters {
            let cluster_count = clustering.iter().filter(|&n| *n == i).count();
            let cluster_index: usize = i.try_into().expect("Failed convert u32 to usize.");
            let mean = &means[cluster_index];
            let color = RgbaColor::new(mean[0], mean[1], mean[2], mean[3]);
            let weight = cluster_count as f32 / data_size as f32;
            centers.push((color, weight));
        }

        centers
    }

    fn update_means(&self, means: &Vec<Vec<f32>>, data: &Vec<Vec<f32>>, weights: &Vec<u32>, clustering: &Vec<u32>) -> Option<Vec<Vec<f32>>> {

        let mut new_means = means.clone();
        let num_clusters = new_means.len();

        // Sum cluster weights.
        let mut cluster_weights = vec![0u32; num_clusters];

        for (i, weight) in weights.iter().enumerate() {
            let cluster_number = clustering[i];
            let cluster_weight = &mut cluster_weights[cluster_number as usize];
            *cluster_weight += weight;
        }

        if cluster_weights.iter().any(|x| *x == 0) {
            return None;
        }

        // Zero means
        for mean in &mut *new_means {
            for mean_value in mean {
                *mean_value = 0f32;
            }
        }

        // Accumulate means sum
        for (i, weight) in weights.iter().enumerate() {

            let cluster_number = clustering[i];
            let cluster_means = &mut new_means[cluster_number as usize];
            let data_values = &data[i];

            for (j, data_value) in data_values.iter().enumerate() {
                cluster_means[j] += data_value * (*weight as f32);
            }
        }

        // Mean values
        for (i, cluster_weight) in cluster_weights.iter().enumerate() {
            let cluster_means = &mut new_means[i];

            for mean_value in cluster_means {
                *mean_value /= *cluster_weight as f32;
            }
        }

        Some(new_means)
    }

    fn update_clustering(&self, clustering: &Vec<u32>, data: &Vec<Vec<f32>>, means: &Vec<Vec<f32>>) -> Option<Vec<u32>> {

        let num_clusters = means.len();
        let mut changed = false;
        let mut distances = vec![0f32; num_clusters];
        let mut new_clustering = clustering.clone();

        for (data_ix, data_value) in data.iter().enumerate() {
            for (cluster_ix, mean) in means.iter().enumerate() {
                distances[cluster_ix] = self.distance(data_value, mean);
            }

            let new_cluster_id = self.min_index(&distances);

            if new_clustering[data_ix] != new_cluster_id as u32 {
                changed = true;
                new_clustering[data_ix] = new_cluster_id as u32;
            }
        }

        if !changed {
            return None;
        }

        let counts = new_clustering
            .iter()
            .counts();

        if counts.len() < num_clusters {
            return None;
        }

        Some(new_clustering)
    }

    fn distance(&self, point_a: &Vec<f32>, point_b: &Vec<f32>) -> f32 {

        let mut sum_squared_diffs = 0f32;

        for (i, _) in point_a.iter().enumerate() {
            sum_squared_diffs += (point_a[i] - point_b[i]).powf(2f32);
        }

        sum_squared_diffs.sqrt()
    }

    fn min_index(&self, values: &Vec<f32>) -> usize {

        let mut index_of_min_value: usize = 0;
        let mut small_value = values[index_of_min_value];

        for (i, value) in values.iter().enumerate() {
            if *value < small_value {
                small_value = *value;
                index_of_min_value = i;
            }
        }

        index_of_min_value
    }

    fn initialize(&self, histogram: &Vec<HistogramEntry>) -> (Vec<Vec<f32>>, Vec<u32>) {

        let mut data: Vec<Vec<f32>> = Vec::new();
        let mut weights: Vec<u32> = Vec::new();

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