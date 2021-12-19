/*
    SHA256
    ======
    -- Rust implementation of the Secure Hashing Algorithm.
    |> Specification Sheet: https://csrc.nist.gov/csrc/media/publications/fips/180/2/archive/2002-08-01/documents/fips180-2.pdf
    |> Wikipedia: https://en.wikipedia.org/wiki/SHA-2
*/


use core::{u8, u32, u64};
use std::string::String;
use std::vec::Vec;


const H: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
    0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];


// TODO: The actual function
pub fn sha256(message: String) -> String {
    let mut message_vec = message.bytes().collect::<Vec<u8>>();
    
    /*
        Padding
        =======
        -- Pads the input message for it to be evenly split into 512-Bit Chunks.
    */
    let message_length = message_vec.len() * 8;
    message_vec.push(0x80 as u8);
    while (message_vec.len() * 8 + 64) % 512 != 0 { message_vec.push(0x00 as u8); }
    for b in message_length.to_ne_bytes() { message_vec.push(b); }
    assert_eq!((message_vec.len() * 8) % 512, 0, "Message was not properly padded.");


    /*
        Parsing
        =======
        -- Parse the padded message into 512-Bit Chunks.
    */
    let mut chunks: Vec<Vec<u8>> = Vec::new();
    while message_vec.len() != 0 {
        let mut new_chunk = Vec::with_capacity(64);

        for _ in 0 .. 64 {
            new_chunk.push(message_vec.pop().unwrap());
        }
        
        assert_eq!(new_chunk.len(), 64, "Could not parse message into chunks properly.");

        chunks.push(new_chunk);
    }

    assert_ne!(chunks.len(), 0, "Could not split message into any chunks.");


    /*
        Computation
        ===========
        |> TODO: Finish this section.
    */
    for c in chunks {
        // TODO: Create Message Schedule
        let mut message_schedule = Vec::<u32>::new();
        for t in 0 .. 64 {
            if t <= 15 {
                // message_schedule.push(c[t * 4 : (t * 4) + 4])
            } else {

            }
        }

        assert_eq!(message_schedule.len(), 64, "Could not properly create a message schedule.");

        let mut a = H[0];
        let mut b = H[1];
        let mut c = H[2];
        let mut d = H[3];
        let mut e = H[4];
        let mut f = H[5];
        let mut g = H[6];
        let mut h = H[7];

        for t in 0 .. 64 {
            let t1: u32 = h + sigma_1(e) + ch(e, f, g) + K[t] + message_schedule[t];
            let t2: u32 = sigma_0(a) + maj(a, b, c);

            h = g;
            g = f;
            f = e;
            e = d + t1;
            d = c;
            c = b;
            b = a;
            a = t1 + t2;
        }
    }


    return String::new();
}


/*
    Helper Functions
    ================
    -- Simple functions for ease of use, operations defined in specification sheet.
*/
#[inline(always)]
const fn rotate_right(x: u32, n: u32) -> u32 { (x >> n) | (x << u32::BITS - n) }

#[inline(always)]
const fn rotate_left(x: u32, n: u32) -> u32 { (x << n) | (x >> u32::BITS - n) }

#[inline(always)]
const fn ch(x: u32, y: u32, z: u32) -> u32 { (x & y) ^ (x & z) }

#[inline(always)]
const fn maj(x: u32, y: u32, z: u32) -> u32 { (x & y) ^ (x & z) ^ (y & z) }

#[inline(always)]
const fn sigma_0(x: u32) -> u32 { rotate_right(x, 2) ^ rotate_right(x, 13) ^ rotate_right(x, 22) }

#[inline(always)]
const fn sigma_1(x: u32) -> u32 { rotate_right(x, 6) ^ rotate_right(x, 11) ^ rotate_right(x, 25) }

#[inline(always)]
const fn lc_sigma_0(x: u32) -> u32 { rotate_right(x, 7) ^ rotate_right(x, 18) ^ (x >> 3) }

#[inline(always)]
const fn lc_sigma_1(x: u32) -> u32 { rotate_right(x, 17) ^ rotate_right(x, 19) ^ (x >> 10) }


// #[cfg(test)]
// mod tests {
//     use super::sha256;

//     #[test]
//     fn hash_one() {
//         let to_be_hashed: &str = "Hello, World!";
//         let proper_hash: &str = "DFFD6021BB2BD5B0AF676290809EC3A53191DD81C7F70A4B28688A362182986F";
//         assert_eq!(sha256(to_be_hashed.to_owned()), String::from(proper_hash));
//     }

//     #[test]
//     fn hash_two() {
//         let to_be_hashed: &str = "qwertypassu321";
//         let proper_hash: &str = "D8A5EC5F100B86C9CAD1AB984E0C2AF3D045AE6CFC9529A6F7C9CD0678E719D1";
//         assert_eq!(sha256(to_be_hashed.to_owned()), String::from(proper_hash));
//     }
// }