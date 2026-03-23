use std::ptr;

unsafe fn dangerous_function() {
    let x = 42;
    let raw_ptr = &x as *const i32;
    let _value = *raw_ptr;
}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    
    unsafe {
        println!("r1 is: {}", *r1);
    }
    
    let address = 0x012345usize;
    let _r = unsafe {
        &*(address as *const i32)
    };
}

unsafe trait UnsafeTrait {
    fn do_something(&self);
}

unsafe impl UnsafeTrait for i32 {
    fn do_something(&self) {
        println!("{}", self);
    }
}

extern "C" {
    fn external_function();
}