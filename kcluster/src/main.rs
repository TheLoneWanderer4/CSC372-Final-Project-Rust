use std::env;
use std::fs;

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

//NOTE: DO .copy ON ANY VECTOR YOU'RE PASSING IN, OTHERWISE RUST DOES WEIRD VODO MAGIC I DONT UNDERSTAND WITH TRANSFERING OWERNSHIP AND CAUSES AN ERROR
fn cluster_stable(cluster_a: &Vec<Vec<(f32,f32)>>, cluster_b: &Vec<Vec<(f32,f32)>>) -> bool{
    let matching = cluster_a.iter().zip(cluster_b.iter()).filter(|(a, b)| is_stabe(a, b)).count();
    return matching == cluster_a.len();
}

fn is_stabe(list_a: &Vec<(f32, f32)>, list_b: &Vec<(f32, f32)>) -> bool{
    let matching = list_a.iter().zip(list_b.iter()).filter(|&(a, b)| a == b).count();
    return matching == list_a.len();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // use 1 since args[0] is the program name
    let file: Vec<String> = get_file_vector(&args[1]);

    let k = file[0].parse::<usize>().unwrap();
    let n = file[1].parse::<usize>().unwrap();

    if k > n {
        println!("Can't have more clusters than data points");
        return;
    }

    let points: Vec<(f32, f32)> = get_points(file[2..].to_vec());
    let mut previous_clusters: Vec<Vec<(f32,f32)>> = Vec::new();
    let clusters: Vec<Vec<(f32,f32)>> = get_initial_clusters(k, &points);

    while !cluster_stable(&clusters, &previous_clusters) {
        previous_clusters = clusters.clone();
        
        // compute modifications to clusters 
        
        println!("{:?}", compute_centroid(&points));
    }
}

