pub fn check(candidate: &str) -> bool {
    let vec_candidate = candidate.to_lowercase().
                                            chars().
                                            filter(|c| c.is_alphabetic()).
                                            collect::<Vec<char>>();
    let mut vec_candidate_clean = vec_candidate.clone();
    vec_candidate_clean.sort();
    vec_candidate_clean.dedup();
    vec_candidate_clean.len() == vec_candidate.len()
}
