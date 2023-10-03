// This file is intended for defining all of the traits relevent to simulating evolution.
// This entails:
// DNA
// Population ()
// Generation (?)
// Evolution (natural selection into the next generation)
//

use std::{fmt::Display, ops::Index};

use rand::{seq::SliceRandom, thread_rng, Rng};

pub trait Dna {
    fn get_dna(&self) -> serde_json::Value; // should this really be a string?
    fn get_species(&self) -> &'static str;
}

pub trait Fitness<F> {
    fn evaluate_fitness(&self) -> f32;
    // should have more functions in here?
}

pub trait Breedable<T>
where
    Self: Sized,
{
    fn reproduce(&self, second: &Self) -> Result<Self, String>;
    fn mutate(&mut self);
}

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

// what is a population:
// a collection of individuals who can reproduce.
impl<T> Population<T>
where
    T: Dna,
    T: Fitness<T>,
    T: Display,
    T: Breedable<T>,
{
    pub fn new(pop: Vec<T>) -> Population<T> {
        Population { populace: pop }
    }

    pub fn len(&self) -> usize {
        self.populace.len()
    }

    pub fn is_empty(&self) -> bool {
        self.populace.is_empty()
    }

    pub fn breed_from_parents(&self, first: &T, second: &T) -> Result<T, String> {
        let child = first.reproduce(second);
        match child {
            Ok(child) => Ok(child),
            Err(e) => Err(e),
        }
    }

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

    fn generation(&self) -> usize {
        0
    }
}
