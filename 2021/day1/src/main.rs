fn main() {
    // change the `input.txt` file by yours
    let entries = include_str!("input.txt").split("\n");
    let mut max: i32 = i32::MIN; // ensure that no other number will be greater
    let mut counter = 0;

    let c: Vec<i32> = entries
        .into_iter()
        .filter_map(|e| e.parse::<i32>().ok())
        .collect();

    for i in c {
        if i > max {
            counter = counter + 1;
        }
        max = i;
    }
    counter = counter - 1; // because the first entry should not be counted

    dbg!(counter);
}
