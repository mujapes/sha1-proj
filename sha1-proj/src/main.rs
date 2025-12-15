use rand::random;

fn main() {
    let m = "01100001 01100010 01100011";
    let m_bytes: Vec<_> = m
        .split_whitespace()
        .map(|b| match u8::from_str_radix(b, 2) {
            Ok(byte) => byte,
            Err(e) => {
                eprintln!("{}", e);
                0
            }
        })
        .collect();
    let m_len = m_bytes.len() as u64 * 8;
    let padded_m = m_bytes
        .push(1)
        .extend(vec![0u8; 447 - m_len as usize])
        .extend(m_len.to_be_bytes());
    let random5: [u32; 5] = core::array::from_fn(|| random());
    let mut ABCDE = random5;
    let words = padded_m
        .chunks_exact(4)
        .map(|chunk| u32::from_be_bytes(chunk.try_into().expect("chunk should be 4 bytes")))
        .collect();
    for i in 0..80 {
        if i >= 16 {
            let pre_shift = words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16];
            words.push((pre_shift << 1) | (pre_shift >> 31));
        }
    }

    println!("Hello, world!");
}
