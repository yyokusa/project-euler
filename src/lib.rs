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

pub mod largest_prime_factor {
    fn largest_prime_factor(n: &mut u64) -> u64 {
        let mut i = 2;
        while i * i <= *n {
            if *n % i == 0 {
                *n /= i;
            } else {
                i += 1;
            }
        }
        *n
    }

    pub fn solve_problem_03() {
        let mut number = 600851475143;
        let result = largest_prime_factor(&mut number);
        println!("result: {:?}", result);
    }
}