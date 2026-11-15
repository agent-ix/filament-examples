use quire_canonical::{read, sha256, to_vec, FixedShape, Limits, Writer};

#[derive(serde::Serialize, FixedShape)]
struct Example {
    b: f64,
    a: &'static str,
}

fn text(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).unwrap()
}

fn main() {
    // Two spellings of the same JSON value give the same bytes and digest.
    let first = read(br#"{"b": 2.50, "a": 1.0}"#, 1024).unwrap();
    let second = read(br#"{"a": 1, "b": 2.5e0}"#, 1024).unwrap();
    let limits = Limits::new(1024);
    println!("first : {}", text(&to_vec(&first, limits).unwrap()));
    println!("second: {}", text(&to_vec(&second, limits).unwrap()));
    println!("first  digest: {}", sha256(&first, limits).unwrap());
    println!("second digest: {}", sha256(&second, limits).unwrap());

    // A typed value: members sort, 1e21 prints as ECMAScript does.
    let typed = to_vec(&Example { b: 1e21, a: "ö" }, limits).unwrap();
    println!("typed : {}", text(&typed));

    // Push events yourself; the writer refuses events out of order.
    let mut bytes = Vec::new();
    let mut writer = Writer::new(&mut bytes, Limits::new(64));
    writer.begin_object().unwrap();
    writer.name("b").unwrap();
    writer.begin_array().unwrap();
    writer.integer(1).unwrap();
    writer.null().unwrap();
    writer.end_array().unwrap();
    writer.name("a").unwrap();
    writer.string("ö").unwrap();
    writer.end_object().unwrap();
    let written = writer.finish().unwrap();
    println!("writer: {} ({written} bytes)", text(&bytes));

    // A malformed input is refused with the byte offset; an integer past 2^53 is refused.
    println!("malformed: {:?}", read(b"{\"a\": }", 1024).err());
    let mut over = Vec::new();
    let mut writer = Writer::new(&mut over, Limits::new(64));
    println!("2^53 + 1: {:?}", writer.integer(9_007_199_254_740_993).err());
}
