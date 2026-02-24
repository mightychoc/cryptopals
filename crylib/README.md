# Crylib*

This directory contains the cryptographic primitives developed and extended while solving the cryptopals challenges. It is structured like so:

- [analysis](./src/analysis/) contains code used for analysing cipher and plaintext. Think letter-frequency scoring, cipher type detection or chaining mode (e.g. ECB) detection.
- [asmy](./src/asym/) bundles everything related to asymmetric ciphers/public-key cryptography.
- [attack](./src/attack/) contains primitives for attacking flawed cryptographic primitives (or flawed implementations).
- [ecc](./src/ecc/) is all about elliptic-curve cryptography.
- [hash](./src/hash/) holds implementation of various hashing algorithms.
- [kdf](./src/kdf/) contains code related to key-derivation functions for password hashing.
- [mac](./src/mac) is the place where message authentication code implementations are located.
- [sym](./src/sym/) bundles everything related to symmetric ciphers.
- [util](./src/util/) contains utility code like hexadecimal and base64 encoding/decoding, a suiting XOR implementation or PRNGs.

_*The choice of abbreviation is not coincidental._