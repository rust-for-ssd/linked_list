use linked_list::LinkedList;
use rv_unit::Testable;

fn test_push() {
    let mut ll: LinkedList<i32> = linked_list::LinkedList::new();
    ll.push_head(1);
    assert_eq!(ll.len(), 1);
    assert_eq!(ll.pop_head(), Some(1));
    assert_eq!(ll.len(), 0);
}

fn test_ll_eq() {
    let mut ll_1: LinkedList<i32> = linked_list::LinkedList::new();
    let mut ll_2: LinkedList<i32> = linked_list::LinkedList::new();
    ll_1.push_head(1);
    ll_2.push_head(1);
    assert_eq!(ll_1, ll_2);
    ll_1.push_tail(2);
    ll_2.push_tail(2);
    assert_eq!(ll_1, ll_2);

    let mut ll_1: LinkedList<i32> = linked_list::LinkedList::new();
    let mut ll_2: LinkedList<i32> = linked_list::LinkedList::new();
    ll_1.push_head(1);
    ll_2.push_tail(1);
    assert_eq!(ll_1, ll_2);
}

fn test_ll_ne() {
    let mut ll_1: LinkedList<i32> = linked_list::LinkedList::new();
    let mut ll_2: LinkedList<i32> = linked_list::LinkedList::new();
    ll_1.push_head(1);
    ll_2.push_head(2);
    assert_ne!(ll_1, ll_2);
}

fn test_len() {
    let mut ll: LinkedList<i32> = linked_list::LinkedList::new();
    assert_eq!(ll.len(), 0);
    ll.push_head(1);
    assert_eq!(ll.len(), 1);
    ll.push_head(2);
    assert_eq!(ll.len(), 2);
    ll.push_tail(3);
    assert_eq!(ll.len(), 3);
    ll.pop_head();
    assert_eq!(ll.len(), 2);
    ll.pop_tail();
    assert_eq!(ll.len(), 1);
    ll.pop_tail();
    assert_eq!(ll.len(), 0);
    ll.pop_tail();
    assert_eq!(ll.len(), 0);
}

pub fn get_test_suite() -> &'static [&'static dyn Testable] {
    &[
        &test_push,
        &test_len,
        &test_ll_eq,
        &test_ll_ne,
    ]
}
