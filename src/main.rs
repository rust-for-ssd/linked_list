#![no_std]
#![no_main]

#[cfg(not(debug_assertions))]
extern crate panic_halt;

#[cfg(debug_assertions)]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    hprintln!("{}", info);
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

use riscv_semihosting::{debug, hprintln, dbg};
use riscv_rt::entry;

use linked_list::{self, LinkedList};



use embedded_alloc::LlffHeap as Heap;
#[global_allocator]
static HEAP: Heap = Heap::empty();


#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
    let mut ll: LinkedList<i32> = linked_list::LinkedList::new();
    dbg!(&ll);
    ll.push_head(1);
    dbg!(&ll);
    let el1 = ll.pop_head();
    dbg!(el1);
    dbg!(&ll);

    ll.push_tail(2);
    dbg!(&ll);
    ll.push_tail(3);
    dbg!(&ll);
    let el1 = ll.pop_node_head();
    dbg!(el1);
    dbg!(&ll);
    let el1 = ll.pop_node_tail();
    dbg!(el1);
    dbg!(&ll);
    debug::exit(debug::EXIT_SUCCESS);
    loop { }
}
