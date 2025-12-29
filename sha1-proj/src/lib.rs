const K: [u32; 4] = [0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xCA62C1D6];
const f: [fn(u32, u32, u32) -> u32; 4] = [
    |b, c, d| (b & c) | (!b & d),
    |b, c, d| b ^ c ^ d,
    |b, c, d| (b & c) | (b & d) | (c & d),
    |b, c, d| b ^ c ^ d
];
#[allow(non_snake_case)]
let mut H: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
// m == "abc" == 01100001 01100010 01100011 == 0x616263 
let m = "abcd";
let m_bit_cnt = m.len() as u64 * 8;

fn chunkify(m: &str) -> Vec<[u32; 16]> {
    m.as_bytes()
    .chunks(56)
    .map(|chunk| {
        let mut msgspace: Vec<u8> = Vec::from(chunk);
        for pad in 0..56-chunk.len() { match pad {
            0 => msgspace.push(128u8),
            _ => msgspace.push(0u8)
        } }
        std::array::from_fn( |i| {
            if i < 15 { return u32::from_be_bytes(msgspace[i*4..(i+1)*4].try_into().unwrap()) }
            if i == 15 { return (m_bit_cnt >> 32) as u32 }
            m_bit_cnt as u32
        } )
        /*
        [0u32; 80][..16].copy_from_slice( {
            msgspace.chunks(4)
                .map( |word| u32::from_be_bytes(word.try_into().unwrap()) )
                .collect();
            } )
        let mut words: Vec<u32> = msgspace.chunks(4)
            .map( |word| u32::from_be_bytes(word.try_into().unwrap()) )
            .collect();
        let m_bit_cnt = m.len() as u64 * 8;
        words.push((m_bit_cnt >> 32) as u32);
        words.push(m_bit_cnt as u32);
        words
        */
    })
    .collect();
} 
//let mut chunks: Vec<[u32; 16]> = 
//println!("{:?}", chunked);
fn hash(m: Vec<[u32; 16]>) -> [u32; 5] {
    m.iter()
        .fold( H, |res, chunk| {
            let mut hash = H; 
            let mut wordspace = [0u32; 80];
            wordspace[..16].copy_from_slice(chunk);
            for i in 0..80 {
                // W(i) = S1(W(i−3) ⊕ W(i−8) ⊕ W(i−14) ⊕ W(i−16))
                if i > 15 { 
                    wordspace[i] = (
                        wordspace[i-3] ^ wordspace[i-8] ^ 
                        wordspace[i-14] ^ cwordspace[i-16]
                        ).rotate_left(1));
                }
                let TEMP = hash[0].rotate_left(5)
                    .wrapping_add( f[i/20](hash[1], hash[2], hash[3]) )
                    .wrapping_add(hash[4])
                    .wrapping_add(wordspace[i])
                    .wrapping_add(K[i/20]);
                hash = [
                    TEMP,
                    hash[0],
                    hash[1].rotate_left(30),
                    hash[2],
                    hash[3]
                ];
            }
            res.iter()
                .enumerate()
                .fold( res, |res, (i ,w)| {
                    res.wrapping_add(hash[i])
                } )
            [
            res[0].wrapping_add(hash[0]),
            res[1].wrapping_add(hash[1]),
            res[2].wrapping_add(hash[2]),
            res[3].wrapping_add(hash[3]),
            res[4].wrapping_add(hash[4])
            ]
        } )
    /*
    for chunk in &mut chunks {
        let mut hash = H; 
        //let mut wordspace = [0u32; 80][..16].copy_from_slice(chunk);
        for i in 0..80 {
            if i > 15 {
            // W(i) = S1(W(i−3) ⊕ W(i−8) ⊕ W(i−14) ⊕ W(i−16))
                let pre_rot = chunk[i-3] ^ chunk[i-8] ^ chunk[i-14] ^ chunk[i-16];
                chunk.push(pre_rot.rotate_left(1));
            }
            let TEMP = hash[0].rotate_left(5)
                .wrapping_add( f[i/20](hash[1], hash[2], hash[3]) )
                .wrapping_add(hash[4])
                .wrapping_add(chunk[i])
                .wrapping_add(K[i/20]);
            hash = [
                TEMP,
                hash[0],
                hash[1].rotate_left(30),
                hash[2],
                hash[3]
            ];
        }
        for i in 0..5 { H[i] = H[i].wrapping_add(hash[i]) } 
    }
}
fn sha1(m: &str) -> [u32; 5] { hash(chunkify(m)) }
// println!("{:?}", chunked);
println!("{:?}", H);

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use sha1::{Sha1, Digest};

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
