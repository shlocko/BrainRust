use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&args[1]).expect("Did not find source file");
    let mut input: String;
    let mut input_chars: Vec<char> = vec![];
    let chars: Vec<char> = content.chars().collect();
    const MEM_SIZE: usize = 300000;

    let mut mem: [u8; MEM_SIZE] = [0; MEM_SIZE];
    let mut index = 0;
    let mut input_index = 0;

    if args.len() > 2{
        input = fs::read_to_string(&args[2]).expect("Did not find input file");
        input_chars = input.chars().collect();
    }

    let mut i = 0;
    while i < chars.len(){
        match chars[i]{
            '>' => if index < MEM_SIZE{index += 1},
            '<' => if index > 0{index -= 1},
            '+' => mem[index] += 1,
            '-' => mem[index] -= 1,
            '.' => println!("{}", mem[index] as char),
            ',' => {
                if input_chars.len() > 0  && input_index < input_chars.len(){
                    let input_char = input_chars[input_index];
                    input_index += 1;
                    mem[index] = input_char as u8;
                }else{
                    mem[index] = 0;
                }
            }
            '[' => {
                if mem[index] == 0 {
                    let mut temp_count = 0;
                    let mut skip = 0;
                    for x in i+1..chars.len(){
                        skip += 1;
                        if chars[x] == '['{
                            temp_count += 1;
                        }else if chars[x] == ']'{
                            if temp_count == 0 {
                                i += skip;
                                break;
                            }else{
                                temp_count -= 1;
                            }
                        }
                    }
                }
            },
            ']' => {
                if mem[index] != 0 {
                    let mut temp_count = 0;
                    for x in 1..i {
                        if chars[i-x] == ']'{
                            temp_count += 1;
                        }else if chars[i-x] == '['{
                            if temp_count == 0{
                                i -= x;
                                break;
                            }else{
                                temp_count -= 1;
                            }
                        }
                    }
                }
            },
            _ => {},
        }
        i+=1;
    }
}
