struct Node {
    values: Vec<i32>,
    children: Vec<Node>,
}

impl Node {
    pub fn values<'a>(&'a self) -> Box<dyn Iterator<Item = &'a i32> + 'a> {
        Box::new(
            self.values
                .iter()
                .chain(self.children.iter().map(|n| n.values()).flatten()),
        )
    }

    pub fn values2(&self) -> Box<dyn Iterator<Item = &i32> + '_> {
        Box::new(
            self.values
                .iter()
                .chain(self.children.iter().map(|n| n.values2()).flatten()),
        )
    }

    pub fn values3(&self) -> impl Iterator<Item = &i32> {
        self.values
            .iter()
            .chain(self.children.iter().map(|n| n.values.iter()).flatten())
    }

    fn get_values(&self) -> Box<dyn Iterator<Item = &i32> + '_> {
        Box::new(self
        	.values
            .iter()
            .chain(self.children.iter().map(|n| n.get_values()).flatten())
        )
    }
}

fn main() {
    let n = Node {
        values: vec![1, 2, 3],
        children: vec![
            Node {
                values: vec![4, 5],
                children: vec![],
            },
            Node {
                values: vec![6, 7],
                children: vec![],
            },
        ],
    };
    println!("{:#?}", n.values3().collect::<Vec<_>>());
}
