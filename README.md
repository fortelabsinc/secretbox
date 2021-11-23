# secretbox

Rust-only implementation of NaCl/Libsodium's secure networking protocol used in multiple network protocols

## `secretbox` vs `sodiumoxide` — which to use?
This crate only implements the secretbox algorithm. If you need to use other features from the libsodium library, you currently need to use `sodiumoxide`.

`secretbox` does not contain assembly-level optimization for its cryptographic primitives right now. However its algorithms are fast enough for real-time voice encryption (see below for benchmarks).

`secretbox` is does not use C-bindings which might be hard to build on some systems. Libsodium, the C-library used by `sodiumoxide`, sometimes changes some of its source tarballs, causing build failures.

`secretbox` is not hardened against timing attacks. However it likely does not affect security as the secret that could potentially be leaked is different for every single sealed box.

## Built with

- `uint`
- `x25519_dalek`

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning.

## Authors

- Charlotte D <darkkirb@gmail.com> - *initial work*

## License

This project is licensed under the 2-Clause BSD License - see the [LICENSE](LICENSE) file for details.

## Benchmark results

### Chacha20

| Architecture | CPU | Clock Speed | MB/s | MB/s (SIMD) |
|-|-|-|-|-|
| x86_64 | Intel Core i7-4790K | 4GHz | 615MB/s | 633MB/s |
| x86_64 | Intel Xeon (Skylake) | 2.1GHz | 380MB/s | 456MB/s |
| x86_64 | AMD Ryzen 5 1600 | 3.2GHz | TBD | TBD |
| ARMv8 | Cortex A53 | 1.2GHz | 118MB/s | 103MB/s |
| ARMv8 | Qualcomm Kryo 260 | 1.8GHz | 192MB/s | TBD |

### Poly-1305 (1KB)
| Architecture | CPU | Clock Speed | kp/s | kp/s (SIMD) |
|-|-|-|-|-|
| x86_64 | Intel Core i7-4790K | 4GHz | 152kp/s | 150kp/s |
| x86_64 | Intel Xeon (Skylake) | 2.1GHz | 96kp/s | 87kp/s |
| x86_64 | AMD Ryzen 5 1600 | 3.2GHz | TBD | TBD |
| ARMv8 | Cortex A53 | 1.2GHz | 15.7kp/s | 16kp/s |
| ARMv8 | Qualcomm Kryo 260 | 1.8GHz | 36.5kp/s | TBD |

### Salsa20
| Architecture | CPU | Clock Speed | MB/s | MB/s (SIMD) |
|-|-|-|-|-|
| x86_64 | Intel Core i7-4790K | 4GHz | 548MB/s | 594MB/s |
| x86_64 | Intel Xeon (Skylake) | 2.1GHz | 411MB/s | 444MB/s |
| x86_64 | AMD Ryzen 5 1600 | 3.2GHz | TBD | TBD |
| ARMv8 | Cortex A53 | 1.2GHz | 173MB/s | 100MB/s |
| ARMv8 | Qualcomm Kryo 260 | 1.8GHz | 293MB/s | TBD |

### Seal 1KB (default Salsa20 Poly-1305)
| Architecture | CPU | Clock Speed | kp/s |
|-|-|-|-|
| x86_64 | Intel Core i7-4790K | 4GHz | 72.7kp/s |
