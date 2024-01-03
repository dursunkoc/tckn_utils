use tckn_utils::{generate, validate};

fn main() {
    let tckn = generate();
    let valid = validate(tckn.as_str());
    println!("TCKN: {tckn} => validate: {valid}");
}
