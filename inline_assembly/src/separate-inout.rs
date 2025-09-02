use std::arch::asm;

fn main(){
let x: u64 = 3;
let mut y: u64 = 5;
unsafe {
    asm!("add {0}, {0}, 5", inout(reg) x => y);
}
assert_eq!(y, 8);
println!("{y}")

}