<h1 style="font-size:50px" align="center">RustyRosetta</h1>

<h3 align="center">Comprehensive utility crate that contains the implementation/wrapper of some common data operations and re-exports frequently used types for convenience.</h3>


```
RustyRosetta/src                                    
 ├──authenticated_type.rs → generate a keyed hash for any type that implements the 'Serialize', 'Deserialize' trait using Blake3
 ├──checksum.rs → a data checksum wrapper using crc32fast
 ├──codec.rs → implements the custom 'Encode' & 'Decode' trait for any 'Serialize' ('Deserialize' respectively) type, with the default implementation using the 'Postcard' serializer, runtime stack allocated buffers and preallocated buffers
 ├──compress.rs → a data compression wrapper using lz4_flex
 ├──crypt.rs → an encryption/decryption wrapper using ChaCha20
 └──lib.rs → re-exports some commonly used types
```

### TODO
- Operation chaining