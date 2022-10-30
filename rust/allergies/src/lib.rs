pub struct Allergies {
    list_allergies: Vec<Allergen>
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

fn allergeis_by_score(score: u32) -> Vec<Allergen> {
    let mut allergies = Vec::<Allergen>::new();

    if score <= 0 {
        return allergies;
    }
    if score == 2 { return Vec::from([Allergen::Peanuts]); }
    if score == 4 { return Vec::from([Allergen::Shellfish]); }
    if score == 8 { return Vec::from([Allergen::Strawberries]); }

    if score >= 128 { allergies.push(Allergen::Cats) }
    if score >= 64 { allergies.push(Allergen::Pollen) }
    if score >= 32 { allergies.push(Allergen::Chocolate) }
    if score >= 16 { allergies.push(Allergen::Tomatoes) }
    if score >= 8 { allergies.push(Allergen::Strawberries) }
    if score >= 4 { allergies.push(Allergen::Shellfish) }
    if score >= 2 { allergies.push(Allergen::Peanuts) }
    if score >= 1 { allergies.push(Allergen::Eggs) }

    return allergies;
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let allergies = allergeis_by_score(score);
        Self{list_allergies: allergies.to_vec()}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        for i in 0 .. self.list_allergies.len() {
            if self.list_allergies[i] == *allergen {
                return true;
            }
        }
        false
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Vec::from(self.list_allergies.to_vec())
    }
}
