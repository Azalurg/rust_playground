mod candidate;
mod guild;
mod tests;

fn main() {
    let mut candidate = candidate::Candidate::new(String::from("Alex"));
    candidate.take_test();
    let results = tests::analyze_candidate(&candidate);
    match results {
        candidate::TestResults::Passed => {
            let member = guild::Member::invite_candidate(candidate);
            println!("We have new member: {} (lv {}).", member.name, member.level)
        },
        _ => println!("Unfortunately candidate {} didn't pass exam.", candidate.name)
    }
}
