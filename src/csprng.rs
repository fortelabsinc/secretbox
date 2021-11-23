//! CSPRNG (cryptographically secure pseudo random number generator)
//!
//! This trait is used for generating cryptographically secure data (such as a salsa20-poly1502 key). This trait is implemented by all of `rand`'s CSPRNGs.

/// CSPRNG trait used for generating CSPRNG data.
pub trait CSPRNG: Sized {
    fn generate(&mut self, data: &mut [u8]);
}

#[cfg(feature = "rand")]
impl<T> CSPRNG for T
where
    T: rand::Rng + rand::CryptoRng,
{
    fn generate(&mut self, data: &mut [u8]) {
        self.fill(data);
    }
}
