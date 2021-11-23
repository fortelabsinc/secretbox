#[macro_use]
extern crate criterion;

use criterion::Criterion;
use rand::rngs::OsRng;
use rand::RngCore;
use secretbox::{CipherType, SecretBox};

fn secretbox_benchmark(c: &mut Criterion) {
    c.bench_function("seal salsa20 poly1305", |b| {
        let plaintext = [0u8; 1024];
        let (salsa20, _) = SecretBox::from_random_key(&mut OsRng, CipherType::Salsa20);
        let mut nonce = [0u8; 24];
        OsRng.fill_bytes(&mut nonce);
        b.iter(|| {
            salsa20.seal(&plaintext[..], nonce);
        })
    });
    c.bench_function("seal chacha20 poly1305", |b| {
        let plaintext = [0u8; 1024];
        let (chacha20, _) = SecretBox::from_random_key(&mut OsRng, CipherType::Chacha20);
        let mut nonce = [0u8; 24];
        OsRng.fill_bytes(&mut nonce);
        b.iter(|| {
            chacha20.seal(&plaintext[..], nonce);
        })
    });
    c.bench_function("unseal salsa20 poly1305", |b| {
        let plaintext = [0u8; 1024];
        let (salsa20, _) = SecretBox::from_random_key(&mut OsRng, CipherType::Salsa20);
        let mut nonce = [0u8; 24];
        OsRng.fill_bytes(&mut nonce);
        let mut salsa20_enc = Vec::new();
        salsa20_enc.extend_from_slice(&nonce);
        salsa20_enc.extend(salsa20.seal(&plaintext[..], nonce));
        b.iter(|| {
            salsa20.easy_unseal(&salsa20_enc);
        });
    });
    c.bench_function("unseal chacha20 poly1305", |b| {
        let plaintext = [0u8; 1024];
        let (chacha20, _) = SecretBox::from_random_key(&mut OsRng, CipherType::Chacha20);
        let mut nonce = [0u8; 24];
        OsRng.fill_bytes(&mut nonce);
        let mut chacha20_enc = Vec::new();
        chacha20_enc.extend_from_slice(&nonce);
        chacha20_enc.extend(chacha20.seal(&plaintext[..], nonce));
        b.iter(|| {
            chacha20.easy_unseal(&chacha20_enc);
        });
    });
}

criterion_group!(benches, secretbox_benchmark);
criterion_main!(benches);
