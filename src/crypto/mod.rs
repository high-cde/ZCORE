use sha2::{Sha256, Digest};

/// ZCORE: Generatore Hash di base
/// Utilizzato per firmare blocchi, transazioni e validare il Ledger.
pub fn generate_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// ZCORE: Protocollo Quantum Ghostnet
/// Converte un indirizzo pubblico in una "Ghost Identity" usa e getta.
pub fn generate_ghost_alias(public_address: &str, system_salt: &str) -> String {
    let payload = format!("{}{}", public_address, system_salt);
    let full_hash = generate_hash(&payload);
    
    // Estraiamo solo i primi 16 caratteri dell'hash per l'alias fantasma
    format!("GHOST-{}", &full_hash[0..16])
}
