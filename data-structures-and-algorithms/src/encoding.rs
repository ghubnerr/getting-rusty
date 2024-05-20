use std::collections::BTreeMap;

#[derive(Debug)]
pub enum HuffNode {
    Tree(Box<HuffNode>, Box<HuffNode>),
    Leaf(char),
}

pub struct HScore {
    h: HuffNode,
    s: i32,
}

impl HuffNode {
    pub fn print_lfirst(&self, depth: i32, dir: char) {
        match self {
            HuffNode::Tree(l, r) => {
                l.print_lfirst(depth + 1, '/');
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                println!("{}{}*", spc, dir);
                r.print_lfirst(depth + 1, '\\');
            }
            HuffNode::Leaf(c) => {
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                println!("{}{}{}", spc, dir, c);
            }
        }
    }
    pub fn encode_char(&self, c: char) -> Option<Vec<char>> {
        // could return vec of bool but chars print nicer
        match self {
            HuffNode::Tree(l, r) => {
                if let Some(mut v) = l.encode_char(c) {
                    v.insert(0, '0');
                    return Some(v);
                }
                if let Some(mut v) = r.encode_char(c) {
                    v.insert(0, '1');
                    return Some(v);
                }
                None
            }
            HuffNode::Leaf(nc) => {
                if c == *nc {
                    Some(Vec::new())
                } else {
                    None
                }
            }
        }
    }

    pub fn encode_str(&self, s: &str) -> Option<Vec<char>> {
        let mut res = Vec::new();
        for c in s.chars() {
            let v = self.encode_char(c)?;
            res.extend(v.into_iter());
        }
        Some(res)
    }
}

pub fn build_tree(s: &str) -> HuffNode {
    let mut map = BTreeMap::new();
    for c in s.chars() {
        // if map has already add 1 else put 1
        let n = *map.get(&c).unwrap_or(&0);
        map.insert(c, n + 1);
    }

    let mut tlist: Vec<HScore> = map
        .into_iter()
        .map(|(k, s)| HScore {
            h: HuffNode::Leaf(k),
            s,
        })
        .collect();

    while tlist.len() > 1 {
        let last = tlist.len() - 1;
        for i in 0..last - 1 {
            if tlist[i].s < tlist[last - 1].s {
                tlist.swap(i, last - 1);
            }
            if tlist[last - 1].s < tlist[last].s {
                tlist.swap(last - 1, last);
            }
        }
        let a_node = tlist.pop().unwrap();
        let b_node = tlist.pop().unwrap();
        let nnode = HuffNode::Tree(Box::new(a_node.h), Box::new(b_node.h));
        tlist.push(HScore {
            h: nnode,
            s: a_node.s + b_node.s,
        });
    }
    tlist.pop().unwrap().h
}
