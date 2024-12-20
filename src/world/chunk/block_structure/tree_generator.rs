use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Branch {
    position: (u32, u32),
    direction: (i32, i32),
    change_direction_chance: f32,
    branching_chance: f32,
    die_chance: f32,
}

impl Branch {
    fn new(x: u32, y: u32, dir_x: i32, change_direction_chance: f32, branching_chance: f32, die_chance: f32) -> Self {
        Self {
            position: (x,y),
            direction: (dir_x,1),
            change_direction_chance,
            branching_chance,
            die_chance,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Log,
    LogLeft,
    LogRight,
    LogHorisontal,
    Leaves,
    Air,
}

pub struct Tree {
    pub grid: [[Tile; 16]; 16],
    branches: Vec<Branch>,
    branches_to_add: Vec<Branch>,
    branches_to_remove: Vec<Branch>,
    finished_branches: Vec<Branch>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            grid: [[Tile::Air; 16]; 16],
            branches: vec![Branch::new(8, 0, 0, 0.75, 0.0, 0.0)],
            branches_to_add: vec![],
            branches_to_remove: vec![],
            finished_branches: vec![]
        }
    }

    pub fn grow(&mut self) {
        while !self.branches.is_empty() {
            for branch in self.branches.iter_mut() {
                let tile = match branch.direction {
                    (0, 1) => Tile::Log,
                    (1, 1) => Tile::LogRight,
                    (-1, 1) => Tile::LogLeft, 
                    (1, 0) => Tile::LogHorisontal,
                    (-1, 0) => Tile::LogHorisontal,
                    _ => Tile::Log,
                };
    
                self.grid[branch.position.0 as usize][branch.position.1 as usize] = tile;
                
                if branch.die_chance > rand::random() {
                    self.branches_to_remove.push(branch.clone());
                    self.finished_branches.push(branch.clone());
                    continue;
                }
                else {
                    branch.die_chance += 0.15;
                }

                branch.position.0 = (branch.position.0 as i32 + branch.direction.0) as u32;
                branch.position.1 = (branch.position.1 as i32 + branch.direction.1) as u32;
                if branch.change_direction_chance > rand::random() {
                    let mut rng = rand::thread_rng();
                    branch.direction = match tile {
                        Tile::Log => {
                            (rng.gen_range(-1..=1), 1)
                        },
                        Tile::LogLeft => {
                            match rng.gen_range(0..=3) {
                                0 => (-1, 1),
                                1 => (0, 1),
                                2 => (-1, 0),
                                3 => (-1, 0),
                                _ => (0, 0)
                            }
                        },
                        Tile::LogRight => {
                            match rng.gen_range(0..=3) {
                                0 => (1, 1),
                                1 => (0, 1),
                                2 => (1, 0),
                                3 => (1, 0),
                                _ => (0, 0)
                            }
                        },
                        Tile::LogHorisontal => {
                            if branch.direction.0 > 0 {
                                (1, rng.gen_range(0..=1))
                            }
                            else {
                                (-1, rng.gen_range(0..=1))
                            }
                        },
                        _ => (0, 0)
                    };
                }

                if branch.branching_chance > rand::random() {
                    let direction: i32 = match rand::random::<bool>() {
                        true => 1,
                        false => -1
                    };
                    
                    self.branches_to_add.push(
                        Branch::new(
                            (branch.position.0 as i32 + direction) as u32,
                            branch.position.1,
                            direction,
                            branch.change_direction_chance / 2.0,
                            0.2,
                            branch.die_chance,
                        )
                    );
                }
                else {
                    branch.branching_chance += 0.2;
                }
            }
    
            self.branches.append(&mut self.branches_to_add);
            self.branches_to_add.clear();
    
            self.branches = self.branches.clone().into_iter().filter(|branch| !self.branches_to_remove.contains(branch)).collect();
            self.branches_to_remove.clear();
        }
    }

    pub fn generate_leaves(&mut self) {
        for branch in self.finished_branches.iter() {
            self.grid[branch.position.0 as usize+1][branch.position.1 as usize] = Tile::Leaves;
            self.grid[branch.position.0 as usize-1][branch.position.1 as usize] = Tile::Leaves;
            self.grid[branch.position.0 as usize][branch.position.1 as usize+1] = Tile::Leaves;
            // self.grid[branch.position.0 as usize][branch.position.1 as usize-1] = Tile::Leaves;
        }
    }
}