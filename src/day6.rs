use std::collections::HashMap;

#[test]
fn test_day6() {
    assert_eq!(
        part1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"),
        "42"
    )
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

pub fn part1(input: &str) -> String {
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
    let root = objects
        .values()
        .find(|e| e.parent.is_none())
        .expect("expected a root node")
        .name;

    // add `depth` everywhere
    traverse_subtree(&mut objects, root, 0);

    // sum up all the depths
    objects.values().map(|n| n.depth).sum::<usize>().to_string()
}

pub fn part2(_input: &str) -> String {
    "".into()
}
