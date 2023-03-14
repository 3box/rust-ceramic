use multihash::{Hasher, Sha2_256};
use std::fmt::{Debug, Display, Formatter, Error};

pub struct Ahash([u32; 8]);
impl std::ops::Add for Ahash {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output{
        Self([
            self.0[0].wrapping_add(rhs.0[0]),
            self.0[1].wrapping_add(rhs.0[1]),
            self.0[2].wrapping_add(rhs.0[2]),
            self.0[3].wrapping_add(rhs.0[3]),
            self.0[4].wrapping_add(rhs.0[4]),
            self.0[5].wrapping_add(rhs.0[5]),
            self.0[6].wrapping_add(rhs.0[6]),
            self.0[7].wrapping_add(rhs.0[7]),
        ])
    }
}

impl Ahash {
    fn sha256_digest(s: &str) -> [u8; 32] {
        let mut hasher = Sha2_256::default();
        hasher.update(s.as_bytes());
        hasher.finalize().try_into().unwrap()
    }

    pub fn digest(s: &str) -> Ahash {
        let d = Ahash::sha256_digest(s);
        Ahash([
            u32::from_le_bytes(d[0..4].try_into().unwrap()),
            u32::from_le_bytes(d[4..8].try_into().unwrap()),
            u32::from_le_bytes(d[8..12].try_into().unwrap()),
            u32::from_le_bytes(d[12..16].try_into().unwrap()),
            u32::from_le_bytes(d[16..20].try_into().unwrap()),
            u32::from_le_bytes(d[20..24].try_into().unwrap()),
            u32::from_le_bytes(d[24..28].try_into().unwrap()),
            u32::from_le_bytes(d[28..32].try_into().unwrap()),
        ])
    }

    pub fn to_hex(&self) -> String {
        vec![
            hex::encode(u32::to_le_bytes(self.0[0])),
            hex::encode(u32::to_le_bytes(self.0[1])),
            hex::encode(u32::to_le_bytes(self.0[2])),
            hex::encode(u32::to_le_bytes(self.0[3])),
            hex::encode(u32::to_le_bytes(self.0[4])),
            hex::encode(u32::to_le_bytes(self.0[5])),
            hex::encode(u32::to_le_bytes(self.0[6])),
            hex::encode(u32::to_le_bytes(self.0[7])),
        ].join("").to_uppercase()
    }

    pub fn identity() -> Ahash {
        Ahash([0,0,0,0, 0,0,0,0])
    }
}

impl Debug for Ahash {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        f.debug_struct("Ahash")
         .field("hex", &self.to_hex())
         .field("u32_8", &self.0)
         .finish()
    }
}

impl Display for Ahash {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>{
        write!(f, "{}", self.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn hello(){
        assert_eq!(
            Ahash::digest("hello").to_hex(),
            "2CF24DBA5FB0A30E26E83B2AC5B9E29E1B161E5C1FA7425E73043362938B9824",
        )
    }

    #[test]
    fn other(){
        let other_hash = Ahash::digest("other");
        expect![[r#"
            Ahash {
                hex: "D9298A10D1B0735837DC4BD85DAC641B0F3CEF27A47E5D53A54F2F3F5B2FCFFA",
                u32_8: [
                    277490137,
                    1483976913,
                    3628850231,
                    459582557,
                    669989903,
                    1398636196,
                    1060065189,
                    4207882075,
                ],
            }
        "#]].assert_debug_eq(&other_hash)
    }

    #[test]
    fn word_lists(){
        let cases = [
            [include_str!("tests/bip_39.txt"), "0D021C91D40FD1D87C3ECECB3DEECA30EA3768F87A6618EDD5E6878F4727D7B2"],
            [include_str!("tests/eff_large_wordlist.txt"), "339BEABED2F5700BEFF323A75680E5A16D0DA176816665355A67817A73302782"],
            [include_str!("tests/eff_short_wordlist_1.txt"), "70907533251C0E2B66552827B29A5C4F381F4301C961C194B066C21B005A5A73"],
            [include_str!("tests/eff_short_wordlist_2.txt"), "618BD4C217340D8EF106048CF3341DDD6C366695F6A914233F22EF16E8E84DD3"],
            [include_str!("tests/wordle_words5_big.txt"), "F6ED9C0621C12669791A97B71BAB582B05951F7B2827AAB31A6212381E13D769"],
            [include_str!("tests/wordle_words5.txt"), "11013813008811902644706A2874A0E155BE2ACC17FA6FA953C9406450DF870F"],
        ];

        for [dict, expected] in cases {
            let mut total = Ahash::identity();
            for word in dict.split(" ") {
                total = total + Ahash::digest(word)
            }
            assert_eq!(total.to_hex(), expected)
        }
    }
}

fn main() {
    let ah = Ahash::digest("hello");
    println!("{:#?}", ah);
    println!("{}", ah);
}
