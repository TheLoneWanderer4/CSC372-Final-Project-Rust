use std::env;
use std::fs;

/*
    Name : Amin Sennour and Mahmood Gladney
    Class : CSC 372
    Assigment : Final Project -> Part 2
    File : main.rs
    Instructor : Dr. Mccann 
    Due Date : November 23rd 2020 
    Description : 
        Document can be found at : 
        https://github.com/TheLoneWanderer4/CSC372-Final-Project-Rust
    
        A program that implments "k-means" clustering on a set of xy coordinates

        the program produces as output the final k-clusters and their centroids, as well as
        how man itterations it took to compute them.
    Requirments :
        Language : Rust
        Extra :
            None
    Problems :
        There is no input validation, so an incorrectly structured, or missing, input file
        will produce unpredictable erros. 
*/

/*
 * Purpose : open a file of name : file_name, and return the contents of that 
 *           file as a string.
 */
fn get_file_string(file_name: &String) -> String {
    return fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");
}

/*
 * Purpose : take a file name and return a vector containing each of the "lines"
 *           of that file their newlines and cardige return characters removed. 
 * Params : 
 *    file_name : the name of the file to parse 
 * Return : 
 *    a vector containing the cleaned lines of the file "file_name"
 */
fn get_file_vector(file_name: &String) -> Vec<String> {
    return get_file_string(file_name)
        .split("\n")
        // adding an extra split to remove the "\r" of windows files 
        .map(|x| String::from(x.strip_suffix("\r").unwrap_or(x)))
        .collect::<Vec<String>>();
}

/*
 * Purpose : given a vector of strings where each string is of the form 
 *           "f32 f32" compute a vector of points (f32, f32) representing the 
 *           data from each string 
 *
 *           this function assumes the strings in data are well formated, and 
 *           will produce errors if they are not 
 * Params : 
 *    data : the vector of strings
 * Return : 
 *    a vector of (f32, f32)
 */
fn get_points(data: Vec<String>) -> Vec<(f32,f32)> {
    return data
        .into_iter()
        .map(|line: String| -> (f32, f32){
            let hold: Vec<&str> = line.split(" ").collect();
            let x = hold[0].parse::<f32>().unwrap();
            let y = hold[1].parse::<f32>().unwrap();
            return (x,y);
        })
        .collect();
}

/*
 * Purpose : take a value k and vector of points and return the inital k clusters from the points
 *           this function assumes k < the length of points, and will produce errors if it is not
 * Params : 
 *  k       : the number of clusters to make. This is assumed to be less than the length of points 
 *  points  : the vector of points used to generate the initial clusters 
 * Return : 
 *  a vector of k vectors of length one each containing one value from points 0 through k-1 
 */ 
fn get_initial_clusters(k: usize, points: &Vec<(f32, f32)>) -> Vec<Vec<(f32,f32)>> {
    if k == 0 {
        return Vec::new();
    } else {
        let mut new = Vec::new();
        new.push(points[k-1]);
        let mut ret = get_initial_clusters(k-1, points);
        ret.push(new);
        return ret;
    }
}

/*
 * Purpose : compute the centroid of a given vector of points 
 *           a centroid is defined as a tuple of the mean of the x and the mean of the y
 *           values of the points in the vector. 
 * Params : 
 *  cluster : the vector to compute the centroid of 
 * Return : (mean of x, mean of y)
 */
fn compute_centroid(cluster: &Vec<(f32, f32)>) -> (f32, f32) {
    return (
        cluster.iter().fold(0.0, |acc, x| acc + x.0) / cluster.len() as f32,
        cluster.iter().fold(0.0, |acc, x| acc + x.1) / cluster.len() as f32
    );
}
/*
 * Purpose : Takes in two cluster and compares them to see if they contain the same values,
 *           if so, return true as the cluster is consider stable
 * Params : 
 *  cluster_a : a vector of vector of tuples with two floats, repersenting the x,y positions
 *  cluster_b : a vector of vector of tuples with two floats, repersenting the x,y positions
 * Return : true if the clusters are the same, false otherwise
 */
fn cluster_stable(cluster_a: &Vec<Vec<(f32,f32)>>, cluster_b: &Vec<Vec<(f32,f32)>>) -> bool{
    let matching = cluster_a.iter().zip(cluster_b.iter()).filter(|(a, b)| is_stabe(a, b)).count();
    return matching == cluster_a.len();
}
/*
 * Purpose : Similar to cluster_stable, but compares the internal vectors of the clusters to 
 *           see if they contain the same values. Helper function for cluster_stable   
 * Params : 
 *  list_a : a vector of tuples with two floats, repersenting the x,y positions
 *  list_b : a vector of tuples with two floats, repersenting the x,y positions
 * Return : true if the two vectors are the same, false otherwise
 */
fn is_stabe(list_a: &Vec<(f32, f32)>, list_b: &Vec<(f32, f32)>) -> bool{
    let matching = list_a.iter().zip(list_b.iter()).filter(|&(a, b)| a == b).count();
    return matching == list_a.len();
}
/*
 * Purpose : Takes in a list of centroids, and a data point, and returns the index of the
 *           centroid that the data point should be assigned to.
 * Params : 
 *  centroids : a vector of centroids which are a tuple of two float val (x,y)
 *  data_point: a tuple of two float vals, (x,y)
 * Return : an integer index of that points to the centroid point that the data point
 * should be assigned to its cluster
 */
fn assgn_to_cluster(centroids: &Vec<(f32,f32)>, data_point: (f32,f32)) -> usize{
    let mut smallest_dist = compute_distance(centroids[0], data_point);
    let mut index = 0;
    let mut i = 1;
    while i < centroids.len(){
        let temp_dist = compute_distance(centroids[i], data_point);
        if temp_dist < smallest_dist{
            smallest_dist = temp_dist;
            index = i;
        }
        i = i + 1;
    }
    return index;
} 
/*
 * Purpose : compute the distance between a centroid point and a data point
 *           as define by the spec. 
 * Params : 
 *  centroid : tuple of two float val (x,y)
 *  data_point: a tuple of two float vals, (x,y)
 * Return : a float value repersenting the distance between the two points
 */
fn compute_distance(centroid: (f32, f32), data_point: (f32,f32)) -> f32{
    return (data_point.0 - centroid.0).powi(2) + (data_point.1 - centroid.1).powi(2);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // use 1 since args[0] is the program name
    let file: Vec<String> = get_file_vector(&args[1]);

    let k = file[0].parse::<usize>().unwrap();
    let n = file[1].parse::<usize>().unwrap();

    // error checking
    if k > n {
        println!("Can't have more clusters than data points");
        return;
    }

    // declare state variables 
    let points: Vec<(f32, f32)> = get_points(file[2..].to_vec());
    let mut previous_clusters: Vec<Vec<(f32,f32)>> = Vec::new();
    let mut clusters: Vec<Vec<(f32,f32)>> = get_initial_clusters(k, &points);
    let mut iterations = 0;

    // main loop
    while !cluster_stable(&clusters, &previous_clusters) {
        previous_clusters = clusters.clone();
        let centroids: Vec<(f32,f32)> = previous_clusters.iter().map(|x| compute_centroid(x)).collect();

        clusters = clusters.iter().map(|_| Vec::new()).collect();
        for x in &points {
            let new_index = assgn_to_cluster(&centroids, *x);
            clusters[new_index].push(*x);
        }
        iterations+=1;
    }

    // produce required output 
    
    println!("The final centroid locations are:\n");
    for i in 0..clusters.len() {
        println!("u({}) = {:?}",i+1, compute_centroid(&clusters[i]));
    }
    println!("\n{} iterations were required.", iterations);
}

