// -- Imports and setup ---
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rv_unit::test_runner)]

use riscv_rt::entry;
use core::panic::PanicInfo;
use rv_unit::Testable;

// -- Custom Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rv_unit::test_panic_handler(TEST_SUITE, info)
}

use embedded_alloc::LlffHeap as Heap;
#[global_allocator]
static HEAP: Heap = Heap::empty();
// -- Run the tests
#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    rv_unit::test_runner(TEST_SUITE);
    loop {}
}

use linked_list::LinkedList;

fn test_push() {
    let mut ll: LinkedList<i32> = linked_list::LinkedList::new();
    ll.push_head(1);
    assert_eq!(ll.len(), 1);
    assert_eq!(ll.pop_head(), Some(1));
    assert_eq!(ll.len(), 0);
    assert_eq!(ll.pop_head(), None);
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

const TEST_SUITE: &[&dyn Testable] =
    &[
        &test_push,
        &test_len,
        &test_ll_eq,
        &test_ll_ne,
    ];
