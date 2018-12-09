pub mod errors;
use crate::errors::TreeParseError;

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    metadata: Vec<i32>,
}

impl Node {
    fn new(num_children: usize, num_metadata: usize) -> Node {
        Node {
            children: Vec::with_capacity(num_children),
            metadata: Vec::with_capacity(num_metadata),
        }
    }

    pub fn sum_metadata(&self) -> i32 {
        self.metadata.iter().sum::<i32>()
            + self
                .children
                .iter()
                .map(|child| child.sum_metadata())
                .sum::<i32>()
    }

    pub fn value(&self) -> i32 {
        if self.children.is_empty() {
            self.metadata.iter().sum::<i32>()
        } else {
            let mut value = 0;
            for n in &self.metadata {
                let index = (n - 1) as usize;
                if let Some(child) = self.children.get(index) {
                    value += child.value();
                }
            }

            value
        }
    }

    pub fn read<'a, T: Iterator<Item = &'a i32>>(numbers: &mut T) -> Result<Node, TreeParseError> {
        let num_children = match numbers.next() {
            Some(n) => *n as usize,
            None => {
                return Err(TreeParseError::MissingData);
            }
        };

        let num_metadata = match numbers.next() {
            Some(n) => *n as usize,
            None => {
                return Err(TreeParseError::MissingData);
            }
        };

        let mut node = Node::new(num_children, num_metadata);

        for _ in 0..num_children {
            let child = Node::read(numbers)?;
            node.children.push(child);
        }

        for _ in 0..num_metadata {
            match numbers.next() {
                Some(n) => {
                    node.metadata.push(*n);
                }

                None => {
                    return Err(TreeParseError::MissingData);
                }
            };
        }

        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_tree() -> Node {
        let input = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        Node::read(&mut input.iter()).unwrap()
    }

    #[test]
    fn example_sum_of_metadata() {
        let root_node = example_tree();

        assert_eq!(138, root_node.sum_metadata());
    }

    #[test]
    fn example_node_value() {
        let root_node = example_tree();
        assert_eq!(66, root_node.value());
    }
}
