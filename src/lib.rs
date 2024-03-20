pub fn square_of_sum(n: u32) -> u32 {
    // (1..=n).sum::<u32>().pow(2) // Better use methods .sum() & .pow, rather than using for loop.
    let sum:u32 = (n*(n+1))/2; // Yes, this was better than using iteration using mathematical formula "sum of an arithmetic series"
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    // (1..=n).map(|x| x * x).sum::<u32>() // .map method call in the range to make transformation which is |x| x * x, then add the transformed range.
    let sum:u32 = (n*(n+1)*((2*n)+1))/6; // Faulhaber's formula
    sum
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

