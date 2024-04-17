use core::fmt;

pub struct BST<T: Ord> {
    val: T,
    left: Option<Box<BST<T>>>,
    right: Option<Box<BST<T>>>,
}

impl<T: Ord + Default> Default for BST<T> {
    fn default() -> Self {
        BST {
            val: T::default(),
            left: None,
            right: None,
        }
    }
}

impl<T: Ord + Clone> BST<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, val: T) {
        if val <= self.val {
            match &mut self.left {
                Some(s) => s.insert(val),
                None => self.left = Some(Box::new(Self::new(val))),
            }
        } else {
            match &mut self.right {
                Some(s) => s.insert(val),
                None => self.right = Some(Box::new(Self::new(val))),
            }
        }
    }

    pub fn max(&self) -> &T {
        let mut h = &self.right;
        let mut max = &self.val;
        while let Some(node) = h {
            max = &node.val;
            h = &node.right;
        }

        max
    }

    pub fn min(&self) -> &T {
        let mut h = &self.left;
        let mut min = &self.val;
        while let Some(node) = h {
            min = &node.val;
            h = &node.left;
        }

        min
    }

    pub fn contains(&self, val: &T) -> bool {
        if self.val == *val {
            true
        } else if *val < self.val {
            match &self.left {
                None => false,
                Some(node) => node.contains(val),
            }
        } else {
            match &self.right {
                None => false,
                Some(node) => node.contains(val),
            }
        }
    }

    pub fn try_delete(&mut self, val: T) -> bool {
        if val < self.val {
            if let Some(left_node) = &mut self.left {
                if left_node.val == val {
                    self.val = left_node.val.clone();
                    self.left = None;
                    true
                } else {
                    left_node.try_delete(val)
                } 
            }else {
                    false 
                }
        } else {
            if let Some(right_node) = &mut self.right {
                if right_node.val == val {
                    self.val = right_node.val.clone();
                    self.right = None;
                    true
                } else {
                    right_node.try_delete(val)
                }
            } else {
                false
            }
        }
    }
}

impl<T: Ord + fmt::Display> BST<T> {
    fn as_str_internal(&self, level: usize) -> String {
        let mut string = String::new();
        match &self.right {
            Some(s) => string += &s.as_str_internal(level + 1),
            None => {}
        };
        string += &format!(
            "{spacing}|--> {val}\n",
            spacing = " ".repeat(level * 5),
            val = self.val
        );
        match &self.left {
            Some(s) => string += &s.as_str_internal(level + 1),
            None => {}
        };

        string
    }
}

impl<T: Ord + fmt::Display> fmt::Display for BST<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str_internal(0))?;
        Ok(())
    }
}
