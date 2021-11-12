use std::fmt;
use std::collections::HashMap;
use std::collections::VecDeque;


#[derive(Debug,Clone)]
struct PhotoFragment {
    matrix: Vec<Vec<bool>>,
    up: i32,
    down: i32,
    left: i32,
    right: i32,
    id: i32,
    oriented: bool
}

impl PhotoFragment {
    fn reflect_horizontal(&mut self) {
        for line in self.matrix.iter_mut() {
            line.reverse();
        }
    }

    fn reflect_vertical(&mut self) {
        self.matrix.reverse();
    }

    fn rotate_clockwise(&mut self) {
        let mut new_matrix: Vec<Vec<bool>> =Vec::new();
        for col in 0..self.matrix[0].len() {
            let mut new_line: Vec<bool> = Vec::new();
            for row in  0..self.matrix.len() {
                new_line.insert(0,self.matrix[row][col])
            }
            new_matrix.push(new_line);
        }
        self.matrix = new_matrix
    }

    fn top_edge(&self) -> i32 {
       _edge_helper(&self.matrix[0])
    }

    fn bottom_edge(&self) -> i32 {
        _edge_helper(&self.matrix[self.matrix.len()-1])
    }

    fn left_edge(&self) -> i32 {
        _edge_helper(&self.matrix.iter().map(|x| x[0]).collect())
    }

    fn right_edge(&self) -> i32 {
        _edge_helper(&self.matrix.iter().map(|x| x[x.len()-1]).collect())
    }
}

fn _edge_helper(edge: &Vec<bool>) -> i32 {
    let mut total = 0;
    for (i, &pixel) in edge.iter().enumerate() {
        if pixel {
            total += i32::pow(2,i as u32);
        }
    }
    total
}

impl fmt::Display for PhotoFragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let thing = self.matrix.iter()
            .map(|x| x.iter().map(|&v| if v {"#"} else {"."})
                             .collect::<String>())
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", thing)
    }
}

fn main() {
    let fragments = read_fragments("./input.txt".to_string());
    let oriented_pieces = orient_pieces(fragments);
    let joined = join_pieces(oriented_pieces);
    monster_hunt(joined);
    

}


fn read_fragments(filename: String) -> HashMap<i32, Box<PhotoFragment>> {
    let mut fragments: HashMap<i32, Box<PhotoFragment>> = HashMap::new();

    for fragment in std::fs::read_to_string(filename).unwrap_or("".to_string()).split("\n\n") {
        let lines: Vec<_> = fragment.split("\n").collect();
        let mut matrix: Vec<Vec<bool>> = Vec::new();
        let tile_number = &lines[0][5..9].parse::<i32>().unwrap_or(0);
        for line in &lines[1..] {
            matrix.push(line.chars().map(|x| x == '#').collect());
        }
        let frag: PhotoFragment = PhotoFragment{matrix, up:0, down:0, left:0, right:0, id: *tile_number, oriented: false};
        fragments.insert(*tile_number as i32, Box::new(frag));
    }

    fragments
}

fn orient_pieces(mut fragments: HashMap<i32, Box<PhotoFragment>>) -> HashMap<i32, Box<PhotoFragment>> {
    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut oriented: VecDeque<i32> = VecDeque::new();
    let first = fragments.keys().cloned().next().unwrap();
    queue.push_back(first);
    let mut all_oriented = false;
    while !all_oriented {
        let fragment_id = queue.pop_front().unwrap();
        let mut fragment = fragments.remove(&fragment_id).unwrap();
        fragment.oriented = true;
        let pieces: Vec<_> = fragments.keys().cloned().collect();
        all_oriented = true;
        for piece_id in pieces.iter() {
            let mut piece = fragments.remove(&piece_id).unwrap();
            let mut matched =false;
            if !piece.oriented {
                all_oriented = false;
                for i in 0..2 {
                    for _ in 0..4 {
                        if fragment.up == 0 && fragment.top_edge() == piece.bottom_edge() {
                            piece.down = fragment.id;
                            fragment.up = piece.id;
                            matched = true;
                        } else if fragment.down == 0 && fragment.bottom_edge() == piece.top_edge() {
                            piece.up = fragment.id;
                            fragment.down = piece.id;
                            matched = true;
                        } else if fragment.right == 0 && fragment.right_edge() == piece.left_edge() {
                            piece.left = fragment.id;
                            fragment.right = piece.id;
                            matched = true;
                        } else if fragment.left == 0 && fragment.left_edge() == piece.right_edge() {
                            piece.right = fragment.id;
                            fragment.left = piece.id;
                            matched = true;
                        }
                        if matched {
                            queue.push_back(piece.id);
                            break;
                        }
                        piece.rotate_clockwise()
                    }

                    if matched {
                        break;
                    } else if i == 0 { 
                        piece.reflect_vertical();
                    }
                }
                piece.oriented = matched;
            }
            if piece.oriented {
                oriented.push_back(piece.id);
            } 
            fragments.insert(piece.id, piece);
            
        }

        fragments.insert(fragment_id, fragment);
        oriented.push_back(fragment_id);
        while oriented.len() > 0 {
            let new_val = oriented.pop_front().unwrap();
            if !queue.contains(&new_val) {
                queue.push_back(new_val);
            }
        };
            
        
    }
    let mut final_fragments: HashMap<i32,Box<PhotoFragment>> = HashMap::new();
    let fragment_ids: Vec<_> = fragments.keys().cloned().collect();
    for frag_id in fragment_ids.iter() {
        let mut final_fragment = fragments.remove(&frag_id).unwrap();
        let remaining_fragments: Vec<_> = fragments.keys().cloned().collect();
        for f_id in remaining_fragments.iter() {
            let possible_join = fragments.get_mut(&f_id).unwrap();
            if final_fragment.right_edge() == possible_join.left_edge() {
                final_fragment.right = possible_join.id;
                possible_join.left = final_fragment.id;
            } else if final_fragment.left_edge() == possible_join.right_edge() {
                final_fragment.left = possible_join.id;
                possible_join.right = final_fragment.id;
            } else if final_fragment.bottom_edge() == possible_join.top_edge() {
                final_fragment.down = possible_join.id;
                possible_join.up = final_fragment.id;
            } else if final_fragment.top_edge() == possible_join.bottom_edge() {
                final_fragment.up = possible_join.id;
                possible_join.down = final_fragment.id;
            } 
        }
        final_fragments.insert(final_fragment.id, final_fragment);
    }
    trim_edges(final_fragments)
}

fn join_pieces(mut fragments: HashMap<i32, Box<PhotoFragment>>) -> HashMap<i32, Box<PhotoFragment>> {
    let ids: Vec<_> = fragments.keys().cloned().collect();
    for id in ids.iter() {
        let c = fragments.remove(&id);
        match c {
            Some(mut fragment) => {
                while fragment.up != 0 {
                    let above = fragments.remove(&fragment.up).unwrap();
                    let mut new_matrix = above.matrix;
                    new_matrix.append(&mut fragment.matrix);
                    fragment.matrix = new_matrix;
                    fragment.up = above.up;
                    fragment.left = above.left;
                    fragment.right = above.right;
                    fragment.id = above.id;
                }
                while fragment.down != 0 {
                    let mut below = fragments.remove(&fragment.down).unwrap();
                    fragment.matrix.append(&mut below.matrix);
                    fragment.down = below.down;
                }
                fragments.insert(fragment.id, fragment);
            }
            _ => {}
        }
    
    }
    let ids: Vec<_> = fragments.keys().cloned().collect();
    for id in ids.iter() {
        let c = fragments.remove(&id);
        match c {
            Some(mut fragment) => {
                while fragment.left != 0 {
                    let left = fragments.remove(&fragment.left).unwrap();
                    let mut new_matrix = left.matrix;
                    for i in 0..new_matrix.len() {
                        new_matrix[i].append(&mut fragment.matrix[i]);
                    }
                    fragment.matrix = new_matrix;
                    fragment.left = left.left;
                }
                while fragment.right != 0 {
                    let mut right = fragments.remove(&fragment.right).unwrap();
                    let mut new_matrix = fragment.matrix;
                    for i in 0..new_matrix.len(){
                        new_matrix[i].append(&mut right.matrix[i]);
                    }
                    fragment.matrix = new_matrix;
                    fragment.right = right.right;
                }
                fragments.insert(fragment.id, fragment);
            }
            _ => {}
        }
    }
    fragments
}


fn trim_edges(mut fragments: HashMap<i32, Box<PhotoFragment>>) -> HashMap<i32, Box<PhotoFragment>>{
    for (_, fragment) in fragments.iter_mut() {
        fragment.matrix.remove(0);
        fragment.matrix.remove(fragment.matrix.len() -1);
        for line in fragment.matrix.iter_mut() {
            line.remove(0);
            line.remove(line.len() -1);
        }
    }
    fragments
}

fn monster_hunt(fragments: HashMap<i32, Box<PhotoFragment>> ) -> usize {
    /*
    
                      # 
    #    ##    ##    ###
     #  #  #  #  #  #   
    */
    let mut monster_count = 0;
    let mut fragment = fragments.into_values().next().unwrap();
    let monster_shape = vec![(0,0), (1,1), (1,4), (0,5), (0,6), (1,7),
                       (1,10), (0,11), (0,12), (1,13), (1,16), 
                       (0,17),(-1 as isize,18),(0,18),(0,19)];
    let roughness =  &fragment.matrix.iter().cloned().map(|x| x.iter().filter(|&x| *x == true).count()).sum::<usize>();
    for i in 0..8 {
        for row in 1..fragment.matrix.len() {
            for col in 0..fragment.matrix[0].len() {
                if *fragment.matrix.get(row).unwrap_or(&Vec::new()).get(col).unwrap_or(&false) {
                    let mut monster = true;
                    for (y,x) in monster_shape.iter() {
                        if !*fragment.matrix.get((row as isize+y) as usize).unwrap_or(&Vec::new()).get(col+x).unwrap_or(&false) {
                            monster = false;
                        }
                    }
                    if monster {
                        monster_count +=1;
                    }
                }
            }
        }
        if monster_count > 0{
            break;
        }
        fragment.rotate_clockwise();
        if i == 4{
            fragment.reflect_horizontal();
        }
    }
    println!("Monsters: {} Water Roughness:{}", monster_count, roughness - 15* monster_count);
    monster_count
}