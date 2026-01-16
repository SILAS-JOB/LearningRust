struct User {
    username: String,
    isActive: bool,
    email: String,
}

fn main() {
    let user = User {
        username: String::from("pilas"),
        isActive: true,
        email: String::from("pilas@gmail.com"),
    };
    for i in &user {
        println!("{&i}")
    }
}
