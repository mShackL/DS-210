
// Importation of necessary libraries
use std::time::SystemTime;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom;
use rand::Rng;

fn main() {
    // Defining the file path from which data will be read
    let file_path = "CA-GrQc.txt";
    // Read the graph from the specified file
    let graph = read_graph(file_path);

    println!("Computation Times- ");

    // Measure and print the computation time for average distance calculation
    let before_avg_dist = SystemTime::now();
    let avg_dist = average_distance(&graph);
    let after_avg_dist = SystemTime::now();
    print_duration("Average distance", before_avg_dist, after_avg_dist);

    // Measure and print the computation time for maximum distance calculation
    let before_max_dist = SystemTime::now();
    let max_dist = max_distance(&graph);
    let after_max_dist = SystemTime::now();
    print_duration("Maximum distance", before_max_dist, after_max_dist);

    // Measure and print the computation time for median distance calculation
    let before_median_dist = SystemTime::now();
    let median_dist = median_distance(&graph);
    let after_median_dist = SystemTime::now();
    print_duration("Median distance", before_median_dist, after_median_dist);

    // Measure and print the computation time for degree distribution calculation
    let before_degree_dist = SystemTime::now();
    let degree_dis = degree_distribution(&graph);
    let after_degree_dist = SystemTime::now();
    print_duration("Degree distribution", before_degree_dist, after_degree_dist);

    // Printing the calculated values
    println!("Average distance between all node pairs: {:.2}", avg_dist);
    println!("Maximum distance between all node pairs: {:2}", max_dist);
    println!("Median distance between all node pairs: {:.2}", median_dist);
    println!("\nDegree Distribution- ");
    for (degree, count) in &degree_dis {
        println!("Degree {}: Count {}", degree, count);
    }
}


// Function to read the graph from a text file
fn read_graph(file_path: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        if let Ok(line) = line {
            let mut parts = line.split_whitespace();
            if let (Some(node1), Some(node2)) = (parts.next(), parts.next()) {
                graph.entry(node1.to_string()).or_insert(Vec::new()).push(node2.to_string());
            }
        }
    }
    graph
}

// Function to calculate the distance between two nodes using BFS
fn bfs_distance(graph: &HashMap<String, Vec<String>>, start_node: &str, target_node: &str) -> usize {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: Vec<(String, usize)> = vec![(start_node.to_string(), 0)];
    
    while let Some((current_node, distance)) = queue.pop() {
        if &current_node == target_node {
            return distance;
        }
        visited.insert(current_node.clone());
        if let Some(neighbors) = graph.get(&current_node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    queue.push((neighbor.to_string(), distance + 1));
                }
            }
        }
    }
    usize::max_value() 
}

// Function to calculate the average distance between all node pairs
fn average_distance(graph: &HashMap<String, Vec<String>>) -> f64 {
    let nodes: Vec<&String> = graph.keys().collect();
    let mut total_distance = 0;
    let mut pairs_calculated = 0;

    let mut rng = rand::thread_rng();
    
    let start_node = nodes.choose(&mut rng).unwrap();
    let target_node = nodes.choose(&mut rng).unwrap();
    if start_node != target_node {
        let distance = bfs_distance(&graph, start_node, target_node);
        if distance != usize::max_value() {
            total_distance += distance;
            pairs_calculated += 1;
        }
    }
    
    if pairs_calculated == 0 {
        0.0
    } else {
        total_distance as f64 / pairs_calculated as f64
    }
}


// Function to calculate the maximum distance between all node pairs using Floyd-Warshall algorithm
fn max_distance(graph: &HashMap<String, Vec<String>>) -> usize {
    let nodes: Vec<&String> = graph.keys().collect();
    let num_nodes = nodes.len();
    let mut dist = vec![vec![usize::max_value(); num_nodes]; num_nodes];

    for i in 0..num_nodes {
        dist[i][i] = 0;
        if let Some(neighbors) = graph.get(nodes[i]) {
            for neighbor in neighbors {
                if let Some(j) = nodes.iter().position(|&node| node == neighbor) {
                    dist[i][j] = 1;
                }
            }
        }
    }

    for k in 0..num_nodes {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                if dist[i][k] != usize::max_value() && dist[k][j] != usize::max_value() {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    // Finding the maximum distance
    let mut max_dist = 0;
    for i in 0..num_nodes {
        for j in 0..num_nodes {
            if dist[i][j] != usize::max_value() {
                max_dist = max_dist.max(dist[i][j]);
            }
        }
    }

    max_dist
}




// Function to find the median distance between all pairs of nodes
fn median_distance(graph: &HashMap<String, Vec<String>>) -> f64 {
    let nodes = graph.keys().collect::<Vec<_>>();
    let num_nodes = nodes.len();

    let mut distances = vec![vec![usize::max_value(); num_nodes]; num_nodes];

    for (i, node1) in nodes.iter().enumerate() {
        if let Some(neighbors) = graph.get(*node1) {
            for node2 in neighbors {
                if let Some(j) = nodes.iter().position(|&x| x == node2) {
                    distances[i][j] = 1; 
                }
            }
        }
    }

    for k in 0..num_nodes {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                if distances[i][k] != usize::max_value() && distances[k][j] != usize::max_value() {
                    distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
                }
            }
        }
    }

    let mut all_distances = Vec::new();
    for i in 0..num_nodes {
        for j in 0..num_nodes {
            if i != j && distances[i][j] != usize::max_value() {
                all_distances.push(distances[i][j]);
            }
        }
    }

    all_distances.sort_unstable();


    let len = all_distances.len();
    if len % 2 == 0 {
        (all_distances[len / 2 - 1] + all_distances[len / 2]) as f64 / 2.0
    } else {
        all_distances[len / 2] as f64
    }
}


// Function to calculate the degree distribution of the graph
fn degree_distribution(graph: &HashMap<String, Vec<String>>) -> HashMap<usize, usize> {
    let mut degree_dist: HashMap<usize, usize> = HashMap::new();

    // Iterating over each node in the graph
    for (_, neighbors) in graph {
        // Getting the degree of the current node
        let degree = neighbors.len();
        // Incrementing the count of nodes with this degree in the distribution
        let count = degree_dist.entry(degree).or_insert(0);
        *count += 1;
    }

    degree_dist
}

// Function to print the duration of computations
fn print_duration(name: &str, before: SystemTime, after: SystemTime) {
    let difference = after.duration_since(before)
        .expect("Clock may have gone backwards");
    println!("{} took {:?}", name, difference);
}



// Function to generate a random graph for testing cases
fn generate_random_graph(num_nodes: usize) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut rng = rand::thread_rng();

    for i in 0..num_nodes {
        let node = format!("Node{}", i);
        let num_neighbors = rng.gen_range(1..=5); 
        let mut neighbors = Vec::new();
        for _ in 0..num_neighbors {
            let neighbor = format!("Node{}", rng.gen_range(0..num_nodes));
            neighbors.push(neighbor);
        }
        graph.insert(node, neighbors);
    }

    graph
}

// Unit tests for the functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_distance_simple() {
        
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string()]);
        graph.insert("B".to_string(), vec!["A".to_string(), "C".to_string()]);
        graph.insert("C".to_string(), vec!["B".to_string()]);

        let distance = bfs_distance(&graph, "A", "C");

        assert_eq!(distance, 2);
    }

    #[test]
    fn test_bfs_distance_not_connected() {
        
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        graph.insert("A".to_string(), vec!["B".to_string()]);
        graph.insert("B".to_string(), vec!["A".to_string()]);
        graph.insert("C".to_string(), vec!["D".to_string()]);
        graph.insert("D".to_string(), vec!["C".to_string()]);

        let distance = bfs_distance(&graph, "A", "C");

        assert_eq!(distance, usize::max_value());
    }
    
    #[test]
    fn test_average_distance() {
        let graph = generate_random_graph(1000);
    
        let sample_size = 4000;
        let avg_dist = average_distance(&graph);
    
        assert!(avg_dist >= 100.0 && avg_dist <= 1000.0); 
    }

    #[test]
    fn test_max_distance() {
        let graph = generate_random_graph(2000);

        let max_dist = max_distance(&graph);

        assert!(max_dist >= 10 && max_dist <= 30); 
    }

    #[test]
    fn test_median_distance() {
        let graph = generate_random_graph(2000);

        let median_dist = median_distance(&graph);

        assert!(median_dist >= 1.0 && median_dist <= 20.0); 
    }
    
}

