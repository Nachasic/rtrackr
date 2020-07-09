mod config;
mod rules;
mod activities;
mod classifier;


pub use self::{
    classifier::{ Classifier, Classifiable },
    config::ClassifierConfig,
};
