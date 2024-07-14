use uuid_v7::gen_uuid_v7;

fn main() {
    for _ in 0..10 {
        let uuid_v7 = gen_uuid_v7();
        println!("Generated UUIDv7: {}", uuid_v7);
    }
}