// Struct for positions
#[derive(PartialEq, PartialOrd, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
    g_cost: Option<i32>,
    h_cost: Option<i32>,
    parrent: Option<Box<Self>>,
}
impl Default for Vec2 {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            g_cost: None,
            h_cost: None,
            parrent: None,
        }
    }
}
impl Vec2 {
    fn f_cost(self) -> Option<i32> {
        let f_cost: i32 = self.g_cost.unwrap() + self.h_cost.unwrap();
        Some(f_cost)
    }
    fn copy(self) -> Vec2 {
        Vec2{
            x: self.x,
            y: self.y,
            g_cost: self.g_cost,
            h_cost: self.h_cost,
            parrent: self.parrent,
        }
    }
}

// Find start and end
// fn find_positions(arr: &mut Vec<String>) -> (Vec2, Vec2) {
//     // vars for storig the start and end
//     let a: Option<Vec2> = None;
//     let b: Option<Vec2> = None;
//     // Loop over the array of strings
//     for (y, s) in arr.iter().enumerate() {
//         for (x, c) in s.chars().enumerate() {
//             match c {
//                 'A' => Vec2{
//                         x: x.try_into().unwrap(),
//                         y: y.try_into().unwrap(), 
//                         ..Default::default()},
//                 'B' => Vec2{
//                         x: x.try_into().unwrap(), 
//                         y: y.try_into().unwrap(), 
//                         ..Default::default()},
//                 _ => continue
//             };
//         }
//     }

//     // Rust magic
//     (a.unwrap(), b.unwrap())
// }

// Find item with lowest f_cost
fn lowest_f_cost(arr: &Vec<Vec2>) -> Vec2 {
    let mut lowest: Option<Vec2> = None;
    for item in arr.into_iter() {
        if lowest == None {
            lowest = Some(item.to_owned().copy());
            continue;
        }
        if item.clone().f_cost() < lowest.to_owned().unwrap().copy().f_cost()
        || item.clone().f_cost() == lowest.to_owned().unwrap().copy().f_cost()
        && item.h_cost < lowest.clone().unwrap().h_cost
        {
            let i = item.to_owned().copy();
            lowest = Some(i);
        }
    }
    lowest.expect("No lowest found")
}

fn find_neighbours(point: &Vec2, map: &mut Vec<String>) -> Vec<Vec2> {
    let mut neighbours: Vec<Vec2> = vec![];

    for x in -1..2 {
        for y in -1..2 {
            // Center
            if x == 0 && y == 0 { continue; }

            // Corners
            if x == -1 && y == -1 { continue; } // top left
            if x ==  1 && y == -1 { continue; } // top right
            if x == -1 && y ==  1 { continue; } // bottom left
            if x ==  1 && y ==  1 { continue; } // bottom right

            let check_x = point.x + x;
            let check_y = point.y + y;

            // Outside map
            if check_x < 0 { continue; }
            if check_x >= map.len().try_into().unwrap() { continue; }
            if check_y < 0 { continue; }
            if check_y >= map.len().try_into().unwrap() { continue; }

            neighbours.append(&mut vec![Vec2{x: check_x, y: check_y, ..Default::default()}]);
        }
    }

    neighbours
}

fn get_distance(point_a: &Vec2, point_b: &Vec2) -> i32 {
    let y = (point_a.y - point_b.y).abs();
    let x = (point_a.x - point_b.x).abs();

    if x > y {
        return 14 * y + 10 * (x - y);
    }
    
    return 14 * x + 10 * (y - x);
}

fn retrace_path(point: &Vec2, map: &mut Vec<String>) {
    let mut node: Vec2 = point.to_owned().copy();
    while node.parrent != None {
        map[node.y as usize] = map[node.y as usize].chars().enumerate() 
            .map(|(i, c)| if i == node.x as usize { 'P' } else { c })
            .collect::<String>();

        node = node.parrent.unwrap().to_owned().copy();
    }
}

fn find_path(start: &Vec2, end: &Vec2, map: &mut Vec<String>) {
    // vec of positions to evaluate
    let mut open: Vec<Vec2> = vec![start.to_owned().copy()];
    // vec of positions already evaluated
    let mut closed: Vec<Vec2> = vec![];

    while open.len() > 0 {
        let current: Vec2 = lowest_f_cost(&open);

        {
            let mut indecies: Vec<usize> = Vec::new();
            for (i, p) in open.iter().enumerate() {
                if p.x == current.x && p.y == current.y {
                    indecies.append(&mut vec![i]);
                }
            }
            let mut n = 0;
            for i in indecies.iter() {
                open.remove(*i - n);
                n += 1;
            }
        }

        closed.append(&mut vec![current.to_owned().copy()]);

        if current == *end {
            retrace_path(&current, map);
            // Found
            return;
        }

        for mut neighbour in find_neighbours(&current, map).into_iter() {
            if closed.contains(&neighbour) { continue; }
            if map.get(neighbour.y as usize).unwrap().as_bytes()[neighbour.x as usize] as char == ' ' { continue; }

            let new_movement_cost = current.g_cost.unwrap() + get_distance(&current, &neighbour);
            if new_movement_cost < current.g_cost.unwrap() || !open.contains(&neighbour) && !closed.contains(&neighbour) {
                print!("new   ");
                neighbour.g_cost = Some( new_movement_cost.clone() );
                neighbour.h_cost = Some( get_distance(&neighbour, &end) );
                neighbour.parrent = Some(Box::new(current.to_owned().copy()));

                open.append(&mut vec![neighbour.to_owned().copy()])
            }
        }
    }
}

fn print_map(map: &mut Vec<String>) {
    for string in map.iter() {
        println!("{string}");
    }
}

fn main() {
    // Make map

    // ' ' = walkable
    // '#' = wall
    // 'A' = start
    // 'B' = end
    // 'P' = final path
    let mut map: Vec<String> = vec![
        "##########".to_string(),
        "#        #".to_string(),
        "#        #".to_string(),
        "#        #".to_string(),
        "#        #".to_string(),
        "##########".to_string(),
    ];
    print_map(&mut map);
    // Get start and end positions
    // let (start, end) = find_positions(&mut map);
    let mut start = Vec2 { x: 1, y: 1, ..Default::default() };
    let mut end = Vec2 { x: 8, y: 4, ..Default::default()};
    start.g_cost = Some(get_distance(&start, &start));
    start.h_cost = Some(get_distance(&start, &end));
    find_path(&start, &end, &mut map);
    println!();
    print_map(&mut map);
}
