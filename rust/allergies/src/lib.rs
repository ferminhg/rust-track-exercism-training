pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

// repetition is fun!
const ALL_ALLERGENS: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { allergens: allergies_calculator(score)}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }
}

fn allergies_calculator(score: u32) -> Vec<Allergen> {
    ALL_ALLERGENS.into_iter().filter(
        |allergen| {
            let mask = allergen.clone() as u32;
            (score & mask) == mask
        }
    ).collect()
}
