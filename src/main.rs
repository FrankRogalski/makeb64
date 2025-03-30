use rand::random;

const DIGITS: usize = 120 / 6;

fn main() {
    for _ in 0..10 {
        let num = random::<u128>() & (2u128.pow(DIGITS as u32 * 6) - 1);
        let res = decode(&encode(num));
        println!("{num}, {res}");
    }
}

fn decode(string: &str) -> u128 {
    let mut num: u128 = 0;
    for chr in string.chars().rev() {
        num <<= 6;
        num |= match chr {
            i @ 'A'..='Z' => i as u8 - b'A',
            i @ 'a'..='z' => i as u8 - b'a' + 26,
            i @ '0'..='9' => i as u8 - b'0' + 52,
            '-' => 62,
            '_' => 63,
            _ => unreachable!(
                "unreachable char while decoding '{}'({:X})",
                chr, chr as u32
            ),
        } as u128
    }
    num
}

fn encode(mut num: u128) -> String {
    let mut string = String::with_capacity(DIGITS);
    for _ in 0..DIGITS {
        string.push(match (num & 0b111_111) as u8 {
            i @ 0..26 => (i + b'A') as char,
            i @ 26..52 => (i - 26 + b'a') as char,
            i @ 52..62 => (i - 52 + b'0') as char,
            62 => '-',
            63 => '_',
            _ => unreachable!("how did we get here"),
        });
        num >>= 6;
    }
    string
}
