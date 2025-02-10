use clap::Parser;
use num_bigint::BigUint;
use num_traits::One;

fn main() {
    let args = Args::parse();
    let p: BigUint = args.p.parse().expect("Ensure argument p is an integer");
    let q: BigUint = args.q.parse().expect("Ensure argument q is an integer");

    let (n, e, d) = generate_keys(p, q);

    println!("Public Key");
    println!("n: {}", n);
    println! {"e: {}\n", e};
    println!("Private Key");
    println!("d: {}", d);
}

fn generate_keys(p: BigUint, q: BigUint) -> (BigUint, BigUint, BigUint) {
    let n = &p * &q;
    let phi_n = (p - 1u8) * (q - 1u8);
    let e = select_e(&phi_n);
    let d = e.modinv(&phi_n).unwrap();

    (n, e, d)
}

fn select_e(phi_n: &BigUint) -> BigUint {
    let mut e = phi_n - BigUint::one();
    loop {
        e -= BigUint::one();

        if gcd(&e, phi_n) == BigUint::one() {
            return e;
        }
    }
}

fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if a == &BigUint::ZERO {
        b.clone()
    } else {
        gcd(&(b % a), a)
    }
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

#[cfg(test)]
mod test {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn test_gcd1() {
        let b: BigUint = "13019230129310921231231244".parse().unwrap();
        let a: BigUint = "102312039".parse().unwrap();
        assert_eq!(BigUint::from(3u8), gcd(&a, &b));
    }

    #[test]
    fn test_gcd2() {
        let b: BigUint = "2064".parse().unwrap();
        let a: BigUint = "2221".parse().unwrap();
        assert_eq!(BigUint::from(1u8), gcd(&a, &b));
    }

    #[test]
    fn test_key_gen1() {
        let p: BigUint = 3u8.into();
        let q: BigUint = 11u8.into();
        let (n, e, d) = generate_keys(p, q);

        assert_eq!(BigUint::from(33u8), n);
        assert_eq!(BigUint::from(17u8), e);
        assert_eq!(BigUint::from(13u8), d);
    }

    #[test]
    fn test_key_gen2() {
        let p: BigUint = 59u8.into();
        let q: BigUint = 17u8.into();
        let (n, e, d) = generate_keys(p, q);

        assert_eq!(BigUint::from(1003u16), n);
        assert_eq!(BigUint::from(925u16), e);
        assert_eq!(BigUint::from(309u16), d);
    }
    #[test]
    fn test_key_gen3() {
        let p: BigUint = 7907u64.into();
        let q: BigUint = 7717u64.into();
        let (n, e, d) = generate_keys(p, q);

        assert_eq!(BigUint::from(61018319u64), n);
        assert_eq!(BigUint::from(61002691u64), e);
        assert_eq!(BigUint::from(12200539u64), d);
    }
}
