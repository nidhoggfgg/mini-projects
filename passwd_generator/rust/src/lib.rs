use sha3::{Digest, Sha3_512};

pub fn gen_passwd(
    auth: &str,
    target: &str,
    digits: u32,
    uppercase: bool,
    number: bool,
    symbols: &[char],
) -> String {
    let salt = "";
    let mut seed = String::new();
    seed.push_str(auth);
    seed.push_str(target);
    seed.push_str(&digits.to_string());
    seed.push_str(salt);
    _gen_passwd(&seed, digits, uppercase, number, symbols)
}

mod mt19937;

impl mt19937::MT19937 {
    fn choice<T: Copy>(&mut self, array: &[T]) -> T {
        array[self.gen_below(array.len() as u32) as usize]
    }

    fn shuffle<T>(&mut self, array: &mut [T]) {
        for i in (1..array.len()).rev() {
            let j = self.gen_below(i as u32 + 1) as usize;
            array.swap(i, j);
        }
    }

    fn gen_below(&mut self, stop: u32) -> u32 {
        self.gen_u32() % stop
    }
}

fn rng_with_seed(seed: &str) -> mt19937::MT19937 {
    let mut hasher = Sha3_512::new();
    hasher.update(seed.as_bytes());
    let hashed = hasher.finalize();

    let mut result = Vec::with_capacity(64);
    let mut i = 4;
    while i <= hashed.len() {
        let bytes: &[u8; 4] = &hashed[i - 4..i].try_into().unwrap();
        let u = u32::from_le_bytes(*bytes);
        result.push(u);
        i += 4;
    }

    mt19937::MT19937::new_with_slice(&result)
}

fn _gen_passwd(seed: &str, digits: u32, uppercase: bool, number: bool, symbols: &[char]) -> String {
    let mut rng = rng_with_seed(seed);
    let mut digits = digits;

    if digits < 6 {
        return "".to_string();
    }

    let mut charset = Vec::with_capacity(26);
    let mut result = Vec::with_capacity(digits as usize);

    for c in 'a'..='z' {
        charset.push(c);
    }

    if number {
        let mut tmp = Vec::with_capacity(10);
        for c in '0'..='9' {
            tmp.push(c);
        }
        result.push(rng.choice(&tmp));
        charset.extend(tmp);
        digits -= 1;
    }

    if uppercase {
        let mut tmp = Vec::with_capacity(26);
        for c in 'A'..='Z' {
            tmp.push(c);
        }
        result.push(rng.choice(&tmp));
        charset.extend(tmp);
        digits -= 1;
    }

    result.push(rng.choice(symbols));
    charset.extend_from_slice(symbols);
    digits -= 1;

    for _i in 0..digits {
        result.push(rng.choice(&charset));
    }

    rng.shuffle(&mut result);
    let result: String = result.iter().collect();

    result
}
