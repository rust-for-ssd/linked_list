#![no_std]
#![no_main]


extern crate alloc;
use alloc::boxed::Box;


#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize
}

type Link<T> = Option<*mut Node<T>>;

#[derive(Debug)]
pub struct Node<T> {
    prev: Link<T>,
    next: Link<T>,
    val: T
}

impl <T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0
        }
    }

    pub fn push_head(&mut self, val: T) {
        let node_ptr: *mut Node<T> = Box::into_raw(Box::new(Node {prev: None, next: None, val}));
        if let Some(head) = self.head {
            unsafe {
            (*head).prev = Some(node_ptr);
            (*node_ptr).next = Some(head);
            }
        } else {
            self.tail = Some(node_ptr);
        }
        self.head = Some(node_ptr);
        self.len += 1;
    }

    pub fn push_tail(&mut self, val: T) {
        let node_ptr: *mut Node<T> = Box::into_raw(Box::new(Node {prev: None, next: None, val}));
        if let Some(tail) = self.tail {
            unsafe {
            (*tail).next = Some(node_ptr);
            (*node_ptr).prev = Some(tail);
            }
        } else {
            self.head = Some(node_ptr);
        }
        self.tail = Some(node_ptr);
        self.len += 1;
    }

    pub fn pop_head(&mut self) -> Option<T> {
        if let Some(head) = self.head {
            let head = unsafe {Box::from_raw(head)};
            self.head = (*head).next;
            self.len -= 1;
            if self.len == 0 {
                self.tail = None;
            }
            Some((*head).val)
        } else {
            None
        }
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        if let Some(tail) = self.tail {
            let tail = unsafe {Box::from_raw(tail)};
            self.tail = (*tail).prev;
            self.len -= 1;
            if self.len == 0 {
                self.head = None;
            }
            Some((*tail).val)
        } else {
            None
        }
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
