// Ex14 -- enum dispatch: encode_val — cannot dispatch on any()/not() conditions via enum

pub struct Encoder;

pub fn encode_val_enum(enc: &Encoder, data: &[i32]) -> Vec<u8> {
    let _ = enc;
    let mut buf = Vec::new();
    for item in data {
        let s = format!("{:?}", item);
        buf.extend_from_slice(s.as_bytes());
    }
    buf
}
