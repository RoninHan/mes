use bcrypt::{hash, DEFAULT_COST, verify};

fn main() {
    let password = "admin123";
    let hash_result = hash(password, DEFAULT_COST).unwrap();
    println!("Generated hash: {}", hash_result);
    
    let stored_hash = "$2a$10$7JB720yubVSZvUI0rEqK/.VqGOZTH.ulu33dHOiBE/TP57ErKW9Cu";
    let verify_result = verify(password, stored_hash).unwrap();
    println!("Verification result: {}", verify_result);
}
