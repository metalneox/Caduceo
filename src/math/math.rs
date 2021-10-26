pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        n if n % 2 == 0 => collatz(n/2) .map(|x| x + 1),
        n => match n.overflowing_mul(3) {
            (_,false) => match (n*3).overflowing_add(1){
                             (_,false) => collatz(3 * n + 1).map(|x| x + 1),
                             (_,true) => None
                         }
            (_,true) => None
        }
    } 
}