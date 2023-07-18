use crate::color::Distance;
use crate::floss::flosses::Floss;
use image::{buffer::Pixels, Rgba};
use rayon::prelude::*;
use std::cmp::Ordering;

pub fn vote(k: usize, pixels: Pixels<'_, Rgba<u8>>, flosses: Vec<Floss>) -> Vec<Floss> {
    if flosses.len() <= k {
        return flosses;
    }

    let converted_flosses: Vec<(usize, image::Rgba<u8>)> = flosses
        .iter()
        .enumerate()
        .map(|(i, f)| (i, f.color))
        .collect();

    let ballots: Vec<_> = pixels
        .par_bridge()
        .filter_map(|p| make_ballot(p, &converted_flosses))
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

        let min = *tallies.iter().filter(|&&t| t > 0).min().unwrap();
        let all_way_tie = tallies.iter().all(|&t| t == 0 || t == min);
        let surviving_candiates = tallies.iter().filter(|&&t| t > 0).count();
        if all_way_tie || surviving_candiates <= k {
            for (is_eliminated, count) in eliminated.iter_mut().zip(tallies) {
                if count == 0 {
                    *is_eliminated = true;
                }
            }
            break;
        }

        let mut remaining = 0;
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

fn make_ballot(pixel: &Rgba<u8>, flosses: &[(usize, Rgba<u8>)]) -> Option<Vec<usize>> {
    if pixel.0[3] == 0 {
        // Fully transparent pixels don't get a vote.
        return None;
    }

    let mut measured: Vec<_> = flosses
        .iter()
        .map(|(i, floss_color)| (i, pixel.distance(floss_color)))
        .collect();
    measured.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));
    Some(measured.drain(..).map(|m| *m.0).collect())
}

fn cast_ballot(mut tally: Vec<u32>, ballot: &[usize], eliminated: &[bool]) -> Vec<u32> {
    let index = *ballot.iter().find(|&&i| !eliminated[i]).unwrap();
    tally[index] += 1;
    tally
}

fn merge_tallies(tally: Vec<u32>, other: Vec<u32>) -> Vec<u32> {
    tally.iter().zip(other.iter()).map(|(a, b)| a + b).collect()
}
