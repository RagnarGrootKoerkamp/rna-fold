use std::cmp::min;

use itertools::Itertools;
use rand::Rng;

/// An owned sequence.
pub type Sequence = Vec<u8>;
/// A sequence slice.
pub type Seq<'a> = &'a [u8];

pub type Cost = usize;

const ALPH: [u8; 4] = ['A' as u8, 'C' as u8, 'G' as u8, 'U' as u8];

/// Function that determines whether two characters can be matched.
fn complement(a: u8, b: u8) -> bool {
    match (a as char, b as char) {
        ('C', 'G') | ('G', 'C') | ('A', 'U') | ('U', 'A') => true,
        _ => false,
    }
}

pub fn random(n: usize) -> Sequence {
    //let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(2 as u64);
    let mut rng = rand::thread_rng();
    (0..n).map(|_| ALPH[rng.gen_range(0..4)]).collect()
}

/// The standard cubic DP algorithm.
pub fn fold_dp(a: Seq) -> Cost {
    let dp = fold_dp_matrix(a);
    return dp[0][a.len() - 1];
}

type DP = Vec<Vec<usize>>;

pub fn fold_dp_matrix(a: &[u8]) -> DP {
    let n = a.len();
    println!("n: {n}");
    // dp[i][j]: the cost of folding i..=j
    // min: dp[i][i]
    // max: dp[0][n-1]
    let mut dp = vec![vec![0 as Cost; n + 1]; n + 1];
    for i in 1..=n {
        dp[i][i - 1] = 0;
    }
    for j in 0..n {
        for i in (0..=j).rev() {
            // i is not matched
            dp[i][j] = 1 + dp[i + 1][j];
            // i is matched to k
            for k in i + 1..=j {
                if complement(a[i], a[k]) {
                    dp[i][j] = min(dp[i][j], dp[i + 1][k - 1] + dp[k + 1][j]);
                }
            }
        }
    }
    for i in 0..n {
        for j in (i + 1..n).rev() {
            print!("{}", if complement(a[i], a[j]) { '\\' } else { ' ' })
        }
        println!();
    }
    for i in 0..n {
        for j in (i + 1..n).rev() {
            print!(
                "{}",
                if dp[i][j] == 1 + dp[i + 1][j] || dp[i][j] == 1 + dp[i][j - 1] {
                    ' '
                } else {
                    '+'
                }
            )
        }
        println!();
    }
    for i in 0..n {
        for x in dp[i][i..n].iter().rev() {
            print!("{x:2} ")
        }
        println!();
    }
    analyze_matrix(a, &dp);
    dp
}

pub fn analyze_matrix(a: Seq, dp: &DP) {
    use plotters::prelude::*;
    println!("image..");

    let n = a.len();

    let root_area = BitMapBackend::new("img/scatter.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_all_label_area_size(40)
        .build_cartesian_2d(0..n, 0. ..n as f64 / 10.)
        .unwrap();

    ctx.configure_mesh().disable_mesh().draw().unwrap();

    let mut data: Vec<_> = (0..n)
        .flat_map(|i| (i..n).map(move |j| (i, j)))
        .map(|(i, j)| (j - i, dp[i][j]))
        .collect();
    data.sort();

    ctx.draw_series(
        data.iter()
            .map(|&(len, cost)| Circle::new((len, cost as f64), 2, &RED)),
    )
    .unwrap();

    let avgs = data
        .iter()
        .group_by(|&l| l.0)
        .into_iter()
        .map(|(l, rs)| -> (usize, f64) {
            let rs = rs.collect_vec();
            let avg = rs.iter().map(|&(_, c)| c).sum::<usize>() as f64 / rs.len() as f64;
            (l, avg)
        })
        .collect_vec();

    ctx.draw_series(LineSeries::new(avgs, &BLUE)).unwrap();
    ctx.draw_series(LineSeries::new([(0, 0.), (n, dp[0][n - 1] as f64)], &GREEN))
        .unwrap();
}
