use std::collections::HashMap;

#[test]
fn test_day6() {
    assert_eq!(
        part1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"),
        "42"
    );

    assert_eq!(
        part2("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"),
        "4"
    );
}

#[derive(Debug)]
struct Object<'a> {
    name: &'a str,
    depth: usize,
    children: Vec<&'a str>,
    parent: Option<&'a str>,
}

/// this will add a `depth` to all the nodes while its traversing
fn traverse_subtree<'a>(
    // well… there was my first real fight with the borrow checker…
    // recursive graph-like structures sure are a pain to work with…
    mut objects: &mut HashMap<&'a str, Object<'a>>,
    node: &'a str,
    depth: usize,
) {
    let node = objects.get_mut(node).unwrap();
    node.depth = depth;
    let children = node.children.clone(); // <- because we can’t have this borrow alive later :-(

    for child in &children {
        traverse_subtree(&mut objects, child, depth + 1);
    }
}

fn read_objects<'a>(input: &'a str) -> HashMap<&'a str, Object<'a>> {
    let direct_orbits = input.trim().lines().map(|line| {
        let mut pair = line.split(')');
        (
            pair.next().expect("expected object"),
            pair.next().expect("expected object"),
        )
    });

    // lets first create some linked nodes…
    let mut objects = HashMap::new();
    for (a, b) in direct_orbits {
        objects.entry(b).or_insert(Object {
            name: b,
            depth: 0,
            children: vec![],
            parent: None,
        });

        let a_obj = objects.entry(a).or_insert(Object {
            name: a,
            depth: 0,
            children: vec![],
            parent: None,
        });
        // b orbits a, so add it as a's child
        a_obj.children.push(b);
        // and the reverse parent relation
        objects.entry(b).and_modify(|b| b.parent = Some(a));
    }

    // find the `root`, which is the node that has no parent
    let root = find_root(&objects);

    // add `depth` everywhere
    traverse_subtree(&mut objects, root, 0);

    objects
}

fn find_root<'a>(objects: &HashMap<&'a str, Object<'a>>) -> &'a str {
    objects
        .values()
        .find(|e| e.parent.is_none())
        .expect("expected a root node")
        .name
}

pub fn part1(input: &str) -> String {
    let objects = read_objects(input);

    // sum up all the depths
    objects.values().map(|n| n.depth).sum::<usize>().to_string()
}

fn trace_path<'a>(
    objects: &HashMap<&'a str, Object<'a>>,
    mut path: Vec<&'a str>,
    name: &'a str,
) -> Vec<&'a str> {
    let node = &objects[name];
    path.push(name);
    match node.parent {
        Some(parent) => trace_path(objects, path, parent),
        None => {
            path.reverse();
            path
        }
    }
}

pub fn part2(input: &str) -> String {
    let objects = read_objects(input);

    // so, since this is a tree, we just need to find the first common ancestor…
    let mut path_to_you = trace_path(&objects, vec![], "YOU");
    let mut path_to_san = trace_path(&objects, vec![], "SAN");

    let common_ancestor = path_to_san
        .iter()
        .zip(path_to_you.iter())
        .take_while(|(a, b)| a == b)
        .last()
        .unwrap()
        .0
        .clone(); // <- say whaaaat? I need to clone the `&str` because otherwise
                  // I can’t pop from `path_to_san`?

    // and essentially, the number of traversals is:
    // depth of first parent of YOU relative to common ancestor
    // +
    // depth of first parent of SAN relative to common ancestor
    path_to_you.pop();
    path_to_san.pop();
    let common_depth = objects[common_ancestor].depth;
    let traversals = objects[path_to_san.last().unwrap()].depth - common_depth
        + objects[path_to_you.last().unwrap()].depth
        - common_depth;

    traversals.to_string()
}
