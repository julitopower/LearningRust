use std::str::FromStr;

fn gcdiv(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {
    println!("Hello there, world!");
    println!("{}", gcdiv(12, 24));
    let mut numbers = Vec::<u64>::new();
    for arg in std::env::args().skip(1) {
        println!("{}", arg);
        numbers.push(u64::from_str(&arg).expect("bad"));
    }

    numbers.push(2);
    numbers.push(u64::from_str("2").expect("whaat"));
    println!("{:?}", numbers);
}

#[test]
fn test_gcdiv() {
    assert_eq!(gcdiv(12, 24), 12);
}
