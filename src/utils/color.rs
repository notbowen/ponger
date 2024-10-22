use rand::RngCore;

pub fn random_color() -> (u8, u8, u8) {
    let mut bytes = [0_u8; 3];
    rand::thread_rng().fill_bytes(&mut bytes);
    (bytes[0], bytes[1], bytes[2])
}
