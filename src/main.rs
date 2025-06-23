// don't use nothing from the user space
#![no_std]
#![no_main]

// vga module to encapsulate unsafe
mod vga_buffer;

// Bring panic from core
use core::panic::PanicInfo;

// This is how panic works on bare metal
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);    
    loop{}
}

// unsafe = trust me bro; No mangle = maintain the same name; write the string on boot.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hail to King Terry the Terrible{}\n", "!");
    panic!("PUTA QUE PARIU FODEU TUDO AQUI NESSE CARALHO!");   
 loop {}
}
