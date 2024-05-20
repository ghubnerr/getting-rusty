#![allow(dead_code)]

#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>); // self.0

impl<T: PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None) // Provide type annotation for LinkedList
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }
    pub fn push_back(&mut self, data: T) {
        match self.0 {
            // this means the first field of the struct
            Some((_, ref mut child)) => {
                let child: &mut LinkedList<T> = child;
                child.push_back(data)
            }
            None => self.0 = Some((data, Box::new(LinkedList(None)))),
        }
    }
}

use std::cell::RefCell;
// refcell hides the mutability, immutable outside, but can mutate interior
use std::rc::{Rc, Weak};
// reference counter pointer | Weak lets you drop something so yo don't have cyclical loops

pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>, // holds a non-owning reference to the managed allocation. (can drop if there are pointers)
}

pub struct DoublyLinkedList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            last: None,
            first: None,
        }
    }

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next: Some(r.clone()),
                    prev: None,
                }));

                // tell the first object htis is now in front of it
                let mut m = r.borrow_mut(); // this is a method from refcell but rc implements deref
                m.prev = Some(Rc::downgrade(&new_front));
                // put this in the front

                self.first = Some(new_front);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                let new_back = Rc::new(RefCell::new(DbNode {
                    data,
                    prev: Some(r.clone()),
                    next: None,
                }));

                // tell the first object this is now in front of it
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut(); // this is a method from refcell but rc implements deref
                                             // put this in the front
                self.last = Some(Rc::downgrade(&new_back));
                m.next = Some(new_back);
            }
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            }
        }
    }
}
