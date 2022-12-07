//! This module provides small utility functions and types 
//! that can be used throughout the entire programm


/// A struct for that holds a single vocabulary pair
#[derive(Debug)]
pub struct Pair {
    pub key: String,
    pub definition: String
}


/// Type alias for `Vec<Pair>`
pub type VocabSet = Vec<Pair>;
