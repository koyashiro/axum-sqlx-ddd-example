use std::str::FromStr;

use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash as Argon2PasswordHash, PasswordHasher, PasswordVerifier,
        SaltString,
    },
    Argon2,
};

use crate::domain::{error::ValidationError, user_credential::value_object::Password};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(password: &Password) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_str().as_bytes(), &salt)
            .unwrap()
            .to_string();
        Self(password_hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn verify(&self, password: &str) -> bool {
        let password_hash = Argon2PasswordHash::new(&self.0).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &password_hash)
            .is_ok()
    }
}

impl FromStr for PasswordHash {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Argon2PasswordHash::new(s).map_err(|e| ValidationError::password_hash(e, s.to_string()))?;
        Ok(Self(s.to_owned()))
    }
}

impl From<PasswordHash> for String {
    fn from(value: PasswordHash) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_hash_from_str() {
        let tests = vec![
            "$argon2id$v=19$m=4096,t=3,p=1$RmpQMU5TSmFBdmNjUVNuVg$nnSr+lk9BTv6SmxgGeffDw",
            "$argon2id$v=19$m=4096,t=3,p=1$Q3ZXSkZId3ZqdUlPcENjYg$/JInbIV+Sq4f334Fu43DNA",
            "$argon2id$v=19$m=4096,t=3,p=1$cGJzUlJIOVZmSEtjMlFNcg$Q3uh4/flwqqVyuyTrDWMTw",
            "$argon2id$v=19$m=4096,t=3,p=1$eFhKaWtxUTZRZDFacFFlbg$OqTq4630+k1UI6Jw9CZiyw",
            "$argon2id$v=19$m=4096,t=3,p=1$RklSeGZyNHVDbkJQdW1WZQ$SBDhSoliXmhPGUmSvdc5jw",
        ];

        for input in tests {
            let result = PasswordHash::from_str(input);
            assert!(result.is_ok(), "input: {input}, result: {result:?}",);
        }
    }

    #[test]
    fn password_hash_test() {
        let tests = vec![
            "5626739865258883",
            "wopjejwnvpbkejss",
            "TLLFWWQTJLEFSFCT",
            "$#^@*@$%!@&@&#!&",
            "HxV^C%6ho&RR&Xaz",
        ];

        for input in tests {
            let password = Password::try_from(input.to_owned()).unwrap();
            let password_hash = password.to_hash();
            assert!(
                password_hash.verify(password.as_str()),
                "input: {input}, password: {password:?}, password_hash: {password_hash:?}",
            );
        }
    }
}
