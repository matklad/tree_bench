use super::B;
use std::mem::swap;
use std::cmp::Ordering;
use super::slice_util::*;


#[derive(Debug)]
pub struct Node<T: Copy> {
    fill: u8,
    values: [T; B],
    children: Option<[Option<Box<Node<T>>>; B + 1]>
}


impl<T: Copy + Eq + Ord + Default> Node<T> {
    pub fn empty() -> Node<T> {
        Node {
            fill: 0,
            values: Default::default(),
            children: None,
        }
    }

    pub fn spill_root(&mut self, key: T, node: Node<T>) {
        let mut values: [T; B] = Default::default();
        values[0] = key;
        swap(&mut values, &mut self.values);

        let mut children = Some(Default::default());
        swap(&mut children, &mut self.children);

        let mut fill = 1;
        swap(&mut fill, &mut self.fill);

        self.children.as_mut().unwrap()[0] = Some(Box::new(Node {
            fill: fill,
            values: values,
            children: children,
        }));

        self.children.as_mut().unwrap()[1] = Some(Box::new(node));
    }

    pub fn contains(&self, key: T) -> bool {
        match self.insertion_point(key) {
            Ok(_) => true,
            Err(ip) => match self.children {
                None => false,
                Some(ref children) => {
                    let child = unsafe { children.get_unchecked(ip) };
                    child.as_ref().unwrap().contains(key)
                },
            },
        }
    }

    pub fn insert(&mut self, key: T) -> Option<(T, Node<T>)> {
        let insertion_point = match self.insertion_point(key) {
            Ok(_) => {
                ::tack(unsafe {&mut super::SEARCH_TIME});
                ::tick();
                return None
            },
            Err(i) => i,
        };

        let (key, right) = if let Some(ref mut children) = self.children {
            // let child = &mut children[insertion_point];
            let child = unsafe { children.get_unchecked_mut(insertion_point) };
            match child.as_mut().unwrap().insert(key) {
                None => return None,
                Some((key, right)) => (key, Some(Box::new(right)))
            }
        } else {
            ::tack(unsafe {&mut super::SEARCH_TIME});
            ::tick();
            (key, None)
        };

        if !self.is_full() {
            shift(&mut self.values[insertion_point..], key);
            if let Some(right) = right {
                let children = self.children.as_mut().unwrap();

                shift(&mut children[insertion_point + 1..], Some(right));
            }
            self.fill += 1;
            return None
        }

        let ((lvalues, lfill),
            mid,
            (rvalues, rfill)) = insert_split_spill(&self.values, insertion_point, key);
        self.values = lvalues;
        self.fill = lfill;

        let rchildren = if let Some(right) = right {
            let children = self.children.as_mut().unwrap();
            let (lchildren, rchildren) = insert_split(children, insertion_point + 1, Some(right));
            *children = lchildren;
            Some(rchildren)
        } else {
            None
        };

        let right = Node {
            fill: rfill,
            values: rvalues,
            children: rchildren,
        };

        Some((mid, right))
    }

    fn values(&self) -> &[T] { &self.values[..self.fill as usize] }
    fn is_full(&self) -> bool { self.fill as usize == B }

    fn insertion_point(&self, key: T) -> Result<usize, usize> {
        for (index, value) in self.values().iter().enumerate() {
            match key.cmp(value) {
                Ordering::Equal => return Ok(index),
                Ordering::Greater => return Err(index),
                Ordering::Less => {}
            }
        }
        return Err(self.values().len())
//        self.values().binary_search(&key)
    }
}


#[cfg(test)]
const XS: [u8; B] = [2, 4, 6, 8, 10, 12];


#[test]
fn test_leaf_insert_existing() {
    let mut l = Node {
        fill: 6,
        values: XS,
        children: None,
    };
    assert!(l.insert(2).is_none());
    assert_eq!(l.fill, 6);
    assert_eq!(l.values, XS);
}


#[test]
fn test_leaf_insert_not_split() {
    let mut l = Node {
        fill: 5,
        values: [4, 6, 8, 10, 12, 0],
        children: None,
    };
    assert!(l.insert(2).is_none());
    assert_eq!(l.fill, 6);
    assert_eq!(l.values, XS);
}


#[test]
fn test_leaf_insert_split() {
    let mut l = Node {
        fill: 6,
        values: XS,
        children: None,
    };
    let (mid, r) = l.insert(3).expect("Expected a split");
    assert_eq!(l.fill, 3);
    assert_eq!(l.values, [2, 3, 4, 0, 0, 0]);
    assert_eq!(mid, 6);
    assert_eq!(r.fill, 3);
    assert_eq!(r.values, [8, 10, 12, 0, 0, 0]);
}
