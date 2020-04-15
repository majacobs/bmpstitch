use crate::color::{Color, Rgb};
use crate::floss::flosses::Floss;
use std::collections::HashMap;

pub fn reduce_to_known<'a, C>(
    k: usize,
    points: &Vec<C>,
    mut flosses: Vec<Floss<'a>>,
) -> Vec<Floss<'a>>
where
    C: Color + From<Rgb>,
{
    if flosses.len() <= k {
        return flosses;
    }

    let mut flosses: Vec<(_, C)> = flosses.drain(..).map(|f| (f, f.color.into())).collect();

    loop {
        println!("Down to {} colors", flosses.len());

        let mut measured_flosses = iterate(flosses, points);
        if measured_flosses.len() <= k {
            return measured_flosses.iter().map(|f| (f.0).0).collect();
        }

        measured_flosses.sort_by_key(|f| f.1);
        let (_worst, remaining) = measured_flosses.split_first().unwrap();
        flosses = remaining.iter().map(|f| f.0).collect();
    }
}

fn iterate<'a, C>(flosses: Vec<(Floss<'a>, C)>, points: &Vec<C>) -> Vec<((Floss<'a>, C), usize)>
where
    C: Color + From<Rgb>,
{
    let mut all_distances = HashMap::new();

    for point in points.iter() {
        let mut closest = std::usize::MAX;
        let mut closest_distance = std::f32::MAX;

        for (i, (_, color)) in flosses.iter().enumerate() {
            let d = point.dist(color);
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

        const DISTANCE: f32 = 25.0;
        for other in right.iter_mut().rev() {
            if (x.0).1.dist(&(other.0).1) <= DISTANCE {
                x.1 += other.1;
                other.1 = 0;
                break;
            }
        }
    }

    centers_with_count
}
