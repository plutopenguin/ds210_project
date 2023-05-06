use std::collections::VecDeque;
type Vertex = usize;
use crate::graph::Graph;
use std::io;
pub fn take_input(test_input: Option<&isize>) -> isize {
    let result: isize;
    if let Some(test_input) = test_input {
        //if test_input is not None, this function returns the value in test_input.
        result = *test_input;
    } else {
        //if test_input is None, this function takes a user input and returns that.
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not read input.");
        result = input.trim().parse::<isize>().expect("The input must be an integer.");
    }
    result
}
pub fn compute_distances_bfs(start: Vertex, graph: &Graph) -> Vec<Option<u32>> {
    //This is a modified version of the function compute_and_print_distance_bfs
    //from lecture 28. Instead of printing distances, it returns the Vec<Option<u32>>
    //within which they are stored.
    let mut distance: Vec<Option<u32>> = vec![None;graph.n];
    distance[start] = Some(0);
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        for u in graph.outedges[v].iter() {
            if let None = distance[*u] {
                distance[*u] = Some(distance[v].unwrap() + 1);
                queue.push_back(*u);
            }
        }
    }
    distance
}
pub fn compute_single_distance_bfs(start: Vertex, graph: &Graph, test_input: Option<&isize>) -> String {
    //First, compute_distances_bfs is called.
    let distance = compute_distances_bfs(start, graph);
    //This function takes an input that essentially slices into the variable "distance"
    //and returns the distance between the Vertex "start" and the Vertex "input"
    let input = take_input(test_input);
    let result: String;
    if input > graph.outedges.len() as isize || input < 0 {
        result = format!("The user number must be between 0 and {}.",graph.outedges.len()-1);
    } else if distance[input as usize].unwrap() == 1 {
        //sometimes you get lucky...
        result = format!("Hey, we know each other! {} and {} are 1 edge apart",start,input);
    } else {
        result = format!("{} and {} are {} edges apart",start,input,distance[input as usize].unwrap());
    }
    result
}
pub fn compute_direct_connections_bfs(start: Vertex, graph: &Graph) -> String {
    //First, compute_distances_bfs is called.
    let distance = compute_distances_bfs(start, graph);
    //This function returns the number of users with distance 1 from the Vertex "start"
    //and returns a Vec containing the users.
    let mut stack_direct = Vec::new();
    let mut count_direct = 0;
    for i in 0..distance.len() {
        if distance[i].unwrap() == 1 {
            stack_direct.push(i);
            count_direct += 1;
        }
    }
    let result = format!("Number of direct connections to {}: {}\nDirect connections: {:?}",start,count_direct,stack_direct);
    result
}
pub fn compute_mean_distances_bfs(start: Vertex, graph: &Graph) -> String {
    //First, compute_distances_bfs is called.
    let distance = compute_distances_bfs(start, graph);
    //This function returns the mean of the values in the variable "distance"
    let mut sum = 0;
    let mut count = 0;
    for i in 0..distance.len() {
        match distance[i] {
            Some(value) => {
                sum+=value;
                count+=1;
            } None => {
                continue;
            }
        }
    }
    let result = format!("Mean distance from {} to other users: {}",start,sum as f64 / count as f64);
    result
}
pub fn directory_bfs(graph: &Graph, test_input1: Option<&isize>, test_input2: Option<&isize>, test_input3: Option<&isize>) -> String {
    //This is the function to be called in main.rs.
    //Normally, only the first argument is needed.
    //The last three arguments are to be used for simulating user inputs for testing.
    println!("Your dataset includes users ranging from 0 to {}", graph.outedges.len()-1);
    println!("Choose a user:");
    let input1 = take_input(test_input1);
    if input1 >= graph.outedges.len() as isize || input1 < 0 {
        format!("The user number must be within the given bounds.")
    } else {
        println!("What would you like to see?\n
        1: Distance to a certain other user\n
        2: Direct connections\n
        3: Mean distance to other users\n
        4: Exit program\n");
        let input2 = take_input(test_input2);
        match input2 {
            1 => {
                println!("Enter another user:");  
                let input3 = take_input(test_input3);
                format!("{}",compute_single_distance_bfs(input1 as usize,graph,Some(&input3)))
            } 2 => {
                format!("{}",compute_direct_connections_bfs(input1 as usize,graph))
            } 3 => {
                format!("{}",compute_mean_distances_bfs(input1 as usize,graph))
            } 4 => {
                format!("Goodbye!")
            } _ => {
                format!("The input must be 1, 2, 3, or 4.")
            }
        }
    }
}