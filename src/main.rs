use cashier::Cashier;

fn main() {
    let mut cashier = cashier::redis::RedisCashier::new();
    cashier.connect("redis://127.0.0.1/").unwrap();

    cashier.set("key", "value").unwrap();
    assert_eq!(cashier.get("key").unwrap().unwrap(), "value");
}
