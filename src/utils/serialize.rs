pub use crate::utils::Error;
use tokio::io::{AsyncBufRead, AsyncReadExt};
pub fn serialize<S>(data: &S) -> Vec<u8>
    where
        S: serde::Serialize + ?Sized,
{
    bincode::serialize(data).unwrap()
}

pub async fn deserialize<T, U>(buff_reader: &mut T) -> Result<U,Error>
    where
        U: serde::de::DeserializeOwned,
        T: AsyncBufRead + std::marker::Unpin,
{
    let mut content:Vec<u8> = Vec::new();
    buff_reader.read_to_end(&mut content).await?;
    let data: U = bincode::deserialize(&content)?;
    Ok(data)
}

pub fn deserialize_from_bytes<U>(bytes: &[u8]) -> Result<U,Error>
    where
        U: serde::de::DeserializeOwned,
{
    let data: U = bincode::deserialize(&bytes)?;
    Ok(data)
}