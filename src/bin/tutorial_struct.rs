fn main() {
    let s = String::from("hello world");
    let word = first_word(&s[..]);
    println!("The first word is: {}", word);

    let user = build_user(String::from(""), String::from(""));
    println!(
        "username: {}, email: {}, sign_in_count: {}, active: {}",
        user.get_username(),
        user.email,
        user.sign_in_count,
        user.active
    );
    let user2 = User {
        email: String::from("a"),
        username: String::from("b"),
        ..user
    };
    println!(
        "username: {}, email: {}, sign_in_count: {}, active: {}",
        user2.get_username(),
        user2.email,
        user2.sign_in_count,
        user2.active
    );

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    let v8 = IpAddr::V8 {
        a: 127,
        b: 0,
        c: 0,
        d: 1,
    };
    route(&home);
    route(&loopback);
    route(&v8);
    route2(&home);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return s;
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn get_username(&self) -> &String {
        &self.username
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
    V8 { a: u8, b: u8, c: u8, d: u8 },
}

fn route(ip_type: &IpAddr) {
    match ip_type {
        IpAddr::V4(a, b, c, d) => println!("{} {} {} {}", a, b, c, d),
        IpAddr::V6(s) => println!("{}", s),
        IpAddr::V8 { a, b, c, d } => println!("{} {} {} {}", a, b, c, d),
    }
}

fn route2(ip_type: &IpAddr) {
    if let IpAddr::V4(a, b, c, d) = ip_type {
        println!("{} {} {} {}", a, b, c, d);
    } else {
        println!("not v4");
    }
}
