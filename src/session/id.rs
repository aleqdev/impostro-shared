use std::ops::{Deref, DerefMut};

use rand::Rng;
use serde::{Deserialize, Serialize};

use super::SessionCounter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct SessionId(pub [char; 6]);

impl Deref for SessionId {
    type Target = [char; 6];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SessionId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SessionId {
    pub fn new_random(SessionCounter(ref mut counter): &mut SessionCounter) -> Self {
        let mut rng = rand::thread_rng();
        let mut val: [char; 6] = ['0'; 6];
        for x in val[1..].iter_mut() {
            *x = rng.gen_range(b'A'..=b'Z') as char;
        }
        val[0] = *counter.to_string().as_bytes().last().unwrap() as char;
        *counter += 1;
        Self(val)
    }
}

