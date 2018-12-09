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

    #[test]
    fn parse_example_nodes() {
        let numbers: Vec<i32> = vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let mut iterator = numbers.iter();
        let root = Node::read(&mut iterator).unwrap();

        assert_eq!(138, root.sum_metadata());
    }
}
