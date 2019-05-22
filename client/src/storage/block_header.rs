pub struct BlockHeader {
    pub block_number: u32,
    pub value: String,
}

impl BlockHeader {
    pub fn new(block_number: u32, value: String) -> BlockHeader {
        BlockHeader {
            block_number,
            value,
        }
    }
}