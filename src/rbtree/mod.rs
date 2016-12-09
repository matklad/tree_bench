use std::ptr;


mod node;


use super::IntSet;
use self::node::Node;


pub static mut SEARCH_TIME: u64 = 0;
pub static mut FIXUP_TIME: u64 = 0;


#[derive(Debug)]
pub struct RBTree<T: Ord> {
    root: *mut Node<T>,
}


impl<T: Ord> Drop for RBTree<T> {
    fn drop(&mut self) {
        if !self.root.is_null() {
            unsafe { Box::from_raw(self.root); }
        }
    }
}


impl<T: Ord> RBTree<T> {
    pub fn new() -> RBTree<T> { RBTree { root: ptr::null_mut() } }

    pub fn contains(&self, key: T) -> bool {
        if self.root.is_null() {
            return false
        }
        let root = unsafe { &(*self.root) };
        root.contains(key)
    }

    pub fn insert(&mut self, key: T) {
        if self.root.is_null() {
            self.root = Node::new_root(key);
            return;
        }
        ::tick();
        unsafe {
            (*self.root).insert(key);
            let parent = (*self.root).parent;
            if !parent.is_null() {
                self.root = parent;
            }
        };
    }
}


impl IntSet for RBTree<i64> {
    fn empty() -> Self { RBTree::new() }

    fn get(&self, key: i64) -> bool { self.contains(key) }

    fn put(&mut self, key: i64) { self.insert(key) }

    fn report() {
        unsafe {
            println!("Search: {} µs\nFixup:  {} µs\n", SEARCH_TIME / 1000, FIXUP_TIME / 1000);
            SEARCH_TIME = 0;
            FIXUP_TIME = 0;
        }
    }
}


#[test]
fn rbtree_vs_btreeset() {
    use std::collections::BTreeSet;
    super::compare_test::<BTreeSet<i64>, RBTree<i64>>();
}

