use ahash::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("day08.txt");

    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let directions = directions
        .chars()
        .map(|it| {
            if it == 'R' {
                Direction::Right
            } else {
                Direction::Left
            }
        })
        .collect_vec();

    let nodes = nodes
        .lines()
        .map(|it| {
            let (parent, children) = it.split_once(" = ").unwrap();
            let (left, right) = children
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(", ")
                .unwrap();

            (parent.to_owned(), (left.to_owned(), right.to_owned()))
        })
        .collect::<HashMap<_, _>>();

    let mut current_nodes = nodes
        .iter()
        .map(|it| (it.0, 0))
        .filter(|it| it.0.ends_with("A"))
        .collect_vec();

    for direction in directions.into_iter().cycle() {
        let mut should_continue = false;

        for (current_node, steps) in &mut current_nodes {
            if !current_node.ends_with("Z") {
                should_continue = true;

                let node = nodes.get(*current_node).unwrap();

                match direction {
                    Direction::Left => *current_node = &node.0,
                    Direction::Right => *current_node = &node.1,
                }

                *steps += 1;
            }
        }

        if !should_continue {
            break;
        }
    }

    println!(
        "{:#?}",
        current_nodes.into_iter().map(|it| it.1).collect_vec()
    );
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}
