use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u32 = rng.gen_range(0..4095);

    println!("{}", format_args!("{:04o}", n1));
}
