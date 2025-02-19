#![no_std]
#![no_main]


extern crate alloc;
use alloc::boxed::Box;
use core::ptr::NonNull;
use core::marker::PhantomData;


#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    _phantom: PhantomData<T>
}

type Link<T> = Option<NonNull<Node<T>>>;

pub struct Node<T> {
    prev: Link<T>,
    next: Link<T>,
    val: T
}

impl <T: core::cmp::PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &LinkedList<T>) -> bool { 
        if self.len() != other.len() {
            return false
        }

        let node_1 = self.head;
        let node_2 = other.head;
        for _ in 0..self.len() {

            match (node_1, node_2) {
                (Some(node_1), Some(node_2)) => {
                    let val_1 = unsafe { node_1.read().val};
                    let val_2 = unsafe { node_2.read().val};

                    if val_1 != val_2 {
                        return false;
                    }
                },
                (None, None) => return true,
                _ => return false
            }

        }
        return true;
    }
    

}

impl <T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
            _phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_head(&mut self, val: T) {
        let new_head: NonNull<Node<T>> = 
            NonNull::new(Box::into_raw(Box::new(Node {
                prev: None, 
                next: None, 
                val
            }))).expect("Node ptr is null!");

        if let Some(old_head) = self.head {
            unsafe {
                (*old_head.as_ptr()).prev = Some(new_head);
                (*new_head.as_ptr()).next = Some(old_head);
            }
        } else {
            self.tail = Some(new_head);
        }
        self.head = Some(new_head);
        self.len += 1;
    }

    pub fn push_tail(&mut self, val: T) {
        let new_tail: NonNull<Node<T>> = 
            NonNull::new(Box::into_raw(Box::new(Node {
                prev: None,
                next: None,
                val
            }))).expect("Node ptr is null!");

        if let Some(old_tail) = self.tail {
            unsafe {
                (*old_tail.as_ptr()).next = Some(new_tail);
                (*new_tail.as_ptr()).prev = Some(old_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = Some(new_tail);
        self.len += 1;
    }

    pub fn pop_head_node(&mut self) -> Option<Node<T>> {
        if let Some(old_head) = self.head {
            let new_head = unsafe {old_head.as_ref()}.next;
            self.head = new_head;
            self.len -= 1;
            if let Some(new_head) = new_head {
                unsafe {
                    (*new_head.as_ptr()).prev = None;
                }
            } else {
                self.tail = None;
            }
            let node = unsafe {old_head.read()};
            Some(node)
        } else {
            None
        }
    }

    pub fn pop_head(&mut self) -> Option<T> {
        if let Some(head) = self.pop_head_node() {
            Some(head.val)
        } else {
            None
        }
    }

    pub fn pop_tail_node(&mut self) -> Option<Node<T>> {
        if let Some(old_tail) = self.tail {
            let new_tail = unsafe {old_tail.as_ref()}.prev;
            self.tail = new_tail;
            self.len -= 1;
            if let Some(new_tail) = new_tail {
                unsafe {
                    (*new_tail.as_ptr()).next = None;
                }
            } else {
                self.head = None
            }
            let node = unsafe {old_tail.read()};
            Some(node)
        } else {
            None
        }
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        if let Some(tail) = self.pop_tail_node() {
            Some(tail.val)
        } else {
            None
        }
    }
}
