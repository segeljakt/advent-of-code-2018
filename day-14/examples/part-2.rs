
fn main() {
    let input = "110201";
    let digit_pattern: Vec<u8> = input
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect();
    
    let mut elves: [usize; 2] = [0,1];
    let mut score: Vec<u8> = vec![3,7];

    let mut start = 0;

    loop {
        for _ in 0..1000 {
            add_recipes(&mut elves, &mut score);
        }
        for s in start..start+1000 {
            let mut is_match = true;
            for i in 0..digit_pattern.len() {
                if digit_pattern[i] != score[i+s] {
                    is_match = false;
                    break;
                }
            }
            if is_match {
                println!("Match found at: {}", s);
                return;
            }
        }
        start += 1000;
        println!("start = {}", start);
        println!("score = {}", score.len());
    }

}

fn add_recipes(elves: &mut [usize; 2], score: &mut Vec<u8>) {
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
}
