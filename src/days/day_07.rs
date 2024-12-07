pub fn run(file_input: &str) {
    let calculations: [fn(u64, u64) -> u64; 3] = [
        |a, b| a * b,
        |a, b| a + b,
        |a, b| a * 10_u64.pow((b as f32).log10() as u32 + 1) + b,
    ];

    let result: u64 = file_input.lines().map(|line| {
        let (res, expression) = line.split_once(": ").expect("Wrong format! ': ' not detected on line!");
        let res_i: u64 = res.parse().expect("Wrong format! Result not digit!");

        let e_list: Vec<u64> = expression.split(' ').map(|s| s.parse().expect("Wrong format! Not number in expression!")).collect();
        assert!(e_list.len() >= 2);

        let calculation_count = e_list.len() as u32 - 1;

        for i in 0..calculations.len().pow(calculation_count) {
            let mut calc_res = e_list[0];
            for n in 0..calculation_count {
                calc_res = calculations[(i / calculations.len().pow(n)) % calculations.len()](calc_res, e_list[1 + n as usize]);
            }
            if calc_res == res_i {
                return res_i;
            }
        }
        0
    }).sum();

    println!("Problem 2: {}", result);
}
