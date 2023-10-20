use std::{fmt::Display, ops::Index};

use rand::{seq::SliceRandom, thread_rng, Rng};

/// Defines the genetic makeup of an individual
/// Note: DNA can be represented in many ways, and so its represented
/// through json.
pub trait Dna {
    /// Get the dna of the individual. This is a json object.
    fn get_dna(&self) -> serde_json::Value;

    /// Get the species of the individual. Statically defined
    /// for each individual, as it is a property of the individual
    /// which does not change.
    fn get_species(&self) -> &'static str;
}

/// Defines how an individual can be evaluated among its peers.
pub trait Fitness<F> {
    /// Evaluate the fitness of an individual
    /// Higher number is better. This value then normalized
    /// by other functions using this value. Such as [Population::get_weights]
    fn evaluate_fitness(&self) -> f32;
}

/// Defines how an individual can reproduce
/// and mutate. Note: reproduce uses self and a second individual to
/// create a new individual.
pub trait Breedable<T>
where
    Self: Sized,
{
    fn reproduce(&self, second: &Self) -> Result<Self, String>;
    fn mutate(&mut self);
}

/// Population is a collection of individuals who can reproduce.
/// This is the main struct that is used to evolve a population.
#[derive(Debug)]
pub struct Population<T>
where
    T: Dna,
    T: Fitness<T>,
    T: Display,
    T: Breedable<T>,
{
    pub populace: Vec<T>,
}

impl<T> Population<T>
where
    T: Dna,
    T: Fitness<T>,
    T: Display,
    T: Breedable<T>,
{
    /// Create a new population from a vector of individuals
    pub fn new(pop: Vec<T>) -> Population<T> {
        Population { populace: pop }
    }

    /// get the number of members of the population
    pub fn len(&self) -> usize {
        self.populace.len()
    }

    /// check if the population is empty
    pub fn is_empty(&self) -> bool {
        self.populace.is_empty()
    }

    /// Attempt to create a new individual from two parents
    /// Note: This is essential a wrapper for the [Breedable] trait of the individual,
    /// specifically the [Breedable::reproduce] function.
    pub fn breed_from_parents(&self, first: &T, second: &T) -> Result<T, String> {
        let child = first.reproduce(second);
        match child {
            Ok(child) => Ok(child),
            Err(e) => Err(e),
        }
    }

    /// Select the top percentage of the population. You must use a value
    /// between 0 and 100. This function will return an error if you do not.
    pub fn selection(&self, top_percentage: usize) -> Result<Vec<&T>, String> {
        if top_percentage > 100 || top_percentage == 0 {
            Err(format!(
                "Cannot select {} percent of the population",
                top_percentage
            ))
        } else {
            // since the weights are normalized, we can simply check >= min_fitness
            let mut selected = Vec::new();
            let weights = &self.get_weights();
            for (i, weight) in weights.iter().enumerate() {
                if *weight >= (100 - top_percentage) as f32 {
                    selected.push(&self.populace[i]);
                }
            }

            Ok(selected)
        }
    }

    /// Get the weights of the population. This is a vector of f32s
    /// that represent the fitness of each individual. This is used
    /// by the [Population::selection] function to select the top
    /// percentage of the population for reproduction to the next generation.
    pub fn get_weights(&self) -> Vec<f32> {
        let mut max_fitness = 0.0;
        let mut weights = Vec::new();

        for individual in self.populace.iter() {
            let fitness = individual.evaluate_fitness();
            if fitness > max_fitness {
                max_fitness = fitness;
            }
            weights.push(fitness);
        }
        // normalize
        weights
            .iter_mut()
            .for_each(|i| *i = *i / max_fitness * 100.0);
        weights
    }
}

/// Indexing for the population. This allows you to index into the population
/// e.g: `population[0]` will return the first individual in the population.
impl<T> Index<usize> for Population<T>
where
    T: Dna,
    T: Fitness<T>,
    T: Display,
    T: Breedable<T>,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.populace[index]
    }
}

/// Defines how a population can evolve.
/// This is the main trait that is used to evolve a population.
pub trait Evolvable<T> {
    fn evolve(&mut self) -> Result<(), String>;
    fn generation(&self) -> usize;
}

impl<T> Evolvable<T> for Population<T>
where
    T: Dna,
    T: Fitness<T>,
    T: Display,
    T: Breedable<T>,
{
    /// Evolve the population. This will create a new generation of the population
    /// and mutate the population in place. This function will return an error if
    /// there are issues from the [Population::breed_from_parents] function.
    fn evolve(&mut self) -> Result<(), String> {
        let mut new_pop: Vec<T> = Vec::new();
        let rng = &mut thread_rng();

        let mut elite = self.selection(10)?;

        // if there are no elites, we need to find the best individual and call it the elite
        if elite.is_empty() {
            let mut max_fitness = 0.0;
            let mut max_index = 0;
            for (i, entity) in self.populace.iter().enumerate() {
                let fitness = entity.evaluate_fitness();
                if fitness >= max_fitness {
                    max_fitness = fitness;
                    max_index = i;
                }
            }
            elite = vec![&self.populace[max_index]];
        }

        for _ in 0..self.len() {
            let first = elite.choose(rng).unwrap();
            let second = elite.choose(rng).unwrap();
            // kinda gross we need to dereference once, but idk how to do it better
            let child = self.breed_from_parents(*first, *second);
            match child {
                Ok(mut child) => {
                    if rng.gen_range(0..100) < 10 {
                        // println!("Mutating child");
                        child.mutate();
                    }
                    new_pop.push(child);
                }
                Err(e) => return Err(e),
            }
        }
        self.populace = new_pop;
        Ok(())
    }

    /// TODO: This needs to be updated to actually track the generation
    fn generation(&self) -> usize {
        0
    }
}
