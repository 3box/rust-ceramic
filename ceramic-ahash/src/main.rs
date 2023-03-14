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
        ].join("")
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
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
        )
    }

    #[test]
    fn other(){
        let other_hash = Ahash::digest("other");
        expect![[r#"
            Ahash {
                hex: "d9298a10d1b0735837dc4bd85dac641b0f3cef27a47e5d53a54f2f3f5b2fcffa",
                ahash_sha256: [
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
}

fn main() {
    let ah = Ahash::digest("hello");
    println!("{:#?}", ah);
    println!("{}", ah);
}
