#[derive(Debug)]
enum RlpItem {
    String(Vec<u8>),
}

struct Encoder;

impl Encoder {
    fn new() -> Self {
        Self
    }

    fn encode(&self, item: &RlpItem) -> Vec<u8> {
        match item {
            RlpItem::String(bytes) => self.encode_string(bytes),
        }
    }

    fn encode_string(&self, bytes: &[u8]) -> Vec<u8> {
        match bytes.len() {
            1 if bytes[0] <= 0x7f => vec![bytes[0]],

            len if len <= 55 => {
                let mut encoded = Vec::with_capacity(1 + len);
                encoded.push(0x80 + len as u8);
                encoded.extend_from_slice(bytes);
                encoded
            }

            len => {
                let len_bytes = len.to_be_bytes();
                let len_bytes_trimmed = self.trim_leading_zeros(&len_bytes);
                let len_of_len = len_bytes_trimmed.len();
                let mut encoded = Vec::with_capacity(1 + len_of_len + len);
                encoded.push(0xb7 + len_of_len as u8);
                encoded.extend_from_slice(len_bytes_trimmed);
                encoded.extend_from_slice(bytes);
                encoded
            }
        }
    }

    fn trim_leading_zeros<'a>(&self, bytes: &'a [u8; 8]) -> &'a [u8] {
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

    println!("Encoded dog: 0x{:02x?}", encoded);
}
