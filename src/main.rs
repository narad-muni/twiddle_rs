use twiddler::Twiddle;

#[derive(Twiddle)]
struct x {
    pub a: u8,
    pub b: u16,
}

fn main() {
    let mut z = x {a:1, b:7811};

    z.twiddle();

    println!("{} {}", z.a, z.b);
}