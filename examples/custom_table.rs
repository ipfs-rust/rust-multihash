use std::convert::TryFrom;

use tiny_multihash::derive::Multihash;
use tiny_multihash::typenum::{U20, U25, U64};
use tiny_multihash::{
    Digest, Error, Hasher, Multihash, MultihashCode, Sha2Digest, Sha2_256, Size, StatefulHasher,
};

// You can implement a custom hasher. This is a SHA2 256-bit hasher that returns a hash that is
// truncated to 160 bits.
#[derive(Default, Debug)]
pub struct Sha2_256Truncated20(Sha2_256);
impl StatefulHasher for Sha2_256Truncated20 {
    type Size = U20;
    type Digest = Sha2Digest<Self::Size>;
    fn update(&mut self, input: &[u8]) {
        self.0.update(input)
    }
    fn finalize(&self) -> Self::Digest {
        let digest = self.0.finalize();
        let truncated = &digest.as_ref()[..20];
        Self::Digest::try_from(truncated).expect("digest sizes always match")
    }
    fn reset(&mut self) {
        self.0.reset();
    }
}

#[derive(Clone, Copy, Debug, Eq, Multihash, PartialEq)]
#[mh(max_size = U64)]
pub enum Code {
    /// Example for using a custom hasher which returns truncated hashes
    #[mh(code = 0x12, hasher = Sha2_256Truncated20, digest = tiny_multihash::Sha2Digest<U20>)]
    Sha2_256Truncated20,
    /// Example for using a hasher with a bit size that is not exported by default
    #[mh(code = 0xb219, hasher = tiny_multihash::Blake2bHasher::<U25>, digest = tiny_multihash::Blake2bDigest<U25>)]
    Blake2b200,
}

fn main() {
    // Create new hashes from some input data. This is done through the `Code` enum we derived
    // Multihash from.
    let blake_hash = Code::Blake2b200.digest(b"hello world!");
    println!("{:02x?}", blake_hash);
    let truncated_sha2_hash = Code::Sha2_256Truncated20.digest(b"hello world!");
    println!("{:02x?}", truncated_sha2_hash);

    // Sometimes you might not need to hash new data, you just want to get the information about
    // a Multihash.
    let truncated_sha2_bytes = truncated_sha2_hash.to_bytes();
    let unknown_hash = Multihash::<U64>::from_bytes(&truncated_sha2_bytes).unwrap();
    println!("SHA2 256-bit hash truncated to 160 bits:");
    println!("  code: {:x?}", unknown_hash.code());
    println!("  size: {}", unknown_hash.size());
    println!("  digest: {:02x?}", unknown_hash.digest());

    // Though you might want to hash something new, with the same hasher that some other Multihash
    // used.
    Code::try_from(unknown_hash.code())
        .unwrap()
        .digest(b"hashing something new");
}
