use std::{
    cmp::Ordering,
    collections::{
    HashMap,
    HashSet,
}};

use rand::prelude::*;

use crate::img_utils::{
    histogram::Histogram,
    rgb_color::RgbColor,
    color_point::ColorPoint,
};

struct ClusterEntry {
    histogram_point: ColorPoint,
    cluster_number: u32,
    weighted_colors: Vec<u32>,
}

impl ClusterEntry {
    pub fn new(histogram_point: ColorPoint, cluster_number: u32) -> Self {
        Self {
            histogram_point,
            cluster_number,
            weighted_colors: {
                histogram_point.color.to_vec().iter().map(|value| {
                    (*value as u32) * histogram_point.weight as u32
                })
                .collect::<Vec<_>>()
            },
        }
    }
}

pub fn cluster(histogram: &Histogram,
    number_of_clusters: u32,
    max_try_count: u32) -> Vec<ColorPoint> {

    let mut cluster_data = init_cluster_data(histogram, number_of_clusters as usize);

    match cluster_data.len() < 1 {
        true => return Vec::new(),
        false => (),
    }

    let mut cluster_centers = allocate_centers(number_of_clusters);
    let mut try_counter: u32 = 0_u32;

    while try_counter < max_try_count {

        match calc_cluster_centers(&cluster_data, number_of_clusters) {
            Some(new_cluster_centers) => {
                cluster_centers = new_cluster_centers;
            }
            None => break,
        }

        match calc_data_clusters(&cluster_data, &cluster_centers) {
            Some(data_clusters) => {
                // update cluster numbers.
                cluster_data.iter_mut().enumerate().for_each(|(i, entry)| {
                    entry.cluster_number = data_clusters[i];
                })
            }
            None => break,
        }

        try_counter += 1;
    }

    let mut centers: Vec<ColorPoint> = Vec::new();
    let total_number_of_points = cluster_data.len();

    cluster_centers.iter().enumerate().for_each(|(cluster_number, center)| {
        let number_of_points_in_cluster = cluster_data.iter().filter(|&entry| entry.cluster_number == cluster_number as u32).count();
        let weight = number_of_points_in_cluster as f32 / total_number_of_points as f32;

        match RgbColor::from_f32_vec(center) {
            Ok(color) => {
                centers.push(ColorPoint::new(color, weight));
            },
            Err(err) => println!("Unable to get Rgb color for cluster_number: {cluster_number}. Err: {err}"),
        };
    });

    centers
}

fn calc_cluster_centers(cluster_data: &Vec<ClusterEntry>, number_of_clusters: u32) -> Option<Vec<Vec<f32>>> {

    // Sum cluster weights.
    // And accumulate cluster centers sum.
    let mut cluster_weights = HashMap::new();
    let mut cluster_centers = allocate_centers(number_of_clusters as u32);

    cluster_data.iter().for_each(|cluster_entry| {
        let cluster_number = &cluster_entry.cluster_number;

        // Sum cluster weights.
        let cluster_weight = cluster_weights.entry(cluster_number).or_insert(0_f32);
        *cluster_weight += &cluster_entry.histogram_point.weight;

        // Accumulate cluster centers sum.
        let cluster_center = &mut cluster_centers[*cluster_number as usize];
        cluster_center.iter_mut().enumerate().for_each(|(i, center)| {
            *center += cluster_entry.weighted_colors[i] as f32;
        });
    });

    // Mean cluster centers.
    cluster_centers.iter_mut().enumerate().for_each(|(cluster_number, cluster_center)| {

        match cluster_weights.get(&(cluster_number as u32)) {
            Some(cluster_weight) => {
                cluster_center.iter_mut().for_each(|center| {
                    *center /= cluster_weight;
                });
            },
            _ => (),
        }
    });

    Some(cluster_centers)
}

fn calc_data_clusters(cluster_data: &Vec<ClusterEntry>, cluster_centers: &Vec<Vec<f32>>) -> Option<Vec<u32>> {

    let mut new_data_clusters = cluster_data.iter().map(|entry| { entry.cluster_number }).collect::<Vec<_>>();
    let mut changed = false;
    let mut distances = vec![0_f32; cluster_centers.len()];

    cluster_data.iter().enumerate().for_each(|(entry_ix, entry)| {

        // for each point we calc distances to each cluster center.
        cluster_centers.iter().enumerate().for_each(|(cluster_number, center)| {
            distances[cluster_number] = distance(&entry.histogram_point.color.to_f32_vec(), center);
        });

        // pick up cluster with minimum distance to it.
        let new_cluster_number = min_distance_index(&distances);

        // apply new cluster number to point.
        match entry.cluster_number != new_cluster_number as u32 {
            true => {
                // note cluster changed.
                changed = true;
                new_data_clusters[entry_ix] = new_cluster_number as u32;
            }
            false => (),
        }
    });

    match !changed {
        true => return None,
        false => (),
    }

    // calc new number of clusters in data.
    let counts: HashSet<&u32> = HashSet::from_iter(new_data_clusters.iter());

    // if lost any cluster then return None.
    match counts.len() < cluster_centers.len() {
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

fn min_distance_index(values: &Vec<f32>) -> usize {

    let mut index_of_min_value: usize = 0;
    let mut small_value = values[index_of_min_value];

    values.iter().enumerate().for_each(|(i, value)| {
        match value.total_cmp(&small_value) {
            Ordering::Less => {
                small_value = *value;
                index_of_min_value = i;
            },
            _ => (),
        }
    });

    index_of_min_value
}

fn allocate_centers(num_clusters: u32) -> Vec<Vec<f32>> {
    let num_dimentions = RgbColor::dim();
    vec![vec![0_f32; num_dimentions]; num_clusters as usize]
}

fn init_cluster_data(histogram: &Histogram,
    number_of_clusters: usize) -> Vec<ClusterEntry> {

    let vec = histogram.to_vec();

    match vec.len() < 1 {
        true => return Vec::new(),
        false => (),
    }

    let mut rng = rand::thread_rng();

    let cluster_data = vec.iter().enumerate().map(|(i, color_point)| {

        let cluster_number = match i < number_of_clusters {
            true => i,
            false => rng.gen_range(0..number_of_clusters),
        };

        ClusterEntry::new(
            color_point.clone(),
            cluster_number as u32)

    })
    .collect::<Vec<_>>();

    cluster_data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_success() {

        let point_a = vec![1.0; 4];
        let point_b = vec![3.0; 4];
        let actual = distance(&point_a, &point_b);
        assert_eq!(actual, 4.0);

        let point_a = vec![-1.0; 4];
        let point_b = vec![-3.0; 4];
        let actual = distance(&point_a, &point_b);
        assert_eq!(actual, 4.0);

        let point_a = vec![-1.0; 4];
        let point_b = vec![1.0; 4];
        let actual = distance(&point_a, &point_b);
        assert_eq!(actual, 4.0);
    }

    #[test]
    fn min_distance_index_success() {
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let actual = min_distance_index(&values);
        assert_eq!(actual, 0);

        let values = vec![2.0, 3.0, 4.0, 1.0];
        let actual = min_distance_index(&values);
        assert_eq!(actual, 3);

        let values = vec![2.0, 3.0, 1.0, 4.0];
        let actual = min_distance_index(&values);
        assert_eq!(actual, 2);
    }
}
