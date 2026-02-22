use rlp::{Decoder, Encoder, RlpItem};

fn main() {
    let encoder = Encoder::new();
    let decoder = Decoder::new();

    // Test 1: Simple string
    let item1 = RlpItem::String(b"dog".to_vec());
    let encoded1 = encoder.encode(&item1);
    println!("Encoded dog: 0x{}", hex::encode(&encoded1));
    let (decoded1, _) = decoder.decode(&encoded1).unwrap();
    assert_eq!(item1, decoded1);
    println!("✓ String test passed\n");

    // Test 2: Nested list
    let item2 = RlpItem::List(vec![
        RlpItem::String(b"dog".to_vec()),
        RlpItem::String(b"cat".to_vec()),
        RlpItem::List(vec![
            RlpItem::String(b"rabbit".to_vec()),
            RlpItem::String(b"hamster".to_vec()),
        ]),
    ]);

    let encoded2 = encoder.encode(&item2);
    println!(
        "Encoded ['dog', 'cat', ['rabbit', 'hamster']]: 0x{}",
        hex::encode(&encoded2)
    );

    let (decoded2, bytes_read) = decoder.decode(&encoded2).unwrap();
    println!("Bytes read: {}", bytes_read);
    println!("Total encoded length: {}", encoded2.len());

    assert_eq!(item2, decoded2);
    println!("✓ Nested list test passed");

    // Optional: Print hex breakdown for debugging
    println!("\nEncoded bytes: {:02x?}", encoded2);
}
