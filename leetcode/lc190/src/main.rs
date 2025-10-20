/// Reverse bits of a given 32 bits signed integer.

pub fn reverse_bits(n: i32) -> i32 {
    let mut out = 0b0;
    for mask in 0..31 {
        if n & 1 << mask == 1 << mask {
            out |= 1 << (31 - mask);
        }
    }

    out
}

pub fn reverse_bits2(mut n: i32) -> i32 {
    let mut out = 0b0;
    for _ in 0..31 {
        out |= n & 1;
        out = out << 1;
        n = n >> 1;
    }

    out
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(reverse_bits(43261596), 964176192);
        assert_eq!(reverse_bits(2147483644), 1073741822);
        assert_eq!(reverse_bits2(43261596), 964176192);
        assert_eq!(reverse_bits2(2147483644), 1073741822);
    }
}
