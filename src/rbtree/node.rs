use std::cmp::Ordering;
use std::ptr;
use std::mem::swap;


#[derive(Debug)]
pub struct Node<T> {
    pub parent: *mut Node<T>,
    is_red: bool,
    key: T,
    left: *mut Node<T>,
    right: *mut Node<T>,
}


impl<T: Ord> Node<T> {
    pub fn new_root(key: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Node {
            is_red: false,
            key: key,
            parent: ptr::null_mut(),
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }))
    }

    pub fn contains(&self, key: T) -> bool {
        let child = match key.cmp(&self.key) {
            Ordering::Equal => return true,
            Ordering::Less => self.left,
            Ordering::Greater => self.right,
        };

        if child.is_null() {
            return false;
        }

        unsafe { (*child).contains(key) }
    }

    pub fn insert(&mut self, key: T) {
        let this: *mut Node<T> = self;
        let child = match key.cmp(&self.key) {
            Ordering::Equal => {
                ::tack(unsafe { &mut super::SEARCH_TIME });
                return
            },
            Ordering::Less => &mut self.left,
            Ordering::Greater => &mut self.right,
        };


        if child.is_null() {
            ::tack(unsafe { &mut super::SEARCH_TIME });
            *child = Node::new_leaf(this, key);
            unsafe { fixup(*child) };
        } else {
            unsafe { (**child).insert(key) };
        }
    }

    fn new_leaf(parent: *mut Node<T>, key: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Node {
            is_red: true,
            key: key,
            parent: parent,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
        }))
    }
}


fn is_red<T>(u: *const Node<T>) -> bool {
    !u.is_null() && unsafe { (*u).is_red }
}


unsafe fn fixup<T>(mut u: *mut Node<T>) {
    ::tick();
    loop {
        debug_assert!({ (*u).is_red });
        let mut w = { (*u).parent };
        if w.is_null() {
            (*u).is_red = false;
            break;
        }

        if !is_red((*w).left) {
            flip_left(w);
            u = w;
            w = (*u).parent;
        }

        if !is_red(w) {
            break;
        }

        let g = (*w).parent;
        debug_assert!(!is_red(g));

        if !is_red((*g).right) {
            flip_right(g);
            break;
        }
        flip(g);
        u = g;
    }
    ::tack(unsafe { &mut super::FIXUP_TIME });
}


unsafe fn flip<T>(u: *mut Node<T>) {
    (*u).is_red = !((*u).is_red);
    (*(*u).left).is_red = !((*(*u).left).is_red);
    (*(*u).right).is_red = !((*(*u).right).is_red);
}


unsafe fn flip_left<T>(u: *mut Node<T>) {
    let right = (*u).right;
    swap_colors(u, right);
    swap_child(u, right);
    (*u).parent = right;
    (*u).right = (*right).left;
    if !(*u).right.is_null() {
        (*((*u).right)).parent = u;
    };
    (*right).left = u;
}


unsafe fn flip_right<T>(u: *mut Node<T>) {
    let left = (*u).left;
    swap_colors(u, left);
    swap_child(u, left);
    (*u).parent = left;
    (*u).left = (*left).right;
    if !(*u).left.is_null() {
        (*((*u).left)).parent = u;
    };
    (*left).right = u;
}


unsafe fn swap_colors<T>(u: *mut Node<T>, v: *mut Node<T>) {
    swap(&mut (*u).is_red, &mut (*v).is_red);
}


unsafe fn swap_child<T>(old: *mut Node<T>, new: *mut Node<T>) {
    let parent = (*old).parent;
    (*new).parent = parent;
    if parent.is_null() { return; }
    let slot = if (*parent).left == old {
        &mut (*parent).left
    } else {
        debug_assert!((*parent).right == old);
        &mut (*parent).right
    };
    *slot = new;
}


#[cfg(test)]
#[allow(unused)]
fn graph_vis<T: ::std::fmt::Display>(u: *const Node<T>) -> String {
    unsafe {
        let mut result = format!("{} [color=\"{}\"];\n",
                                 (*u).key,
                                 if (*u).is_red { "red" } else { "black" });

        for &child in &[(*u).left, (*u).right] {
            if !child.is_null() {
                result += &format!("{} -> {};\n", (*u).key, (*child).key);
                result += &format!("{} -> {};\n", (*child).key, (*(*child).parent).key);
                result += &graph_vis(child);
            }
        }
        result
    }
}


#[cfg(test)]
fn check_invariant<T>(u: *const Node<T>) -> usize {
    unsafe {
        if u.is_null() {
            return 1
        }
        let parent = (*u).parent;
        if parent.is_null() {
            assert!(!is_red(u), "Root is not black");
        }

        let left = (*u).left;
        let right = (*u).right;

        assert!(left.is_null() || left != right, "Duplicate child");
        assert!(left.is_null() || (*left).parent as *const Node<T> == u,
        "Wrong parent link on the left");
        assert!(right.is_null() || (*right).parent as *const Node<T> == u,
        "Wrong parent link on the right");

        if is_red(right) {
            assert!(is_red(left), "Not left leaning");
        }

        if is_red(u) {
            assert!(!is_red(left) && !(is_red(right)), "Two consecutive red edges");
        }

        let left_path = check_invariant(left);
        let right_path = check_invariant(right);
        assert_eq!(left_path, right_path, "Different number of black nodes");
        return left_path + if is_red(u) { 0 } else { 1 }
    }
}


#[test]
fn test_create_root() {
    let n = Node::new_root(92);
    check_invariant(n);
    unsafe {
        assert_eq!((*n).key, 92);
    }
}


#[test]
fn test_insert() {
    let n = Node::new_root(1);
    unsafe {
        (*n).insert(0);
        assert_eq!((*(*n).left).key, 0);
        check_invariant(n);
        (*n).insert(2);
        assert_eq!( (*(*n).right).key, 2);
        check_invariant(n);
    }
}


#[test]
fn test_insert_same() {
    let n = Node::new_root(92);
    unsafe {
        (*n).insert(92);
        check_invariant(n);
        assert_eq!((*n).left, ptr::null_mut());
        assert_eq!((*n).right, ptr::null_mut());
    }
}


#[test]
fn test_insert_left() {
    let n = Node::new_root(5);
    unsafe {
        (*n).insert(4);
        let left = &mut *(*n).left;
        assert_eq!(left.key, 4);
        check_invariant(n);
        (*n).insert(3);
        let left_left = &*left.left;
        assert_eq!(left.key, 4);
        assert_eq!(left_left.key, 3);
        check_invariant(n);
        assert!(left.parent.is_null());
        assert!((*n).parent == left);
        assert!((*left.left).is_red);
        assert!((*left.right).is_red);

        left.insert(2);
        assert!(left.parent.is_null());
        check_invariant(left);
    }
}


#[test]
fn test_insert_right() {
    let n = Node::new_root(5);
    unsafe {
        (*n).insert(6);
        let root = &mut *(*n).parent;
        assert_eq!((*n).key, 5);
        assert_eq!(root.key, 6);
        assert!(root.parent.is_null());
        check_invariant(root);
        root.insert(7);
        assert!(root.parent.is_null());
        check_invariant(root);

        root.insert(8);
        assert!(root.parent.is_null());
        check_invariant(root);
        assert_eq!(root.key, 6);
        assert_eq!((*root.left).key, 5);
        assert_eq!((*root.right).key, 8);
        assert_eq!((*(*root.right).left).key, 7);
    }
}


#[cfg(test)]
fn check_inserts(keys: &[u32]) {
    unsafe {
        let mut root = Node::new_root(keys[0]);
        for &k in &keys[1..] {
            (*root).insert(k);
            if !(*root).parent.is_null() {
                root = (*root).parent;
            }
            assert!((*root).parent.is_null());
            check_invariant(root);
        }

        Box::from_raw(root);
    }
}


#[test]
fn many_inserts() {
    check_inserts(&[5, 1, 2, 8, 7, 3]);
}


#[test]
fn random_inserts() {
    use rand;
    let n = rand::random::<usize>() % 100;
    let mut elements = vec![];
    for _ in 0..n {
        elements.push(rand::random::<u32>() % 100)
    }
    check_inserts(&elements);
}
