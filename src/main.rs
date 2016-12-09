extern crate rand;
extern crate time;


use std::time::Instant;
use std::collections::BTreeSet;
use btree::BTree;
use rbtree::RBTree;


mod btree;
mod rbtree;


static mut TMP: u64 = 0;


pub fn tick() {
    unsafe { TMP = time::precise_time_ns() }
}


pub fn tack(timer: &mut u64) {
    unsafe { *timer += time::precise_time_ns() - TMP }
}


trait IntSet {
    fn empty() -> Self;
    fn get(&self, key: i64) -> bool;
    fn put(&mut self, key: i64);
    fn report() {}
}


impl IntSet for BTreeSet<i64> {
    fn empty() -> Self { BTreeSet::new() }

    fn get(&self, key: i64) -> bool { self.contains(&key) }

    fn put(&mut self, key: i64) { self.insert(key); }
}


#[cfg(test)]
fn compare_test<A, B>() where A: IntSet, B: IntSet {
    let mut atree = A::empty();
    let mut btree = B::empty();

    const N: usize = 100_000;

    for _ in 0..N {
        let q = rand::random::<usize>();
        let r = rand::random::<i64>() % 10_000;
        if q % 3 == 0 {
            atree.put(r);
            btree.put(r);
        } else {
            assert_eq!(atree.get(r), btree.get(r));
        }
    }
}


fn main() {
    const N: usize = 1_000_000;
    const M: i64 = 100_000_000;
    let to_insert = random_vec(N, M);
    let to_lookup = random_vec(100_000, M);
    benchmark::<BTreeSet<i64>>("std::collections::BTreeSet", &to_insert, &to_lookup);
    benchmark::<BTree<i64>>("BTree", &to_insert, &to_lookup);
    benchmark::<RBTree<i64>>("RBTree", &to_insert, &to_lookup);
}


fn random_vec(size: usize, range: i64) -> Vec<i64> {
    let mut result = Vec::with_capacity(size);
    for _ in 0..size {
        result.push(rand::random::<i64>() % range);
    }
    result
}


fn timeit<T, F: FnOnce() -> T>(message: &str, f: F) -> T {
    println!("{}: ", message);
    let now = Instant::now();
    let r = f();
    let duration = now.elapsed();
    let nanos = duration.subsec_nanos() as u64 + duration.as_secs() * 1_000_000_000;
    let micros = nanos / 1000;
    if micros < 10 {
        println!("{:?} ns", nanos);
    } else if micros < 10000 {
        println!("{:?} µs\n", micros);
    } else {
        println!("{:?} ms\n", micros / 1_000);
    }
    r
}


fn benchmark<A: IntSet>(name: &str, to_insert: &[i64], to_lookup: &[i64]) {
    let mut tree = A::empty();

    timeit(&format!("Inserting {} random keys into {}", to_insert.len(), name), || {
        for &key in to_insert {
            tree.put(key);
        }
    });
    A::report();

    let mut hash = 0;
    timeit(&format!("Retrieving {} random keys", to_lookup.len()), || {
        for &key in to_lookup {
            hash += if tree.get(key) { 1 } else { 0 };
        }
    });

    println!("hash = {}\n\n", hash);
}


