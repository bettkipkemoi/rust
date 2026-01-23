fn main() {
    distance_in_kms(4, "kms");
}

fn distance_in_kms(value: i32, unit_label: &str) {
    println!("The distance is: {value}{unit_label}");
}