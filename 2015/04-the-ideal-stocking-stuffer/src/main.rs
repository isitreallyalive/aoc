use openssl::hash::{MessageDigest, hash};

#[aoc::solution(
    "The lowest positive number that produces a hash that starts with 00000 is {part1:?}.",
    "The lowest positive number that produces a hash that starts with 000000 is {part2:?}"
)]
fn main() {
    let (mut part1, mut part2) = (None, None);
    let mut i = 1;
    loop {
        let digest = hash(MessageDigest::md5(), format!("{INPUT}{}", i).as_bytes())
            .expect("should be hashable");

        // check for first five
        if part1.is_none() && hash_starts_with_zeros(&digest, 5) {
            part1 = Some(i);
        }

        // check for first six
        if part2.is_none() && hash_starts_with_zeros(&digest, 6) {
            part2 = Some(i);
        }

        // break if both solutions have been found
        if part1.is_some() && part2.is_some() {
            break;
        }

        i += 1;
    }
}

fn hash_starts_with_zeros(hash: &[u8], count: usize) -> bool {
    let full = count / 2;
    let remaining_nibble = count % 2;

    // check full zero bytes
    for &byte in &hash[..full.min(hash.len())] {
        if byte != 0 {
            return false;
        }
    }

    // check remaining nibble if needed
    if remaining_nibble == 1 && full < hash.len() {
        hash[full] & 0xF0 == 0
    } else {
        full <= hash.len()
    }
}
