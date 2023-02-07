use astar_rna_fold::{fold_dp, random};

fn main() {
    //let a = "GGGCUAUUAGCUCAGUUGGUUAGAGCGCACCCCUGAUAAGGGUGAGGUCGCUGAUUCGAAUUCAGCAUAGCCCA".as_bytes();
    let a = random(2000);
    //for a in std::io::stdin().lines() {
    //let a = a.unwrap().to_ascii_uppercase();
    println!("Cost: {}", fold_dp(&a));
    //}
}
