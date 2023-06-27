pub mod multiples {
    fn gauss_summation_for(target: u32, number: u32) -> u32 {
        let target = target / number;
        number * (target * (target + 1) / 2)
    }

    pub fn solve_problem_01() {
        let gauss_summation_for_3 = gauss_summation_for(1000, 3);
        let gauss_summation_for_5 = gauss_summation_for(1000, 5);
        let gauss_summation_for_15 = gauss_summation_for(1000, 15);
        let result: u32 = gauss_summation_for_3 + gauss_summation_for_5 - gauss_summation_for_15;
        println!("result: {:?}", result);
    }
}

pub mod fibonacci {
    fn fibonacci(n: u32, memo: &mut Vec<u64>) -> u64 {
        let mut i = 2;
        while memo[memo.len() - 1] < n as u64 {
            memo.push(memo[i as usize - 1] + memo[i as usize - 2]);
            i += 1;
        }
        // sum evens
        return memo.iter().filter(|&x| x % 2 == 0).sum();
    }
    pub fn solve_problem_02() {
        let mut memo: Vec<u64> = vec![1, 2];
        let result = fibonacci(4_000_000, &mut memo);
        println!("result: {:?}", result);
    }
}