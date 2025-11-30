use std::{cmp::{max, min, Ordering}, collections::HashMap, env, error::Error, fmt::{self, Debug, Display}, fs::{self, File}, hash::Hash, io::{ErrorKind, Write}, ops::{Add, AddAssign, Mul, Sub, SubAssign}, path::PathBuf, str::FromStr};
use num::{CheckedAdd, CheckedSub, Integer, Unsigned};
use log::warn;

/// Compass directions
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West
}

/// Defines a direction used by a Particle. May be either a compass `Direction` or an `i32` bearing.
pub trait Heading {}

impl Heading for Direction {}
impl Heading for i32 {}

/// Representation of a point moving in a direction
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Particle<T: Heading> {
    pub starting_point: Coordinate<i32>,
    pub heading: T
}

/// A standard 2D Cartesian Coordinate
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

/// Renders Coordinate as `(x,y)`
impl<T: Display> Display for Coordinate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add for Coordinate<T>{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<T: Unsigned + CheckedAdd<Output = T>> CheckedAdd for Coordinate<T>{
    fn checked_add(&self, other: &Self) -> Option<Self> {
        if let(Some(x), Some(y)) = (self.x.checked_add(&other.x), self.y.checked_add(&other.y)) {
            Some(Self{x, y})
        }
        else {
            None
        }
    }
}

impl<T: Unsigned + CheckedSub<Output = T>> CheckedSub for Coordinate<T>{
    fn checked_sub(&self, other: &Self) -> Option<Self> {
        if let(Some(x), Some(y)) = (self.x.checked_sub(&other.x), self.y.checked_sub(&other.y)) {
            Some(Self{x, y})
        }
        else {
            None
        }
    }
}

impl<T: Sub<Output = T>> Sub for Coordinate<T>{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl<T: AddAssign> AddAssign for Coordinate<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: SubAssign> SubAssign for Coordinate<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T> Mul<T> for Coordinate<T>
where T: Mul<Output = T> + Copy
{
    type Output = Coordinate<T>;
    fn mul(self, n: T) -> Coordinate<T> {
        Coordinate {
            x: self.x * n,
            y: self.y * n,
        }
    }
}

/// Y values are treated as more significant than X values; this preserves the _reading order_ used in a number of puzzles.
impl<T: Eq + PartialEq + Ord + Copy> Ord for Coordinate<T> {
    // Reading order: Y then X
    fn cmp(&self, other: &Self) -> Ordering {
       (self.y, self.x).cmp(&(other.y, other.x))
    }
}
impl<T: Ord + PartialOrd> PartialOrd for Coordinate<T> where T: std::marker::Copy {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Copy + 'static> Coordinate<T> {
    pub fn from<U: num::cast::AsPrimitive<T>>(other: Coordinate<U>) -> Coordinate<T> {
        Coordinate {
            x: other.x.as_(),
            y: other.y.as_(),
        }
    }
}


impl<T: Integer + Copy> Coordinate<T> {
    /// All co-ordinates directly neighbouring the square on a grid, excluding diagonals
    pub fn neighbours(&self) ->  impl Iterator<Item = Self> + use<'_, T> {
        
        [ Coordinate{x: self.x - num::one(), y: self.y},
              Coordinate{x: self.x + num::one(), y: self.y},
              Coordinate{x: self.x, y: self.y - num::one()},
              Coordinate{x: self.x, y: self.y + num::one()},
        ].into_iter()
    }
    /// All co-ordinates directly neighbouring the square on a grid, including diagonals
    pub fn extended_neighbours(&self) -> impl Iterator<Item = Self> + use<'_, T> {
        [ 
            Coordinate{x: self.x - num::one(), y: self.y - num::one()},
            Coordinate{x: self.x - num::one(), y: self.y },
            Coordinate{x: self.x - num::one(), y: self.y + num::one()},
            Coordinate{x: self.x,              y: self.y - num::one()},
            Coordinate{x: self.x,              y: self.y + num::one()},
            Coordinate{x: self.x + num::one(), y: self.y - num::one()},
            Coordinate{x: self.x + num::one(), y: self.y },
            Coordinate{x: self.x + num::one(), y: self.y + num::one()},
        ].into_iter()
    }
    /// Returns all co-ordinates directly neighbouring the square on an alternating hex grid:
    /// ```text
    ///   1 2
    ///  3 X 4
    ///   5 6
    /// ```
    pub fn hex_neighbours(&self) -> impl Iterator<Item = Self> + use<'_, T> {
        [
            Coordinate{x: self.x - num::one() - num::one(), y: self.y}, 
            Coordinate{x: self.x + num::one() + num::one(), y: self.y}, 
            Coordinate{x: self.x + num::one(), y: self.y - num::one()}, 
            Coordinate{x: self.x + num::one(), y: self.y + num::one()},
            Coordinate{x: self.x - num::one(), y: self.y - num::one()},
            Coordinate{x: self.x - num::one(), y: self.y + num::one()}
        ].into_iter()

    }

    pub fn triangle_neighbours(&self) -> impl Iterator<Item = Self> + use<'_, T> {
        self.neighbours()
        .filter(|c| (c.x >= num::zero() && c.y >= num::zero() && c.y == self.y) ||
             (c.y < self.y && c.y % (num::one::<T>() + num::one()) != c.x %(num::one::<T>() + num::one())) || 
             (c.y> self.y && c.x % (num::one::<T>() + num::one()) == c.y%( num::one::<T>() + num::one()) ))
}

    /// Taxicab / manhattan distance: difference between X coordinates plus difference between Y coordinates
    pub fn manhattan_distance(&self, other: &Self) -> T  {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }
    /// The neighbouring `Coordinate` in the supplied `Direction`
    pub fn neighbour(&self, direction: Direction) -> Self {
        match direction {
            Direction::NorthWest =>  Coordinate { x: self.x - num::one() , y: self.y - num::one() },
            Direction::North =>  Coordinate { x: self.x, y: self.y - num::one() },
            Direction::NorthEast => Coordinate { x: self.x + num::one() , y: self.y - num::one() },
            Direction::East =>  Coordinate { x: self.x + num::one() , y: self.y },
            Direction::SouthEast =>  Coordinate { x: self.x + num::one() , y: self.y + num::one() },
            Direction::South => Coordinate { x: self.x  , y: self.y + num::one() },
            Direction::SouthWest =>  Coordinate { x: self.x - num::one() , y: self.y + num::one() },
            Direction::West => Coordinate { x: self.x - num::one() , y: self.y },
        }
    }
}
impl<T: Integer + Copy + CheckedSub + CheckedAdd + Unsigned> Coordinate<T> {

        /// The neighbouring `Coordinate` in the supplied `Direction`, checked for under / overflow
        pub fn checked_neighbour(&self, direction: Direction) -> Option<Self> {
            match direction {
                Direction::NorthWest =>  self.checked_sub(&Coordinate{x: num::one::<T>(), y: num::one::<T>()}),
                Direction::North =>  self.checked_sub(&Coordinate{x: num::zero::<T>(), y: num::one::<T>()}),
                Direction::NorthEast =>  if let Some(loc) = self.checked_neighbour(Direction::North) {loc.checked_neighbour(Direction::East)} else {None},
                Direction::East =>  self.checked_add(&Coordinate{x: num::one::<T>(), y: num::zero::<T>()}),
                Direction::SouthEast =>  self.checked_add(&Coordinate{x: num::one::<T>(), y: num::one::<T>()}),
                Direction::South => self.checked_add(&Coordinate{x: num::zero::<T>(), y: num::one::<T>()}),
                Direction::SouthWest =>  if let Some(loc) = self.checked_neighbour(Direction::South) {loc.checked_neighbour(Direction::West)} else {None},
                Direction::West => self.checked_sub(&Coordinate{x: num::one::<T>(), y: num::zero::<T>()}),
            }
        }
        /// The neighbouring `Coordinates` in the four compass `Direction`s, checked for under / overflow
        pub fn checked_neighbours(&self) -> impl Iterator<Item = Coordinate<T>> + use<'_, T> {
        
            [self.checked_neighbour(Direction::North),
            self.checked_neighbour(Direction::East),
            self.checked_neighbour(Direction::South),
            self.checked_neighbour(Direction::West)].into_iter().flatten()
        }
        /// The neighbouring `Coordinates` in the eight `Direction`s, checked for under / overflow
        pub fn checked_extended_neighbours(&self) -> impl Iterator<Item = Coordinate<T>> + use<'_, T> {
            [self.checked_neighbour(Direction::North),
            self.checked_neighbour(Direction::NorthEast),
            self.checked_neighbour(Direction::East),
            self.checked_neighbour(Direction::SouthEast),
            self.checked_neighbour(Direction::South),
            self.checked_neighbour(Direction::SouthWest),
            self.checked_neighbour(Direction::West),
            self.checked_neighbour(Direction::NorthWest)].into_iter().flatten()
        }
}

/// Describes a rectangle aligned with the x,y,z axes by way of its top left and bottom right corners
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rectangle<T> {
    pub top_left: Coordinate<T>,
    pub bottom_right: Coordinate<T>
}

impl<T: Integer + Copy> Rectangle<T> {
    /// The area of the `Rectangle`
    pub fn area(&self) -> T {
        let width =self.top_left.x.max(self.bottom_right.x) - self.top_left.x.min(self.bottom_right.x);
        let height = self.top_left.y.max(self.bottom_right.y) - self.top_left.y.min(self.bottom_right.y);
        width * height
    }

    pub fn new(first: Coordinate<T>, second: Coordinate<T>) -> Rectangle<T> {
        let top_left = Coordinate{ x:  first.x.min(second.x), y: first.y.min(second.y)};
        let bottom_right = Coordinate{ x:  first.x.max(second.x), y: first.y.max(second.y)};
        Rectangle { top_left, bottom_right }
    }

    /// Does the rectangle contain the specified point in space?
    pub fn contains(&self, point: &Coordinate<T>) -> bool {
            point.x >= self.top_left.x && point.x <= self.bottom_right.x &&
            point.y >= self.top_left.y && point.y <= self.bottom_right.y 
        }
    /// If there is an overlap between this and the other `Rectangle`, return the `Rectangle` describing the overlap
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.top_left.x > other.bottom_right.x || other.top_left.x > self.bottom_right.x ||
           self.top_left.y > other.bottom_right.y || other.top_left.y > self.bottom_right.y {
            None
        } else {
            Some(Rectangle{
                top_left: Coordinate{     x: max(self.top_left.x, other.top_left.x),
                                          y: max(self.top_left.y, other.top_left.y)},
                bottom_right: Coordinate{ x: min(self.bottom_right.x, other.bottom_right.x),
                                          y: min(self.bottom_right.y, other.bottom_right.y)},
            })
        }
    }
}

/// Standard 3D Cartesian Coordinate
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Coordinate3d<T> {
    pub x: T,
    pub y: T,
    pub z: T
}
/// Renders Coordinate as (x,y,z)
impl<T: Display> Display for Coordinate3d<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl<T: Add<Output = T>> Add for Coordinate3d<T>{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<T: Sub<Output = T>> Sub for Coordinate3d<T>{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl<T: AddAssign> AddAssign for Coordinate3d<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: SubAssign> SubAssign for Coordinate3d<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> Mul<T> for Coordinate3d<T>{
    type Output = Coordinate3d<T>;
    fn mul(self, n: T) -> Coordinate3d<T> {
        Coordinate3d {
            x: self.x * n,
            y: self.y * n,
            z: self.z * n
        }
    }
}

impl<T: Integer + Copy> Coordinate3d<T> {
    /// Returns all co-ordinates directly neighbouring the point in 3D space along X/Y/Z axes
    pub fn neighbours(&self) -> Vec<Self> {
        vec![ Coordinate3d{x: self.x - num::one(), y: self.y, z: self.z},
              Coordinate3d{x: self.x + num::one(), y: self.y, z: self.z},
              Coordinate3d{x: self.x, y: self.y - num::one(), z: self.z},
              Coordinate3d{x: self.x, y: self.y + num::one(), z: self.z},
              Coordinate3d{x: self.x, y: self.y, z: self.z - num::one()},
              Coordinate3d{x: self.x, y: self.y, z: self.z + num::one()},
        ]
    }

    /// Returns all co-ordinates directly neighbouring the point in 3D space including diagonals
    pub fn extended_neighbours(&self) -> Vec<Self> {
        vec![ 
            Coordinate3d{x: self.x - num::one(), y: self.y - num::one(), z: self.z - num::one()},
            Coordinate3d{x: self.x - num::one(), y: self.y,              z: self.z - num::one()},
            Coordinate3d{x: self.x - num::one(), y: self.y + num::one(), z: self.z - num::one()},
            Coordinate3d{x: self.x,              y: self.y - num::one(), z: self.z - num::one()},
            Coordinate3d{x: self.x,              y: self.y,              z: self.z - num::one()},
            Coordinate3d{x: self.x,              y: self.y + num::one(), z: self.z - num::one()},
            Coordinate3d{x: self.x + num::one(), y: self.y - num::one(), z: self.z - num::one()},
            Coordinate3d{x: self.x + num::one(), y: self.y,              z: self.z - num::one()},
            Coordinate3d{x: self.x + num::one(), y: self.y + num::one(), z: self.z - num::one()},

            Coordinate3d{x: self.x - num::one(), y: self.y - num::one(), z: self.z},
            Coordinate3d{x: self.x - num::one(), y: self.y,              z: self.z},
            Coordinate3d{x: self.x - num::one(), y: self.y + num::one(), z: self.z},
            Coordinate3d{x: self.x,              y: self.y - num::one(), z: self.z},
            Coordinate3d{x: self.x,              y: self.y + num::one(), z: self.z},
            Coordinate3d{x: self.x + num::one(), y: self.y - num::one(), z: self.z},
            Coordinate3d{x: self.x + num::one(), y: self.y,              z: self.z },
            Coordinate3d{x: self.x + num::one(), y: self.y + num::one(), z: self.z},

            Coordinate3d{x: self.x - num::one(), y: self.y - num::one(), z: self.z + num::one()},
            Coordinate3d{x: self.x - num::one(), y: self.y,              z: self.z + num::one()},
            Coordinate3d{x: self.x - num::one(), y: self.y + num::one(), z: self.z + num::one()},
            Coordinate3d{x: self.x,              y: self.y - num::one(), z: self.z + num::one()},
            Coordinate3d{x: self.x,              y: self.y,              z: self.z + num::one()},
            Coordinate3d{x: self.x,              y: self.y + num::one(), z: self.z + num::one()},
            Coordinate3d{x: self.x + num::one(), y: self.y - num::one(), z: self.z + num::one()},
            Coordinate3d{x: self.x + num::one(), y: self.y,              z: self.z + num::one()},
            Coordinate3d{x: self.x + num::one(), y: self.y + num::one(), z: self.z + num::one()},

        ]
    }

    /// Taxicab / manhattan distance: difference between X coordinates plus difference between Y coordinates plus difference between Z coordinates
    pub fn manhattan_distance(&self, other: &Self) -> T  {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y) + self.z.max(other.z) - self.z.min(other.z)
    }
}

/// Describes a cuboid aligned with the x,y,z axes by way of its top left back and bottom right front corners
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Cuboid<T> {
    pub top_left_back: Coordinate3d<T>,
    pub bottom_right_front: Coordinate3d<T>
}
impl<T: Integer + Copy> Cuboid<T> {
    /// The volume of the cuboid
    pub fn volume(&self) -> T {
        let width = self.top_left_back.x.max(self.bottom_right_front.x) - self.top_left_back.x.min(self.bottom_right_front.x);
        let height = self.top_left_back.y.max(self.bottom_right_front.y) - self.top_left_back.y.min(self.bottom_right_front.y);
        let depth = self.top_left_back.z.max(self.bottom_right_front.z) - self.top_left_back.z.min(self.bottom_right_front.z);
        width * height * depth
    }
    /// Takes any two points in space and to build the `Cuboid` defined by them.
    pub fn new(first: Coordinate3d<T>, second: Coordinate3d<T>) -> Cuboid<T> {
        let top_left_back = Coordinate3d{ x:  first.x.min(second.x), y: first.y.min(second.y), z: first.z.min(second.z)};
        let bottom_right_front = Coordinate3d{ x:  first.x.max(second.x), y: first.y.max(second.y),z: first.z.max(second.z)};
        Self { top_left_back, bottom_right_front }
    }

    /// If the supplied `Cuboid` intersects with this one, returns the cuboid defined by the intersection points between the two. Otherwise return `None`
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if self.top_left_back.x > other.bottom_right_front.x || other.top_left_back.x > self.bottom_right_front.x ||
           self.top_left_back.y > other.bottom_right_front.y || other.top_left_back.y > self.bottom_right_front.y || 
           self.top_left_back.z > other.bottom_right_front.z || other.top_left_back.z > self.bottom_right_front.z
           {
            None
        } else {
            Some(Self{ 
                top_left_back:      Coordinate3d{ x: max(self.top_left_back.x, other.top_left_back.x),
                                                  y: max(self.top_left_back.y, other.top_left_back.y),
                                                  z: max(self.top_left_back.z, other.top_left_back.z)},
                bottom_right_front: Coordinate3d{ x: min(self.bottom_right_front.x, other.bottom_right_front.x),
                                                  y: min(self.bottom_right_front.y, other.bottom_right_front.y),
                                                  z: min(self.bottom_right_front.z, other.bottom_right_front.z)},
            })
        }
    }
    /// Does the cuboid contain the specified point in space?
    pub fn contains(&self, point: &Coordinate3d<T>) -> bool {
        point.x >= self.top_left_back.x && point.x <= self.bottom_right_front.x &&
        point.y >= self.top_left_back.y && point.y <= self.bottom_right_front.y &&
        point.z >= self.top_left_back.z && point.z <= self.bottom_right_front.z
    }
}



/// Generic struct used to select an item based on a minimum score.
/// Use with std::collections::BinaryHeap for problems requiring Djikstra's
/// Algorithm or A*

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct ScoredItem<N, T> {
    pub cost: N,
    pub item: T
}

impl<N:  Ord + PartialOrd, T: Ord + PartialOrd> Ord for ScoredItem<N, T> {
    fn cmp(&self, other: &ScoredItem<N, T>) -> Ordering {
        (&other.cost, &other.item).cmp(&(&self.cost, &self.item))
    }
}

impl<N: Ord+ PartialOrd, T: Ord + PartialOrd> PartialOrd for ScoredItem<N, T> {
    fn partial_cmp(&self, other: &ScoredItem<N, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


/// Parses a grid of digits in the form of a string to a HashMap<Coordinate<T>, V>
/// 
/// The Y value represents the line number, so increases down the page.
/// 
/// Example usage: 
/// 
/// `parse_number_grid::<i32, i32>("12\n34")`
/// 
/// will return a `HashMap<Coordinate<i32>, i32>` equivalent to:
/// ```
/// # #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
/// # pub struct Coordinate<T> {
/// # pub x: T,
/// # pub y: T }
/// #
/// # use std::collections::HashMap;
/// HashMap::<Coordinate<usize>,i32>::from([
///     (Coordinate{x:0, y:0}, 1),
///     (Coordinate{x:1, y:0}, 2),
///     (Coordinate{x:0, y:1}, 3),
///     (Coordinate{x:1, y:2}, 4)
/// ]);
/// ```
pub fn parse_number_grid<T, V>(data: &str) -> HashMap<Coordinate<T>, V> where 
        T: Integer + Copy + TryFrom<usize> + Hash,
        V: FromStr, 
        <V as FromStr>::Err: Debug  {

    let mut grid: HashMap<Coordinate<T>, V> = HashMap::new();

    for (y, line) in data.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate(){
            grid.insert( Coordinate{x: T::try_from(x).ok().unwrap(),  y: T::try_from(y).ok().unwrap()}, 
                        String::from(c).parse::<V>().unwrap());
        }
    }

    grid
}

/// Returns the lowest common multiple of two numbers
pub fn lcm<T: Integer + Copy>(first: T , second: T) -> T {
    first * second / gcd(first, second)
}

/// Returns the greatest common divisor of two numbers
pub fn gcd<T: Integer + Copy >(first: T, second: T) -> T {
    let mut max = first;
    let mut min = second;
    if min > max {
        (min, max) = (max,min)
    }

    loop {
        let res = max % min;
        if res == num::zero() {
            return min;
        }
        max = min;
        min = res;
    }
}



/// Retrieves and caches a day's input, returning it as a string
/// 
/// This helper function follows the [automation guidelines on the /r/adventofcode community wiki](https://www.reddit.com/r/adventofcode/wiki/faqs/automation).
/// 
/// Specifically:
/// `get_daily_input()` will attempt only one time to download any given input file
/// That file is stored locally on an indefinite basis and must be manually deleted
/// in the event that the download is corrupt; no automatic retry logic is provided.
/// 
/// The download file will be stored in `$HOME/.aochelpers/year/day`. Should this
/// file already exist, no download will be attempted.
/// 
/// In the event that the input cannot be downloaded, the error message will instead be
/// written to the input file, preventing further download attempts
/// 
/// The `User-Agent` header is set to `Rust AoCHelpers: docs.rs/aochelpers/latest/aochelpers/fn.get_daily_input.html by wilkotom@sleepawaytheafternoon.uk`
/// 
/// Session token is determined by the contents of `$HOME/.aochelpers/token` if it exists;
/// if it does not, this file will be created from the contents of the `AOCTOKEN` 
/// environment variable, if any.
pub fn get_daily_input(day: i32, year: i32) -> Result<String, Box<dyn Error>> {
    if let Some(mut path) = dirs::home_dir() {
        let rel_path = [".aochelpers", &year.to_string()];
        for element in rel_path {
            path.push(element);
            if let Ok(metadata) = fs::metadata(&path) {
                if !metadata.is_dir(){
                    return  Err(Box::new(
                        std::io::Error::new(ErrorKind::Other, format!("Path {} exists, but is not a directory", path.display()))
                    ));
                }
            }
        }
        path.push(day.to_string());
        if let Ok(mut content) = std::fs::read_to_string(&path) {
            remove_newlines(&mut content);
            Ok(content)
        } else {
            warn!("Fetching puzzle input for {} day {} from remote server", year,day);
            let mut token_path = path.clone();
            token_path.pop();
            fs::create_dir_all(&token_path)?;
            token_path.pop();
            token_path.push("token");
            let token = get_or_create_setting_file(&token_path, "AOCTOKEN")?;
            let client = reqwest::blocking::Client::builder()
                .user_agent("Rust AoCHelpers: docs.rs/aochelpers/latest/aochelpers/fn.get_daily_input.html by wilkotom@sleepawaytheafternoon.uk")
                .build()?;
            let res = client.get(format!("https://adventofcode.com/{}/day/{}/input", year, day))
                .header("Cookie", format!("session={}", token))
                .send()?;

            if res.status().is_success() {
                let mut response_text = res.text()?;
                remove_newlines(&mut response_text);
                let mut dir_path: PathBuf = path.clone();
                dir_path.pop();
                fs::create_dir_all(dir_path)?;        
                let mut file: File = File::create(path)?;
                file.write_all(response_text.as_bytes())?;
                Ok(response_text)
            } else {
                let mut file: File = File::create(path)?;
                file.write_all(res.status().as_str().as_bytes())?;
                Err(Box::new(std::io::Error::new(ErrorKind::Other, format!("Response was {}", res.status()))))
            }
        }
    } else {
        Err(Box::new(Box::new(std::io::Error::new(ErrorKind::Other,"Couldn't determine home directory"))))
    }
}

fn get_or_create_setting_file(path: &PathBuf, env_var: &str) -> Result<String, Box<dyn Error>> {
    if let Ok(contents) = std::fs::read_to_string(path) {
        Ok(contents)
    } else if let Ok(contents) = env::var(env_var) {
        let mut file: File = File::create(path)?;
        file.write_all(contents.as_bytes())?;
        Ok(contents)
    } else {
        Err(Box::new(
            std::io::Error::new(
                ErrorKind::Other, format!("Can't determine setting from {} or {}", path.display(), env_var))
        ))
    }
}

fn remove_newlines(s: &mut String) {
    while s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

/// Convenience-based struct converting typical AoC node labels consisting of numbers and
/// letters to a numeric representation, saveing all that tedious mucking about with lifetimes.
/// 
/// Note that Labels are case-insensitive.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Label {
    value: usize
}

impl Label {
    /// Returns true if the given character matches the end of this label.
    /// Returns false if it does not.
    pub fn ends_with(&self, c: char) -> bool {
        if let Some(v) = c.to_digit(36) {
            self.value % 36 == v as usize
        } else {
            false
        }
    }
}

impl FromStr for Label {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Label, anyhow::Error> {
        if s.is_empty() {
            Ok(Self{value:0})
        } else {
            Ok(Self { value: usize::from_str_radix(s,36)? })
        }
    }
}

impl From<usize> for Label {
    fn from(value: usize) -> Self {
        Label { value }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut digits = Vec::new();
        let mut remainder = self.value;
        while remainder > 0 {
            let digit = char::from_digit((remainder % 36 ) as u32 , 36).unwrap();
            digits.push(digit);
            remainder /= 36;
        }
        write!(f, "{}", digits.iter().rev().collect::<String>())
    }
}

/// Takes an Everybody Codes input from the local Downloads folder if unavailable in the cache.
/// In the future this may be expanded to download inputs directly.
/// 
pub fn get_everybodycodes_input(day: i32, year: i32, part: i32) -> Result<String, Box<dyn Error>> {
    if let Some(mut path) = dirs::home_dir() {
        let rel_path = [".everybodycodes", &year.to_string()];
        for element in rel_path {
            path.push(element);
            if let Ok(metadata) = fs::metadata(&path) {
                if !metadata.is_dir(){
                    return  Err(Box::new(
                        std::io::Error::new(ErrorKind::Other, format!("Path {} exists, but is not a directory", path.display()))
                    ));
                }
            }
        }
        path.push(format!("everybody_codes_e{}_q{:02}_p{}.txt", year, day, part));
        if let Ok(mut content) = std::fs::read_to_string(&path) {
            remove_newlines(&mut content);
            Ok(content)
        } else {
        let mut downloads_path = dirs::home_dir().unwrap();
        downloads_path.push("Downloads");
        downloads_path.push(format!("everybody_codes_e{}_q{:02}_p{}.txt", year, day, part));
        if let Ok(mut content) = std::fs::read_to_string(&downloads_path) {
            remove_newlines(&mut content);
            let mut file: File = File::create(path)?;
            file.write_all(content.as_bytes())?;
            Ok(content)
        } else {
            Err(Box::new(std::io::Error::new(ErrorKind::NotFound, format!("Couldn't find input file {:?} in cache or Downloads folder", downloads_path))))
        }
        }
        
     } else {
         Err(Box::new(Box::new(std::io::Error::new(ErrorKind::Other,"Couldn't determine home directory"))))
     }
}


/// Representation of a grid of objects which can be accessed by Coordinate<T>
/// Suited to dense grids of objects such as word search puzzles
/// Less suited to sparse grids as empoty points are stored
/// 
/// Methods are deliberately similar to those presented by `std::collections::HashMap``.
/// 
/// TODO: add tests
pub struct Grid<V>(Vec<Vec<Option<V>>>);


impl<V: Copy> Grid<V> {
    pub fn new() -> Self {
        Grid(Vec::new())
    }

    pub fn insert<T: PartialEq + PartialOrd + Into<usize> + From<usize> + Copy>(&mut self, c: Coordinate<T>, v: V) -> Option<V> {
        while c.y >= self.0.len().into() {
            self.0.push(Vec::new());
        }
        while c.x >= self.0[c.y.into()].len().into() {
            self.0[c.y.into()].push(None);
        }
        let old_value = self.0[c.y.into()][c.x.into()];
        self.0[c.y.into()][c.x.into()] = Some(v);
        old_value
    }

    pub fn get<T: PartialEq + PartialOrd + Into<usize> + From<usize> + Copy>(&self, c: &Coordinate<T>) -> Option<V> {
         if c.y >= self.0.len().into() || c.x>= self.0[c.y.into()].len().into() {
            None
        } else {
            self.0[c.y.into()][c.x.into()]
        }
    }

    pub fn contains_key<T: PartialEq + PartialOrd + Into<usize> + From<usize> + Copy>(&self, c: &Coordinate<T>) -> bool {
        if c.y >= self.0.len().into() || c.x>= self.0[c.y.into()].len().into() {
            false
        } else {
            self.0[c.y.into()][c.x.into()].is_some()
        }
    }

    pub fn iter<T: PartialEq + PartialOrd + Into<usize> + From<usize> + Copy>(&self)-> impl Iterator<Item = (Coordinate<T>, V)> + use<'_, T, V> {
       (0..self.0.len()).flat_map(move |y| 
            (0..self.0[y].len())
            .map(move |x|(Coordinate{x,y}, self.0[y][x])))
        .filter(|(_,v)| v.is_some())
        .map(|(k,v)| (Coordinate{x: k.x.into(), y: k.y.into()}, v.unwrap()))
    }

    pub fn keys<T: PartialEq + PartialOrd + Into<usize> + From<usize> + Copy>(&self)-> impl Iterator<Item = Coordinate<T>> + use<'_, T, V> {
        (0..self.0.len()).flat_map(move |y| 
             (0..self.0[y].len())
             .map(move |x|(Coordinate{x,y}, self.0[y][x])))
         .filter(|(_,v)| v.is_some())
         .map(|(k,_)| Coordinate::<T>{x: k.x.into(), y: k.y.into()})
 
     }
 

}

impl<V: Copy> Default for Grid<V> {
    fn default() -> Self {
        Grid::new()
    }
}