
fn main() {
    let max_len: usize = 110201 + 10;
    let mut elves: [usize; 2] = [0,1];
    let mut score: Vec<u8> = vec![3,7];

    loop {
        let sum: u8 = elves
            .iter()
            .map(|e| score[*e])
            .sum::<u8>();

        let digits: Vec<u8> = sum
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .collect();

        score.extend(digits);

        elves
            .iter_mut()
            .for_each(|e| *e = (*e + 1 + score[*e] as usize) % score.len());

        if score.len() == max_len {
            break;
        }
    }

    println!("Last 10:");
    score
        .into_iter()
        .rev()
        .take(10)
        .collect::<Vec<u8>>()
        .iter()
        .rev()
        .for_each(|s| print!("{}", s));
    println!("");

}
