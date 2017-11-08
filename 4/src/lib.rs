#[derive(Debug)]
pub struct Tree<T> {
    root: Option<Box<Node<T>>>,
}
#[derive(Debug)]
pub struct Node<T> {
    key:T,
    L:Option<Box<Tree<T>>>,
    R:Option<Box<Tree<T>>>,
}

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree{
            root:None,
        }
    }


    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {
        match self.root {
            None=> {
                self.root = Some(Box::new(Node{key:key,L:None,R:None,}));
                return true;
            },
            Some(ref mut k)=>{
                if k.key < key{
                    match k.R {
                        Some(ref mut k)=> {return k.insert(key);},
                        None=> {
                            k.R = Some(Box::new(Tree{root:Some(Box::new(Node{key:key,L:None,R:None,}))}));
                            return true;
                        },
                    }
                } else if k.key > key{
                    match k.L {
                        Some(ref mut k)=> {return k.insert(key);},
                        None=> {
                            k.L = Some(Box::new(Tree{root:Some(Box::new(Node{key:key,L:None,R:None,}))}));
                            return true;
                        },
                    }
                } else {
                    return false;
                }
            }
        }

    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.root {
            None=>false,
            Some(ref k)=> {
                if k.key < *key{
                    match k.R {
                        Some(ref k)=> {
                            return k.find(key);
                        }
                        None=> {return false;},
                    }
                } else if k.key > *key{
                    match k.L {
                        Some(ref k)=> {
                            return k.find(key);
                        }
                        None=> {return false;},
                    }
                } else {
                    return true;
                }
            }
        }

    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        let mut v: Vec<&T> = Vec::new();
        let mut lv: Vec<&T> = Vec::new();
        let mut rv: Vec<&T> = Vec::new();
        match self.root {
            None=>{return v;},
            Some(ref k)=>{
                v.push(&k.key);
                match k.L {
                    Some(ref x)=> {
                        lv = x.preorder();
                        v.append(&mut lv);
                    },
                    None => {},
                }
                match k.R {
                    Some(ref x)=> {
                        rv = x.preorder();
                        v.append(&mut rv);
                    },
                    None => {},
                }
                return v;
            }
        }

    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut v: Vec<&T> = Vec::new();
        let mut lv: Vec<&T> = Vec::new();
        let mut rv: Vec<&T> = Vec::new();
        match self.root {
            None=>{return v;},
            Some(ref k)=>{
                match k.L {
                    Some(ref x)=> {
                        lv = x.inorder();
                        v.append(&mut lv);
                    },
                    None => {},
                }
                v.push(&k.key);
                match k.R {
                    Some(ref x)=> {
                        rv = x.inorder();
                        v.append(&mut rv);
                    },
                    None => {},
                }
                return v;
            }
        }
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut v: Vec<&T> = Vec::new();
        let mut lv: Vec<&T> = Vec::new();
        let mut rv: Vec<&T> = Vec::new();
        match self.root {
            None=>{return v;},
            Some(ref k)=>{
                match k.L {
                    Some(ref x)=> {
                        lv = x.postorder();
                        v.append(&mut lv);
                    },
                    None => {},
                }
                match k.R {
                    Some(ref x)=> {
                        rv = x.postorder();
                        v.append(&mut rv);
                    },
                    None => {},
                }
                v.push(&k.key);
                return v;
            }
        }
    }
}

#[test]
fn it_works() {
    let mut t = Tree {root:None};//can I print the tree? like
    t.insert(1);
    t.insert(2);
    t.insert(3);
    t.insert(4);
    t.insert(5);
    t.insert(22);
    t.insert(18);
    t.insert(24);
    t.insert(50);
    t.insert(35);
    t.insert(31);
    t.insert(44);
    t.insert(70);
    t.insert(66);
    t.insert(90);
    println!("{:?}",t.preorder());
    println!("{:?}",t.inorder());
    println!("{:?}",t.postorder());

    assert_eq!(t.find(&66),true);
    println!("{:?}",t);
}
