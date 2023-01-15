use std::ops::Index;
use sscanf::sscanf;
use std::collections::HashMap;

enum FSItem<'a>{
    Dir{
        root: Option<&'a FSItem<'a>>,
        parent: Option<&'a FSItem<'a>>,
        contents: HashMap<String, FSItem<'a>>
    },
    File(usize)
}

impl <'a> FSItem<'a>{
    fn new_root() -> Self{
        Self::Dir{
            root: None,
            parent: None,
            contents: HashMap::new()
        }
    }

    fn new_file(size: usize) -> Self{
        Self::File(size)
    }

    fn new_dir(parent: &'a Self) -> Self{
        let root = if let Self::Dir{root, ..} = parent { root } else { panic!("Parent should be dir!") };
        Self::Dir{
            root: *root, 
            parent: Some(parent),
            contents: HashMap::new()
        }
    }

    fn size(&self) -> usize{
        match self{
            Self::File(size) => *size,
            Self::Dir{contents, .. } => contents.iter().map(|(_key, val)| val.size()).sum()
        }
    }

    fn parent(&self) -> &Self{
        match self{
            Self::Dir{parent, .. } => match parent{
                Some(parent_dir) => parent_dir,
                None => &self,
            },
            Self::File(_) => panic!("Attempted to get parent of file!")
        }
    }
    
    fn root(&self) -> &Self{
        match self{
            Self::Dir{root, .. } => match root{
                Some(root_dir) => root_dir,
                None => &self,
            },
            Self::File(_) => panic!("Attempted to get root of file!")
        }
    }
}

impl<'a> Index<&str> for FSItem<'a>{
    type Output = FSItem<'a>;

    fn index(&self, index: &str) -> &Self::Output {
        match self{
            Self::Dir{contents, ..} => match index{
                ".." => &self.parent(),
                "/"  => &self.root(),
                x => &contents[x]
            },
            Self::File(_) => panic!("Attempted to index into file!")
        }
    }
}

/// Sums the size of all directories with size of at most 100,000
/// based on example terminal output
/// # Examples
/// ```
/// use advent_of_code_2022_07::sum_dirs;
/// use std::fs;
///
/// assert_eq!(sum_dirs(&fs::read_to_string("example_input.txt").unwrap()), 9543);
/// ```
pub fn sum_dirs(input: &String) -> usize{
    todo!()
}
