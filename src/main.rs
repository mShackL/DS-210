
fn main() {
    let file_path = "CA-GrQc.txt"; // Holds path to file from which data will be read
    let graph = read_graph(file_path);
    let sample_size = 1000;    // Setting sample size from specified text file
    let avg_dist = average_distance(&graph, sample_size);
    let max_dist = max_distance(&graph);
    let median_dist = median_distance(&graph);
    let degree_dis = degree_distribution(&graph);
    // let std_dev_dist = standard_deviation_distance(&graph);
    // let b_centrality = betweenness_centrality(&graph);

    println!("Random sample of 1000 node pairs-\n ");
    println!("Average distance between all node pairs: {:.2}", avg_dist);
    println!("Maxium distance between all node pairs: {:2}", max_dist);
    println!("Median distance between all node pairs: {:.2}", median_dist);
    // println!("Standard deviation of distance between all node pairs: {:.2}", std_dev_dist);
    println!("\nDegree Distribution- ");
    for (degree, count) in &degree_dis {
        println!("Degree {}: Count {}", degree, count);
    }
    
}

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom;

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

fn average_distance(graph: &HashMap<String, Vec<String>>, sample_size: usize) -> f64 {
    let nodes: Vec<&String> = graph.keys().collect();
    let mut total_distance = 0;
    let mut pairs_calculated = 0;

    let mut rng = rand::thread_rng();
    
    for _ in 0..sample_size {
        let start_node = nodes.choose(&mut rng).unwrap();
        let target_node = nodes.choose(&mut rng).unwrap();
        if start_node != target_node {
            let distance = bfs_distance(&graph, start_node, target_node);
            if distance != usize::max_value() {
                total_distance += distance;
                pairs_calculated += 1;
            }
        }
    }
    
    if pairs_calculated == 0 {
        0.0
    } else {
        total_distance as f64 / pairs_calculated as f64
    }
}



// Max distance function using the Floyd-Warshall algorithm
fn max_distance(graph: &HashMap<String, Vec<String>>) -> usize {
    let nodes = graph.keys().collect::<Vec<_>>();
    let num_nodes = nodes.len();

    
    let mut distances = vec![vec![usize::max_value(); num_nodes]; num_nodes];
    
    for i in 0..num_nodes {
        distances[i][i] = 0;
    }

    for (i, node1) in nodes.iter().enumerate() {
        if let Some(neighbors) = graph.get(*node1) {
            for node2 in neighbors {
                if let Some(j) = nodes.iter().position(|&x| x == node2) {
                    distances[i][j] = 1; // Assuming unweighted graph
                }
            }
        }
    }

    // Floyd-Warshall algorithm
    for k in 0..num_nodes {
        for i in 0..num_nodes {
            for j in 0..num_nodes {
                if distances[i][k] != usize::max_value() && distances[k][j] != usize::max_value() {
                    distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
                }
            }
        }
    }

    // Find the maximum distance
    let mut max_dist = 0;
    for i in 0..num_nodes {
        for j in 0..num_nodes {
            if distances[i][j] != usize::max_value() && distances[i][j] > max_dist {
                max_dist = distances[i][j];
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


fn degree_distribution(graph: &HashMap<String, Vec<String>>) -> HashMap<usize, usize> {
    let mut degree_dist: HashMap<usize, usize> = HashMap::new();

    // Iterate over each node in the graph
    for (_, neighbors) in graph {
        // Get the degree of the current node
        let degree = neighbors.len();
        // Increment the count of nodes with this degree in the distribution
        let count = degree_dist.entry(degree).or_insert(0);
        *count += 1;
    }

    degree_dist
}


// fn standard_deviation_distance(graph: &HashMap<String, Vec<String>>) -> f64 {
//     let mut distances: Vec<usize> = Vec::new();

//     for (start_node, _) in graph.iter() {
//         for (target_node, _) in graph.iter() {
//             if start_node != target_node {
//                 let distance = bfs_distance(&graph, start_node, target_node);
//                 distances.push(distance);
//             }
//         }
//     }

//     let mean_distance: f64 = distances.iter().sum::<usize>() as f64 / distances.len() as f64;
//     let variance: f64 = distances.iter().map(|&x| ((x as f64) - mean_distance).powi(2)).sum::<f64>() / distances.len() as f64;
//     variance.sqrt()
// }




// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_bfs_distance_simple() {
        
//         let mut graph: HashMap<String, Vec<String>> = HashMap::new();
//         graph.insert("A".to_string(), vec!["B".to_string()]);
//         graph.insert("B".to_string(), vec!["A".to_string(), "C".to_string()]);
//         graph.insert("C".to_string(), vec!["B".to_string()]);

//         let distance = bfs_distance(&graph, "A", "C");

//         assert_eq!(distance, 2);
//     }

//     #[test]
//     fn test_bfs_distance_not_connected() {
        
//         let mut graph: HashMap<String, Vec<String>> = HashMap::new();
//         graph.insert("A".to_string(), vec!["B".to_string()]);
//         graph.insert("B".to_string(), vec!["A".to_string()]);
//         graph.insert("C".to_string(), vec!["D".to_string()]);
//         graph.insert("D".to_string(), vec!["C".to_string()]);

//         let distance = bfs_distance(&graph, "A", "C");

//         assert_eq!(distance, usize::max_value());
//     }

    
// }

