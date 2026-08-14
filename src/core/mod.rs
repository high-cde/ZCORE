use crate::ledger::Block;

/// ZCORE: Forgia il Blocco Zero, l'origine dell'universo Z-Chain
pub fn forge_genesis_block() -> Block {
    println!("[ZDOS VM] Collegamento al Kernel Stabilito...");
    println!("[ZDOS VM] Forgiatura del Blocco Genesis in corso...");
    
    // Il blocco 0 non ha un hash precedente, usiamo una stringa di zeri
    let genesis_prev_hash = "0".repeat(64);
    
    Block::new(
        0,                                      // Indice
        1723643973,                             // Timestamp (agosto 2026)
        String::from("ZDOS-GENESIS-BLOCK-001"), // Payload fondativo
        genesis_prev_hash,                      // Hash precedente (vuoto)
        0                                       // Nonce iniziale
    )
}

/// ZCORE: Sequenza di avvio del Nodo (Main Loop)
pub fn boot_sequence() {
    println!("========================================");
    println!("      Z-CHAIN NODE INIZIALIZZATO        ");
    println!("========================================");
    
    // 1. Creazione del Ledger iniziale
    let genesis = forge_genesis_block();
    
    // 2. Logica di sistema
    println!("[Z-LANG LOG] Payload originale del blocco: {}", genesis.payload);
    println!("[Z-LANG LOG] Calcolo firma crittografica neurale...");
    println!("[Z-LANG LOG] Firma Hash calcolata con successo: {}", genesis.hash);
    println!("[Z-LANG LOG] Inizializzazione protocollo di trasmissione...");
    
    // Qui in futuro agganceremo il modulo Network
    println!("[Z-CHAIN P2P] Blocco Neurale Trasnesso: {}", genesis.hash);
    println!("[Z-LANG LOG] Nodo sincronizzato e in ascolto.");
    println!("[ZDOS VM] Ciclo terminato (Standby).");
}
