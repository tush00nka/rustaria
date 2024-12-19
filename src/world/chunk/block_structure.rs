pub struct BlockStructure {
    pub data: Vec<Vec<u32>>,
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
            data
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

        data.reverse();

        Self {
            data
        }
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.data[0].len()
    }
}