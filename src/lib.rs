#![deprecated(since = "0.1.3", note = "use the crypto_box crate instead.")]
#[cfg(feature = "unsafe-do-not-enable")]
pub mod chacha20;
#[cfg(feature = "unsafe-do-not-enable")]
pub mod csprng;
#[cfg(feature = "unsafe-do-not-enable")]
pub(crate) mod kdf;
#[cfg(feature = "unsafe-do-not-enable")]
pub mod poly1305;
#[cfg(feature = "unsafe-do-not-enable")]
pub mod salsa20;

use chacha20poly1305::aead::{AeadInPlace, NewAead};

#[cfg(feature = "unsafe-do-not-enable")]
uint::construct_uint! {
    pub struct U256(4);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CipherType {
    Salsa20,
    Chacha20,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SecretBox {
    key: [u8; 32],
    cipher: CipherType,
}

impl SecretBox {
    /// Creates a new SecretBox instance
    ///
    /// Returns None if the passed key is shorter than 32 bytes
    pub fn new<R>(key: R, cipher: CipherType) -> Option<Self>
    where
        R: AsRef<[u8]>,
    {
        let k = key.as_ref();
        if k.len() != 32 {
            return None;
        }
        let mut key = [0_u8; 32];
        key.copy_from_slice(k);
        Some(Self { key, cipher })
    }
    /// Creates a new SecretBox instance by doing an ECDH key exchange using curve25519
    ///
    /// Returns None if the passed public key is not 32 bytes long.
    #[cfg(feature = "curve25519")]
    pub fn from_ecdh<T, R>(
        peer_public_key: T,
        rng: &mut R,
        cipher: CiperType,
    ) -> Option<(Self, [u8; 32])>
    where
        T: AsRef<[u8]>,
        R: rand::Rng + rand::CryptoRng,
    {
        let k = peer_public_key.as_ref();
        if k.len() != 32 {
            return None;
        }
        let mut key = [0_u8; 32];
        key.copy_from_slice(k);
        let peer_pubkey: x25519_dalek::PublicKey = From::from(key);
        let privkey = x25519_dalek::EphermalSecret::new(rng);
        let pubkey: x25519_dalek::PublicKey = From::from(&privkey);
        let shared_secret = privkey.diffie_hellman(&peer_pubkey);
        Some((
            Self {
                key: *shared_secret.as_bytes(),
                cipher,
            },
            *pubkey.as_bytes(),
        ))
    }
    /// Creates a new SecretBox instance with a generated key
    pub fn from_random_key<R>(rng: &mut R, cipher: CipherType) -> (Self, [u8; 32])
    where
        R: rand::Rng + rand::CryptoRng,
    {
        let mut buf = [0u8; 32];
        rng.fill_bytes(&mut buf);
        (Self { key: buf, cipher }, buf)
    }
    /// This function returns an encrypted and authenticated copy of the message. The key and nonce
    /// pair must be unique for every message.
    pub fn seal(&self, message: &[u8], nonce: [u8; 24]) -> Vec<u8> {
        match self.cipher {
            CipherType::Chacha20 => {
                let nonce = chacha20poly1305::XNonce::from_slice(&nonce);
                let aead = chacha20poly1305::XChaCha20Poly1305::new_from_slice(&self.key).unwrap();
                let mut buf = Vec::with_capacity(message.len() + 16);
                buf.extend([0; 16]);
                buf.extend(message);
                let tag = aead
                    .encrypt_in_place_detached(nonce, &[], &mut buf[16..])
                    .unwrap();
                buf[0..16].copy_from_slice(tag.as_slice());
                buf
            }
            CipherType::Salsa20 => {
                let nonce = xsalsa20poly1305::Nonce::from_slice(&nonce);
                let aead = xsalsa20poly1305::XSalsa20Poly1305::new_from_slice(&self.key).unwrap();
                let mut buf = Vec::with_capacity(message.len() + 16);
                buf.extend([0; 16]);
                buf.extend(message);
                let tag = aead
                    .encrypt_in_place_detached(nonce, &[], &mut buf[16..])
                    .unwrap();
                buf[0..16].copy_from_slice(tag.as_slice());
                buf
            }
        }
    }

    /// This function works like the above, except that it automatically generates a unique nonce.
    pub fn easy_seal(&self, message: &[u8]) -> Vec<u8> {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;
        let mut nonce = [0u8; 24];
        rng.fill_bytes(&mut nonce);
        let mut v = Vec::with_capacity(message.len() + 16 + 24);
        v.extend_from_slice(&nonce);
        v.extend_from_slice(&self.seal(message, nonce));
        v
    }

    /// This function works like unseal, except that it finds the nonce automatically.
    pub fn easy_unseal(&self, data: &[u8]) -> Option<Vec<u8>> {
        let mut nonce = [0; 24];
        nonce.copy_from_slice(&data[..24]);
        self.unseal(&data[24..], nonce)
    }

    /// This function tries to authenticate and decrypt a box
    pub fn unseal(&self, data: &[u8], nonce: [u8; 24]) -> Option<Vec<u8>> {
        match self.cipher {
            CipherType::Chacha20 => {
                let (tag, data) = data.split_at(16);
                let tag = chacha20poly1305::Tag::from_slice(tag);
                let nonce = chacha20poly1305::XNonce::from_slice(&nonce);
                let aead = chacha20poly1305::XChaCha20Poly1305::new_from_slice(&self.key).unwrap();
                let mut buf = Vec::with_capacity(data.len());
                buf.extend(data);
                aead.decrypt_in_place_detached(nonce, &[], &mut buf, tag)
                    .ok()?;
                Some(buf)
            }
            CipherType::Salsa20 => {
                let (tag, data) = data.split_at(16);
                let tag = chacha20poly1305::Tag::from_slice(tag);
                let nonce = xsalsa20poly1305::Nonce::from_slice(&nonce);
                let aead = xsalsa20poly1305::XSalsa20Poly1305::new_from_slice(&self.key).unwrap();
                let mut buf = Vec::with_capacity(data.len());
                buf.extend(data);
                aead.decrypt_in_place_detached(nonce, &[], &mut buf, tag)
                    .ok()?;
                Some(buf)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pynacl_vector() {
        let key = b"\x1b\x27\x55\x64\x73\xe9\x85\xd4\x62\xcd\x51\x19\x7a\x9a\x46\xc7\x60\x09\x54\x9e\xac\x64\x74\xf2\x06\xc4\xee\x08\x44\xf6\x83\x89".clone();
        let nonce = b"\x69\x69\x6e\xe9\x55\xb6\x2b\x73\xcd\x62\xbd\xa8\x75\xfc\x73\xd6\x82\x19\xe0\x03\x6b\x7a\x0b\x37".clone();
        let plain = b"\xbe\x07\x5f\xc5\x3c\x81\xf2\xd5\xcf\x14\x13\x16\xeb\xeb\x0c\x7b\x52\x28\xc5\x2a\x4c\x62\xcb\xd4\x4b\x66\x84\x9b\x64\x24\x4f\xfc\xe5\xec\xba\xaf\x33\xbd\x75\x1a\x1a\xc7\x28\xd4\x5e\x6c\x61\x29\x6c\xdc\x3c\x01\x23\x35\x61\xf4\x1d\xb6\x6c\xce\x31\x4a\xdb\x31\x0e\x3b\xe8\x25\x0c\x46\xf0\x6d\xce\xea\x3a\x7f\xa1\x34\x80\x57\xe2\xf6\x55\x6a\xd6\xb1\x31\x8a\x02\x4a\x83\x8f\x21\xaf\x1f\xde\x04\x89\x77\xeb\x48\xf5\x9f\xfd\x49\x24\xca\x1c\x60\x90\x2e\x52\xf0\xa0\x89\xbc\x76\x89\x70\x40\xe0\x82\xf9\x37\x76\x38\x48\x64\x5e\x07\x05".clone();
        let ciphertext = b"\xf3\xff\xc7\x70\x3f\x94\x00\xe5\x2a\x7d\xfb\x4b\x3d\x33\x05\xd9\x8e\x99\x3b\x9f\x48\x68\x12\x73\xc2\x96\x50\xba\x32\xfc\x76\xce\x48\x33\x2e\xa7\x16\x4d\x96\xa4\x47\x6f\xb8\xc5\x31\xa1\x18\x6a\xc0\xdf\xc1\x7c\x98\xdc\xe8\x7b\x4d\xa7\xf0\x11\xec\x48\xc9\x72\x71\xd2\xc2\x0f\x9b\x92\x8f\xe2\x27\x0d\x6f\xb8\x63\xd5\x17\x38\xb4\x8e\xee\xe3\x14\xa7\xcc\x8a\xb9\x32\x16\x45\x48\xe5\x26\xae\x90\x22\x43\x68\x51\x7a\xcf\xea\xbd\x6b\xb3\x73\x2b\xc0\xe9\xda\x99\x83\x2b\x61\xca\x01\xb6\xde\x56\x24\x4a\x9e\x88\xd5\xf9\xb3\x79\x73\xf6\x22\xa4\x3d\x14\xa6\x59\x9b\x1f\x65\x4c\xb4\x5a\x74\xe3\x55\xa5".clone();
        let s = SecretBox::new(key, CipherType::Salsa20).unwrap();
        let output = s.seal(&plain[..], nonce);
        println!("output: {}, ciphertext: {}", output.len(), ciphertext.len());
        assert_eq!(&output[..], &ciphertext[..]);
        let output2 = s.unseal(&output[..], nonce).unwrap();
        assert_eq!(&output2[..], &plain[..]);
    }

    #[test]
    fn easy_seal_unseal() {
        use rand::RngCore;
        let mut rng = rand::rngs::OsRng;
        let mut key = [0u8; 32];
        rng.fill_bytes(&mut key);
        let plain = b"hello world".to_vec();
        let s = SecretBox::new(key, CipherType::Salsa20).unwrap();
        let sealed = s.easy_seal(&plain);
        let unsealed = s.easy_unseal(&sealed).unwrap();
        assert_eq!(&unsealed[..], &plain[..]);
    }
}
