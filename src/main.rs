mod graph;
mod bfs;
use crate::graph::Graph;
use crate::bfs::directory_bfs;
use std::fs::File;
use csv::ReaderBuilder;

fn read_file(filename: &str) -> Graph {
    //This function takes a csv file with a header and lines in the format
    //from,to where "from" and "to" are whole numbers
    let file = File::open(filename).expect("Could not open file");
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(file);
    csv_reader.headers().expect("Could not read headers");
    //outedges is a Vec that stores each line of the file as a separate 
    //tuple containing pairs of usize values
    let mut outedges: Vec<(usize, usize)> = Vec::new();
    for line in csv_reader.records() {
        let record = line.expect("Could not read line");
        let from = record[0].parse().expect("Could not parse first user");
        let to = record[1].parse().expect("Could not parse second user");
        outedges.push((from, to));
    }
    //Next, the maximum value in the dataset is found for
    //the first argument of Graph::create_undirected
    let mut max = 0 as usize;
    for (from, to) in &outedges {
        if from > &max {
            max = *from;
        }
        if to > &max {
            max = *to;
        }
    }
    let data = Graph::create_undirected(max+1, &outedges);
    data
}

//the following tests use the same file used in the main function
#[test]
fn test_good() {
    //test function for valid inputs
    let new = read_file(r"C:\Users\taiyo\OneDrive\Desktop\ds210_project\project\twitch\twitch\DE\musae_DE_edges.csv");
    //tests compute_single_distance_bfs()
    assert_eq!(directory_bfs(&new,Some(&792),Some(&1),Some(&458)), "792 and 458 are 3 edges apart");
    //tests the hidden gem in compute_single_distance_bfs()
    assert_eq!(directory_bfs(&new,Some(&943),Some(&1),Some(&540)), "Hey, we know each other! 943 and 540 are 1 edge apart");
    //tests compute_direct_connections()
    assert_eq!(directory_bfs(&new,Some(&0),Some(&2),None),"Number of direct connections to 0: 7\nDirect connections: [2145, 2498, 2684, 5358, 7275, 7787, 9206]");
    //tests compute_mean_distances()
    assert_eq!(directory_bfs(&new,Some(&2),Some(&3),None),"Mean distance from 2 to other users: 2.3966098125921245");
}
#[test]
fn test_bad() {
    //test function for invalid integer inputs
    let new = read_file(r"C:\Users\taiyo\OneDrive\Desktop\ds210_project\project\twitch\twitch\DE\musae_DE_edges.csv");
    //invalid 1st input
    assert_eq!(directory_bfs(&new,Some(&-92),Some(&2),None),"The user number must be within the given bounds.");
    //invalid 2nd input
    assert_eq!(directory_bfs(&new,Some(&8634),Some(&-5),None),"The input must be 1, 2, 3, or 4.");
    //invalid 3rd input, i.e. invalid input for compute_single_distance_bfs()
    assert_eq!(directory_bfs(&new,Some(&234),Some(&1),Some(&9876)),"The user number must be between 0 and 9497.");
}
//the program panics with non-integer inputs as expected, but having a non-integer in any of the Option<&isize> parameters
//prevents the program from compiling so it cannot be demonstrated with test functions.

fn main() {
    let new = read_file(r"C:\Users\taiyo\OneDrive\Desktop\ds210_project\project\twitch\twitch\DE\musae_DE_edges.csv");
    println!("{}",directory_bfs(&new,None,None,None));
}