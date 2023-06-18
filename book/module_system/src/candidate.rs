use rand::Rng;

#[derive(Debug)]
pub enum TestResults {
    Passed,
    Failed,
    NotSure
}

pub struct Candidate {
    pub name: String,
    pub test_results: Option<TestResults>
}

impl Candidate {
    pub fn new(name: String) -> Candidate{
        Candidate{name, test_results: None}
    }

    pub fn take_test(&mut self){
        match rand::thread_rng().gen_range(1..=10) {
            1 => self.test_results = Some(TestResults::Failed),
            10 => self.test_results = Some(TestResults::Passed),
            _ => self.test_results = Some(TestResults::NotSure)
        }
    }
}