pub fn raindrops(n: u32) -> String {
    // unimplemented!("what sound does Raindrop #{} make?", n)
    let mut sound = String::new();
    if n % 3 == 0 {
        sound.push_str("Pling");
    }
    if n % 5 == 0 {
        sound.push_str("Plang");
    }
    if n % 7 == 0 {
        sound.push_str("Plong");
    }
    if sound.len() == 0 {
        n.to_string()
    } else {
        sound
    }
}
