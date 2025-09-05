#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "compiler")]
pub mod compiler;

#[cfg(feature = "runtime")]
pub mod runtime;

// avoid reference conflicts. 
// TODO: needed by non-native env, needs furnish..
#[cfg(not(feature = "std"))]
#[no_mangle]
pub extern "C" fn _fini() -> ! {
    loop {}
}

#[cfg(not(feature = "std"))]
#[no_mangle]
static end: u8 = 0;
