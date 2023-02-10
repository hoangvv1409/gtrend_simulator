use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("filename required");
    }

    let filename = String::from(args[1].clone());
    let data = gtrend_simulator::read_file(filename).unwrap();

    let result = gtrend_simulator::run(data);

    println!("{:?}", result);
}
