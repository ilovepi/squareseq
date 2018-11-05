#![feature(test)]
#![feature(extern_crate_item_prelude)]

extern crate test;

use std::collections::{BTreeMap, BTreeSet};
use std::f64;
use time::PreciseTime;

type Set = BTreeSet<i32>;
type Map = BTreeMap<Set, Option<Vec<Vec<i32>>>>;

// not important, just for prototyping
fn expand(x: &mut Set) -> Vec<Vec<i32>> {
    if (x.len()) == 1 {
        let inner: Vec<i32> = x.clone().into_iter().collect();
        let mut tmp = Vec::new();
        tmp.push(inner);
        return tmp;
    }
    let mut ret = Vec::new();

    for item in x.clone() {
        x.remove(&item);
        let remainder = expand(x);
        for mut v in remainder {
            v.insert(0, item);
            ret.push(v);
        }
        x.insert(item);
    }

    return ret;
}

fn is_perfect_sq(x: i32) -> bool {
    let d = x as f64;
    let sq = d.sqrt();
    let fl = sq.floor();
    let diff = sq - fl;
    0.0 == diff
}

// for benchmarking
fn set_perfect(x: i32, map: &BTreeSet<i32>) -> bool {
    map.contains(&x)
}

// for benchmarking
fn map_perfect(x: i32, map: &BTreeMap<i32, i32>) -> bool {
    map.contains_key(&x)
}

// for prototyping
fn all_perfect(v: &Vec<i32>) -> bool {
    for i in 0..v.len() - 1 {
        for j in 1..v.len() {
            if !is_perfect_sq(v[i] + v[j]) {
                return false;
            }
        }
    }
    return true;
}

// for prototyping
fn all_pairs(v: Vec<i32>) -> Vec<(i32, i32)> {
    let mut ret = Vec::new();
    for i in 0..v.len() - 1 {
        for j in i + 1..v.len() {
            if is_perfect_sq(v[i] + v[j]) {
                ret.push((v[i], v[j]));
                ret.push((v[j], v[i]));
            }
        }
    }
    ret
}

fn make_map(set: &Set, map: &mut Map, count: &mut i32) {
    // if we already have a solution, exit query early
    if map.contains_key(set) {
        return;
    }
    *count = *count + 1;

    // Base Case
    let len = set.len();
    let mut solution_list = Vec::new();
    let mut found = false;
    match len {
        // don/t care about sets smaller than 2, mark as None
        0 | 1 => {},
        // this is our base case, make sure we get those values
        2 => {
            if is_perfect_sq(set.into_iter().sum()) {
                let mut tmp: Vec<i32> = set.into_iter().cloned().collect();
                solution_list.push(tmp.clone());
                tmp.reverse();
                solution_list.push(tmp);
                found = true;
            }
        }
        // the main way we calculate/memoize the results
        _ => {
            let mut local_set = set.clone();

            for item in set {
                local_set.remove(&item);
                let key = local_set.clone();
                make_map(&key, map, count);
                if let Some(Some(solution)) = map.get(&key) {
                    for vector in solution {
                        if is_perfect_sq(item + vector[0]) {
                            found = true;
                            let mut new_solution = vector.clone();
                            new_solution.insert(0, item.clone());
                            solution_list.push(new_solution);
                        }
                    }
                }

                local_set.insert(item.clone());
            }
        }
    }

    // at the end insert the result from this query
    if !found {
        map.insert(set.clone(), None);
    } else {
        map.insert(set.clone(), Some(solution_list));
    }
}

fn main() {
    let mut x: Set = Set::new();
    let mut map = Map::new();
    for n in 1..16 {
        x.insert(n);
        let mut count = 0;
        let start = PreciseTime::now();
        make_map(&x, &mut map, &mut count);
        let end = PreciseTime::now();
        println!("{:?}", map.get(&x));
        println!("Size of Map: {:?}", map.len());
        println!("Number of Additional Calculations: {:?}", count);
        println!("Elapsed Time: {:?}", start.to(end));
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::Bencher;
    const MAX: i32 = 32000;

    #[bench]
    fn float_is_sq(b: &mut Bencher) {
        let bench = |max: i32| {
            for n in 1..max {
                is_perfect_sq(n);
            }
        };

        b.iter(|| bench(MAX));
    }

    #[bench]
    fn btreeset_is_sq(b: &mut Bencher) {
        let mut set = Set::new();
        for n in 1..MAX {
            let sq = n * n;
            set.insert(sq);
        }

        let bench = |max: i32, set: &Set| {
            for n in 1..max {
                set_perfect(n, &set);
            }
        };

        b.iter(|| bench(MAX, &set));
    }

    #[bench]
    fn btreemap_is_sq(b: &mut Bencher) {
        let mut set = BTreeMap::new();
        for n in 1..MAX {
            let sq = n * n;
            set.insert(n, sq);
        }

        let bench = |max: i32, set: &BTreeMap<i32, i32>| {
            for n in 1..max {
                map_perfect(n, &set);
            }
        };

        b.iter(|| bench(MAX, &set));
    }

}
