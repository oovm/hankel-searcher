use percolation::{Cell, SquareSite};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let mut ss = SquareSite::uniform(100, 2);
    for i in ss.take(100) {
        let sum = i.iter().sum::<usize>() as f32;
        let mean = sum / i.len() as f32;
        println!("{mean:>.2}: {i:?}");
    }
}
#[test]
fn test2() {
    let mut ss = Cell::new(1);
    ss.set_id(6);
    ss.replace_id(6, 5);
    println!("{:#?}", ss);
}
pub struct BayesianEstimation<const N: usize> {
    posterior: Vec<[f32; N]>,
    rng: SmallRng,
}
#[derive(Debug)]
pub struct Answer {
    answer: Vec<u32>,
    correct: usize,
}

impl<const N: usize> BayesianEstimation<N> {
    pub fn new(questions: usize) -> Self {
        let p = 1.0 / N as f32;
        Self { posterior: vec![[p; N]; questions], rng: SmallRng::from_entropy() }
    }
    /// check: get the number of correct answers, can call only once
    /// return: the answer of attempt, check(answer) == answer.len
    pub fn attempt<F>(&mut self, check: F) -> Vec<Answer>
    where
        F: Fn(&[u32]) -> usize,
    {
        for _ in 0..self.posterior.len() {
            let attempt = self
                .posterior
                .iter()
                .map(|probs| {
                    let dist = WeightedIndex::new(probs).unwrap();
                    self.rng.sample(&dist)
                })
                .collect();
            let correct = check(&attempt);
            let out = Answer { answer: attempt.to_vec(), correct };
            self.update_posterior(&attempt, correct as u32);
            yield out;
        }
    }

    fn update_posterior(&mut self, attempt: &[u32], correct: u32) {
        for (i, &answer) in attempt.iter().enumerate() {
            let prior = self.posterior[i];
            let likelihood = if answer == correct { 1.0 } else { 0.0 };
            let evidence = prior.iter().sum::<f32>();
            self.posterior[i] = prior.iter().map(|&p| p * likelihood / evidence).collect::<Vec<_>>().try_into().unwrap();
        }
    }
}

#[test]
fn main() {
    let answers = vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1];
    let mut bayesian_estimation = BayesianEstimation::<4>::new(answers.len());
    let check = move |answer: &[u32]| answers.iter().zip(answer).filter(|(a, b)| a == b).count();
    for i in bayesian_estimation.attempt(check) {
        println!("{:?}", i);
    }
}
