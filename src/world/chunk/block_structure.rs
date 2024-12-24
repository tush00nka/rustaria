pub struct BlockStructure {
    pub data: Vec<Vec<u32>>,
    pub bg_data: Vec<Vec<u32>>,
    pub fill_air: bool,
}

impl BlockStructure {
    pub fn new_tree(logs: u32) -> Self {
        // let mut tree = Tree::new();
        // tree.grow();
        // tree.generate_leaves();

        // let mut data = vec![];

        // for y in 0..tree.grid.len() {
        //     let mut row = vec![];
        //     for x in 0..tree.grid[0].len() {
        //         let id = match tree.grid[x][y] {
        //             Tile::Air => 0,
        //             Tile::Leaves => 5,
        //             _ => 4,
        //         };
        //         row.push(id as u32);
        //     }
        //     data.push(row);
        // }

        let mut data = vec![
            vec![0, 5, 5, 5, 0],
            vec![0, 5, 5, 5, 0],
            vec![5, 5, 5, 5, 5],
            vec![5, 5, 5, 5, 5],
        ];
        data.append(&mut vec![vec![0,0,4,0,0]; logs as usize]);

        data.reverse();

        Self {
            data,
            bg_data: vec![vec![0; 16]; 16],
            fill_air: false,
        }
    }

    #[allow(unused)]
    pub fn new_house() -> Self {
        let mut data = vec![
            vec![3,3,3,3,3,3],
            vec![3,0,0,0,0,3],
            vec![0,0,0,0,0,0],
            vec![0,0,0,0,0,0],
            vec![3,3,3,3,3,3],
        ];

        let mut bg_data = vec![vec![3; 6]; 5];

        data.reverse();
        bg_data.reverse();

        Self {
            data,
            bg_data,
            fill_air: true,
        }
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }
}