use std::collections::HashSet;
use std::hash::Hash;
use rand::prelude::*;

fn main() {
    let l = 300;
    let k = 1;
    let mut rng = thread_rng();
    let vals: Vec<u32> = (0..l).map(|_| rng.gen_range(0..l)).collect();
    println!("Hello, world! {:?}", is_k_diverse(vals, k));
    //TODO: Compute the longest k-diverse sequence by updating vals (removing items)
}

fn is_k_diverse(iter: Vec<u32>, k: usize) -> bool
{
    let groups: Vec<&[u32]> = iter.chunks(k).collect();
    groups.into_iter().all(|x| has_unique_elements(x))
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
