pub struct Tree<T> {
    root: Option<Box<Node<T>>>,
}
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
            None=> {return true;},
            Some(ref mut k)={
                if k.key < key{
                    match k.R {
                        Some(ref mut k)=> {return k.insert(key);},
                        None=> true,
                    }
                }
                if k.key > key{
                    match k.L {
                        Some(ref mut k)=> k.insert(key),
                        None=> true,
                    }
                }
                if k.key == key{
                    return false;
                }
            }
        }

    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.root {
            None=>false,
            Some(ref mut k)=> {
                if k.key < key{
                    match k.R {
                        Some(ref mut k)=> k.find(key),
                        None=> false,
                    }
                }
                if k.key > key{
                    match k.L {
                        Some(ref mut k)=> k.find(key),
                        None=> false,
                    }
                }
                if k.key == key{
                    return true;
                }
            }
        }

    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        let mut v: Vec<&T> = Vec::new();
        let mut s: Vec<&T> = Vec::new();
        match self.root {
            None=>return v;
            Some(ref mut k)=>{
                match k.L {
                    Some(ref mut x)=> {
                        v.push(k.key);
                        s = k.L.preorder();
                        v.append(s);
                        s = k.R.preorder();
                        v.append(s);
                        return v;
                    }
                    None => return v;
                }
                match k.R {
                    Some(ref mut x)=> {
                        v.push(k.key);
                        s = k.L.preorder();
                        v.append(s);
                        s = k.R.preorder();
                        v.append(s);
                        return v;
                    }
                    None => return v;
                }
            }
        }

    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut v: Vec<&T> = Vec::new();
        let mut lv: Vec<&T> = Vec::new();
        let mut rv: Vec<&T> = Vec::new();
        match self.root {
            None=>return v;
            Some(ref mut k)=>{
                match k.L {
                    Some(ref mut x)=> {
                        lv = k.L.preorder();
                        v.append(lv);
                        v.push(k.key);
                        rv = k.R.preorder();
                        v.append(rv);
                        return v;
                    }
                    None => return v;
                }
                match k.R {
                    Some(ref mut x)=> {
                        lv = k.L.preorder();
                        v.append(lv);
                        v.push(k.key);
                        rv = k.R.preorder();
                        v.append(rv);
                        return v;
                    }
                    None => return v;
                }
            }
        }
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut v: Vec<&T> = Vec::new();
        let mut lv: Vec<&T> = Vec::new();
        let mut rv: Vec<&T> = Vec::new();
        match self.root {
            None=>return v;
            Some(ref mut k)=>{
                match k.L {
                    Some(ref mut x)=> {
                        lv = k.L.preorder();
                        v.append(lv);
                        rv = k.R.preorder();
                        v.append(rv);
                        v.push(k.key);
                        return v;
                    }
                    None => return v;
                }
                match k.R {
                    Some(ref mut x)=> {
                        lv = k.L.preorder();
                        v.append(lv);
                        rv = k.R.preorder();
                        v.append(rv);
                        v.push(k.key);
                        return v;
                    }
                    None => return v;
                }
            }
        }
    }
