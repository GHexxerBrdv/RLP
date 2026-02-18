#[derive(Debug)]
enum RlpItem {
    String(Vec<u8>),
    List(Vec<RlpItem>),
}

struct Encoder;

impl Encoder {
    fn new() -> Self {
        Self
    }

    fn encode(&self, item: &RlpItem) -> Vec<u8> {
        match item {
            RlpItem::String(bytes) => self.encode_string(bytes),
            RlpItem::List(items) => self.encode_list(items),
        }
    }

    fn encode_string(&self, bytes: &[u8]) -> Vec<u8> {
        let len = bytes.len();
        match len {
            0 => vec![0x80],

            1 if bytes[0] < 0x80 => vec![bytes[0]],

            len if len < 56 => {
                let mut encoded = vec![0x80 + len as u8];
                encoded.extend_from_slice(bytes);
                encoded
            }

            len => {
                let len_bytes = len.to_be_bytes();
                let len_bytes_trimmed = Self::trim_leading_zeros(&len_bytes);
                let len_of_len = len_bytes_trimmed.len();
                let mut encoded = Vec::with_capacity(1 + len_of_len + len);
                encoded.push(0xb7 + len_of_len as u8);
                encoded.extend_from_slice(len_bytes_trimmed);
                encoded.extend_from_slice(bytes);
                encoded
            }
        }
    }

    fn encode_list(&self, items: &[RlpItem]) -> Vec<u8> {
        let mut encoded_items = Vec::new();

        // encode all items in the list
        for item in items {
            encoded_items.extend(self.encode(item));
        }

        let payload_len = encoded_items.len();

        match payload_len {
            len if len < 56 => {
                let mut encoded = Vec::with_capacity(1 + len);
                encoded.push(0xc0 + len as u8);
                encoded.extend(encoded_items);
                encoded
            }
            len => {
                let len_bytes = len.to_be_bytes();
                let len_bytes_trimmed = Self::trim_leading_zeros(&len_bytes);
                let len_of_len = len_bytes_trimmed.len();

                let mut encoded = Vec::with_capacity(1 + len_of_len + len);
                encoded.push(0xf7 + len_of_len as u8);
                encoded.extend_from_slice(len_bytes_trimmed);
                encoded.extend(encoded_items);
                encoded
            }
        }
    }

    fn trim_leading_zeros(bytes: &[u8]) -> &[u8] {
        let mut start = 0;
        while start < bytes.len() && bytes[start] == 0 {
            start += 1;
        }
        if start == bytes.len() {
            &bytes[0..1]
        } else {
            &bytes[start..]
        }
    }
}

fn main() {
    let encoder = Encoder::new();

    let item = RlpItem::String(b"dog".to_vec());
    let encoded = encoder.encode(&item);
    println!("Encoded dog: 0x{}", hex::encode(encoded));

    let item = RlpItem::List(vec![
        RlpItem::String(b"dog".to_vec()),
        RlpItem::String(b"cat".to_vec()),
        RlpItem::List(vec![
            RlpItem::String(b"rabbit".to_vec()),
            RlpItem::String(b"hamster".to_vec()),
        ]),
    ]);
    let encoded = encoder.encode(&item);
    println!(
        "Encoded ['dog', 'cat', ['rabbit', 'hamster']]: 0x{}",
        hex::encode(encoded)
    );
}
