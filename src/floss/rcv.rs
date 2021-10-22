use crate::color::{Color, Rgb};
use crate::floss::flosses::Floss;
use rayon::prelude::*;
use std::cmp::Ordering;

pub fn vote<'a, C>(k: usize, pixels: &[C], flosses: Vec<Floss<'a>>) -> Vec<Floss<'a>>
where
    C: Color + From<Rgb> + Sync + Send + 'static,
{
    if flosses.len() <= k {
        return flosses;
    }

    let converted_flosses: Vec<(usize, C)> = flosses
        .iter()
        .enumerate()
        .map(|(i, f)| (i, f.color.into()))
        .collect();

    let ballots: Vec<_> = pixels
        .par_iter()
        .map(|p| make_ballot(p, &converted_flosses))
        .collect();

    let floss_count = flosses.len();
    let mut eliminated: Vec<bool> = vec![false; floss_count];

    loop {
        let tallies: Vec<u32> = ballots
            .par_iter()
            .fold(
                || vec![0; floss_count],
                |tally, ballot| cast_ballot(tally, ballot, &eliminated),
            )
            .reduce(|| vec![0; floss_count], merge_tallies);

        let mut remaining = 0;
        let min = *tallies.iter().filter(|&&t| t > 0).min().unwrap();
        for (is_eliminated, count) in eliminated.iter_mut().zip(tallies) {
            if count <= min {
                *is_eliminated = true;
            } else if !*is_eliminated {
                remaining += 1;
            }
        }

        if remaining <= k {
            break;
        }
    }

    flosses
        .iter()
        .zip(eliminated.iter())
        .filter_map(
            |(floss, &is_eliminated)| {
                if is_eliminated {
                    None
                } else {
                    Some(*floss)
                }
            },
        )
        .collect()
}

fn make_ballot<C>(pixel: &C, flosses: &[(usize, C)]) -> Vec<usize>
where
    C: Color,
{
    let mut measured: Vec<_> = flosses
        .iter()
        .map(|(i, floss_color)| (i, pixel.dist(floss_color)))
        .collect();
    measured.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
    measured.drain(..).map(|m| *m.0).collect()
}

fn cast_ballot(mut tally: Vec<u32>, ballot: &[usize], eliminated: &[bool]) -> Vec<u32> {
    let index = *ballot.iter().find(|&&i| !eliminated[i]).unwrap();
    tally[index] += 1;
    tally
}

fn merge_tallies(tally: Vec<u32>, other: Vec<u32>) -> Vec<u32> {
    tally.iter().zip(other.iter()).map(|(a, b)| a + b).collect()
}
