
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn rocket_eq(mass: i32) -> i32 {
    // Base case
    let new_mass = mass / 3 - 2;
    if new_mass <= 0 {
        return 0;
    }
    return new_mass + rocket_eq(new_mass);
}

fn main() {
    // println!("Sum: {:?}", rocket_eq(100756));
    // return;

    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mod_mass: i32 = line.parse().expect("Couldn't parse input");
        sum += rocket_eq(mod_mass);
        // sum += mod_mass;
        // sum += mod_mass / 3 - 2;
    }
    println!("Sum: {:?}", sum);
}
