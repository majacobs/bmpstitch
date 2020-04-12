const MIN_ATTEMPTS: usize = 1_000;
const ITERATIONS: usize = 5;
const MIN_DISTANCE: f32 = 800.0;

use rand::Rng;
use crate::color::{Color, Rgb};

pub trait Coord: Sized + Clone {
    fn rand() -> Self;
    fn mean(coords: &Vec<&Self>) -> Self;
    fn distance(&self, other: &Self) -> f32;
}

pub fn run_kmeans<P>(k: usize, points: &Vec<P>) -> Vec<P>
where
    P: Coord,
{
    let mut best_centers = Vec::new();
    let mut best_distance = std::f32::MAX;

    for attempt in 0.. {
        if best_distance < std::f32::MAX && attempt >= MIN_ATTEMPTS {
            break;
        }

        if attempt % 100 == 0 {
            println!("Attempt #{}", attempt);
        }

        let mut centers: Vec<P> = (0..k).map(|_| Coord::rand()).collect();
        let mut distance = std::f32::MAX;
        let mut dead_end = false;

        for _ in 0..ITERATIONS {
            let results = iterate_kmeans(&centers, &points);
            centers = results.0;
            distance = results.1;
            let too_close = are_centers_too_close(&centers);
            if centers.len() < k || too_close {
                dead_end = true;
                break;
            }
        }

        if dead_end || distance > best_distance {
            continue;
        }

        best_centers = centers;
        best_distance = distance;
        println!(
            "Found a good mean (attempt #{}, distance={})",
            attempt, best_distance
        );
    }

    best_centers
}

fn iterate_kmeans<P>(centers: &Vec<P>, points: &Vec<P>) -> (Vec<P>, f32)
where
    P: Coord,
{
    let mut buckets = Vec::new();
    for _ in 0..centers.len() {
        buckets.push(Vec::new());
    }

    for point in points.iter() {
        let mut closest = std::usize::MAX;
        let mut closest_distance = std::f32::MAX;

        for (i, c) in centers.iter().enumerate() {
            let d = c.distance(&point);
            if d < closest_distance {
                closest = i;
                closest_distance = d;
            }
        }
        buckets[closest].push(point);
    }

    let mut new_centers = Vec::new();
    let mut total_distance = 0f32;
    for bucket in buckets.iter() {
        let count = bucket.len() as u16;
        if count == 0 {
            continue;
        }

        let avg = Coord::mean(bucket);

        total_distance += bucket.iter().map(|x| avg.distance(x)).sum::<f32>();
        new_centers.push(avg);
    }
    (new_centers, total_distance)
}

fn are_centers_too_close<P>(centers: &Vec<P>) -> bool
where
    P: Coord,
{
    if centers.len() == 1 {
        return false;
    }

    for (i, c1) in centers.iter().enumerate() {
        for c2 in centers.iter().skip(i + 1) {
            if c1.distance(c2) < MIN_DISTANCE {
                return true;
            }
        }
    }

    false
}

impl Coord for (u8, u8, u8) {
    fn rand() -> Self {
        let mut rng = rand::thread_rng();
        (rng.gen(), rng.gen(), rng.gen())
    }

    fn mean(coords: &Vec<&Self>) -> Self {
        let count = coords.len() as f32;
        let sum = coords.iter().fold((0.0, 0.0, 0.0), |sum, x| {
            (
                sum.0 + f32::from(x.0),
                sum.1 + f32::from(x.1),
                sum.2 + f32::from(x.2),
            )
        });

        (
            (sum.0 / count) as u8,
            (sum.1 / count) as u8,
            (sum.2 / count) as u8,
        )
    }

    fn distance(&self, other: &Self) -> f32 {
        let diff0 = self.0 as f32 - other.0 as f32;
        let diff1 = self.1 as f32 - other.1 as f32;
        let diff2 = self.2 as f32 - other.2 as f32;
        let sum = diff0.powi(2) + diff1.powi(2) + diff2.powi(2);
        sum
        //sum.sqrt()
    }
}

impl Coord for Rgb {
    fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Rgb::new(rng.gen(), rng.gen(), rng.gen())
    }

    fn mean(coords: &Vec<&Self>) -> Self {
        let count = coords.len() as f32;
        let sum = coords.iter().fold((0.0, 0.0, 0.0), |sum, x| {
            (
                sum.0 + f32::from(x.r),
                sum.1 + f32::from(x.g),
                sum.2 + f32::from(x.b),
            )
        });

        Rgb::new
        (
            (sum.0 / count) as u8,
            (sum.1 / count) as u8,
            (sum.2 / count) as u8
        )
    }

    fn distance(&self, other: &Self) -> f32 {
        self.dist(other)
        //let diff0 = self.0 as f32 - other.0 as f32;
        //let diff1 = self.1 as f32 - other.1 as f32;
        //let diff2 = self.2 as f32 - other.2 as f32;
        //let sum = diff0.powi(2) + diff1.powi(2) + diff2.powi(2);
        //sum
        //sum.sqrt()
    }
}
