use anyhow::Result;
use crate::{input::Input, solution::Solution};

pub fn solve(input: &Input) -> Result<Solution> {
    let input_str = input.get_str();
    let part_1_solution: Option<String> = solve_part_1(&input_str);
    let part_2_solution: Option<String> = solve_part_2(&input_str);
    Ok(Solution {
        part_1: part_1_solution,
        part_2: part_2_solution,
    })
}

fn solve_part_1(input: &str) -> Option<String> {
    let mut counter: u64 = 0;
    loop {
        let mut message: String = String::from(input);
        message.push_str(&counter.to_string());
        let md5_hash:String = get_md5_hash(&message);

        println!("counter: {}; md5: {}", counter, md5_hash);

        if md5_hash.starts_with("00000") {
            break;
        }
        counter += 1;
    }
    Some(counter.to_string())
}

fn solve_part_2(input: &str) -> Option<String> {
    Option::None
}

fn get_md5_hash(message: &str) -> String {

    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;

    let k: Vec<u32> = Vec::from([
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
        0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
        0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
        0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
        0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
        0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
        0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
        0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
        0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
        0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
        0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
        0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
        0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
    ]);

    let s: Vec<u32> = Vec::from([
        7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
        5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
        4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
        6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
    ]);

    let mut message_bytes: Vec<u8> = message.to_string().into_bytes();
    let bitcount: u64 = message_bytes.len().saturating_mul(8) as u64;
    
    message_bytes.push(0x80);
    while message_bytes.len() % 64 != 56 {
        message_bytes.push(0x00);
    }

    message_bytes.extend(&[
        bitcount as u8,
        (bitcount >> 8) as u8,
        (bitcount >> 16) as u8,
        (bitcount >> 24) as u8,
        (bitcount >> 32) as u8,
        (bitcount >> 40) as u8,
        (bitcount >> 48) as u8,
        (bitcount >> 56) as u8,
    ]);

    for chunk in message_bytes.chunks_exact_mut(64) {
        let mut a: u32 = a0;
        let mut b: u32 = b0;
        let mut c: u32 = c0;
        let mut d: u32 = d0;

        for i in 0..64 {
            let mut f: u32;
            let g: u32;
            match i {
                0..=15 => {
                    f = (b & c) | ((!b) & d);
                    g = i;
                } 
                16..=31 => {
                    f = (d & b) | ((!d) & c);
                    g = (5*i + 1) % 16;
                }
                32..=47 => {
                    f = b ^ c ^ d;
                    g = (3*i + 5) % 16;
                }
                48..=63 => {
                    f = c ^ (b | (!d));
                    g = (7*i) % 16;
                }
                _ => {
                    panic!("Unknown value for i");
                }
            }

            let m: u32 = u32::from_le_bytes([
                chunk[(g*4) as usize],
                chunk[(g*4+1) as usize],
                chunk[(g*4+2) as usize],
                chunk[(g*4+3) as usize]
            ]);

            f = f.wrapping_add(a).wrapping_add(k[i as usize]).wrapping_add(m);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(s[i as usize]));
        }
        
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    let md5_hash = format!("{:08x}{:08x}{:08x}{:08x}",
        a0.swap_bytes(), b0.swap_bytes(), c0.swap_bytes(), d0.swap_bytes());
    String::from(md5_hash)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_hash() {
        assert_eq!(String::from("d41d8cd98f00b204e9800998ecf8427e"), get_md5_hash(""));
        assert_eq!(String::from("e4d909c290d0fb1ca068ffaddf22cbd0"), get_md5_hash("The quick brown fox jumps over the lazy dog."));
    }
}
