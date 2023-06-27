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
//       c n n  c  n  n  c  n  n  c  n  n  c  n  n 
// 1 1 2 3 5 8 13 21 34 55 89 144 233 377 610
pub mod fibonacci {
    fn fibonacci(n: u32) -> u32 {
        let mut sum  = 0;
        let mut a = 1;
        let mut b = 1;
        let mut c = a + b;
        while c < n {
            sum += c;
            a = b + c;
            b = c + a;
            c = a + b;
        }
        sum
    }
    pub fn solve_problem_02() {
        let result = fibonacci(10);
        println!("result: {:?}", result);
    }
}