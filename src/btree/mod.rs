mod slice_util;
mod node;


use self::node::Node;
use super::IntSet;


const B: usize = 6;

pub static mut SEARCH_TIME: u64 = 0;
pub static mut FIXUP_TIME: u64 = 0;

#[derive(Debug)]
pub struct BTree<T: Copy + Default + Ord + Eq> {
    root: Node<T>,
}


impl<T: Copy + Default + Ord + Eq + ::std::fmt::Debug> BTree<T> {
    pub fn new() -> BTree<T> { BTree { root: Node::empty() } }
    pub fn contains(&self, key: T) -> bool { self.root.contains(key) }
    pub fn insert(&mut self, key: T) {
        ::tick();
        match self.root.insert(key) {
            None => {},
            Some((key, right)) => self.root.spill_root(key, right)
        }
        ::tack(unsafe { &mut FIXUP_TIME })
    }
}


impl IntSet for BTree<i64> {
    fn empty() -> Self { BTree::new() }

    fn get(&self, key: i64) -> bool { self.contains(key) }

    fn put(&mut self, key: i64) { self.insert(key) }

    fn report(){
        unsafe {
            println!("Search: {} µs\nFixup:  {} µs\n", SEARCH_TIME / 1000, FIXUP_TIME / 1000);
            SEARCH_TIME = 0;
            FIXUP_TIME = 0;
        }
    }
}


#[test]
fn btree_vs_btreeset() {
    use std::collections::BTreeSet;
    super::compare_test::<BTreeSet<i64>, BTree<i64>>();
}


