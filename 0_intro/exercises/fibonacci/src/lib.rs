pub fn fibo(v: u32) -> u32 {
    let mut fn2 = 0;
    if v == 0{
        return fn2;
    }
    let mut fn1 = 1;
    if v == 1{
        return fn1;
    }
    let mut re = 0;
    for i in 2..=v{
        re = fn1 + fn2;
        fn2 = fn1;
        fn1 = re;
    }
    return re;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibo_0() {
        assert_eq!(0, fibo(0));
    }

    #[test]
    fn fibo_1() {
        assert_eq!(1, fibo(1));
    }

    #[test]
    fn fibo_2() {
        assert_eq!(1, fibo(2));
    }

    #[test]
    fn fibo_big() {
        assert_eq!(610, fibo(15));
    }
}
