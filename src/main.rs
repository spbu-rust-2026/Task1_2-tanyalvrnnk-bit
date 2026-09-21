use std::io;
fn main() {
    let mut c: i128 = 0;
    loop {
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).unwrap();
        let prov = inp.trim().parse::<i128>();
        if prov.is_err() {
            c -= 10_000_000_000_000_000;
            println!("NaN");
            break;
        }
        let inp = prov.unwrap();
        if inp == -1 {
            break;
        }
        if inp < 0 {
            c -= 10_000_000_000_000_000;
            println!("NaN");
            break;
        }
        c += inp;
    }
    if c >= 0 {
        println!("{}", c);
    }
}
