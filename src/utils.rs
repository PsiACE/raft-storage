use std::convert::TryInto;

use bytes::BytesMut;
use prost::Message;
use sled::Db;

use crate::error::CustomStorageError;

pub fn get<K, V>(db: &Db, k: K) -> Result<V, CustomStorageError>
where
    K: AsRef<[u8]>,
    V: Message + Default,
{
    if let Ok(Some(v)) = db.get(k) {
        decode(v.as_ref())
    } else {
        Ok(V::default())
    }
}

pub fn insert<K, V>(db: &Db, k: K, v: V) -> Result<(), CustomStorageError>
where
    K: AsRef<[u8]>,
    V: Message,
{
    let buf = encode(v)?;
    db.insert(k, &buf[..])?;
    Ok(())
}

pub fn read_be_u64(input: &mut &[u8]) -> u64 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u64>());
    *input = rest;
    u64::from_be_bytes(int_bytes.try_into().unwrap())
}

pub fn decode<V>(v: &[u8]) -> Result<V, CustomStorageError>
where
    V: Message + Default,
{
    Ok(V::decode(v)?)
}

pub fn encode<V>(v: V) -> Result<BytesMut, CustomStorageError>
where
    V: Message,
{
    let mut buf = bytes::BytesMut::with_capacity(v.encoded_len());
    V::encode(&v, &mut buf)?;
    Ok(buf)
}
