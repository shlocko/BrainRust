use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&args[1]).expect("Did not find file");
    const MEM_SIZE: usize = 30;

    let mut mem: [u8; MEM_SIZE] = [0; MEM_SIZE];
    let mut index = 0;
    let mut loop_count = 0;

    for c in content.chars(){
        match c{
            '>' => if index < MEM_SIZE{index += 1},
            '<' => if index > 0{index -= 1},
            '+' => mem[index] += 1,
            '-' => mem[index] -= 1,
            '.' => println!("{}", mem[index] as char),
            '[' => {
                if mem[index] != 0 {
                    loop_count += 1;
                }
            }
            _ => {},
        }
    }
    // println!("{:?}", mem);
}
