use std::boxed::Box;
use std::ops::Index;
use sscanf::sscanf;
use std::collections::HashMap;

#[allow(non_camel_case_types)]
enum Command{
    ls,
    cd(String)
}

struct Dir<'a>{
    root: Option<&'a Self>,
    parent: Option<&'a Self>,
    contents: HashMap<String, Box<dyn FSItem>>
}

struct File{
    size: usize
}

impl <'a> Dir<'a> {
    fn new_root() -> Self{
        Self{
            root: None,
            parent: None,
            contents: HashMap::new()
        }
    }
    
    fn new(parent: &'a Self) -> Self{
        Self{
            root: parent.root, 
            parent: Some(parent),
            contents: HashMap::new()
        }
    }

    fn parent(&self) -> &Self{
        match self.parent{
            Some(parent_dir) => parent_dir,
            None => &self,
        }
    }

    fn root(&self) -> &Self{
        match self.root{
            Some(root_dir) => root_dir,
            None => &self,
        }
    }

    fn add_file(name: &str, size: usize){
        todo!();
    }
}

impl File{
    fn new(size: usize) -> Self{
        Self{
            size
        }
    }
}

trait FSItem{
    fn size(&self) -> usize;
}

impl FSItem for File{
    fn size(&self) -> usize{
        self.size
    }
}

impl <'a> FSItem for Dir<'a>{
    fn size(&self) -> usize{
        self.contents.iter().map(|(_key, val)| val.size()).sum()
    }
}

impl <'a, 'b, 'c, 'd, 'e> Index<&'b str> for Dir<'a>{
    type Output = Box<&'a dyn FSItem>;

    fn  index(&'c self, index: &'b str) -> &'d Self::Output {
        match index{
            ".." => &Box::new(self.parent()),
            "/"  => &Box::new(self.root()),
            x    => &Box::new(&(*self.contents[x]))
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
