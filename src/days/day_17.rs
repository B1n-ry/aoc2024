pub fn run(file_input: &str) {
    let a_reg: u64 = get_content(file_input, "Register A: ").parse().unwrap_or(0);
    let b_reg: u64 = get_content(file_input, "Register B: ").parse().unwrap_or(0);
    let c_reg: u64 = get_content(file_input, "Register C: ").parse().unwrap_or(0);

    let program = get_content(file_input, "Program: ");
    let program: Vec<u8> = program.split(",").map(|s| s.parse::<u8>().expect("Program encountered non-digit!")).collect();

    let output = run_computer(&program, a_reg, b_reg, c_reg);

    println!("Problem 1: {}", output.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));

    let a = create_input(&program, program.len() - 1, 0);
    println!("Problem 2: {}", a.unwrap_or(0));
}

fn get_content(file_input: &str, line_beginning: &str) -> String {
    let after = file_input.split_once(line_beginning).expect(&format!("Didn't contain '{}'", line_beginning)).1;
    let content = after.split_once("\r\n").map_or(after, |(a, _)| a);

    content.to_string()
}

fn combo(op: u8, a_reg: u64, b_reg: u64, c_reg: u64) -> u64 {
    match op {
        4 => a_reg,
        5 => b_reg,
        6 => c_reg,
        7 => panic!("Reserved value used!"),
        i @ _ => i as u64,
    }
}

fn run_computer(program: &[u8], a_reg: u64, b_reg: u64, c_reg: u64) -> Vec<u8> {
    let mut a_reg = a_reg;
    let mut b_reg = b_reg;
    let mut c_reg = c_reg;

    let mut output: Vec<u8> = Vec::new();
    let mut instr_ptr = 0;
    while instr_ptr + 1 < program.len() {  // There is both an opcode and operand
        let opcode = program[instr_ptr];
        let operand = program[instr_ptr + 1];

        match opcode {
            0 => {
                a_reg /= 2_u64.pow(combo(operand, a_reg, b_reg, c_reg) as u32);
            },
            1 => {
                b_reg ^= operand as u64;
            }
            2 => {
                b_reg = combo(operand, a_reg, b_reg, c_reg) % 8;
            },
            3 => {
                if a_reg != 0 {
                    instr_ptr = operand as usize;
                    continue;
                }
            },
            4 => {
                b_reg ^= c_reg;
            },
            5 => {
                output.push(combo(operand, a_reg, b_reg, c_reg) as u8 % 8);
            },
            6 => {
                b_reg = a_reg / 2_u64.pow(combo(operand, a_reg, b_reg, c_reg) as u32);
            },
            7 => {
                c_reg = a_reg / 2_u64.pow(combo(operand, a_reg, b_reg, c_reg) as u32);
            }

            _ => (),
        }
        instr_ptr += 2;
    }

    output
}

fn create_input(program: &[u8], current_index: usize, current_a: u64) -> Option<u64> {

    let current_a = current_a << 3;
    let target = program[current_index];
    for i in 0..8 {
        let n = current_a | i;
        if run_computer(program, n, 0, 0)[0] == target {
            if current_index == 0 {
                return Some(n);
            }
            let out = create_input(program, current_index - 1, n);
            if out.is_some() {
                return out;
            }
        }
    }

    None
}
