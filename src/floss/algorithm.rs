use crate::floss::flosses::Floss;
use std::collections::HashMap;

pub fn reduce_to_known<'a>(
    k: usize,
    points: &Vec<(u8, u8, u8)>,
    mut flosses: Vec<Floss<'a>>,
) -> Vec<Floss<'a>> {
    if flosses.len() <= k {
        return flosses;
    }

    loop {
        println!("Down to {} colors", flosses.len());

        let mut measured_flosses = iterate(flosses, points);
        if measured_flosses.len() <= k {
            return measured_flosses.iter().map(|f| f.0).collect();
        }

        measured_flosses.sort_by_key(|f| f.1);
        let (_worst, remaining) = measured_flosses.split_first().unwrap();
        flosses = remaining.iter().map(|f| f.0).collect();
    }
}

fn iterate<'a>(flosses: Vec<Floss<'a>>, points: &Vec<(u8, u8, u8)>) -> Vec<(Floss<'a>, usize)> {
    let mut all_distances = HashMap::new();

    for point in points.iter() {
        let mut closest = std::usize::MAX;
        let mut closest_distance = std::u32::MAX;

        for (i, c) in flosses.iter().enumerate() {
            let d = distance_p(&c, &point);
            if d < closest_distance {
                closest = i;
                closest_distance = d;
            }
        }
        let distances = all_distances.entry(closest).or_insert(0usize);
        *distances += 1;
    }

    let mut centers_with_count: Vec<_> = all_distances
        .iter()
        .map(|(index, count)| (flosses[*index], *count))
        .collect();

    centers_with_count.sort_by_key(|c| std::usize::MAX - c.1);

    for i in 1..centers_with_count.len() {
        let (left, right) = centers_with_count.split_at_mut(i);
        let mut x = left.last_mut().unwrap();
        if x.1 == 0usize {
            continue;
        }

        const DISTANCE: u32 = 25;
        for other in right.iter_mut().rev() {
            if distance_f(&x.0, &other.0) <= DISTANCE {
                x.1 += other.1;
                other.1 = 0;
                break;
            }
        }
    }

    centers_with_count
}

fn distance_p(floss: &Floss, point: &(u8, u8, u8)) -> u32 {
    let diff0 = floss.red as f32 - point.0 as f32;
    let diff1 = floss.green as f32 - point.1 as f32;
    let diff2 = floss.blue as f32 - point.2 as f32;
    let sum = diff0.powi(2) + diff1.powi(2) + diff2.powi(2);
    sum.sqrt() as u32
}

fn distance_f(a: &Floss, b: &Floss) -> u32 {
    let diff0 = a.red as f32 - b.red as f32;
    let diff1 = a.green as f32 - b.green as f32;
    let diff2 = a.blue as f32 - b.blue as f32;
    let sum = diff0.powi(2) + diff1.powi(2) + diff2.powi(2);
    sum.sqrt() as u32
}
