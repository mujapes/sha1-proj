fn chunkify(m: &str) -> Vec<[u32; 16]> {
    let m_bit_cnt = m.len() as u64 * 8;
    m.as_bytes()
    .iter() 
    .chain( Some(1<<7).iter() )
    .chain( {
        let chunk_n_len = (m.len() + 1) % 64;
        if chunk_n_len > 56 {
            std::iter::repeat_n( 0, 120 - chunk_n_len )
        } else { std::iter::repeat_n( 0, 56 - chunk_n_len ) }
    } )
    .chain(
        (m.len() as u64 * 8)
            .to_be_bytes()
            .into_iter()
    )
    .chunks(64)
    .map( |chunk| {
        let mut msgspace: Vec<u8> = Vec::from(chunk);
        std::array::from_fn( |i| u32::from_be_bytes(
            msgspace[i*4..(i+1)*4]
                .try_into()
                .unwrap()
            )
        )
    } )
    .collect()
} 

fn collapse(m: Vec<[u32; 16]>) -> [u32; 5] {
    const K: [u32; 4] = [0x5A827999, 0x6ED9EBA1, 0x8F1BBCDC, 0xCA62C1D6];
    const f: [fn(u32, u32, u32) -> u32; 4] = [
        |b, c, d| (b & c) | (!b & d),
        |b, c, d| b ^ c ^ d,
        |b, c, d| (b & c) | (b & d) | (c & d),
        |b, c, d| b ^ c ^ d
    ];
    let mut H: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    
    m.iter()
        .fold( H, |hash, chunk| {
            let mut chunk_hash = H; 
            let mut wordspace = [0u32; 80];
            wordspace[..16].copy_from_slice(chunk);
            for i in 0..80 {
                if i > 15 { 
                // W(i) = S1(W(i−3) ⊕ W(i−8) ⊕ W(i−14) ⊕ W(i−16))
                    wordspace[i] = (
                        wordspace[i-3] ^ wordspace[i-8] ^ 
                        wordspace[i-14] ^ wordspace[i-16]
                        ).rotate_left(1);
                }
                let TEMP = chunk_hash[0].rotate_left(5)
                    .wrapping_add( f[i/20](chunk_hash[1], chunk_hash[2], chunk_hash[3]) )
                    .wrapping_add(chunk_hash[4])
                    .wrapping_add(wordspace[i])
                    .wrapping_add(K[i/20]);
                chunk_hash = [
                    TEMP,
                    chunk_hash[0],
                    chunk_hash[1].rotate_left(30),
                    chunk_hash[2],
                    chunk_hash[3]
                ];
            }
            std::array::from_fn( |i| hash[i].wrapping_add(chunk_hash[i]) )
        } )
}

pub fn hash(m: &str) -> [u32; 5] { collapse(chunkify(m)) }


#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use sha1::{Sha1, Digest};

    fn  trusted_sha1(m: &str) -> [u32; 5] {
        let mut hasher = Sha1::new();
        // process input message
        hasher.update(m.as_bytes());
        // acquire hash digest in the form of GenericArray,
        // which in this case is equivalent to [u8; 20]
        let hash = hasher.finalize();
        std::array::from_fn( |i| {
        // convert byte array to u32 array
            u32::from_be_bytes(
                hash[i*4..(i+1)*4].try_into()
                    .unwrap() 
                )
        } )
    }

    #[test]
    fn abc() {
        assert_eq!( trusted_sha1("abc"), hash("abc") );
    }

    #[test]
    fn abcd() {
        assert_eq!( trusted_sha1("abcd"), hash("abcd") );
    }

    #[test]
    fn hello_world() {
        assert_eq!( trusted_sha1("hello world"), hash("hello world") );
    }

    #[test]
    fn empty() {
        assert_eq!( trusted_sha1(""), hash("") );
    }

    #[test]
    fn lorem() {
        const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam pellentesque volutpat ex, et dictum orci finibus eget. Suspendisse felis nisi, maximus ut vulputate ut, condimentum vitae nulla. Praesent in venenatis est. Sed augue nibh, rhoncus hendrerit elit non, sodales suscipit eros. Pellentesque et est tellus. Aenean suscipit molestie blandit. Nullam luctus blandit lectus. Vivamus placerat libero at porttitor mollis. Sed convallis purus ligula, id scelerisque turpis congue sit amet.";
        assert_eq!( trusted_sha1(LOREM), hash(LOREM) );
    }

    #[test]
    fn lorem_ipsum() {
        const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam pellentesque volutpat ex, et dictum orci finibus eget. Suspendisse felis nisi, maximus ut vulputate ut, condimentum vitae nulla. Praesent in venenatis est. Sed augue nibh, rhoncus hendrerit elit non, sodales suscipit eros. Pellentesque et est tellus. Aenean suscipit molestie blandit. Nullam luctus blandit lectus. Vivamus placerat libero at porttitor mollis. Sed convallis purus ligula, id scelerisque turpis congue sit amet. Phasellus pellentesque justo fringilla lorem hendrerit, in sodales justo sagittis. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer consequat maximus tellus sit amet auctor. Aenean malesuada at leo id lacinia. Phasellus quis congue leo. Duis condimentum enim sed mauris faucibus, quis malesuada urna vestibulum. Aenean lobortis nisi augue, molestie vestibulum diam blandit facilisis. Fusce eleifend erat elit, sit amet bibendum est feugiat ut. Aenean rhoncus sapien vel enim tristique cursus. Nam at tortor pulvinar, vestibulum magna a, sagittis metus. Donec vel pulvinar diam. Curabitur ullamcorper commodo vestibulum. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam vel blandit nulla, vitae mollis dolor. Integer porttitor nulla non euismod tempor. Aliquam convallis nisi non tempus suscipit. Proin ornare diam ut congue pellentesque. Vivamus facilisis tincidunt velit. Donec eu lectus id sapien ornare ullamcorper. Phasellus feugiat turpis et imperdiet facilisis. Mauris non mauris finibus, euismod justo vitae, maximus eros. Fusce rutrum arcu a felis ornare, et ultricies felis blandit. Pellentesque nunc arcu, accumsan quis sollicitudin nec, tincidunt non sem. Suspendisse commodo, nisl in dictum pretium, justo neque suscipit lorem, ut porta mauris lorem nec neque. Nullam lorem ligula, tempor vitae sapien a, facilisis fermentum dui. Vivamus pulvinar turpis ac neque ultricies cursus. Sed ut efficitur ex. Phasellus ut pellentesque enim. Pellentesque eros sem, dignissim at porta et, molestie ut ante. Quisque a massa sem. Nunc ac molestie libero. Morbi tempor ut nisl et sodales. Integer ut venenatis est. Sed purus libero, aliquet sed rutrum in, lacinia eget dui. Duis euismod dolor iaculis, mollis turpis fringilla, varius velit. Vestibulum vel arcu mauris. Nullam laoreet mi a enim scelerisque, pulvinar accumsan massa rhoncus. Mauris id ullamcorper mi. Nunc viverra tincidunt felis ut egestas. Nulla vitae sodales purus, eu porttitor quam. Integer ultrices nibh eu aliquam blandit. Curabitur dictum consectetur viverra. Donec at ex ut tellus egestas dictum. Proin feugiat eleifend accumsan. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Phasellus eleifend vel ante id sollicitudin. Fusce felis lacus, efficitur facilisis odio sit amet, feugiat congue massa. Duis tempor id nibh eu semper. Donec lectus metus, sagittis interdum dapibus a, placerat in nibh. Duis porttitor purus non elit eleifend, in porta nibh aliquet. Nulla orci dolor, efficitur sit amet risus ut, posuere pellentesque nibh. Etiam eu nunc dolor. Fusce sed lectus in risus maximus vehicula. Nam venenatis magna risus, ac lobortis mauris accumsan quis.";
        assert_eq!( trusted_sha1(LOREM_IPSUM), hash(LOREM_IPSUM) );
    }
}
