use clap::Parser;
use num_bigint::BigInt;

fn main() {
    let args = Args::parse();

    println!("p: {}, q: {}", args.p, args.q);
}

#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    /// a prime number
    #[arg(short)]
    p: String,
    /// a different prime number
    #[arg(short)]
    q: String,
}

fn gcd(a: BigInt, b: BigInt) -> BigInt {
    if a == BigInt::ZERO {
        b
    } else {
        gcd(b % &a, a)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use num_bigint::BigInt;

    #[test]
    fn test_gcd1() {
        let b: BigInt = "13019230129310921231231244".parse().unwrap();
        let a: BigInt = "102312039".parse().unwrap();
        assert_eq!(BigInt::from(3), gcd(a, b));
    }

    #[test]
    fn test_gcd2() {
        let b: BigInt = "2064".parse().unwrap();
        let a: BigInt = "2221".parse().unwrap();
        assert_eq!(BigInt::from(1), gcd(a, b));
    }
}
