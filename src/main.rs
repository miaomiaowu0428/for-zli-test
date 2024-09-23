use colored::Colorize;
use zino::prelude::*;

fn main() {
    println!("3{}" , "Project Running".blue());
    zino::Cluster::boot().run();
}

