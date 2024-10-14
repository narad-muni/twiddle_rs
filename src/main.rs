use std::time::Instant;

use twiddler::Twiddle;

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
struct X {
    pub a: [u8;2],
    pub b: u16,
    pub x: f64,
    pub z: [f64;2],
    pub y: Y,
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
struct Y {
    pub a: u8,
    pub b: u16,
    pub arr: [u16; 2],
    pub zrr: [Z; 2],
}

#[derive(Debug, Twiddle, Clone, Copy)]
#[repr(C, packed(2))]
struct Z {
    a :u8,
    b: u16,
}

#[derive(Debug, Twiddle, Clone, Copy)]
enum E {
    A(X),
    B(Y),
}

fn main() {
    let mut z = X {a:[1;2], b:7810, x: 2f64, z: [1.05f64, 1.05f64], y: Y {
        a: 2,
        b: 33566,
        arr: [7811, 1092],
        zrr: [Z {a: 123, b: 33566}, Z {a: 30, b: 7811}],
    }};

    let mut e = E::A(z);

    e.twiddle();

    println!("{:?}\n{:?}", e, z);

    // println!("{z:?} {:?}", start.elapsed());
}
