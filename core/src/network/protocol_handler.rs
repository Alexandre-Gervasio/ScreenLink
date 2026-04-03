use serde::{Serialize, Deserialize};
use std::io::{Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Wrapper message for network protocol
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkMessage {
    pub id: u64,
    pub payload: Vec<u8>,
    pub timestamp: u64,
}

/// Protocol handler for serialization/deserialization
pub struct ProtocolHandler;

impl ProtocolHandler {
    pub fn serialize_event(event: &crate::InputEvent) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(bincode::serialize(event)?)
    }

    pub fn deserialize_event(data: &[u8]) -> Result<crate::InputEvent, Box<dyn std::error::Error>> {
        Ok(bincode::deserialize(data)?)
    }

    pub fn serialize_message(msg: &NetworkMessage) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(bincode::serialize(msg)?)
    }

    pub fn deserialize_message(data: &[u8]) -> Result<NetworkMessage, Box<dyn std::error::Error>> {
        Ok(bincode::deserialize(data)?)
    }

    /// Write message with length prefix (for TCP)
    pub async fn send_message(
        writer: &mut (impl AsyncWriteExt + Unpin),
        msg: &NetworkMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let data = Self::serialize_message(msg)?;
        let len = data.len() as u32;
        
        writer.write_all(&len.to_be_bytes()).await?;
        writer.write_all(&data).await?;
        writer.flush().await?;
        
        Ok(())
    }

    /// Read message with length prefix (for TCP)
    pub async fn receive_message(
        reader: &mut (impl AsyncReadExt + Unpin),
    ) -> Result<NetworkMessage, Box<dyn std::error::Error>> {
        let mut len_buf = [0; 4];
        reader.read_exact(&mut len_buf).await?;
        let len = u32::from_be_bytes(len_buf) as usize;
        
        let mut buf = vec![0; len];
        reader.read_exact(&mut buf).await?;
        
        Ok(Self::deserialize_message(&buf)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_serialization() {
        let event = crate::InputEvent::KeyPress {
            keycode: 65,
            down: true,
        };
        
        let encoded = ProtocolHandler::serialize_event(&event).unwrap();
        let decoded = ProtocolHandler::deserialize_event(&encoded).unwrap();
        
        assert!(matches!(decoded, crate::InputEvent::KeyPress { keycode: 65, down: true }));
    }
}
