use std::arch::asm;


fn main() {

    nop();
    asm_output();
    asm_input_output();
    asm_inout();
    asm_inout_x_y();
    mul_shift_adds();
    println!("Hello, world!");
}

fn nop() {
    unsafe {
        asm!("nop");
    }
    println!("nop");

}

fn asm_output(){
    let x: u64;
    unsafe {
        asm!("mov x2, 5", out("x2") x); // move value 5 into x2 register,
                                        // then assign value of x2 register
                                        // to x
    }
    println!("asm output");
    assert_eq!(x, 5);
    println!("{x}");
}

fn asm_input_output(){
    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, {0}, 5",
            out(reg) o,
            in(reg) i,
        );
    }
    println!("asm input output");
    assert_eq!(o, 8);
    println!("o:{}, i:{}", o, i);

}

fn asm_inout(){
    let mut x: u64 = 3;
    unsafe {
        asm!("add {0}, {0}, 5", inout(reg) x);
    }
    assert_eq!(x, 8);
    println!("asm inout");
    println!("x:{}", x);
}

fn asm_inout_x_y(){
    let x: u64 = 3;
    let y: u64;
    unsafe {
        asm!("add {0}, {0}, 5", inout(reg) x => y);
    }
    assert_eq!(y, 8);
    println!("asm inout x => y");
    println!("{}, {}", x, y);
}

fn mul_shift_adds(){
    // Multiply x by 6 using shifts and adds
    let mut x: u64 = 4;
    unsafe {
        asm!(
            "mov x1, {x}",
            "lsl x1, x1, 1",
            "lsl {x}, {x}, 2",
            "add {x}, {x}, x1",
            x = inout(reg) x,
            //tmp = out(reg) _,
        );
    }
    assert_eq!(x, 4 * 6);
    println!("mul_shift_adds");
    println!("x:{x}");
}