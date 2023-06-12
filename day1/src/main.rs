use std::fs;

fn main() {
    const TOP_N_ELVES: usize = 3;
    let mut count: usize = 0;
    let mut answer: i32 = 0;

    let mut cal_counts = sum_elf_cals_vec();
    cal_counts.sort_by(|a, b| b.cmp(a));

    while count < TOP_N_ELVES {
        let cals = cal_counts[count];
        println!("Elf # {count} is carrying {cals} calories");
        answer += cals;
        count += 1;
    }

    println!("The top {TOP_N_ELVES} elves are carrying {answer} calories");
}

fn sum_elf_cals_vec() -> Vec<i32> {
    let contents = get_input();
    let parts = contents.split("\n");
    let mut elf_count = 0;
    let mut elf_cals = 0;
    let mut cal_counts = Vec::<i32>::new();
    for part in parts {
        match part.parse::<i32>() {
            Ok(cal) => {
                elf_cals += cal;
            }
            Err(_e) => {
                cal_counts.insert(elf_count, elf_cals);
                elf_count += 1;
                elf_cals = 0;
            }
        }
    }
    return cal_counts;
}

fn get_input() -> String {
    let contents =
        fs::read_to_string("./input/day1.txt").expect("Should have been able to read the file");
    return contents;
}