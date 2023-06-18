use rand::Rng;
use crate::candidate::{TestResults, Candidate};

pub fn analyze_candidate(candidate: &Candidate) -> TestResults{
    match candidate.test_results {
        Some(TestResults::Passed) => TestResults::Passed,
        Some(TestResults::NotSure) => {
            if rand::thread_rng().gen_bool(0.6){
                TestResults::Passed
            } else {
                TestResults::Failed
            }
        },
        _ => TestResults::Failed
    }
}