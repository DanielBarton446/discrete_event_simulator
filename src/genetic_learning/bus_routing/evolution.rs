// This file is intended for defining all of the traits relevent to simulating evolution.
// This entails:
// DNA
// Population ()
// Generation (?)
// Evolution (natural selection into the next generation)
//

use std::fmt::Display;

use rand::seq::SliceRandom;

pub struct DnaString {
    dna: String,
}

impl DnaString {
    pub fn new(dna: String) -> DnaString {
        DnaString { dna }
    }
    pub fn get_dna(&self) -> &String {
        &self.dna
    }
}

pub trait Dna {
    fn get_dna(&self) -> serde_json::Value; // should this really be a string?
}

pub trait Fitness<F> {
    fn evaluate_fitness(&self) -> f32;
    // should have more functions in here?
}

pub trait Reproductor<T>
where
    Self: Sized,
{
    fn reproduce(&self, second: &serde_json::Value) -> Result<Self, String>;
}

pub struct Population<T>
where
    T: Dna,
    T: Fitness<T>,
    T: Display,
    T: Reproductor<T>,
{
    pub pop: Vec<T>,
}

// what is a population:
// simply a vec of anything which implmenets both DNA and Fitness
impl<T> Population<T>
where
    T: Dna,
    T: Fitness<T>,
    T: Display,
    T: Reproductor<T>,
{
    pub fn new(pop: Vec<T>) -> Population<T> {
        Population { pop }
    }

    pub fn reproduction(&self, first: &T, second: &T) -> Result<T, String> {
        let child = first.reproduce(&second.get_dna());
        match child {
            Ok(child) => Ok(child),
            Err(e) => Err(e),
        }
    }
}

pub struct Evolution<T>
where
    T: Dna,
    T: Fitness<T>,
    T: Display,
    T: Reproductor<T>,
{
    pub population: Population<T>,
    pub generation: u32,
}

impl<T> Evolution<T>
where
    T: Dna,
    T: Display,
    T: Fitness<T>,
    T: Reproductor<T>,
{
    pub fn new(population: Population<T>) -> Evolution<T> {
        Evolution {
            population,
            generation: 0,
        }
    }
    // // Mutate cuz why not
    pub fn evolve(&mut self) {
        let new = self.reproduction();
        for individual in new {
            println!("new individual: {}", individual);
        }
    }

    // For now, this is going to return a copy of the top X percent.
    // In the future, we could just sort and drain.

    // This is really bad.....
    pub fn selection(&self) -> Vec<f32> {
        let mut max_fitness = 0.0;
        let mut weights = Vec::new();

        for individual in &self.population.pop {
            let fitness = individual.evaluate_fitness();
            if fitness > max_fitness {
                max_fitness = fitness;
            }
            weights.push(fitness);
        }
        // normalize
        weights
            .iter_mut()
            .for_each(|i| *i = (*i / max_fitness * 100.0));
        weights
    }

    pub fn reproduction(&mut self) -> Vec<T> {
        // we should set up the whole
        // reproduce

        let gene_weights = self.selection();
        let pop_size = self.population.pop.len();
        let mut rng = rand::thread_rng();

        let mut new_pop = Vec::new();
        for _ in 0..pop_size {
            // TODO: sample only when larget than min weight
            let _min_mom_weight = gene_weights.choose(&mut rng);
            let _min_dad_weight = gene_weights.choose(&mut rng);

            // TODO: sample only when larget than min weight
            let mom = self.population.pop.choose(&mut rng).unwrap();
            let dad = self.population.pop.choose(&mut rng).unwrap();
            let child = self.population.reproduction(mom, dad);
            match child {
                Ok(child) => new_pop.push(child),
                Err(e) => println!("error: {}", e),
            }
        }
        new_pop
    }
}
