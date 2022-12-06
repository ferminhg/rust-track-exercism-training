use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let valid_nucleotides = vec!['A', 'C', 'G', 'T'];
    if !valid_nucleotides.contains(&nucleotide) {
        return Err(nucleotide);
    }
    
    match nucleotides_validator(dna) {
        Err(x) => return Err(x),
        _ => {}
    }
    
    let vec_dna = dna.chars().collect::<Vec<char>>();
    let count = vec_dna.iter(). filter(|&x| *x == nucleotide).count();
    return Ok(count);
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut nucleotides:HashMap<char, usize>= HashMap::from([
        ('A', 0),
        ('C', 0),
        ('G', 0),
        ('T', 0),
    ]);

    match nucleotides_validator(dna) {
        Err(x) => return Err(x),
        _ => {}
    }

    nucleotides = nucleotides
        .into_iter()
        .map(|(k, _)| (k.clone(), count(k.clone(), dna).unwrap()))
        .collect();

    return Ok(nucleotides);
}

fn nucleotides_validator (dna: &str) -> Result<char, char> {
    let valid_nucleotides = vec!['A', 'C', 'G', 'T'];
    let vec_dna = dna.chars().collect::<Vec<char>>();
    if vec_dna.iter().any(|&x| !valid_nucleotides.contains(&x)) {
        return Err(*vec_dna.iter().find(|&x| !valid_nucleotides.contains(&x)).unwrap());
    }   
    Ok(' ')
}
