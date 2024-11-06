use std::collections::HashMap;
use aochelpers::*;

#[test]
fn add_coordinates() {
    let base = Coordinate{x:1, y:1};
    let delta = Coordinate{x:2, y:3};
    let expected = Coordinate{x:3, y:4};
    assert_eq!(base + delta, expected);
}

#[test]
fn sub_coordinates() {
    let base = Coordinate{x:1, y:1};
    let delta = Coordinate{x:2, y:3};
    let expected = Coordinate{x: -1, y:-2};
    assert_eq!(base - delta, expected);
}


#[test]
fn add_assign_coordinates() {
    let mut base = Coordinate{x:1, y:1};
    let delta  = Coordinate{x:2, y:3};
    base += delta;
    let expected = Coordinate{x:3, y:4};
    assert_eq!(base, expected);
}

#[test]
fn sub_assign_coordinates() {
    let mut base = Coordinate{x:1, y:1};
    let delta  = Coordinate{x:2, y:3};
    base -= delta;
    let expected = Coordinate{x: -1, y:-2};
    assert_eq!(base, expected);
}

#[test]
fn directional_neighbour() {
    assert_eq!(Coordinate{x:0,y:0}.neighbour(Direction::North), Coordinate{x:0, y:-1});
    assert_eq!(Coordinate{x:0,y:0}.neighbour(Direction::South), Coordinate{x:0, y:1});
    assert_eq!(Coordinate{x:0,y:0}.neighbour(Direction::East), Coordinate{x:1, y:0});
    assert_eq!(Coordinate{x:0,y:0}.neighbour(Direction::West), Coordinate{x:-1, y:0});
}

#[test]
fn neighbours_2d() {
    assert_eq!( 
        Coordinate {x:1, y:1}.neighbours(), 
        vec![Coordinate{x:0, y:1},
             Coordinate{x:2, y:1},
             Coordinate{x:1, y:0},
             Coordinate{x:1, y:2}]);
}


#[test]
fn extended_neighbours_2d() {
    assert_eq!( 
        Coordinate {x:1, y:1}.extended_neighbours(), 
        vec![Coordinate{x:0, y:0},
             Coordinate{x:0, y:1},
             Coordinate{x:0, y:2},
             Coordinate{x:1, y:0},
             Coordinate{x:1, y:2},
             Coordinate{x:2, y:0},
             Coordinate{x:2, y:1},
             Coordinate{x:2, y:2}]);
}

#[test]
fn hex_neighbours_2d() {
    assert_eq!( 
        Coordinate{x:1, y:1}.hex_neighbours(), 
        vec![Coordinate{x:-1, y:1},
             Coordinate{x:3, y:1},
             Coordinate{x:2, y:0},
             Coordinate{x:2, y:2},
             Coordinate{x:0, y:0},
             Coordinate{x:0, y:2}]);
}

#[test]
fn neighbours_3d() {
    assert_eq!( 
        Coordinate3d{x:1, y:1, z:1}.neighbours(), 
        vec![Coordinate3d{x:0, y:1, z:1},
             Coordinate3d{x:2, y:1, z:1},
             Coordinate3d{x:1, y:0, z:1},
             Coordinate3d{x:1, y:2, z:1},
             Coordinate3d{x:1, y:1, z:0},
             Coordinate3d{x:1, y:1, z:2}]);
}

#[test]
fn extended_neighbours_3d() {
    assert_eq!( 
        Coordinate3d{x:1, y:1, z:1}.extended_neighbours(), 
        vec![Coordinate3d{x: 0, y: 0, z:0},
             Coordinate3d{x: 0, y: 1, z:0},
             Coordinate3d{x: 0, y: 2, z:0},
             Coordinate3d{x: 1, y: 0, z:0},
             Coordinate3d{x: 1, y: 1, z:0},
             Coordinate3d{x: 1, y: 2, z:0},
             Coordinate3d{x: 2, y: 0, z:0},
             Coordinate3d{x: 2, y: 1, z:0},
             Coordinate3d{x: 2, y: 2, z:0},
             
             Coordinate3d{x: 0, y: 0, z:1},
             Coordinate3d{x: 0, y: 1, z:1},
             Coordinate3d{x: 0, y: 2, z:1},
             Coordinate3d{x: 1, y: 0, z:1},
             Coordinate3d{x: 1, y: 2, z:1},
             Coordinate3d{x: 2, y: 0, z:1},
             Coordinate3d{x: 2, y: 1, z:1},
             Coordinate3d{x: 2, y: 2, z:1},
             
             Coordinate3d{x: 0, y: 0, z:2},
             Coordinate3d{x: 0, y: 1, z:2},
             Coordinate3d{x: 0, y: 2, z:2},
             Coordinate3d{x: 1, y: 0, z:2},
             Coordinate3d{x: 1, y: 1, z:2},
             Coordinate3d{x: 1, y: 2, z:2},
             Coordinate3d{x: 2, y: 0, z:2},
             Coordinate3d{x: 2, y: 1, z:2},
             Coordinate3d{x: 2, y: 2, z:2},]);
}

#[test]
fn scored_item_ordering() {
    /* 
    Reminder: std::collections::BinaryHeap is a max-heap, so score comparisons are backward.
    Smallest cost wins.
    (y,x) used as tie-breaker in this case as the payload is a Coordinate
    */
    let first = ScoredItem{ cost: 3, item: Coordinate{x:1, y:1}};
    let second = ScoredItem{ cost: 1, item: Coordinate{x:3, y:6}};
    let third = ScoredItem{ cost: 1, item: Coordinate{x:1, y:1}};
    assert!(first < second);
    assert!(first < third);
    assert!(second < third);
}

#[test]
fn manhattan_distance() {
    assert_eq!(Coordinate{x:0,  y:0 }.manhattan_distance(&Coordinate{x:0,  y:0}), 0);
    assert_eq!(Coordinate{x:0,  y:0 }.manhattan_distance(&Coordinate{x:1,  y:1}), 2);
    assert_eq!(Coordinate{x:1,  y:1 }.manhattan_distance(&Coordinate{x:0,  y:0}), 2);
    assert_eq!(Coordinate{x:0,  y:0 }.manhattan_distance(&Coordinate{x:-1, y:0}), 1);
    assert_eq!(Coordinate{x:-1, y:0 }.manhattan_distance(&Coordinate{x:0,  y:0}), 1);
    assert_eq!(Coordinate{x:-1, y:-1}.manhattan_distance(&Coordinate{x:0,  y:0}), 2);
}

#[test]
fn manhattan_distance_3d() {
    assert_eq!(Coordinate3d{x:0,  y:0,  z:0}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:0 }), 0);
    assert_eq!(Coordinate3d{x:0,  y:0,  z:0}.manhattan_distance(&Coordinate3d{x:1,  y:1, z:0 }), 2);
    assert_eq!(Coordinate3d{x:1,  y:1,  z:0}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:0 }), 2);
    assert_eq!(Coordinate3d{x:0,  y:0,  z:0}.manhattan_distance(&Coordinate3d{x:-1, y:0, z:0 }), 1);
    assert_eq!(Coordinate3d{x:-1, y:0,  z:0}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:0 }), 1);
    assert_eq!(Coordinate3d{x:-1, y:-1, z:0}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:0 }), 2);
    assert_eq!(Coordinate3d{x:1,  y:1,  z:1}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:0 }), 3);
    assert_eq!(Coordinate3d{x:0,  y:0,  z:0}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:1 }), 1);
    assert_eq!(Coordinate3d{x:0,  y:0,  z:0}.manhattan_distance(&Coordinate3d{x:0,  y:0, z:-1}), 1);
    assert_eq!(Coordinate3d{x:0,  y:0,  z:0}.manhattan_distance(&Coordinate3d{x:1,  y:0, z:-1}), 2);
}

#[test]
fn area_2d(){
    let c1 = Coordinate{x:1, y:1};
    let c2 = Coordinate{x:0, y:0};
    let c3 = Coordinate{x:1, y:0};
    let c4 = Coordinate{x:0, y:1};
    assert_eq!(Rectangle::new(c1, c2).area(), 1);
    assert_eq!(Rectangle::new(c2, c1).area(), 1);
    assert_eq!(Rectangle::new(c3, c4).area(), 1);
    assert_eq!(Rectangle::new(c4, c3).area(), 1);
    assert_eq!(Rectangle::new(c1, c3).area(), 0);
    assert_eq!(Rectangle::new(c2, c4).area(), 0);
}

#[test]
fn contains_points(){
    let r1 = Rectangle{
        top_left: Coordinate{x:0, y:0},
        bottom_right: Coordinate{x:2,y:2}
    };
    for x in -1..=3 {
        for y in -1..=3 {
            if x >=0 && x <=2 && y>=0 && y <=2 {
                assert!(r1.contains(&Coordinate { x, y}))
            } else {
                assert!(! r1.contains(&Coordinate { x, y}))
            }
        }

    }
}

#[test]
fn intersection_2d_diagonal_offset() {
    let r1 = Rectangle{
        top_left: Coordinate{x:0, y:0},
        bottom_right: Coordinate{x:2,y:2}
    };
    let r2 = Rectangle{
        top_left: Coordinate{x:1, y:1},
        bottom_right: Coordinate{x:3,y:3}
    };
    let expected = Rectangle{
            top_left: Coordinate{x:1, y:1}, 
            bottom_right: Coordinate{x:2, y:2}};
    assert_eq!(r1.intersection(&r2), Some(expected));
    assert_eq!(r2.intersection(&r1), Some(expected));

    let r3 = Rectangle{top_left: Coordinate { x: -1, y:1}, bottom_right: Coordinate { x: 1, y:3}};
    let expected = Rectangle{top_left: Coordinate{x:0, y:1}, bottom_right: Coordinate{x:1, y:2}};
    assert_eq!(r1.intersection(&r3), Some(expected));
    assert_eq!(r3.intersection(&r1), Some(expected));

}

#[test]
fn intersection_2d_y_offset() {
    let r1 = Rectangle{
        top_left: Coordinate{x:0, y:0},
        bottom_right: Coordinate{x:2,y:2}
    };
    let r2 = Rectangle{
        top_left: Coordinate{x:0, y:1},
        bottom_right: Coordinate{x:2,y:3}
    };
    let expected = Rectangle{
            top_left: Coordinate{x:0, y:1}, 
            bottom_right: Coordinate{x:2, y:2}};
    assert_eq!(r1.intersection(&r2), Some(expected));
    assert_eq!(r2.intersection(&r1), Some(expected));

}

#[test]
fn intersection_2d_x_offset() {
    let r1 = Rectangle{
        top_left: Coordinate{x:0, y:0},
        bottom_right: Coordinate{x:2,y:2}
    };
    let r2 = Rectangle{
        top_left: Coordinate{x:1, y:0},
        bottom_right: Coordinate{x:3,y:2}
    };
    let expected = Rectangle{
            top_left: Coordinate{x:1, y:0}, 
            bottom_right: Coordinate{x:2, y:2}};
    assert_eq!(r1.intersection(&r2), Some(expected));
    assert_eq!(r2.intersection(&r1), Some(expected));

    
}

#[test]
fn intersection_2d_non_overlap() {
    let r1 = Rectangle{
        top_left: Coordinate{x:0, y:0},
        bottom_right: Coordinate{x:1,y:1}
    };
    let r2 = Rectangle{
        top_left: Coordinate{x:2, y:2},
        bottom_right: Coordinate{x:3,y:3}
    };
    assert_eq!(r1.intersection(&r2), None);
    assert_eq!(r2.intersection(&r1), None);
}
#[test]
fn intersection_entirely_bounded() {
    let r1 = Rectangle{
        top_left: Coordinate{x:0, y:0},
        bottom_right: Coordinate{x:3,y:3}
    };
    let r2 = Rectangle{
        top_left: Coordinate{x:1, y:1},
        bottom_right: Coordinate{x:2,y:2}
    };
    let expected = Rectangle{
        top_left: Coordinate{x:1, y:1}, 
        bottom_right: Coordinate{x:2, y:2}};

    assert_eq!(r1.intersection(&r2), Some(expected));
    assert_eq!(r2.intersection(&r1), Some(expected));
}

#[test]
fn single_number_grid() {
    assert_eq!(parse_number_grid::<isize, i32>("1"), HashMap::from([(Coordinate{x:0, y:0}, 1_i32)]))
}

#[test]
fn empty_number_grid() {
    assert_eq!(parse_number_grid::<i32, i32>(""), HashMap::new())
}

#[test]
fn number_grid_row() {
    assert_eq!(parse_number_grid::<i32, i32>("12"), HashMap::from([(Coordinate{x:0, y:0}, 1_i32), (Coordinate{x:1, y:0}, 2_i32)]))
}


#[test]
fn number_grid_col() {
    assert_eq!(parse_number_grid::<i32, i32>("1\n2"), HashMap::from([(Coordinate{x:0, y:0}, 1_i32), (Coordinate{x:0, y:1}, 2_i32)]))
}

#[test]
fn construct_label_empty() {
    let res = "".parse::<Label>();
    assert!(res.is_ok());
    let expected =  Label::from(0_usize);
    let value = res.unwrap();
    assert_eq!(&value, &expected);
    assert_eq!(value.to_string(), "".to_string());
}

#[test]
fn construct_label_number() {
    let res = "12".parse::<Label>();
    assert!(res.is_ok());
    let expected =  Label::from(38);
    let value = res.unwrap();
    assert_eq!(&value, &expected);
    assert_eq!(value.to_string(), "12".to_string());
}

#[test]
fn construct_label_letter() {
    let res = "a".parse::<Label>();
    assert!(res.is_ok());
    let expected =  Label::from(10);
    let value = res.unwrap();
    assert_eq!(&value, &expected);
    assert_eq!(value.to_string(), "a".to_string());}

#[test]
fn construct_label_letters() {
    let res = "az".parse::<Label>();
    assert!(res.is_ok());
    let value: Label = res.unwrap();

    let expected =  Label::from(395);
    assert_eq!(&value, &expected);
    assert_eq!(value.to_string(), "az".to_string());}

#[test]
fn construct_label_caps() {
    let res = "AZ".parse::<Label>();
    assert!(res.is_ok());
    let value: Label = res.unwrap();

    let expected =  Label::from(395);
    assert_eq!(&value, &expected);
    assert_eq!(value.to_string(), "az".to_string());
}
#[test]
fn construct_label_invalid() {
    let res = "hello!".parse::<Label>();
    assert!(res.is_err());
}


#[test]
fn test_label_ends() {
    let res = "AZ".parse::<Label>();
    assert!(res.is_ok());
    let value: Label = res.unwrap();
    assert!(value.ends_with('z'));
    assert!(!value.ends_with('a'))
}