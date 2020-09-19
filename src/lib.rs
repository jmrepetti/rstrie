
type CmpPaths = (Option<&'static str>,  Option<&'static str>, Option<&'static str>);

#[derive(Clone, Default, Debug)]
pub struct Node<T> {
    key: &'static str,
    children: Vec<Node<T>>,
    value: Option<T>
}

impl<T> Node<T> {
    pub fn new() -> Self {
        Node::<T> {
            key: "",
            value: None,
            children: vec![]
        }
    }

    pub fn new_with(key: &'static str, value: Option<T>) -> Self {
        Node::<T> {
            key: key,
            value: value,
            children: vec![]
        }
    }

    pub fn new_with_children(key: &'static str, value: Option<T>, children: Vec<Node<T>>) -> Self {
        Node::<T> {
            key: key,
            value: value,
            children: children
        }
    }

    pub fn add(&mut self, path: &'static str, value: T) -> &mut Node<T>
    where T: Copy
    {
        if self.children.is_empty() {
            let new_node = Self::new_with(path, Some(value));
            self.children.push(new_node);
        } else {
            Self::add_to_node(path, value, self);
        }
        self
    }

    pub fn find(&mut self, path: &'static str) -> Option<Node<T>>
    where T: Copy
    {
        Self::find_in_node(&self, path, path.chars().count())
    }

    /// return how many characters both string share at the beginning
    fn shared_chars_idx(path: &'static str, existing_key: &'static str) -> Option<usize> {

        let mut counter = 0;
        let mut path_iter = path.chars();
        let mut frag_iter = existing_key.chars();

        while let (Some(path_char), Some(frag_char)) = (path_iter.next(), frag_iter.next()) {
            if path_char == frag_char {
                counter = counter + 1;
            } else {
                break;
            }
        };

        if counter > 0 {
            Some(counter)
        } else {
            None
        }
    }

    fn cmp_paths(add_path: &'static str, existing_key: &'static str) ->  CmpPaths {
        if let Some(shared_idx) = Self::shared_chars_idx(add_path, existing_key) {

            let (shared_prefix, l_reminder) = add_path.split_at(shared_idx);
            let (_, r_reminder) = existing_key.split_at(shared_idx);

            let left = if l_reminder.is_empty() {
                None
            } else {
                Some(l_reminder)
            };

            let rigth = if r_reminder.is_empty() {
                None
            } else {
                Some(r_reminder)
            };

            (Some(shared_prefix), left, rigth)

        } else {
            (None, None, None)
        }
    }

    fn add_to_node(path: &'static str, value: T, node: &mut Node<T>)
        where T: Copy
    {
        let mut new_leaf = true;
        for (child_idx, child) in node.children.iter_mut().enumerate() {
            match Self::cmp_paths(path, child.key) {
                (Some(_), None, None) => {
                    child.value = Some(value);
                    return ()
                },
                (None, _, _) => { continue; }, //no match, continue with next child
                (Some(_), Some(path_reminder), None) => { // serch under this node and add
                    Self::add_to_node(path_reminder, value, child);
                    new_leaf = false;
                    break;
                },
                (Some(shared_prefix), maybe_path_reminder, Some(key_reminder)) => {
                    // child get updated with a new key
                    child.key = key_reminder;
                    // child(and children) will move to new_node children
                    let mut children = vec![child.clone()];

                    if let Some(path_reminder) = maybe_path_reminder {
                        // if path_reminder exist, it should be added as child node of new_node
                        let new_child_node = Self::new_with(path_reminder, Some(value));
                        children.push(new_child_node)
                    };

                    // if given path become a new prefix node, it should keep value
                    let new_node = if maybe_path_reminder.is_none() {
                        Self::new_with_children(shared_prefix, Some(value), children)
                    // if given path become a new prefix node, it should keep value
                    } else {
                        Self::new_with_children(shared_prefix, None, children)
                    };

                    node.children.remove(child_idx);
                    node.children.push(new_node);
                    new_leaf = false;
                    break;
                }
            };
        }

        if new_leaf {
            let new_node = Self::new_with(path, Some(value));
            node.children.push(new_node);
        }
    }

    fn find_in_node(current_node: &Node<T>, path: &'static str, chars_left: usize) -> Option<Node<T>>
        where T: Copy
    {
        if path.starts_with(current_node.key) {
            let chars_left = chars_left - current_node.key.chars().count();
            if chars_left > 0 {
                let striped_path = path.strip_prefix(current_node.key).unwrap();
                Self::find_in_children(current_node, striped_path, chars_left)
            } else {
                Some(current_node.to_owned())
            }
        } else {
            None
        }
    }

    fn find_in_children(node: &Node<T>, path: &'static str, chars_left: usize) -> Option<Node<T>>
    where T: Copy
    {
        let mut found: Option<Node<T>> = None;
        for current_node in &node.children {
            found = Self::find_in_node(current_node, path, chars_left);
            if found.is_some() { break; }
        }
        found
    }
}

pub fn new_trie<T>() -> Node<T> {
    Node::<T>::new()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_default_tree() {
        let mut tr = new_trie::<u8>();
        assert_eq!("", tr.key);
    }

    #[test]
    fn shared_chars() {
        assert_eq!(Some(5), Node::<u8>::shared_chars_idx("romane", "romanus"));
    }

    #[test]
    fn compare_paths() {
        // Same string
        assert_eq!((Some("abc"), None, None), Node::<u8>::cmp_paths("abc", "abc"));
        // Shared prefix with remainder on the input string
        assert_eq!((Some("abc"), Some("d"), None), Node::<u8>::cmp_paths("abcd", "abc"));
        // Shared prefix with remainder on the existing prefix, new prefix.
        assert_eq!((Some("ab"), None, Some("c")), Node::<u8>::cmp_paths("ab", "abc"));
        // Shared prefix with remainder on  input and existing string
        assert_eq!((Some("abc"), Some("def"), Some("hij")), Node::<u8>::cmp_paths("abcdef", "abchij"));
        // No match
        assert_eq!((None,None,None), Node::<u8>::cmp_paths("some", "noen"));
    }

    #[test]
    fn romane_romanus_test() {
        let mut tr = new_trie::<u8>();
        tr.add("romane", 1);
        tr.add("romanus", 2);
        tr.add("romulus", 3);
        tr.add("rubens", 4);
        tr.add("ruber", 5);
        tr.add("rubicon", 6);
        tr.add("rubicundus", 7);
        assert_eq!(Some(1), tr.find("romane").unwrap().value);
        assert_eq!(Some(2), tr.find("romanus").unwrap().value);
        assert_eq!(Some(3), tr.find("romulus").unwrap().value);
        assert_eq!(Some(4), tr.find("rubens").unwrap().value);
        assert_eq!(Some(5), tr.find("ruber").unwrap().value);
        assert_eq!(Some(6), tr.find("rubicon").unwrap().value);
        assert_eq!(Some(7), tr.find("rubicundus").unwrap().value);
    }

    #[test]
    fn new_insertion_becomes_prefix_test() {
        let mut tr = new_trie::<u8>();
        tr.add("/abc", 1);
        assert_eq!("/abc", tr.children[0].key);
        tr.add("/ab", 2);
        assert_eq!("/ab", tr.children[0].key);
        assert_eq!("c", tr.children[0].children[0].key);
        assert_eq!(Some(2), tr.find("/ab").unwrap().value);
        assert_eq!(Some(1), tr.find("/abc").unwrap().value);
    }

    #[test]
    fn new_insertion_added_to_existing_prefix_test() {
        let mut tr = new_trie::<u8>();
        tr.add("/abc", 1);
        assert_eq!("/abc", tr.children[0].key);
        tr.add("/abcdef", 2);
        assert_eq!("/abc", tr.children[0].key);
        assert_eq!("def", tr.children[0].children[0].key);
        assert_eq!(Some(1), tr.find("/abc").unwrap().value);
        assert_eq!(Some(2), tr.find("/abcdef").unwrap().value);
    }

    #[test]
    fn new_insertion_with_shared_prefix_chars_test() {
        let mut tr = new_trie::<u8>();
        tr.add("/abc", 1);
        tr.add("/def", 2);
        assert_eq!("/", tr.children[0].key);
        assert_eq!("abc", tr.children[0].children[0].key);
        assert_eq!("def", tr.children[0].children[1].key);
        assert_eq!(Some(1), tr.find("/abc").unwrap().value);
        assert_eq!(Some(2), tr.find("/def").unwrap().value);
        assert!(tr.children[0].children[0].children.is_empty());
        assert!(tr.children[0].children[1].children.is_empty());
    }

    #[test]
    fn new_prefix_with_value() {
        let mut tr = new_trie::<u8>();
        tr.add("newsomething", 1);
        tr.add("newprefix", 2);
        // this will promote a 'new' node with two nodes 'something' and 'prefix'.
        // dbg!(&tr);
        assert_eq!(None, tr.find("new").unwrap().value);
        assert_eq!(Some(1), tr.find("newsomething").unwrap().value);
        assert_eq!(Some(2), tr.find("newprefix").unwrap().value);
    }

    #[test]
    fn override_value() {
        let mut tr = new_trie::<u8>();
        tr.add("newsomething", 1);
        tr.add("newprefix", 2);
        // this will promote a 'new' node with two nodes 'something' and 'prefix'.
        assert_eq!(None, tr.find("new").unwrap().value);
        assert_eq!(Some(1), tr.find("newsomething").unwrap().value);
        assert_eq!(Some(2), tr.find("newprefix").unwrap().value);
        //override
        tr.add("new", 3);
        tr.add("newprefix", 22);
        assert_eq!(Some(3), tr.find("new").unwrap().value);
        assert_eq!(Some(22), tr.find("newprefix").unwrap().value);
    }

    #[test]
    fn tree_with_custom_value() {
        type AValueType = fn(i32, i32) -> i32;

        fn add(x: i32, y: i32) -> i32 {
            x + y
        }
        let mut tr = new_trie::<AValueType>();
        let add_fn: AValueType = add;
        tr.add("add_fn", add_fn);
        let adder = tr.find("add_fn").unwrap().value.unwrap();
        assert_eq!(42, adder(20,22));
    }
}
