use std::fs;
use std::fs::File;
use std::io::{Read, Seek, Write};

pub use pasta_curves::arithmetic::FieldExt;

/// used to dump data for pre-compute
pub trait ConvertToBytes {
    fn convert_to_bytes(&self) -> [u8; 32];

    fn convert_from_bytes(bytes: &[u8; 32]) -> Self;
}

/// Write a field(Fr,Fq) into file.(without montgomery)
#[allow(unused_must_use)]
pub fn write_field_into_file<F: ConvertToBytes + FieldExt>(file_name: &str, filed: F) {
    let mut file = File::create(file_name).unwrap();

    file.write(&filed.convert_to_bytes());
}

/// Read a field(Fr,Fq) into file.(without montgomery)
#[allow(unused_must_use)]
pub fn read_field_from_file<F: ConvertToBytes + FieldExt>(file_name: &str) -> F {
    let mut file = File::open(file_name).unwrap();
    let mut buf = [0; 32];

    file.read(&mut buf);
    F::convert_from_bytes(&buf)
}

/// Write a field(Fr,Fq) into file.(without montgomery)
#[allow(unused_must_use)]
pub fn write_fields_into_file<F: ConvertToBytes + FieldExt>(file_name: &str, fields: &[F]) {
    let mut file = File::create(file_name).unwrap();
    for i in 0..fields.len() {
        file.write(&fields[i].convert_to_bytes());
    }
}

/// Read a field(Fr,Fq) into file.(without montgomery)
#[allow(unused_must_use)]
pub fn read_fields_from_file<F: ConvertToBytes + FieldExt>(file_name: &str, size: u64) -> Vec<F> {
    let mut file = File::open(file_name).unwrap();

    let file_len = fs::metadata(file_name).unwrap().len();
    assert!(file_len >= (size * 32), "file don't have enough bytes");

    let mut result: Vec<F> = vec![];
    let mut buf = [0; 32];

    for _ in 0..size {
        file.read(&mut buf);
        let fd = F::convert_from_bytes(&buf);
        result.push(fd);
    }
    result
}
