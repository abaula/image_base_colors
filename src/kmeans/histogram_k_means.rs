use std::cmp::Ordering;
use itertools::Itertools;
use rand::prelude::*;

use crate::{
    histogram::Histogram,
    img_utils::{
        rgba_color::RgbaColor,
        color_point::ColorPoint,
    },
};

pub fn cluster(histogram: &Histogram,
    number_of_clusters: u32,
    max_try_count: u32) -> Vec<ColorPoint> {

    let vec = histogram.get_vec();

    if vec.len() < 1 {
        return Vec::new();
    }

    let data_size: u32 = vec.len().try_into().expect("Failed convert usize to u32.");
    let num_dimentions = RgbaColor::get_size();

    let (data, weights) = initialize(&vec);

    let mut clustering = init_clustering(data_size, number_of_clusters);
    let mut means = allocate_means(number_of_clusters, num_dimentions);
    let mut counter: u32 = 0_u32;

    while counter < max_try_count {
        if let Some(new_means) = update_means(&means, &data, &weights, &clustering) {
            means = new_means;
        } else {
            break;
        }

        if let Some(new_clustering) = update_clustering(&clustering, &data, &means) {
            clustering = new_clustering;
        } else {
            break;
        }

        counter += 1;
    }

    let mut centers: Vec<ColorPoint> = Vec::new();

    for i in 0..number_of_clusters {
        let cluster_count = clustering.iter().filter(|&n| *n == i).count();
        let cluster_index: usize = i.try_into().expect("Failed convert u32 to usize.");
        let mean = &means[cluster_index];
        let color = RgbaColor::new(mean[0], mean[1], mean[2], mean[3]);
        let weight = cluster_count as f32 / data_size as f32;
        centers.push(ColorPoint::new(color, weight));
    }

    centers
}

fn update_means(
    means: &Vec<Vec<f32>>,
    data: &Vec<Vec<f32>>,
    weights: &Vec<f32>,
    clustering: &Vec<u32>,
) -> Option<Vec<Vec<f32>>> {
    let mut new_means = means.clone();
    let num_clusters = new_means.len();

    // Sum cluster weights.
    let mut cluster_weights = vec![0_f32; num_clusters];

    for (i, weight) in weights.iter().enumerate() {
        let cluster_number = clustering[i];
        let cluster_weight = &mut cluster_weights[cluster_number as usize];
        *cluster_weight += weight;
    }

    if cluster_weights.iter().any(|x| is_zero(x)) {
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

fn update_clustering(
    clustering: &Vec<u32>,
    data: &Vec<Vec<f32>>,
    means: &Vec<Vec<f32>>,
) -> Option<Vec<u32>> {
    let num_clusters = means.len();
    let mut changed = false;
    let mut distances = vec![0_f32; num_clusters];
    let mut new_clustering = clustering.clone();

    for (data_ix, data_value) in data.iter().enumerate() {
        for (cluster_ix, mean) in means.iter().enumerate() {
            distances[cluster_ix] = distance(data_value, mean);
        }

        let new_cluster_id = min_index(&distances);

        if new_clustering[data_ix] != new_cluster_id as u32 {
            changed = true;
            new_clustering[data_ix] = new_cluster_id as u32;
        }
    }

    if !changed {
        return None;
    }

    let counts = new_clustering.iter().counts();

    if counts.len() < num_clusters {
        return None;
    }

    Some(new_clustering)
}

fn distance(point_a: &Vec<f32>, point_b: &Vec<f32>) -> f32 {
    let mut sum_squared_diffs = 0_f32;

    for (i, _) in point_a.iter().enumerate() {
        sum_squared_diffs += (point_a[i] - point_b[i]).powf(2_f32);
    }

    sum_squared_diffs.sqrt()
}

fn min_index(values: &Vec<f32>) -> usize {
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

fn initialize(histogram: &Vec<ColorPoint>) -> (Vec<Vec<f32>>, Vec<f32>) {
    let mut data: Vec<Vec<f32>> = Vec::new();
    let mut weights: Vec<f32> = Vec::new();

    for entry in histogram {
        data.push(entry.color.to_vec());
        weights.push(entry.weight);
    }

    (data, weights)
}

fn init_clustering(size: u32, number_of_clusters: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();

    let clustering: Vec<u32> = (0..size)
        .map(|x| {
            if x < number_of_clusters {
                x
            } else {
                rng.gen_range(0..number_of_clusters)
            }
        })
        .collect();

    clustering
}

fn allocate_means(num_clusters: u32, num_dimentions: u32) -> Vec<Vec<f32>> {
    vec![vec![0_f32; num_dimentions as usize]; num_clusters as usize]
}

fn is_zero(value: &f32) -> bool {
    match 0_f32.total_cmp(value) {
        Ordering::Equal => true,
        _ => false
    }
}
