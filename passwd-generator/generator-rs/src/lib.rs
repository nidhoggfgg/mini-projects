use sha3::{Digest, Sha3_512};

/// generator passwd
/// ```
/// use generator::gen_passwd;
///
/// let auth = "OTZ";
/// let target = "facebook";
/// let symbols = ['.', '@', '_', '-', ':', '!'];
/// let passwd = gen_passwd(auth, target, 16, true, true, &symbols);
/// assert_eq!(passwd, "793HjDt8_xt2cdnM");
/// ```
pub fn gen_passwd(
    auth: &str,
    target: &str,
    digits: u32,
    uppercase: bool,
    number: bool,
    symbols: &[char],
) -> String {
    let salt = "don't ​crack ​this!😱";
    let mut seed = String::new();
    seed.push_str(auth);
    seed.push_str(target);
    seed.push_str(&digits.to_string());
    seed.push_str(salt);
    _gen_passwd(&seed, digits, uppercase, number, symbols)
}

mod mt19937;

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

    let mut charset: Vec<char> = ('a'..='z').collect();
    let mut result = Vec::with_capacity(digits as usize);

    if number {
        let tmp: Vec<char> = ('0'..='9').collect();
        result.push(rng.choice(&tmp));
        charset.extend(tmp);
        digits -= 1;
    }

    if uppercase {
        let tmp: Vec<char> = ('A'..='Z').collect();
        result.push(rng.choice(&tmp));
        charset.extend(tmp);
        digits -= 1;
    }

    result.push(rng.choice(symbols));
    charset.extend_from_slice(symbols);
    digits -= 1;

    for _ in 0..digits {
        result.push(rng.choice(&charset));
    }

    rng.shuffle(&mut result);
    let result: String = result.iter().collect();

    result
}

#[cfg(test)]
mod tests {
    use super::gen_passwd;

    #[test]
    fn test() {
        let symbols = ['.', '@', '_', '-', ':', '!'];
        let auth = "jakflsd";
        let mut result = Vec::with_capacity(25);
        for i in 0..25 {
            result.push(gen_passwd(auth, &i.to_string(), 16, true, true, &symbols));
        }
        let correct: Vec<&'static str> = vec![
            "yeQPSe0v_raa!P2@",
            "I0snnG._JuE@K:.C",
            "8LKoZABRIAK-qGCs",
            "StpO6HYkJ@UwD2r1",
            "M2d3JRXkHK@-feL.",
            "cc@Yeh:e7Kx4:Q:S",
            "BSk2Jkue@Ucs6jG5",
            "FyTO@8!07szxf-P7",
            "hgrrxWAu:z-eNu9v",
            "g.QL9Mo.7g.Q5M5J",
            "vw50gdVw32S:T_L.",
            "G5FHO-U5tCy!pPB4",
            "s:PL2lXXg378.AwN",
            "l6DWmtH!csRaB9ql",
            ".TAE@bXb149Bt6s5",
            "D7wg3!Jk@wJ52KN7",
            "DIK.0_FpQL7V41yl",
            "4TRSi!R@XU4ZGyGj",
            "g7.HJMaHDKQo5CxI",
            "J0Xhj!SgpIZ0O:a-",
            "V8I7.RO3!FfLV2LI",
            "nlr-CjL:3yfZ2y8K",
            "KTVEE..MH39go7L3",
            "0hV.CKcPBNidRxje",
            "MA6:WO587kERY4C7",
        ];

        assert_eq!(result, correct);
    }
}
