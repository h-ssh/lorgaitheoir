pub struct Chunk {
    pub offset: u64,
    pub data: Vec<u8>,
}

pub struct Feature {
    pub kind: String,
    pub offset: u64,
    pub data: Vec<u8>,
}
