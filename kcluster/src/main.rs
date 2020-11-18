use std::env;
use std::fs;

fn get_file_string(file_name: &String) -> String {
    return fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");
}

fn get_file_vector(file_name: &String) -> Vec<String> {
    return get_file_string(file_name)
        .split("\n")
        .map(|x| String::from(x.strip_suffix("\r").unwrap_or(x)))
        .collect::<Vec<String>>();
}

fn get_points(data: Vec<String>) -> Vec<(i32,i32)> {
    return data
        .into_iter()
        .map(|line: String| -> (i32, i32){
            let hold: Vec<&str> = line.split(" ").collect();
            let x = hold[0].parse::<i32>().unwrap();
            let y = hold[1].parse::<i32>().unwrap();
            return (x,y);
        })
        .collect();
}

fn get_initial_clusters(k: usize, points: &Vec<(i32, i32)>) -> Vec<Vec<(i32,i32)>> {
    let ret: Vec<Vec<(i32,i32)>> = Vec::new();

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

fn main() {
    let args: Vec<String> = env::args().collect();

    // use 1 since args[0] is the program name
    let file: Vec<String> = get_file_vector(&args[1]);

    let k = file[0].parse::<usize>().unwrap();
    let n = file[1].parse::<i32>().unwrap();

    let points: Vec<(i32, i32)> = get_points(file[2..].to_vec());
    let clusters: Vec<Vec<(i32,i32)>> = get_initial_clusters(k, &points);


    println!("{} {} {:?} {:?}", k, n, points, clusters);
}

