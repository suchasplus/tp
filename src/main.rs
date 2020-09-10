fn main() {
    let mode = std::env::args().nth(1).expect("no mode given");
    let val = std::env::args().nth(2).expect("no val given");
    match mode.as_str() {
        "dec" => println!(
            "{}",
            std::char::from_u32(val.to_string().parse::<u32>().unwrap()).unwrap()
        ),
        "hex" => println!(
            "{}",
            std::char::from_u32(u32::from_str_radix(&val.to_string(), 16).unwrap()).unwrap()
        ),
        _ => panic!("mode error"),
    }
}
