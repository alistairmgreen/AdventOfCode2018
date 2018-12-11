use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Node {
    left: usize,
    right: usize,
}

pub struct CircularList {
    nodes: Vec<Option<Node>>,
}

impl CircularList {
    pub fn with_capacity(capacity: usize) -> CircularList {
        let mut nodes = vec![None; capacity];
        nodes[0] = Some(Node { left: 0, right: 0 });

        CircularList { nodes }
    }

    pub fn left_of(&self, existing_value: usize) -> Option<usize> {
        let node = self.nodes.get(existing_value)?;
        node.map(|n| n.left)
    }

    pub fn right_of(&self, existing_value: usize) -> Option<usize> {
        let node = self.nodes.get(existing_value)?;
        node.map(|n| n.right)
    }

    pub fn insert_after(&mut self, existing_value: usize, new_value: usize) {
        let right_value = self
            .right_of(existing_value)
            .expect("Value to insert before does not exist");

        let new_node = Node {
            left: existing_value,
            right: right_value,
        };
        self.nodes[new_value] = Some(new_node);

        let previous_node = self
            .nodes
            .get_mut(existing_value)
            .expect("Node to insert after does not exist")
            .as_mut()
            .expect("Node to insert after is not initialised");

        previous_node.right = new_value;

        let next_node = self
            .nodes
            .get_mut(right_value)
            .expect("Node to insert before does not exist")
            .as_mut()
            .expect("Node to insert before is not initialised");

        next_node.left = new_value;
    }

    pub fn remove(&mut self, value_to_remove: usize) {
        if let Some(removed_node) = self.nodes[value_to_remove] {
            self.nodes[value_to_remove] = None;

            let mut left_node = self
                .nodes
                .get_mut(removed_node.left)
                .expect("Index of left node out of range")
                .as_mut()
                .expect("Left node does not exist");
            left_node.right = removed_node.right;

            let mut right_node = self
                .nodes
                .get_mut(removed_node.right)
                .expect("Index of right node out of range")
                .as_mut()
                .expect("Right node does not exist");
            right_node.left = removed_node.left;
        }
    }
}

impl fmt::Debug for CircularList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut number = 0;
        write!(f, "CircularList {{ ")?;
        while let Some(Some(node)) = self.nodes.get(number) {
            write!(f, "{} ", number)?;
            number = node.right;
            if number == 0 {
                break;
            }
        }

        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_list_insert() {
        let mut circle = CircularList::with_capacity(3);

        circle.insert_after(0, 1);

        assert_eq!(Some(0), circle.left_of(1));
        assert_eq!(Some(0), circle.right_of(1));

        assert_eq!(Some(1), circle.left_of(0));
        assert_eq!(Some(1), circle.right_of(0));

        circle.insert_after(1, 2);

        assert_eq!(Some(2), circle.right_of(1));
        assert_eq!(Some(0), circle.right_of(2));
        assert_eq!(Some(1), circle.left_of(2));
    }

    #[test]
    fn test_circular_list_remove() {
        let mut circle = CircularList::with_capacity(3);
        circle.insert_after(0, 1);
        circle.insert_after(1, 2);
        circle.remove(1);

        assert_eq!(Some(2), circle.right_of(0));
        assert_eq!(Some(2), circle.left_of(0));
        assert_eq!(Some(0), circle.left_of(2));
        assert_eq!(Some(0), circle.right_of(2));
    }
}
