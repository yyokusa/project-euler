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
