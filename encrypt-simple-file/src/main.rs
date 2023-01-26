use anyhow::anyhow;
use chacha20poly1305::{aead::{stream, Aead, NewAead},
    XChaCha20Poly1305
};
use rand::{rngs::OsRng, RngCore};
use std::{fs::{self, File},
            io::{Read, Write},
        };


fn main() {
    let  mut key = [0u8; 32];
    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut key );
    OsRng.fill_bytes(&mut nonce);
    println!("Encrypting file");
    encrypt_file("test.txt", "test.encrypted", &key, &nonce);

    println!("decrypting file");
    decrypt_file("test.encrypted", "unecnrypted_file.txt", &key, &nonce);
}

pub fn encrypt_file(filepath: &str, dest: &str, key: &[u8; 32], nonce: &[u8; 24])
     -> Result<(), anyhow::Error>{
        let cipher = XChaCha20Poly1305::new(key.into());
        println!("Cipher {:?}", stringify!(cipher));

        let file_data = fs::read(filepath)?;

        let encrypted_file = cipher 
            .encrypt(nonce.into(), file_data.as_ref())
            .map_err(|err| anyhow!("Encrypting Error: {}", err))?;

        fs::write(&dest, encrypted_file)?;

        Ok(())
     }

pub fn decrypt_file(file_path: &str, dest: &str, key: &[u8; 32], nonce: &[u8; 24]) -> Result<(), anyhow::Error> {
    let cipher = XChaCha20Poly1305::new(key.into());

    let file_data = fs::read(file_path)?;

    let decrypted_file = cipher
        .decrypt(nonce.into(), file_data.as_ref())
        .map_err(|err| anyhow!("Decrypting small file: {}", err))?;

        fs::write(&dest, decrypted_file)?;

        Ok(())
}