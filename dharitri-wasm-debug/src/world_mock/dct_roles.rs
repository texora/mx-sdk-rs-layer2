use std::fmt::{self, Write};

#[derive(Clone, Default, Debug)]
pub struct DctRoles(Vec<Vec<u8>>);

impl DctRoles {
    pub fn new(roles: Vec<Vec<u8>>) -> Self {
        DctRoles(roles)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self) -> Vec<Vec<u8>> {
        self.0.clone()
    }
}

impl fmt::Display for DctRoles {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dct_buf = String::new();
        let dct_keys: Vec<Vec<u8>> = self.clone().0.to_vec();

        for value in &dct_keys {
            write!(dct_buf, "{}", hex::encode(value.as_slice()))?;
        }
        Ok(())
    }
}
