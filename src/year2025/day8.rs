use std::{collections::HashSet, fs::read_to_string};

type Point3D = (i64, i64, i64);

pub fn part1() {
    let input = read_to_string("input.txt").unwrap();
    let junction_boxes: HashSet<Point3D> =
        HashSet::from_iter(input.trim().split('\n').map(|line| {
            let parts: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();
            (parts[0], parts[1], parts[2])
        }));

    /*let mut closest_boxes_pairs: HashMap<Point3D, Point3D> = HashMap::new();
    for junction_box in &junction_boxes {
        if closest_boxes_pairs.contains_key(junction_box) {
            continue;
        }

        let mut closest_box = (i64::MAX, i64::MAX, i64::MAX);
        let mut closest_distance: i64 = i64::MAX;

        for other_box in &junction_boxes {
            if *junction_box == *other_box {
                continue;
            }
            let distance = ((junction_box.0 - other_box.0).pow(2)
                + (junction_box.1 - other_box.1).pow(2)
                + (junction_box.2 - other_box.2).pow(2))
            .isqrt();
            if distance < closest_distance {
                closest_distance = distance;
                closest_box = *other_box;
            }
        }
        closest_boxes_pairs.insert(*junction_box, closest_box);
        closest_boxes_pairs.insert(closest_box, *junction_box);
    }

    // We need to start connecting them all into circuits
    let unjoined_boxes: HashSet<Point3D> = junction_boxes.clone();
    let junction_box_circuits: Vec<HashSet<Point3D>> = Vec::new();

    for (box1, box2) in &closest_boxes_pairs {
        // FIXME
        // If one is in a circuit, add the other
        // If both are in different circuits, merge them
        // If both are not in any circuit, create a new one
    }*/

    // Turns out I was solving the wrong problem
    let mut closest_boxes_pairs: Vec<(Point3D, Point3D, i64)> = Vec::new();
    for (n, junction_box) in junction_boxes.iter().enumerate() {
        for other_box in junction_boxes.iter().skip(n + 1) {
            let distance = ((junction_box.0 - other_box.0).pow(2)
                + (junction_box.1 - other_box.1).pow(2)
                + (junction_box.2 - other_box.2).pow(2))
            .isqrt();
            closest_boxes_pairs.push((*junction_box, *other_box, distance));
        }
    }
    closest_boxes_pairs.sort_by_key(|k| k.2);
    closest_boxes_pairs.truncate(1000);

    // We need to start connecting them all into circuits
    let mut junction_box_circuits: Vec<HashSet<Point3D>> = Vec::new();

    for (box1, box2, _) in &closest_boxes_pairs {
        let mut circuit1: Option<&mut HashSet<Point3D>> = None;
        let mut circuit2: Option<&mut HashSet<Point3D>> = None;
        for circuit in &mut junction_box_circuits {
            if circuit.contains(box1) {
                circuit1 = Some(circuit);
            } else if circuit.contains(box2) {
                circuit2 = Some(circuit);
            }
        }
        match (circuit1, circuit2) {
            (Some(c1), Some(c2)) => {
                if c1 != c2 {
                    let to_merge = c2.clone();
                    for b in to_merge {
                        c1.insert(b);
                    }
                    c2.clear(); // Just leave empty circuits
                }
            }
            (Some(c1), None) => {
                c1.insert(*box2);
            }
            (None, Some(c2)) => {
                c2.insert(*box1);
            }
            (None, None) => {
                junction_box_circuits.push(HashSet::from([*box1, *box2]));
            }
        }
    }

    // Get the 3 largest circuits and multiply their sizes
    junction_box_circuits.sort_by_key(|k| 1000 - k.len());
    let result = junction_box_circuits
        .iter()
        .take(3)
        .map(|c| c.len() as i64)
        .product::<i64>();
    println!("Result: {}", result);
}
