use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    println!("Use {} to encode {} using shift cipher", key, s);

    if !key.is_empty() && key.chars().all(|c| c.is_alphabetic() && c.is_lowercase()) {
        let key_bytes = key.as_bytes();
        let mut i = 0;
        let key_len = key_bytes.len();
        let cipher = s.chars()
            .map(|c| {
                let case = if c.is_uppercase() {'A'} else {'a'} as u8;
                let shift = key_bytes[(i%key_len)] -97;
                i += 1;
                if c.is_alphabetic() { (((c as u8 - case + shift) % 26) + case) as char } else { c }
            }).collect::<String>();
        println!("cipher {:?}", cipher);

        Some(cipher)
    } else {
        None
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    println!("Use {} to encode {} using shift cipher", key, s);

    if !key.is_empty() && key.chars().all(|c| c.is_alphabetic() && c.is_lowercase()) {
        let key_bytes = key.as_bytes();
        let mut i = 0;
        let key_len = key_bytes.len();
        let decipher = s.chars()
            .map(|c| {
                let case = if c.is_uppercase() { 'A' } else { 'a' } as u8;
                let shift = 26 - (key_bytes[(i % key_len)] - 97);
                i += 1;
                if c.is_alphabetic() { (((c as u8 - case + shift) % 26) + case) as char } else { c }
            }).collect::<String>();
        println!("decipher {:?}", decipher);

        Some(decipher)
    } else {
        None
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    const KEY_LEN: usize = 100;
    let mut rng = rand::thread_rng();

    let key: String = (0..KEY_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        }).collect();

    println!("{:?}", key);

    let cipher: String = match encode(&key, s) {
        Some(x)=> x,
        None => panic!("issue in encoding!"),
    };

    (key, cipher)
}
