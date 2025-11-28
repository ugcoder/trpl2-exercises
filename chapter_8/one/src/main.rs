use std::collections::HashMap;

// #[derive(Debug, Clone)]
#[derive(Debug)]
struct ClassScore {
    scores: Vec<f64>,
}
impl ClassScore {
    fn new() -> ClassScore {
        ClassScore { scores: Vec::new() }
    }

    fn from(vec: Vec<i32>) -> ClassScore {
        ClassScore {
            scores: vec.into_iter().map(|x| x as f64).collect(),
        }
    }

    fn get_sum(&self) -> f64 {
        self.scores.iter().sum()
    }

    fn len(&self) -> i32 {
        self.scores.len() as i32
    }

    fn average(&self) -> f64 {
        self.scores.iter().sum::<f64>() / self.len() as f64
    }

    fn clone(&self) -> ClassScore {
        ClassScore {
            scores: self.scores.clone(),
        }
    }

    fn sort(&mut self) {
        // self.scores.sort_by(|a, b| b.partial_cmp(a).unwrap());
        self.scores.sort_by(|a, b| a.total_cmp(b))
    }

    fn median(&self) -> f64 {
        // println!("{}", self.len());
        let mid = self.scores.len() / 2;
        if self.len() % 2 == 0 {
            // Even
            // (self.scores[(self.len() / 2 - 1) as usize] + self.scores[(self.len() / 2) as usize])
            //     / 2.0
            (self.scores[mid - 1] + self.scores[mid]) / 2.0
        } else {
            // Odd
            // self.scores[(self.len() / 2 - 1) as usize]
            self.scores[mid]
        }
    }

    fn mode(&self) -> Vec<String> {
        let mut freq = HashMap::new();
        for &s in &self.scores {
            freq.entry(s.to_string())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
        // let max = freq.values().max().copied().unwrap_or(0);
        // let mut sorted_scores: Vec<_> = freq.iter().collect();
        // sorted_scores.sort_by(|a, b| b.1.cmp(&a.1));
        // let s = sorted_scores.into_iter().max_by_key(|x| x.1).unwrap();
        // s.0.clone()
        let max = freq.values().max().copied().unwrap_or(0);
        freq.iter()
            .filter(|&(_, count)| *count == max)
            .map(|(k, _)| k.clone())
            .collect()
    }
}

fn main() {
    let scores = vec![
        88, 92, 96, 75, 96, 83, 92, 88, 96, 100, 88, 92, 79, 96, 88, 92, 96, 85, 88, 92, 96, 88,
        75, 92, 96, 88, 91, 96, 88, 92, 87, 96, 88, 92, 96, 79, 88, 92, 100, 96, 88, 92, 96, 88,
        92, 96, 88, 92, 96, 88,
    ];

    let scores = ClassScore::from(scores);
    println!("{:?}", scores);

    let sum = scores.get_sum();
    println!("The sum of the scores is: {}", sum);

    let numbers = scores.len();
    println!("The number of students is: {}", numbers);

    let average = scores.average();
    println!("The average of the scores is: {}", average);

    let mut sorted_scores = scores.clone();
    sorted_scores.sort();

    let median = scores.median();
    println!("The median of the Scores: {}", median);

    let mode = scores.mode();
    println!("The scores mode is: {:?}", mode);
}
