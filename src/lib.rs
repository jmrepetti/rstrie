
type CmpFrag = (Option<&'static str>,  Option<&'static str>, Option<&'static str>);

#[derive(Clone, Default, Debug)]
pub struct Node<T> {
    frag: &'static str,
    children: Vec<Node<T>>,
    value: Option<T>
}

impl<T> Node<T> {
    pub fn new() -> Self {
        Node::<T> {
            frag: "",
            value: None,
            children: vec![]
        }
    }

    pub fn add(&mut self, path: &'static str, value: T)
        where T: Copy
    {
        for (child_idx, child) in self.children.iter_mut().enumerate() {
            match Self::cmp_frag(path, child.frag) {
                (None, _, _) => { continue; }, //no match, next
                (Some(_), None, None) => { // override existing value
                    child.value = Some(value);
                    return ()
                },
                (Some(_), Some(path_reminder), None) => { // serch under this node and add
                    child.add(path_reminder, value);
                    return ()
                },
                (Some(shared_prefix), path_reminder, Some(frag_rem)) => {
                    let mut new_child = child.clone();
                    new_child.frag = frag_rem;
                    let mut children = vec![new_child];
                    self.children.remove(child_idx);

                    let new_node = if path_reminder.is_none() {
                        Self{frag: shared_prefix, value: Some(value), children}
                    } else {
                        let new_child_node = Self{frag: path_reminder.unwrap(), value: Some(value), children: vec![]};
                        children.push(new_child_node);
                        Self{frag: shared_prefix, value: None, children}
                    };
                    self.children.push(new_node);
                    return ()
                }
            };
        }
        self.children.push(Self{frag: path, value: Some(value), children: vec![]});
    }

    pub fn find(&mut self, path: &'static str) -> Option<Node<T>>
    where T: Copy
    {
        // Self::find_in_node(&self, path, path.chars().count())
        if path.starts_with(self.frag) {
            let chars_left = path.chars().count() - self.frag.chars().count();
            if chars_left > 0 {
                let striped_path = path.strip_prefix(self.frag).unwrap();
                //Self::find_in_children(self, striped_path, chars_left)
                // let mut found: Option<Node<T>> = None;
                for child in self.children.iter_mut() {
                    let found = child.find(striped_path);
                    if found.is_some() { 
                        dbg!("verrd");
                        return found.to_owned();
                    }
                }
                None
            } else {
                Some(self.to_owned())
            }
        } else {
            None
        }
    }

    // fn find_in_node(current_node: &Node<T>, path: &'static str, chars_left: usize) -> Option<Node<T>>
    // where T: Copy
    // {
    //     if path.starts_with(current_node.frag) {
    //         let chars_left = chars_left - current_node.frag.chars().count();
    //         if chars_left > 0 {
    //             let striped_path = path.strip_prefix(current_node.frag).unwrap();
    //             Self::find_in_children(current_node, striped_path, chars_left)
    //         } else {
    //             Some(current_node.to_owned())
    //         }
    //     } else {
    //         None
    //     }
    // }

    // fn find_in_children(node: &Node<T>, path: &'static str, chars_left: usize) -> Option<Node<T>>
    // where T: Copy
    // {
    //     let mut found: Option<Node<T>> = None;
    //     for current_node in &node.children {
    //         found = Self::find_in_node(current_node, path, chars_left);
    //         if found.is_some() { break; }
    //     }
    //     found
    // }

    /// return how many characters both string share at the beginning
    fn shared_pref_idx(lfrag: &'static str, rfrag: &'static str) -> Option<usize> {

        let mut counter = 0;
        let mut lfrag_iter = lfrag.chars();
        let mut rfrag_iter = rfrag.chars();

        while let (Some(lchar), Some(rchar)) = (lfrag_iter.next(), rfrag_iter.next()) {
            if lchar == rchar {
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
    
    // Return (shared_prefix, l_reminder, r_reminder)
    fn cmp_frag(lfrag: &'static str, rfrag: &'static str) ->  CmpFrag {
        if let Some(shared_idx) = Self::shared_pref_idx(lfrag, rfrag) {

            let (shared_prefix, l_reminder) = lfrag.split_at(shared_idx);
            let (_, r_reminder) = rfrag.split_at(shared_idx);

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
}

pub fn new_trie<T>() -> Node<T> {
    Node::<T>::new()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_default_tree() {
        let tr = new_trie::<u8>();
        assert_eq!("", tr.frag);
    }

    #[test]
    fn shared_pref() {
        assert_eq!(Some(5), Node::<u8>::shared_pref_idx("romane", "romanus"));
    }

    #[test]
    fn cmp_frag_test() {
        assert_eq!(
            (Some("frag"), None, None), 
            Node::<u8>::cmp_frag("frag", "frag"));
        assert_eq!(
            (Some("frag"), Some("ment"), None), 
            Node::<u8>::cmp_frag("fragment", "frag"));
        assert_eq!(
            (Some("frag"), None, Some("ance")), 
            Node::<u8>::cmp_frag("frag", "fragance"));
        assert_eq!(
            (Some("frag"), Some("ment"), Some("ance")), 
            Node::<u8>::cmp_frag("fragment", "fragance"));
        assert_eq!(
            (None,None,None), 
            Node::<u8>::cmp_frag("some", "none"));
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
        tr.add("greco", 8);
        assert_eq!(Some(1), tr.find("romane").unwrap().value);
        assert_eq!(Some(2), tr.find("romanus").unwrap().value);
        assert_eq!(Some(3), tr.find("romulus").unwrap().value);
        assert_eq!(Some(4), tr.find("rubens").unwrap().value);
        assert_eq!(Some(5), tr.find("ruber").unwrap().value);
        assert_eq!(Some(6), tr.find("rubicon").unwrap().value);
        assert_eq!(Some(7), tr.find("rubicundus").unwrap().value);
        assert_eq!(Some(8), tr.find("greco").unwrap().value);
    }

    #[test]
    fn foo_test() {
        let mut tr = new_trie::<u8>();
        tr.add("/users", 1);
        dbg!(&tr);
        tr.add("/posts", 2);
        dbg!(&tr);
        tr.add("/comments", 3);
        dbg!(&tr);
        assert_eq!("/", tr.children[0].frag);
        assert_eq!("users", tr.children[0].children[0].frag);
        assert_eq!("posts", tr.children[0].children[1].frag);
        assert_eq!("comments", tr.children[0].children[2].frag);
        assert_eq!(Some(1), tr.find("/users").unwrap().value);
        assert_eq!(Some(2), tr.find("/posts").unwrap().value);
        assert_eq!(Some(3), tr.find("/comments").unwrap().value);
    }

    #[test]
    fn new_insertion_becomes_prefix_test() {
        let mut tr = new_trie::<u8>();
        tr.add("/users", 1);
        assert_eq!("/users", tr.children[0].frag);
        tr.add("/user/x", 2);
        assert_eq!("/user", tr.children[0].frag);
        assert_eq!("s", tr.children[0].children[0].frag);
        assert_eq!("/x", tr.children[0].children[1].frag);
        assert_eq!(Some(1), tr.find("/users").unwrap().value);
        assert_eq!(Some(2), tr.find("/user/x").unwrap().value);
    }

    #[test]
    fn add_as_child_test() {
        let mut tr = new_trie::<u8>();
        tr.add("/abc", 1);
        assert_eq!("/abc", tr.children[0].frag);
        tr.add("/abcdef", 2);
        assert_eq!("/abc", tr.children[0].frag);
        assert_eq!("def", tr.children[0].children[0].frag);
        assert_eq!(Some(1), tr.find("/abc").unwrap().value);
        assert_eq!(Some(2), tr.find("/abcdef").unwrap().value);
    }

    #[test]
    fn promote_shared_prefix_test() {
        let mut tr = new_trie::<u8>();
        tr.add("prefix/word", 1);
        tr.add("prefix/otherword", 2);
        assert_eq!(None, tr.find("prefix/").unwrap().value);
        assert_eq!(Some(1), tr.find("prefix/word").unwrap().value);
        assert_eq!(Some(2), tr.find("prefix/otherword").unwrap().value);
        assert_eq!("prefix/", tr.children[0].frag);
        assert_eq!("word", tr.children[0].children[0].frag);
        assert_eq!("otherword", tr.children[0].children[1].frag);
        assert!(tr.children[0].children[0].children.is_empty());
        assert!(tr.children[0].children[1].children.is_empty());
    }

    #[test]
    fn override_value_test() {
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
