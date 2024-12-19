pub struct BlockStructure {
    pub data: Vec<Vec<u32>>,
    pub bg_data: Vec<Vec<u32>>,
}

impl BlockStructure {
    pub fn new_tree(logs: u32) -> Self {
        let mut data = vec![
            vec![0, 5, 5, 5, 0],
            vec![0, 5, 5, 5, 0],
            vec![5, 5, 5, 5, 5],
            vec![5, 5, 5, 5, 5],
        ];

        for _ in 0..logs {
            data.push(vec![0,0,4,0,0]);
        }

        data.reverse();

        Self {
            data,
            bg_data: vec![vec![0; 5]; 4+logs as usize],
        }
    }

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
            bg_data
        }
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }
}