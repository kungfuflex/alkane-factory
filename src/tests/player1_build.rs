use hex_lit;

pub fn get_bytes() -> Vec<u8> {
  (&hex_lit::hex!("89504e470d0a1a0a0000000d494844520000002000000020080300000044a48ac600000021504c5445fbe9d700000048a50a9fe1efe1f3f7ffffff8f563bb6effb22318cff0000ffd84e5a24d393000000097048597300000b1300000b1301009a9c18000000ae49444154789ca592890e83300c43ed85d1c2ff7ff0d424f4a0c7d066a116d98f3454c1eb8bf000e042066c9b88e8e25b5e1c90cb20cbbb190e483672686c06a43f45dd02c8b80f3efb8b22b82aab0100fbac21d0e76c080c72d6440384e0cf0400838a980160380ecd6715deaa69055aaec41438cf39c010ac406a7308301fd103fb406d853df51063bac618530f9edf00aa5600d26d2c81d4eccf00010374cf804fc07e9fc6cba8a664acff810f7301088c33e23fcc0000000049454e44ae426082")).to_vec()
}
