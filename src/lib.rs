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

#[derive(Debug)]
pub struct Node<T> {
    prev: Link<T>,
    next: Link<T>,
    val: T
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_head() { }
    }
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
            (*tail).next = Some(node_ptr);
            (*node_ptr).prev = Some(tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = Some(new_tail);
        self.len += 1;
    }

    pub fn pop_head_node(&mut self) -> Option<Node<T>> {
        let old_head = self.head?;
        let boxed_node = unsafe {Box::from_raw(old_head.as_ptr())};
        let new_head = boxed_node.next;
        self.head = new_head;
        self.len -= 1;
        if let Some(new_head) = new_head {
            unsafe {
                (*new_head.as_ptr()).prev = None;
            }
        } else {
            self.tail = None;
        }
        Some(*boxed_node)
    }

    pub fn pop_head(&mut self) -> Option<T> {
        let head = self.pop_head_node()?;
        Some(head.val)
    }

    pub fn pop_tail_node(&mut self) -> Option<Node<T>> {
        let old_tail = self.tail?;
        let boxed_node = unsafe {Box::from_raw(old_tail.as_ptr())};
        let new_tail = boxed_node.prev;
        self.tail = new_tail;
        self.len -= 1;
        if let Some(new_tail) = new_tail {
            unsafe {
                (*new_tail.as_ptr()).next = None;
            }
        } else {
            self.head = None
        }
        Some(*boxed_node)
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        let tail = self.pop_tail_node()?;
        Some(tail.val)
    }

    pub fn pop_node_head(&mut self) -> Option<Node<T>> {
        if let Some(head) = self.head {
            let res: Node<T> = unsafe {*head.clone()};
            let head = unsafe {Box::from_raw(head)};
            self.head = (*head).next;
            self.len -= 1;
            if let Some(new_head) = self.head {
                unsafe {(*new_head).prev = None;}
            } else {
                self.tail = None;
            }
            Some(res)
        } else {
            None
        }
    }

    pub fn pop_node_tail(&mut self) -> Option<Node<T>> {
        if let Some(tail) = self.tail {
            let tail = unsafe {Box::from_raw(tail)};
            self.tail = (*tail).prev;
            self.len -= 1;
            if self.len == 0 {
                self.head = None;
            }
            Some(*tail)
        } else {
            None
        }
    }

}
