use crate::crypto;
use serde::{Serialize, Deserialize};

/// ZCORE: Struttura dati nativa del Blocco
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub payload: String, // Dati o Transazioni fantasma
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    /// Inizializza un nuovo blocco e ne calcola l'impronta crittografica
    pub fn new(index: u64, timestamp: u64, payload: String, previous_hash: String, nonce: u64) -> Self {
        let mut block = Block {
            index,
            timestamp,
            payload,
            previous_hash,
            nonce,
            hash: String::new(),
        };
        // Collega il blocco al motore crittografico di ZCORE
        block.hash = block.calculate_hash();
        block
    }

    /// Genera l'hash deterministico basato sul contenuto esatto
    pub fn calculate_hash(&self) -> String {
        let raw_data = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.payload, self.previous_hash, self.nonce
        );
        crypto::generate_hash(&raw_data)
    }
}
