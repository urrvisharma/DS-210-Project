use crate::reading::read_data_from_file; //importing read file function from reading module
use std::io;
use crate::graph::TwitterGraph; 
mod graph; //graph module
mod reading; //reading module

fn main() {
    if let Ok(graph) = read_data_from_file("src/twitter_combined.txt") { //reading Twitter text file located in src
        println!("The graph has been created.");
        //getting input from the user
        let user1 = get_user_input("Please enter the first user id: ");
        let user2 = get_user_input("Please enter the second user id: ");
        //calculating minimum distance if both user id's are valid
        match graph.min_distance(user1, user2) {
            Some(distance) => println!(
                "Minimum distance between {} and {} is {}",
                user1, user2, distance
            ),
            None => println!("At least one of the users is not found in the graph. Please enter a valid user id."),
        }
        //calculating maximum distance if both user id's are valid
        match graph.max_distance(user1, user2) {
            Some(distance) => println!(
                "Maximum distance between {} and {} is {}",
                user1, user2, distance
            ),
            None => println!("At least one of the users is not found in the graph. Please enter a valid user id."),
        }
        //calculating the average distance from one user to every other user in the graph
        let user_for_average = get_user_input("Enter another user id: ");
        match graph.average_distance_of_user(user_for_average) {
            Some(average_distance) => println!(
                "The average distance of user {} to all other IDs in the graph is {:.2}",
                user_for_average, average_distance
            ),
            None => println!("This user is not found in the graph. Please enter a valid user id."),
        }
    } else {
        println!("Error reading data from file");
    }
}
//the function below retrieves the input from the user
//uses a loop to prompt user to enter a valid response
fn get_user_input(prompt: &str) -> u64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

//tests for functions
#[test]
    fn test_add_edge() {
        let mut graph = TwitterGraph::new();

        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        assert_eq!(graph.graph.node_count(), 3);
        assert_eq!(graph.graph.edge_count(), 2);
    }

    #[test]
    fn test_min_distance() {
        let mut graph = TwitterGraph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        assert_eq!(graph.min_distance(1, 4), Some(3));
        assert_eq!(graph.min_distance(1, 5), None); 
    }

    #[test]
    fn test_max_distance() {
        let mut graph = TwitterGraph::new();
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        assert_eq!(graph.max_distance(1, 4), Some(3));
        assert_eq!(graph.max_distance(1, 5), None); 

    }