use soroban_sdk::contractdata;

#[contractdata(storage = instance, key = "admin", ttl = bump_to(500))]
struct Admin(u32);

fn main() {}
