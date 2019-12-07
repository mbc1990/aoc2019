use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

const ANSWER: i32 = 19690720;

fn try_inputs(a: i32, b: i32, opcodes: &mut Vec<i32>) -> i32 {

    opcodes[1] = a;
    opcodes[2] = b;

    let mut i = 0;

    println!("Input: {:?}", &opcodes);
    while i < opcodes.len() - 4 {
        let next_inst = opcodes[i];
        println!("Inst: {:?}", next_inst);
        match next_inst {
            99 => {  // End program
                break;
            },
            1 => {
                let idx = opcodes[i+3] as usize;
                let arg_1_idx  = opcodes[i+1] as usize;
                let arg_2_idx = opcodes[i+2] as usize;
                opcodes[idx] = opcodes[arg_1_idx] + opcodes[arg_2_idx];
                // println!("Arguments: {:?}, {:?}", arg_1, arg_2);
                println!("1: Set {:?} to {:?}", idx, opcodes[idx]);
            },
            2 => {
                let idx = opcodes[i+3] as usize;
                let arg_1_idx  = opcodes[i+1] as usize;
                let arg_2_idx = opcodes[i+2] as usize;
                opcodes[idx] = opcodes[arg_1_idx] * opcodes[arg_2_idx];
                println!("Arguments: {:?}, {:?}", arg_1_idx, arg_2_idx);
                println!("2: Set {:?} to {:?}", idx, opcodes[idx]);
            },
            _ => {}
        }
        i += 4;
        println!("------------------");
    }
    println!("Answer: {:?}", opcodes);
    return opcodes[0];
}

fn main() {
    // let file = File::open("test_input.txt").unwrap();
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        lines.push(line.clone());
    }
    let opcodes: Vec<i32> = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    for i in 0..100 {
        for k in 0..100 {
            let mut temp_opcodes = opcodes.clone();
            let res = try_inputs(i, k, &mut temp_opcodes);
            if res == ANSWER {
                println!("Answer: {:?}, {:?}", i, k);
                return;
            }
        }
    }
}
