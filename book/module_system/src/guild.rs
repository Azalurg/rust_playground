use crate::candidate::Candidate;

pub struct Member{
    pub name: String,
    pub level: u32
}

impl Member{
    fn promote(&mut self){
        self.level +=1;
    }

    pub fn invite_candidate(candidate: Candidate) -> Member {
        let mut new_member = Member { name: candidate.name, level: 1 };
        new_member.promote();
        new_member
    }
}