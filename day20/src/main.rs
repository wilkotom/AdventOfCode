#[derive(Debug,PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Window {
    start:isize,
    end:isize
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut windows: Vec<Window> = Vec::new();
    for line in input.split('\n') {
        let mut numbers = line.split('-');
        let start = numbers.next().unwrap().parse::<isize>().unwrap();
        let end = numbers.next().unwrap().parse::<isize>().unwrap();
        windows.push(Window{start,end});
    }
    windows.sort();
    let mut final_windows: Vec<Window> = Vec::new();
    let mut current_window = windows.remove(0);
    let mut gaps = 0;
    for next_window in windows.iter() {
        if next_window.start > current_window.end +1 {
            gaps += next_window.start - (current_window.end +1);
            final_windows.push(current_window);
            current_window.start = next_window.start;
            current_window.end = next_window.end;
        } else if current_window.end < next_window.end {
            current_window.end = next_window.end
        }
    }
    final_windows.push(current_window);
    println!("Part 1: {:?}", final_windows[0].end +1);
    println!("Part 2: {:?}", gaps);
}
