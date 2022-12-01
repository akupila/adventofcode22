use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    // Create an empty vector of numbers
    let mut totals: Vec<i32> = Vec::new();

    let mut v = 0;
    for l in contents.lines() {
        if l == "" {
            if v > 0 {
                totals.push(v);
                v = 0;
            }
            continue;
        }
        // Parse as integer
        let n = l.parse::<i32>().unwrap();
        v += n;
    }
    // Add remaining value
    totals.push(v);

    // Sort the totals in descending order
    totals.sort_by(|a, b| b.cmp(a));

    println!("Answer 1: {}", totals[0]);
    println!("Answer 2: {}", totals[0] + totals[1] + totals[2]);
}
