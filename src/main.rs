use std::collections::HashMap;
use std::error::Error; 
use std::fmt; 
use csv::ReaderBuilder; 
mod graph; 
use graph::{build_airport, Graph};
use crate::graph::FlightStats;
mod path;
use path::shortest_path;
use std::io::{self, Write}; 

#[derive(Debug, Clone)]
enum columnval { 
    One(String), 
    Two(f64), 
}
#[derive(Debug)]
struct DataFrame{ 
    label: Vec<String>, 
    columns: Vec<Vec<columnval>>, 
    types: Vec<u32>, 

}

// copied from homework 8  starter code
#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for MyError {}
// copied form homework 8  starter code 


impl DataFrame{ 
    fn new() -> Self { 
        DataFrame{ 
            label: Vec::new(), 
            columns: Vec::new(), 
            types: Vec::new(), 

        }
    }
    fn read_csv(&mut self, path: &str, types: &Vec<u32>) -> Result<(),Box<dyn Error>>{ 
        self.types = types.clone(); 
        let mut read = csv::ReaderBuilder::new().delimiter(b',').has_headers(true).from_path(path)?;
        let headers = read.headers()?; 
        self.label = headers.iter().map(|s| s.to_string()).collect(); 

        for result in read.records(){ 
            let r = result?; 
            let mut row: Vec<columnval> = vec![]; 
            for (i , elem) in r.iter().enumerate(){ 
                match types[i]{ 
                    1 => row.push(columnval::One(elem.to_string())), 
                    2 => row.push(columnval::Two(elem.parse::<f64>()?)), 
                    _=> return Err(Box::new(MyError("Unknow type".to_string()))),
                }
            }
            self.columns.push(row)
        }
        Ok(())
    }
    fn print_dataframe(&self) {

        for label in &self.label {
            print!("{:<20}", label);
        }
        println!();


        for row in &self.columns {
            for cell in row {
                match cell {
                    columnval::One(s) => print!("{:<20}", s),
                    columnval::Two(n) => print!("{:<20}", format!("{:.1}", n)),
                }
            }
            println!();
        }
    }

   
    
}



fn main() -> Result<(), Box<dyn Error>> {
    // Load data and read CSV
    let mut df = DataFrame::new();
    let types = vec![1, 1, 2];  
    df.read_csv("Newest.csv", &types)?; 
    

    // Build the graph
    let graph = build_airport(&df);

    // Get user input
    print!("Enter your starting airport code: ");
    io::stdout().flush()?; 
    let mut start = String::new();
    io::stdin().read_line(&mut start)?;
    let start = start.trim();

    print!("Enter your destination airport code ");
    io::stdout().flush()?;
    let mut goal = String::new();
    io::stdin().read_line(&mut goal)?;
    let goal = goal.trim();

    // Find shortest path
    match shortest_path(&graph, start, goal) {
        Some((cost, path)) => {
            println!("Shortest path from {} to {} is:", start, goal);
            println!("Travel Time: {:.2} hours", cost/60.0);
            println!("Shortest path: {:?}", path);
        }
        None => println!("No path found from {} to {}.", start, goal),
    }

    Ok(())
}