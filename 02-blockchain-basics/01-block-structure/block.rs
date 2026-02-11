// define block structure
struct Block {
    id: u64,
    data: String,
    previous_hash: String,
    hash: String
}

impl Block {
    // 'Generator Function"
    fn new(id: u64, data: String, previous_hash: String) -> Self {
        // 아직 해시 계산 로직이 없으니 임시값을 넣는다
        let temp_hash = String::from("temp_hash");

        Block {
            id,
            data,
            previous_hash,
            hash: temp_hash
        }
    }
}

fn main() {
    // Try to generate Genesis Block
    let genesis_block = Block::new(0, String::from("Genesis Block"), String::from("0"));

    println!("Success Generating Block!");
    println!("ID: {}, Data: {}, Hash: {}", genesis_block.id, genesis_block.data, genesis_block.hash);
}