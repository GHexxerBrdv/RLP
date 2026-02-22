use crate::item::RlpItem;

pub struct Decoder;

impl Decoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode(&self, data: &[u8]) -> Result<(RlpItem, usize), String> {
        if data.is_empty() {
            return Err("Empty data".to_string());
        }

        let first_bytes = data[0];
        match first_bytes {
            0x00..=0x7f => Ok((RlpItem::String(vec![first_bytes]), 1)),
            0x80..=0xb7 => {
                let len = (first_bytes - 0x80) as usize;
                if data.len() < 1 + len {
                    return Err("Incomplate data".to_string());
                }
                let bytes = data[1..1 + len].to_vec();
                Ok((RlpItem::String(bytes), 1 + len))
            }
            0xb8..=0xbf => {
                let len_of_len = (first_bytes - 0xb7) as usize;
                if data.len() < 1 + len_of_len {
                    return Err("Incomplete data".to_string());
                }

                let mut len_bytes = [0u8; 8];
                len_bytes[8 - len_of_len..].copy_from_slice(&data[1..1 + len_of_len]);
                let len = usize::from_be_bytes(len_bytes);

                if data.len() < 1 + len_of_len + len {
                    return Err("Incomplete string data".to_string());
                }
                let bytes = data[1 + len_of_len..1 + len_of_len + len].to_vec();
                Ok((RlpItem::String(bytes), 1 + len_of_len + len))
            }
            0xc0..=0xf7 => {
                let total_len = (first_bytes - 0xc0) as usize;
                let (item, payload_consumed) = self.decode_list_payload(&data[1..], total_len)?;
                Ok((item, 1 + payload_consumed))
            }
            0xf8..=0xff => {
                let len_of_len = (first_bytes - 0xf7) as usize;

                if data.len() < 1 + len_of_len {
                    return Err("Incomplete list len data".to_string());
                }

                let mut len_bytes = [0u8; 8];
                len_bytes[8 - len_of_len..].copy_from_slice(&data[1..1 + len_of_len]);
                let total_len = usize::from_be_bytes(len_bytes);

                let (item, payload_consumed) =
                    self.decode_list_payload(&data[1 + len_of_len..], total_len)?;
                Ok((item, 1 + len_of_len + payload_consumed))
            }
        }
    }

    fn decode_list_payload(
        &self,
        data: &[u8],
        total_len: usize,
    ) -> Result<(RlpItem, usize), String> {
        if data.len() < total_len {
            return Err("Incomplete list data".to_string());
        }

        let mut items = Vec::new();
        let mut pos = 0;

        while pos < total_len {
            match self.decode(&data[pos..]) {
                Ok((item, consumed)) => {
                    items.push(item);
                    pos += consumed;
                }
                Err(e) => return Err(format!("Failed to decode list item: {}", e)),
            }
        }
        Ok((RlpItem::List(items), pos))
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}
