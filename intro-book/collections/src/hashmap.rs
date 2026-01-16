use std::collections::HashMap;

pub fn map() {
    let mut users = HashMap::new();

    users.insert("Nome", "pilas");
    users.insert("Idade", "21");
    println!("{:?}", users);
}
