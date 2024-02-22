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

    match vec.len() < 1 {
        true => return Vec::new(),
        false => (),
    }

    let data_size: u32 = vec.len().try_into().expect("Failed convert usize to u32.");
    let (data, data_weights) = initialize(&vec);

    let mut data_clusters = init_clustering(data_size, number_of_clusters);
    let mut cluster_means = allocate_means(number_of_clusters);
    let mut counter: u32 = 0_u32;

    while counter < max_try_count {

        match update_means(&cluster_means, &data, &data_weights, &data_clusters) {
            Some(new_means) => {
                cluster_means = new_means;
            }
            None => break,
        }

        match update_clustering(&data_clusters, &data, &cluster_means) {
            Some(new_clustering) => {
                data_clusters = new_clustering;
            }
            None => break,
        }

        counter += 1;
    }

    let mut centers: Vec<ColorPoint> = Vec::new();

    (0..number_of_clusters).for_each(|i| {
        let cluster_count = data_clusters.iter().filter(|&n| *n == i).count();
        let cluster_index: usize = i.try_into().expect("Failed convert u32 to usize.");
        let mean = &cluster_means[cluster_index];
        let color = RgbaColor::new(mean[0], mean[1], mean[2], mean[3]);
        let weight = cluster_count as f32 / data_size as f32;
        centers.push(ColorPoint::new(color, weight));
    });

    centers
}

fn update_means(cluster_means: &Vec<Vec<f32>>,
    data: &Vec<Vec<f32>>,
    data_weights: &Vec<f32>,
    data_clusters: &Vec<u32>) -> Option<Vec<Vec<f32>>> {

    let number_of_clusters = cluster_means.len();

    // Sum cluster weights.
    let mut cluster_weights = vec![0_f32; number_of_clusters];

    data_weights.iter().enumerate().for_each(|(i, weight)| {
        let cluster_number = data_clusters[i];
        let cluster_weight = &mut cluster_weights[cluster_number as usize];
        *cluster_weight += weight;
    });

    match cluster_weights.iter().any(|x| is_zero(x)) {
        true => return None,
        false => (),
    }

    // Zero means
    let mut new_cluster_means = allocate_means(number_of_clusters as u32);

    // Accumulate means sum
    data_weights.iter().enumerate().for_each(|(i, weight)| {
        let cluster_number = data_clusters[i];
        let cluster_means = &mut new_cluster_means[cluster_number as usize];
        let data_values = &data[i];

        for (j, data_value) in data_values.iter().enumerate() {
            cluster_means[j] += data_value * (*weight as f32);
        }
    });

    // Mean values
    cluster_weights.iter().enumerate().for_each(|(i, cluster_weight)| {
        let cluster_means = &mut new_cluster_means[i];

        for mean_value in cluster_means {
            *mean_value /= *cluster_weight as f32;
        }
    });

    Some(new_cluster_means)
}

fn update_clustering(data_clusters: &Vec<u32>,
    data: &Vec<Vec<f32>>,
    cluster_means: &Vec<Vec<f32>>) -> Option<Vec<u32>> {

    let num_clusters = cluster_means.len();
    let mut changed = false;
    let mut distances = vec![0_f32; num_clusters];
    let mut new_data_clusters = data_clusters.clone();

    data.iter().enumerate().for_each(|(data_ix, data_value)| {
        cluster_means.iter().enumerate().for_each(|(cluster_ix, mean)| {
            distances[cluster_ix] = distance(data_value, mean);
        });

        let new_cluster_ix = min_index(&distances);

        match data_clusters[data_ix] != new_cluster_ix as u32 {
            true => {
                changed = true;
                new_data_clusters[data_ix] = new_cluster_ix as u32;
            }
            false => (),
        }
    });

    match !changed {
        true => return None,
        false => (),
    }

    let counts = new_data_clusters.iter().counts();

    match counts.len() < num_clusters {
        true => return None,
        false => (),
    }

    Some(new_data_clusters)
}

fn distance(point_a: &Vec<f32>, point_b: &Vec<f32>) -> f32 {
    let mut sum_squared_diffs = 0_f32;

    point_a.iter().enumerate().for_each(|(i, _)| {
        sum_squared_diffs += (point_a[i] - point_b[i]).powf(2_f32);
    });

    sum_squared_diffs.sqrt()
}

fn min_index(values: &Vec<f32>) -> usize {

    let mut index_of_min_value: usize = 0;
    let mut small_value = values[index_of_min_value];

    values.iter().enumerate().for_each(|(i, value)| {
        if *value < small_value {
            small_value = *value;
            index_of_min_value = i;
        }
    });

    index_of_min_value
}

fn initialize(histogram: &Vec<ColorPoint>) -> (Vec<Vec<f32>>, Vec<f32>) {

    let mut data: Vec<Vec<f32>> = Vec::new();
    let mut weights: Vec<f32> = Vec::new();

    histogram.iter().for_each(|entry| {
        data.push(entry.color.to_vec());
        weights.push(entry.weight);
    });

    (data, weights)
}

fn init_clustering(size: u32, number_of_clusters: u32) -> Vec<u32> {

    let mut rng = rand::thread_rng();

    let clustering: Vec<u32> = (0..size)
        .map(|x| {
            match x < number_of_clusters {
                true => x,
                false => rng.gen_range(0..number_of_clusters),
            }
        })
        .collect();

    clustering
}

fn allocate_means(num_clusters: u32) -> Vec<Vec<f32>> {
    let num_dimentions = RgbaColor::get_size();
    vec![vec![0_f32; num_dimentions as usize]; num_clusters as usize]
}

fn is_zero(value: &f32) -> bool {
    match 0_f32.total_cmp(value) {
        Ordering::Equal => true,
        _ => false
    }
}
