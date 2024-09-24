use twiddler::Twiddle;

#[derive(Twiddle, Debug)]
struct X {
    pub a: u8,
    pub b: u16,
    pub y: Y,
}

#[derive(Twiddle, Debug)]
struct Y {
    pub a: u8,
    pub b: u16,
    pub arr: [u16; 2],
    pub zrr: [Z; 2],
}

#[derive(Twiddle, Debug)]
struct Z {
    a :u8,
    b: u16,
}

fn main() {
    let mut z = X {a:1, b:7811, y: Y {
        a: 2,
        b: 33566,
        arr: [7811, 1092],
        zrr: [Z {a: 123, b: 33566}, Z {a: 30, b: 7811}],
    }};

    z.twiddle();

    println!("{z:?}");
}