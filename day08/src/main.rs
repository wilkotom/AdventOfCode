#[derive(Debug)]
struct Node {
    child_nodes: Vec<Node>,
    metadata: Vec<usize>
}

impl Node {
    fn metadata_sum(&self) -> usize {
        self.metadata.iter().sum::<usize>() + self.child_nodes.iter().map(|x| x.metadata_sum()).sum::<usize>()
    }

    fn node_value(&self) -> usize {
        if self.child_nodes.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata.iter().map(
                |x| self.child_nodes.get(*x -1)
                    .unwrap_or(&Node{ child_nodes: vec![], metadata: vec![]})
                    .node_value())
                .sum()

        }
    }
}
fn main() {
    let node = parse_packet(&mut std::fs::read_to_string("./input.txt").unwrap().split(' ').map(|v| v.parse::<usize>().unwrap()).rev().collect::<Vec<_>>());
    println!("Part 1: {}\nPart 2: {}", node.metadata_sum(), node.node_value());
}

fn parse_packet(mut packet: &mut Vec<usize>) -> Node {
    let mut child_count = packet.pop().unwrap();
    let mut metadata_count = packet.pop().unwrap();
    let mut node = Node{ child_nodes: vec![], metadata: vec![]};
    while child_count > 0 {
        node.child_nodes.push(parse_packet(packet));
        child_count -=1;
    }
    while metadata_count > 0 {
        node.metadata.push(packet.pop().unwrap());
        metadata_count -=1;
    }
    node
}


#[test]
fn test_part1() {
    let mut packet = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".split(' ').map(|v| v.parse::<usize>().unwrap()).rev().collect::<Vec<_>>();
    let parsed_packet = parse_packet(&mut packet);
    assert_eq!(parsed_packet.metadata_sum(), 138);
}

#[test]
fn test_part2() {
    let mut packet = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".split(' ').map(|v| v.parse::<usize>().unwrap()).rev().collect::<Vec<_>>();
    let parsed_packet = parse_packet(&mut packet);
    assert_eq!(parsed_packet.node_value(), 66);
}

#[test]
fn test_part2_single_packet() {
    let mut packet = "0 3 10 11 12".split(' ').map(|v| v.parse::<usize>().unwrap()).rev().collect::<Vec<_>>();
    let parsed_packet = parse_packet(&mut packet);
    assert_eq!(parsed_packet.metadata_sum(), 33);
    assert_eq!(parsed_packet.node_value(), 33 );
}


#[test]
fn test_part2_nested_packet() {
    let mut packet = "1 1 0 1 99 2".split(' ').map(|v| v.parse::<usize>().unwrap()).rev().collect::<Vec<_>>();
    let parsed_packet = parse_packet(&mut packet);
    assert_eq!(parsed_packet.metadata_sum(), 101);
    assert_eq!(parsed_packet.node_value(), 0 );
}