# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2019-09-21
### Fixes
- Fixed typo in easy_unseal (courtesy of @daaku on gitlab for spotting this)

## [0.1.1] - 2019-07-11
### Fixes
- Fix build on windows as cargo does not seem to support dotfiles

## [0.1.0] - 2019-07-02
### Added
- Chacha20 stream cipher
- Salsa20 stream cipher
- Poly1305 message authentication code
- Key generation using a CSPRNG
- Key generation using ECDH
- Sealing of messages using NaCl's secretbox algorithm
- Unsealing of messages using NaCl's secretbox algorithm
