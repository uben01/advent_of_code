use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::{HashSet};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Connection {
    a: String,
    b: String,
}

impl Connection {
    fn new(a: String, b: String) -> Connection {
        Connection {
            a: String::min(a.clone(), b.clone()),
            b: String::max(a, b),
        }
    }
}

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut components: HashSet<Connection> = HashSet::new();
    for line in buff.lines() {
        let line = line?;

        let mut split = line.split(':');
        let name = split.next().unwrap().trim().to_string();
        let component: Vec<String> = split.next().unwrap().split(' ').skip(1).map(|a| a.trim().to_string()).collect();

        for component in component {
            components.insert(Connection::new(name.clone(), component));
        }
    }

    println!("{:?}", find_connections_to_split(&components));

    return Ok(());
}

fn find_connections_to_split(connections: &HashSet<Connection>) -> Option<usize> {
    // find a, b, c with
    // print all connections to graphviz format
    // for connection in connections {
    //    println!("{} -> {}", connection.a, connection.b);
    // }
    // $ dot -Kneato -Tsvg result.txt > output.svg
    // visually find the separating 3 edges
    // result: vgs->xjb, ffj->lkm, ljl->xhg

    let a = &Connection::new("xjb".to_string(), "vgs".to_string());
    let b = &Connection::new("ffj".to_string(), "lkm".to_string());
    let c = &Connection::new("xhg".to_string(), "ljl".to_string());

    // for (a, b, c) in connections.iter().tuple_combinations() {
        let lefts: HashSet<Vec<String>> = [a, b, c].iter().map(|connection| walk_graph_from_point(&connection.a, &connections, a, b, c)).collect();
        let mut rights: HashSet<Vec<String>> = [a, b, c].iter().map(|connection| walk_graph_from_point(&connection.b, &connections, a, b, c)).collect();
        let graphs: Vec<&Vec<String>> = lefts.union(&mut rights).collect();

        if graphs.len() == 2 {
            return Some(graphs[0].len() * graphs[1].len());
        }
    // }

    return None;
}

fn walk_graph_from_point(point: &String, connections: &HashSet<Connection>, a: &Connection, b: &Connection, c: &Connection) -> Vec<String> {
    let mut points = HashSet::new();
    points.insert(point.clone());
    let mut queue = vec![point.clone()];

    while !queue.is_empty() {
        let element = queue.pop().unwrap();

        for connection in connections {
            if connection == a || connection == b || connection == c {
                continue;
            }
            if connection.a == element {
                if !points.contains(&connection.b) {
                    queue.push(connection.b.clone());
                    points.insert(connection.b.clone());
                }
            } else if connection.b == element {
                if !points.contains(&connection.a) {
                    queue.push(connection.a.clone());
                    points.insert(connection.a.clone());
                }
            }
        }
    }

    let mut vec = Vec::from_iter(points.iter().cloned());
    vec.sort();
    return vec;
}
