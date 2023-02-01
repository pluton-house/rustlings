// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());

    let b = "blue ribbon";
    println!("{:p}", b);
    let c = b;
    println!("{:p}", c);
    let d = &c[8..=10];
    println!("{:p} {0:}", d);
    let e = "a bit further on";
    println!("{:p}", e);
    let f = "  hello there ".trim();
    println!("{:p} {0:}", f);
    let g = &f.to_string();
    println!("{:p} {0:}", g);
    let j = "he";
    println!("{:p} {0:}", j);
    let j = "me";
    println!("{:p} {0:}", j);
    let fl = &1234.56;
    println!("{:p} {0:}", fl);
    let l = "be";
    println!("{:p} {0:}", l);
}
