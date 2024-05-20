use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::rc::{Rc, Weak};

type Rcc<T> = Rc<RefCell<T>>;

pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

//edge list
pub struct EdgeListGraph<E, ID> {
    // data
    v: Vec<(E, ID, ID)>,
}

// pointer based
// good for directed graphs as edges go one way
// using weak pointers means the edge will fail safely if node has been removed
// can stick edge data if needed
pub struct RccGraph<T, E> {
    nodes: Vec<Rcc<RccNode<T, E>>>,
}

pub struct RccNode<T, E> {
    data: T,
    edges: Vec<(E, Weak<RefCell<RccNode<T, E>>>)>,
}

// map based
// maps point from key to value normally quickly
pub struct MapGraph<T, E, ID: Hash> {
    mp: HashMap<ID, T>,
    edges: Vec<(E, ID, ID)>,
}

// mappointer based -> using this one
pub struct MapPGraph<T, E, ID: Hash + Eq> {
    data: HashMap<ID, (T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}

// ------------------------------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct GraphErr {
    mess: String,
}

impl GraphErr {
    pub fn new(s: &str) -> Self {
        GraphErr {
            mess: s.to_string(),
        }
    }
}

pub trait Weighted {
    fn weight(&self) -> i32;
}
impl Weighted for i32 {
    fn weight(&self) -> i32 {
        *self
    }
}

#[derive(Debug)]
pub struct Route<ID> {
    pos: ID,
    path: Option<Rc<Route<ID>>>,
    len: i32,
}

impl<ID: Eq> Route<ID> {
    pub fn start_rc(pos: ID) -> Rc<Self> {
        Rc::new(Route {
            pos,
            path: None,
            len: 0,
        })
    }
    pub fn contains(&self, id: &ID) -> bool {
        if self.pos == *id {
            return true;
        }
        match self.path {
            Some(ref p) => p.contains(id),
            None => false,
        }
    }
}

impl<ID: std::fmt::Debug> Display for Route<ID> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(ref p) = self.path {
            write!(f, "{}--{}-", p, self.len)?;
        }
        write!(f, "{:?}", self.pos)
    }
}

#[derive(Debug)]
pub struct Graph<T, E, ID: Hash + Eq> {
    data: HashMap<ID, (T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>,
}

impl<T, E, ID: Clone + Hash + Eq> Graph<T, E, ID> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: ID, data: T) {
        self.data.insert(id, (data, Vec::new()));
    }

    pub fn add_edge(
        &mut self,
        edge_id: ID,
        from: ID,
        to: ID,
        edge_data: E,
    ) -> Result<(), GraphErr> {
        if !self.data.contains_key(&from) {
            return Err(GraphErr::new("'from' not in nodes"));
        }
        if let Some(ref mut data) = self.data.get_mut(&to) {
            self.edges
                .insert(edge_id.clone(), (edge_data, from.clone(), to));
            data.1.push(edge_id.clone());
        } else {
            return Err(GraphErr::new("'to' not in nodes"));
        }
        self.data.get_mut(&from).unwrap().1.push(edge_id);
        Ok(())
    }
}

impl<T, E: Weighted, ID: Clone + Hash + Eq> Graph<T, E, ID> {
    pub fn shortest_path(&self, from: ID, to: ID) -> Option<Rc<Route<ID>>> {
        self.shortest_path_r(Route::start_rc(from), to)
    }

    pub fn shortest_path_r(&self, from: Rc<Route<ID>>, to: ID) -> Option<Rc<Route<ID>>> {
        let mut toset = std::collections::HashSet::new();
        toset.insert(to);
        self.closest(from, &toset)
    }

    //dijkstra
    pub fn closest(
        &self,
        from: Rc<Route<ID>>,
        to: &std::collections::HashSet<ID>,
    ) -> Option<Rc<Route<ID>>> {
        let mut visited = std::collections::HashSet::new();
        let mut routes = Vec::new();
        routes.push(from);
        // continually pops the last route from routes and checks if it has reached the destination
        loop {
            let curr_route = routes.pop()?;
            if to.contains(&curr_route.pos) {
                return Some(curr_route);
            }
            if visited.contains(&curr_route.pos) {
                continue;
            }
            visited.insert(curr_route.pos.clone());

            let exits = self.data.get(&curr_route.pos)?;
            for edge_id in &exits.1 {
                // list of exits
                let edge = self.edges.get(edge_id)?;
                let npos = if edge.1 == curr_route.pos {
                    // opposite side of edge to current pos
                    edge.2.clone()
                } else {
                    edge.1.clone()
                };
                let nlen = curr_route.len + edge.0.weight();
                let nroute = Rc::new(Route {
                    pos: npos,
                    len: nlen,
                    path: Some(curr_route.clone()),
                });
                if routes.len() == 0 {
                    routes.push(nroute);
                    continue;
                }
                // insert into the list sorted
                let mut i_after = routes.len() - 1;
                loop {
                    if routes[i_after].len > nlen {
                        routes.insert(i_after + 1, nroute);
                        break;
                    }
                    if i_after == 0 {
                        routes.insert(0, nroute);
                        break;
                    }
                    i_after -= 1;
                }
            }
        }
    }
    // greedy salesman: np complete (haven't proven N^2 or N^3) i.e. sudoku

    pub fn greedy_salesman(&self, start: ID) -> Option<Rc<Route<ID>>> {
        let mut to_visit: std::collections::HashSet<ID> =
            self.data.keys().map(|k| k.clone()).collect();
        to_visit.remove(&start);
        let mut route = Route::start_rc(start.clone());
        while to_visit.len() > 0 {
            route = self.closest(route, &to_visit)?;
            to_visit.remove(&route.pos);
        }
        self.shortest_path_r(route, start)
    }
}
