use std::{error::Error, fs, path::Path};
mod codegen;
use bitvec::prelude as bv;
use codegen::dir::{
    mod_Entity::OneOfentity::{directory, file},
    *,
};
use quick_protobuf::{deserialize_from_slice, serialize_into_vec};
use rayon::prelude::*;
use std::io::prelude::*;

enum Mode {
    Compress,
    Extract,
}
#[derive(Debug)]
struct Connection {
    left: Node,
    right: Node,
}
impl core::ops::Index<bool> for Connection {
    type Output = Node;
    fn index(&self, index: bool) -> &Self::Output {
        match index {
            false => &self.left,
            true => &self.right,
        }
    }
}
#[derive(Debug)]
struct NodeStruct {
    cout: u64,
    core: Node,
}
#[derive(Debug)]
enum Node {
    Data(u8),
    Connection(Box<Connection>),
}
impl Node {
    fn recursive_find(&self, target: u8) -> Option<Vec<bool>> {
        match self {
            Node::Data(e) => {
                if *e == target {
                    Some(vec![])
                } else {
                    None
                }
            }
            Node::Connection(e) => {
                if let Some(mut r) = e.left.recursive_find(target) {
                    r.push(false);
                    Some(r)
                } else if let Some(mut r) = e.right.recursive_find(target) {
                    r.push(true);
                    Some(r)
                } else {
                    None
                }
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
    let args = std::env::args();
    let (mode, target, destination) = parse_args(args).unwrap();
    match mode {
        Mode::Compress => {
            let start = std::time::Instant::now();
            let read = read_dir(Path::new(&target)).unwrap();
            println!("read time {:?}", start.elapsed());
            let t = serialize_into_vec(&read).unwrap();
            println!("serialize time {:?}", start.elapsed());
            let frequency = t.iter().fold(vec![0u64; 256], |mut r, e| {
                r[*e as usize] += 1;
                r
            });
            println!("frequency time {:?}", start.elapsed());
            let huffman_encode = huffman_encode(&frequency, t);
            println!(
                "huffman time {:?}, enc size {:?}",
                start.elapsed(),
                huffman_encode.len()
            );
            let ent: Vec<_> = huffman_encode.into_vec();
            println!("ent time {:?}, size {:?}", start.elapsed(), ent.len());
            let out = serialize_into_vec(&Top {
                tree: frequency,
                ent,
            })
            .unwrap();
            println!("serialize time {:?}", start.elapsed());
            fs::write(Path::new(&destination), out).unwrap();
        }
        Mode::Extract => {
            let start = std::time::Instant::now();
            let res =
                deserialize_from_slice::<Top>(fs::read(Path::new(&target)).unwrap().as_slice())
                    .unwrap();
            println!("deserialize time {:?}", start.elapsed());
            let freq = res.tree;
            let data = bv::BitVec::from_vec(res.ent);
            println!("huffman time {:?}", start.elapsed());
            let res = huffman_decode(&freq, data);
            println!("decode time {:?}", start.elapsed());
            let t = deserialize_from_slice::<Entity>(res.as_slice()).unwrap();
            println!("deserialize time {:?}", start.elapsed());
            let path = Path::new(&destination);
            std::fs::create_dir(path).unwrap();
            write_dir(t, path).unwrap();
        }
    }
}
fn huffman_decode(freq: &Vec<u64>, data: bv::BitVec<u8>) -> Vec<u8> {
    let tree = construct_tree(&freq);

    let mut res = Vec::new();
    //traverse the tree
    let mut iter = data.iter();
    while let Some(mut bit) = iter.next() {
        let mut node = &tree;
        while let Node::Connection(e) = node {
            node = &e[*bit];
            bit = match iter.next() {
                Some(e) => e,
                None => break,
            };
        }
        if let Node::Data(e) = node {
            res.push(*e);
        }
    }

    res
}

fn huffman_encode(freq: &Vec<u64>, data: Vec<u8>) -> bv::BitVec<u8> {
    let tree = construct_tree(freq);
    let res = data
        .par_iter()
        .fold(
            || Vec::new(),
            |mut r, e| {
                r.extend(tree.recursive_find(*e).unwrap());
                r
            },
        )
        .flatten();
    let r = res.collect::<Vec<_>>();
    bv::BitVec::from_iter(r.into_iter())
}
fn construct_tree(data: &Vec<u64>) -> Node {
    let mut map = data
        .iter()
        .enumerate()
        .map(|(i, e)| NodeStruct {
            cout: *e,
            core: Node::Data(i as u8),
        })
        .collect::<Vec<_>>();
    map.sort_unstable_by(|a, b| b.cout.cmp(&a.cout));
    while map.len() > 1 {
        let left = map.pop().unwrap();
        let right = map.pop().unwrap();
        map.push(NodeStruct {
            cout: left.cout + right.cout,
            core: Node::Connection(Box::new(Connection {
                left: left.core,
                right: right.core,
            })),
        });
        map.par_sort_unstable_by(|a, b| b.cout.cmp(&a.cout));
    }
    map.into_iter().next().unwrap().core
}
fn write_dir(data: Entity, path: &Path) -> Result<(), Box<dyn Error>> {
    match data.entity {
        directory(e) => {
            let path = &path.join(Path::new(&e.name));
            println!("{:?}", path);
            std::fs::create_dir(path)?;
            for i in e.contents {
                write_dir(i, path)?;
            }
        }
        file(e) => {
            let path = path.join(Path::new(&e.name));
            println!("{:?}", path);
            let mut f = std::fs::File::create(path)?;
            f.write(e.contents.as_slice())?;
        }
        _ => {
            return Err("Entity parse error")?;
        }
    };
    Ok(())
}

fn read_dir(target: &Path) -> Result<Entity, Box<dyn Error>> {
    if target.is_dir() {
        Ok(Entity {
            entity: mod_Entity::OneOfentity::directory(Directory {
                name: target
                    .file_name()
                    .ok_or("file name misiing")?
                    .to_string_lossy()
                    .to_string(),
                contents: target
                    .read_dir()?
                    .filter_map(|e| {
                        if let Ok(e) = e {
                            if let Ok(e) = read_dir(&e.path()) {
                                Some(e)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect(),
            }),
        })
    } else {
        Ok(Entity {
            entity: file(File {
                contents: fs::read(target)?,
                name: target
                    .file_name()
                    .ok_or("no file name")?
                    .to_string_lossy()
                    .to_string(),
            }),
        })
    }
}

fn parse_args(mut args: std::env::Args) -> Result<(Mode, String, String), Box<dyn Error>> {
    let err_msg = "usage: tzip (-c|-x) target destination".to_string();
    args.next();
    let mode: Mode = match args.next() {
        Some(e) => match e.as_str() {
            "-x" => Mode::Extract,
            "-c" => Mode::Compress,
            _ => {
                return Err(err_msg + " \n wrong first option")?;
            }
        },
        _ => {
            return Err(err_msg + " \n no first option")?;
        }
    };
    let target = args
        .next()
        .ok_or(err_msg.clone() + " \n no second option")?;
    let dest = args.next().ok_or(err_msg + " \n no third option")?;
    Ok((mode, target, dest))
}

mod test{
    use super::*;
    #[test]
    fn test_huffman(){
        let data = vec![1,2,3,4,5,6,7,8,9,10];
        let freq = data.iter().fold(vec![0u64; 256], |mut r, e| {
            r[*e as usize] += 1;
            r
        });
        let enc = huffman_encode(&freq, data);
        let dec = huffman_decode(&freq, enc);
        assert_eq!(dec, vec![1,2,3,4,5,6,7,8,9,10]);
    }
}