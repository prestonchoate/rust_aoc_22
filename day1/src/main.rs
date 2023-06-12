use std::fs;

fn main() {
    let contents = get_input();
    let parts = contents.split("\n");
    let mut elf_cal_count = 0;
    let mut elf_num = 1;
    let mut max_cals = 0;
    let mut answer = 0;
    for part in parts {
        match part.parse::<i32>() {
            Ok(cal) => {
                elf_cal_count += cal;
            },
            Err(_e) =>  {
                if elf_cal_count > max_cals {
                    answer = elf_num;
                    max_cals = elf_cal_count;
                }
                elf_cal_count = 0;
                elf_num += 1;
            },
        }
    }

    println!("Max calories is: {max_cals} carried by elf number {answer}");

}


fn get_input() -> String {
let contents = fs::read_to_string("./input/day1.txt")
    .expect("Should have been able to read the file");
    return contents;
}