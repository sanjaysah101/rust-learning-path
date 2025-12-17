trait Item {
    fn name(&self) -> &str;
    fn price(&self) -> f64;
}

struct Potion;
struct Weapon;

impl Item for Potion {
    fn name(&self) -> &str {
        "Potion"
    }

    fn price(&self) -> f64 {
        4.0
    }
}

impl Item for Weapon {
    fn name(&self) -> &str {
        "Weapon"
    }

    fn price(&self) -> f64 {
        5.0
    }
}

struct Inventory<T> {
    items: Vec<T>,
}

impl<T> Inventory<T>
where
    T: Item,
{
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn total_value(&self) -> f64 {
        let mut total: f64 = 0.0;

        for item in self.items.iter() {
            total = total + item.price()
        }

        total
    }
}
fn main() {
    let mut weapon_inv = Inventory::<Weapon>::new();

    // Now we can add weapons
    weapon_inv.add(Weapon);
    weapon_inv.add(Weapon);

    println!("Total value of weapons: {}", weapon_inv.total_value());

    let mut potion_inv = Inventory::<Potion>::new();

    // Now we can add weapons
    potion_inv.add(Potion);
    potion_inv.add(Potion);

    println!("Total value of weapons: {}", potion_inv.total_value());
}
