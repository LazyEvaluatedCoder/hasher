use core::fmt;
use std::fs;
use std::io::Read;
use std::iter::zip;
use std::path::{Path, PathBuf};
use std::{cell::RefCell, rc::Rc};

use super::error::Result;
use crate::multi_writer::MultiWriter;

use digest::Digest;
use digest::DynDigest;
use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};
use sha3::{Sha3_256, Sha3_384, Sha3_512};

const HASH_ALGORITHMS: &[&str] = &[
    "md5",
    "sha1",
    "sha256",
    "sha384",
    "sha512",
    "sha3_256",
    "sha3_384",
    "sha3_512",
];

fn new_hasher(algorithm: &str) -> Result<Rc<RefCell<dyn DynDigest>>> {
    match algorithm {
        "md5" => Ok(Rc::new(RefCell::new(Md5::new()))),
        "sha1" => Ok(Rc::new(RefCell::new(Sha1::new()))),
        "sha256" => Ok(Rc::new(RefCell::new(Sha256::new()))),
        "sha384" => Ok(Rc::new(RefCell::new(Sha384::new()))),
        "sha512" => Ok(Rc::new(RefCell::new(Sha512::new()))),
        "sha3_256" => Ok(Rc::new(RefCell::new(Sha3_256::new()))),
        "sha3_384" => Ok(Rc::new(RefCell::new(Sha3_384::new()))),
        "sha3_512" => Ok(Rc::new(RefCell::new(Sha3_512::new()))),
        _ => Err("Hash algorithm not supported.".into()),
    }
}

struct HexSlice<'a>(&'a [u8]);

impl<'a> fmt::LowerHex for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }
        for &byte in self.0 {
            write!(f, "{:0>2x}", byte)?;
        }
        Ok(())
    }
}

pub fn hash<R>(reader: &mut R, algorithm: &str, all: bool, verbose: bool) -> Result<()>
where
    R: Read,
{
    let mut hashers: Vec<Rc<RefCell<dyn DynDigest>>> = Vec::new();

    if all {
        for algorithm in HASH_ALGORITHMS {
            hashers.push(new_hasher(algorithm)?);
        }
    } else {
        hashers.push(new_hasher(algorithm)?);
    }

    let mut multi_writer = MultiWriter::new(&hashers);
    let bytes_read = multi_writer.write(reader)?;

    if verbose {
        println!("Bytes: {bytes_read}");
    }

    if all {
        for (hasher, algorithm) in zip(hashers, HASH_ALGORITHMS) {
            let digest: Box<[u8]> = hasher.borrow_mut().finalize_reset();
            println!("{algorithm}: {:x}", HexSlice(&digest));
        }
    } else if let Some(hasher) = hashers.first() {
        let digest: Box<[u8]> = hasher.borrow_mut().finalize_reset();
        println!("{algorithm}: {:x}", HexSlice(&digest));
    }

    Ok(())
}

pub fn hash_file(path: &PathBuf, algorithm: &str, all: bool, verbose: bool) -> Result<()> {
    let mut file = fs::File::open(path)?;

    if verbose {
        if let Some(file_name) = Path::new(&path).file_name() {
            if let Some(file_name) = file_name.to_str() {
                println!("Name: {file_name}");
            }
        }
        let file_len = file.metadata()?.len();
        println!("Size: {file_len} bytes");
    }

    hash(&mut file, algorithm, all, false)?;

    Ok(())
}

pub fn hash_text(text: &str, algorithm: &str, all: bool, verbose: bool) -> Result<()> {
    if verbose {
        let len = text.chars().count();

        if len <= 12 {
            println!("Text: {text}");
        } else {
            let chars: String = text.chars().take(9).collect();
            println!("Text: {chars}...");
        }

        println!("Characters: {len}");
    }

    hash(&mut text.as_bytes(), algorithm, all, verbose)?;

    Ok(())
}

pub fn hash_stdin(algorithm: &str, all: bool, verbose: bool) -> Result<()> {
    let mut stdin = std::io::stdin().lock();

    hash(&mut stdin, algorithm, all, verbose)?;

    Ok(())
}