use crypto::{buffer,aes,blockmodes};
use crypto::buffer::{ReadBuffer,WriteBuffer,BufferResult};
use std::ffi::OsString;
use std::fs::read_dir;
use std::path::PathBuf;
use std::{env, io};
use std::io::ErrorKind;

use crate::common::Result;

pub fn aes256_cbc_encrypt(data: &[u8], key: &[u8],  iv: &[u8]) -> Result<Vec<u8>>  {
    let mut encrypt = aes::cbc_encryptor(
        aes::KeySize::KeySize128,  //
        key,
        iv,
        blockmodes::PkcsPadding,
    );
    let mut read_buffer = buffer::RefReadBuffer::new(data);
    let mut result = vec![0; data.len() * 4];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut result);
    encrypt.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
    Ok(result)
}

pub fn aes256_cbc_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>> {
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize128,
        key,
        iv,
        blockmodes::PkcsPadding);

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => { }
        }
    }

    Ok(final_result)
}


pub fn get_project_root() -> std::io::Result<PathBuf> {
    let path = env::current_dir()?;
    let mut path_ancestors = path.as_path().ancestors();

    while let Some(p) = path_ancestors.next() {
        let has_cargo =
            read_dir(p)?
                .into_iter()
                .any(|p| p.unwrap().file_name() == OsString::from("Cargo.lock"));
        if has_cargo {
            return Ok(PathBuf::from(p))
        }
    }
    Err(io::Error::new(ErrorKind::NotFound, "Ran out of places to find Cargo.toml"))

}