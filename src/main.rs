use std::io;
fn main() {
    let mut c: i128 = 0;
    loop {
        let mut inp = String::new();
        if io::stdin().read_line(&mut inp).unwrap() == 0 {
            break;
        }
        let inp = inp.trim();
        if inp.is_empty() {
            continue;
        }
        let prov = inp.parse::<i128>();
        if prov.is_err() {
            c -= 10_000_000_000_000_000;
            println!("NaN");
            break;
        }
        let inp = prov.unwrap();
        if inp == -1 {
            break;
        }
        if inp <= 0 {
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
