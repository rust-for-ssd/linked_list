#![no_std]
#![no_main]

#[cfg(feature = "rv_test")]
mod test_ll;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    
    #[cfg(feature = "rv_test")]
    return rv_unit::test_panic_handler(test_ll::test_linked_list::get_test_suite(), info);

    hprintln!("{}", info);
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

use riscv_semihosting::{debug, hprintln};
use riscv_rt::entry;

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

    #[cfg(feature = "rv_test")]
    {
        // Run the test suite from example module
        rv_unit::test_runner(test_ll::test_linked_list::get_test_suite());
    }

    hprintln!("THIS IS CARGO RUN WITHOUT FLAG!");
    debug::exit(debug::EXIT_SUCCESS);
    loop { }
}
