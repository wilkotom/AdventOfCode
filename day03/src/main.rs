fn main() {
    let data = std::fs::read_to_string(String::from("./input.txt")).unwrap();
    let mut verticals: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
    let mut not_triangles = 0;
    for line in data.split("\n") {
        let mut values: Vec<_> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap_or(0)).collect();
        for i in 0..3 {
            verticals[i].push(values[i]);
        }
        values.sort();
        if values[0] + values[1] > values[2] {
            not_triangles += 1;
        }
        
    }
    println!("Part 1: There are {} non-triangle values", not_triangles);
    let mut all_columns = vec![];
    not_triangles = 0;
    for i in 0..3{
        all_columns.append(verticals.get_mut(i).unwrap());
    }
    for i in (0..all_columns.len()).step_by(3) {
        let total = all_columns[i..i+3].iter().sum::<i32>();
        let max = *all_columns[i..i+3].iter().max().unwrap();
        if (total - max) > max {
            not_triangles +=1;
        }
    }
    println!("Part 2: There are {} non-triangle values", not_triangles);

}
