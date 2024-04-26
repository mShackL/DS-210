
fn main() {
    let file_path = "web-Stanford.txt"; // Holds path to file from which data will be read
    let graph = read_graph(file_path);
    let sample_size = 100;    // Setting sample size from specified text file
    let avg_dist = average_distance(&graph, sample_size);
    let max_dist = max_distance(&graph);
    let median_dist = median_distance(&graph);
    let std_dev_dist = standard_deviation_distance(&graph);
    let c_centrality = closeness_centrality(&graph);

    println!("Average distance between all node pairs: {:.2}", avg_dist);
    println!("Maxium distance between all node pairs: {:2}", max_dist);
    println!("Median distance between all node pairs: {:.2}", median_dist);
    println!("Standard deviation of distance between all node pairs: {:.2}", std_dev_dist);
    for (node, centrality) in c_centrality.iter() {
        println!("The closeness centrality of node {}: {:.2}", node, centrality);
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

// Function to find the maximum distance between all pairs of nodes
fn max_distance(graph: &HashMap<String, Vec<String>>) -> usize {
    let mut max_dist = 0;

    for (start_node, _) in graph.iter() {
        for (target_node, _) in graph.iter() {
            if start_node != target_node {
                let distance = bfs_distance(&graph, start_node, target_node);
                if distance > max_dist {
                    max_dist = distance;
                }
            }
        }
    }
    max_dist
}


// Function to find the median distance between all pairs of nodes
fn median_distance(graph: &HashMap<String, Vec<String>>) -> f64 {
    let mut distances: Vec<usize> = Vec::new();

    for (start_node, _) in graph.iter() {
        for (target_node, _) in graph.iter() {
            if start_node != target_node {
                let distance = bfs_distance(&graph, start_node, target_node);
                distances.push(distance);
            }
        }
    }
    distances.sort_unstable();
    let len = distances.len();
    if len % 2 == 0 {
        (distances[len / 2 - 1] + distances[len / 2]) as f64 / 2.0
    } else {
        distances[len / 2] as f64
    }
}


fn standard_deviation_distance(graph: &HashMap<String, Vec<String>>) -> f64 {
    let mut distances: Vec<usize> = Vec::new();

    for (start_node, _) in graph.iter() {
        for (target_node, _) in graph.iter() {
            if start_node != target_node {
                let distance = bfs_distance(&graph, start_node, target_node);
                distances.push(distance);
            }
        }
    }

    let mean_distance: f64 = distances.iter().sum::<usize>() as f64 / distances.len() as f64;
    let variance: f64 = distances.iter().map(|&x| ((x as f64) - mean_distance).powi(2)).sum::<f64>() / distances.len() as f64;
    variance.sqrt()
}



fn closeness_centrality(graph: &HashMap<String, Vec<String>>) -> HashMap<String, f64> {
    let mut closeness_centralities: HashMap<String, f64> = HashMap::new();

    for (node, _) in graph.iter() {
        let mut total_distance = 0;
        let mut close_nodes_count = 0;

        for (other_node, _) in graph.iter() {
            if node != other_node {
                let distance = bfs_distance(graph, node, other_node);
                if distance != usize::max_value() {
                    total_distance += distance;
                    close_nodes_count += 1;
                }
            }
        }

        if close_nodes_count > 0 {
            let closeness_centrality = (close_nodes_count as f64) / (total_distance as f64);
            closeness_centralities.insert(node.clone(), closeness_centrality);
        } else {
            closeness_centralities.insert(node.clone(), 0.0);
        }
    }

    closeness_centralities
}




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

    
}

