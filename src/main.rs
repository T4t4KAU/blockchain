use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

// 定义区块结构
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    // 创建新区块
    fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    // 计算区块的哈希值
    fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // 挖矿（工作量证明）
    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}

// 定义区块链结构
#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    // 创建新区块链
    fn new(difficulty: usize) -> Blockchain {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty,
        };

        blockchain.create_genesis_block();
        blockchain
    }

    // 创建创世区块
    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        self.chain.push(genesis_block);
    }

    // 获取最新的区块
    fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    // 添加新区块
    fn add_block(&mut self, data: String) {
        let latest_block = self.get_latest_block().clone();
        let mut new_block = Block::new(latest_block.index + 1, data, latest_block.hash);
        new_block.mine_block(self.difficulty);
        self.chain.push(new_block);
    }

    // 验证区块链的有效性
    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // 检查当前区块的哈希是否正确
            if current_block.hash != current_block.calculate_hash() {
                println!("Current block hash is invalid");
                return false;
            }

            // 检查前一个区块的哈希是否匹配
            if current_block.previous_hash != previous_block.hash {
                println!("Previous block hash does not match");
                return false;
            }
        }
        true
    }
}

fn main() {
    // 创建一个难度为4的区块链
    let mut blockchain = Blockchain::new(4);

    // 添加一些区块
    blockchain.add_block("First Block".to_string());
    blockchain.add_block("Second Block".to_string());
    blockchain.add_block("Third Block".to_string());

    // 打印区块链
    println!("{:#?}", blockchain);

    // 验证区块链
    println!("Is blockchain valid? {}", blockchain.is_chain_valid());
}