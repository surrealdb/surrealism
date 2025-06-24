use surrealism::surrealism;

#[surrealism]
fn can_drive(age: i64) -> bool {
    age >= 18
}

#[surrealism]
fn create_user((name, age): (String, i64), enabled: bool) -> String {
    return format!("Created user {name} of age {age}. Enabled? {enabled}");
}
