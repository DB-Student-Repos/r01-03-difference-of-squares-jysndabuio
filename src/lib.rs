pub fn square_of_sum(n: u32) -> u32 {
    let mut square_of_sum:u32 = 0;

    for x in 1..=n {
        square_of_sum += x;
    }
    return square_of_sum*square_of_sum;
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum_of_squares:u32 = 0;

    for x in 1..=n {
        sum_of_squares += x*x;
    }
    return sum_of_squares
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

