//! The field as the map table knows it (design/command.md): the Positions (позиції) a unit can hold, the Road
//! heads (в'їзди) where columns come in, the Grey zone (сіра зона) where nothing of ours is placed, and the Routes
//! (маршрути) between them: a graph of the roads and tracks and a field grid, with what blocks each kind of mover,
//! and the shortest ways from each road head, worked out once.
//!
//! Nothing here touches the battle by itself: the field is built when something asks for it (the Allotment's
//! positions, the overlay, Command's columns), and it draws on no random stream.

use crate::geo::{self, Polyline, RoadKind};
use bevy::math::Vec2;
use std::{
    cmp::Ordering,
    collections::BinaryHeap,
    sync::OnceLock,
};

/// What kind of place a Position is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PositionKind {
    /// A prepared position: a dug trench line with its firing points (the diorama's own posts).
    Trench,
    /// The edge of a tree line (посадка), facing out.
    Treeline,
    /// The edge of the built-up area: ruined houses (руїна).
    Ruin,
    /// A dug-in hide for a tank (капонір).
    Hide,
}

impl PositionKind {
    /// (English, Ukrainian).
    pub fn name(self) -> (&'static str, &'static str) {
        match self {
            PositionKind::Trench => ("trench", "окоп"),
            PositionKind::Treeline => ("tree line", "посадка"),
            PositionKind::Ruin => ("ruin", "руїна"),
            PositionKind::Hide => ("hide", "капонір"),
        }
    }

    /// Whether men (riflemen, an ATGM team) can hold it.
    pub fn holds_men(self) -> bool {
        self != PositionKind::Hide
    }

    /// Whether a vehicle can hold it: a tank in its hide, or hull-down in the edge of a tree line.
    pub fn holds_vehicles(self) -> bool {
        matches!(self, PositionKind::Hide | PositionKind::Treeline)
    }
}

/// Whose side of the grey zone a place is on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ground {
    Ours,
    Grey,
    Theirs,
}

#[derive(Clone, Debug)]
pub struct Position {
    pub kind: PositionKind,
    pub at: Vec2,
    /// The way it faces: out of the tree line, towards the enemy.
    pub facing: Vec2,
    pub ground: Ground,
}

/// Where columns come in: a name (English, Ukrainian) and the place on the box's edge.
#[derive(Clone, Debug)]
pub struct RoadHead {
    pub name: (String, String),
    pub at: Vec2,
}

/// What moves along a route, and so what a stretch of it costs.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Mover {
    Tracked,
    Wheeled,
    Foot,
}

impl Mover {
    pub const ALL: [Mover; 3] = [Mover::Tracked, Mover::Wheeled, Mover::Foot];

    /// How fast it goes on this kind of ground, against a road: a stretch's cost is its length over this.
    fn pace(self, class: Class) -> f32 {
        match (self, class) {
            (Mover::Foot, _) => 1.0,
            (_, Class::Road) => 1.0,
            (Mover::Tracked, Class::Track) => 0.9,
            (Mover::Wheeled, Class::Track) => 0.75,
            (Mover::Tracked, Class::Field) => 0.6,
            (Mover::Wheeled, Class::Field) => 0.35,
        }
    }
}

/// The going on a stretch.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Class {
    Road,
    Track,
    Field,
}

/// A stretch between two nodes of the route graph: its going, its length, and whether only men on foot get through
/// (it crosses a tree line outside its gaps, or the river away from a bridge).
#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub to: usize,
    pub class: Class,
    pub len: f32,
    pub foot_only: bool,
}

pub struct Graph {
    pub nodes: Vec<Vec2>,
    pub edges: Vec<Vec<Edge>>,
    /// The field grid's spacing and size, to find the node nearest a point.
    grid: (f32, usize, usize),
    grid_start: usize,
}

/// The shortest ways from one road head for one kind of mover: each node's cost and the node it is reached from.
pub struct Tree {
    pub cost: Vec<f32>,
    pub prev: Vec<usize>,
    pub root: usize,
}

pub struct Field {
    pub positions: Vec<Position>,
    pub road_heads: Vec<RoadHead>,
    /// The grey zone: from this z to that one (the enemy is to +z on both days).
    pub grey: (f32, f32),
    pub graph: Graph,
    /// For each road head, in order, a tree per mover (`Mover::ALL`'s order).
    pub trees: Vec<Vec<Tree>>,
}

static FIELD: OnceLock<Field> = OnceLock::new();

/// The diorama's field, built the first time it is asked for.
pub fn field() -> &'static Field {
    FIELD.get_or_init(build)
}

impl Field {
    pub fn ground_at(&self, p: Vec2) -> Ground {
        ground_at(self.grey, p)
    }

    /// The way from a road head to a place for a mover: the road head, the graph's nodes, the place; `None` where the
    /// graph does not reach.
    pub fn path(&self, head: usize, mover: Mover, to: Vec2) -> Option<Polyline> {
        let tree = &self.trees.get(head)?[Mover::ALL.iter().position(|m| *m == mover)?];
        let end = self.graph.nearest(to, mover, tree)?;
        let mut pts = vec![to];
        let mut n = end;
        while n != tree.root {
            pts.push(self.graph.nodes[n]);
            n = tree.prev[n];
            if n == usize::MAX {
                return None;
            }
        }
        pts.push(self.graph.nodes[tree.root]);
        pts.push(self.road_heads[head].at);
        pts.reverse();
        pts.dedup_by(|a, b| a.distance(*b) < 1.0);
        Some(Polyline::smoothed(pts, 2))
    }
}

fn ground_at(grey: (f32, f32), p: Vec2) -> Ground {
    if p.y < grey.0 {
        Ground::Ours
    } else if p.y > grey.1 {
        Ground::Theirs
    } else {
        Ground::Grey
    }
}

fn build() -> Field {
    let d = crate::data::data();
    let lay = geo::layout();
    let grey = d.grey_zone;
    let mut positions = Vec::new();
    let push = |kind, at: Vec2, facing: Vec2, positions: &mut Vec<Position>| positions.push(Position { kind, at, facing, ground: ground_at(grey, at) });
    // The diorama's own posts first, in their order (the Allotment names them by index), then the hides, then what
    // the ground offers.
    for &p in &lay.ua_positions {
        push(PositionKind::Trench, p, Vec2::Y, &mut positions);
    }
    for &((x, z), (fx, fz)) in &d.hides {
        push(PositionKind::Hide, Vec2::new(x, z), Vec2::new(fx, fz).normalize_or(Vec2::Y), &mut positions);
    }
    // Tree lines: every 80 m along each belt, outside its gaps and clear of the posts, on the edge that faces the
    // enemy (+z); a belt running north-south gives both its edges, facing east and west, for the flanks.
    for belt in &lay.belts {
        let len = belt.line.length();
        let mut s = 40.0;
        while s < len - 40.0 {
            let in_gap = belt.gaps.iter().any(|&(g0, g1)| s > g0 - 15.0 && s < g1 + 15.0);
            let p = belt.line.point(s);
            let n = belt.line.normal(s);
            if !in_gap && geo::in_bounds(p) {
                let sides: &[f32] = if n.y.abs() > 0.5 { &[1.0] } else { &[1.0, -1.0] };
                for &k in sides {
                    let out = if n.y.abs() > 0.5 { n * n.y.signum() } else { n * k };
                    let at = p + out * (belt.width * 0.5 + 3.0);
                    if positions.iter().all(|q| q.at.distance(at) > 45.0) {
                        push(PositionKind::Treeline, at, out, &mut positions);
                    }
                }
            }
            s += 80.0;
        }
    }
    // The built-up edges facing the enemy: the town's and the dachas' southern edges, every 220 m.
    for r in [lay.town, lay.dachas] {
        if r.width() < 1.0 {
            continue;
        }
        let mut x = r.min.x + 110.0;
        while x < r.max.x - 60.0 {
            let at = Vec2::new(x, r.max.y + 6.0);
            if geo::in_bounds(at) && positions.iter().all(|q| q.at.distance(at) > 45.0) {
                push(PositionKind::Ruin, at, Vec2::Y, &mut positions);
            }
            x += 220.0;
        }
    }
    let road_heads: Vec<RoadHead> = d.road_heads.iter().map(|h| RoadHead { name: h.name.clone(), at: Vec2::new(h.at.0, h.at.1) }).collect();
    let graph = Graph::build(lay);
    let trees = road_heads.iter().map(|h| Mover::ALL.iter().map(|&m| graph.tree(h.at, m)).collect()).collect();
    Field { positions, road_heads, grey, graph, trees }
}

const GRID: f32 = 100.0;

impl Graph {
    fn build(lay: &geo::Layout) -> Graph {
        let (nx, nz) = ((2.0 * geo::HALF_X / GRID) as usize, (2.0 * geo::HALF_Z / GRID) as usize);
        let mut nodes = Vec::new();
        let mut edges: Vec<Vec<Edge>> = Vec::new();
        let grid_at = |i: usize, j: usize| Vec2::new(-geo::HALF_X + GRID * (i as f32 + 0.5), -geo::HALF_Z + GRID * (j as f32 + 0.5));
        // The roads and tracks, resampled every 40 m.
        let mut road_nodes: Vec<(usize, usize)> = Vec::new(); // (road, node)
        for (ri, road) in lay.roads.iter().enumerate() {
            let class = if road.kind == RoadKind::Track { Class::Track } else { Class::Road };
            let len = road.line.length();
            let n = (len / 40.0).ceil().max(1.0) as usize;
            let mut last: Option<usize> = None;
            for k in 0..=n {
                let p = road.line.point(len * k as f32 / n as f32);
                if !geo::in_bounds(p) {
                    last = None;
                    continue;
                }
                let id = nodes.len();
                nodes.push(p);
                edges.push(Vec::new());
                road_nodes.push((ri, id));
                if let Some(prev) = last {
                    let len = nodes[prev].distance(p);
                    edges[prev].push(Edge { to: id, class, len, foot_only: false });
                    edges[id].push(Edge { to: prev, class, len, foot_only: false });
                }
                last = Some(id);
            }
        }
        // Where two roads meet.
        for a in 0..road_nodes.len() {
            for b in a + 1..road_nodes.len() {
                let ((ra, na), (rb, nb)) = (road_nodes[a], road_nodes[b]);
                let len = nodes[na].distance(nodes[nb]);
                if ra != rb && len < 30.0 {
                    edges[na].push(Edge { to: nb, class: Class::Road, len, foot_only: false });
                    edges[nb].push(Edge { to: na, class: Class::Road, len, foot_only: false });
                }
            }
        }
        // The field grid, eight ways from each node.
        let grid_start = nodes.len();
        for j in 0..nz {
            for i in 0..nx {
                nodes.push(grid_at(i, j));
                edges.push(Vec::new());
            }
        }
        let id = |i: usize, j: usize| grid_start + j * nx + i;
        for j in 0..nz {
            for i in 0..nx {
                for (di, dj) in [(1i32, 0i32), (0, 1), (1, 1), (1, -1)] {
                    let (ii, jj) = (i as i32 + di, j as i32 + dj);
                    if ii < 0 || jj < 0 || ii >= nx as i32 || jj >= nz as i32 {
                        continue;
                    }
                    let (a, b) = (id(i, j), id(ii as usize, jj as usize));
                    let foot_only = blocked(lay, nodes[a], nodes[b]);
                    let len = nodes[a].distance(nodes[b]);
                    edges[a].push(Edge { to: b, class: Class::Field, len, foot_only });
                    edges[b].push(Edge { to: a, class: Class::Field, len, foot_only });
                }
            }
        }
        // Each road node off onto the field, to the grid nodes round it.
        for &(_, n) in &road_nodes {
            let p = nodes[n];
            let (i, j) = (((p.x + geo::HALF_X) / GRID - 0.5).floor() as i32, ((p.y + geo::HALF_Z) / GRID - 0.5).floor() as i32);
            for (di, dj) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let (ii, jj) = (i + di, j + dj);
                if ii < 0 || jj < 0 || ii >= nx as i32 || jj >= nz as i32 {
                    continue;
                }
                let g = id(ii as usize, jj as usize);
                let foot_only = blocked(lay, p, nodes[g]);
                let len = p.distance(nodes[g]);
                edges[n].push(Edge { to: g, class: Class::Field, len, foot_only });
                edges[g].push(Edge { to: n, class: Class::Field, len, foot_only });
            }
        }
        Graph { nodes, edges, grid: (GRID, nx, nz), grid_start }
    }

    /// The cheapest ways from the node nearest a place, for a mover (Dijkstra).
    fn tree(&self, from: Vec2, mover: Mover) -> Tree {
        let root = (0..self.nodes.len()).min_by(|&a, &b| self.nodes[a].distance(from).total_cmp(&self.nodes[b].distance(from))).unwrap_or(0);
        let mut cost = vec![f32::INFINITY; self.nodes.len()];
        let mut prev = vec![usize::MAX; self.nodes.len()];
        let mut heap = BinaryHeap::new();
        cost[root] = 0.0;
        heap.push(Visit(0.0, root));
        while let Some(Visit(c, n)) = heap.pop() {
            if c > cost[n] {
                continue;
            }
            for e in &self.edges[n] {
                if e.foot_only && mover != Mover::Foot {
                    continue;
                }
                // Men on foot get through a tree line or across the river, slowly.
                let slow = if e.foot_only { 2.0 } else { 1.0 };
                let next = c + e.len * slow / mover.pace(e.class);
                if next < cost[e.to] {
                    cost[e.to] = next;
                    prev[e.to] = n;
                    heap.push(Visit(next, e.to));
                }
            }
        }
        Tree { cost, prev, root }
    }

    /// The node a mover would make for to reach a place: of the grid nodes round it (and any road node within
    /// 60 m), the reachable one that brings it there cheapest.
    fn nearest(&self, to: Vec2, mover: Mover, tree: &Tree) -> Option<usize> {
        let (g, nx, nz) = self.grid;
        let (i, j) = (((to.x + geo::HALF_X) / g - 0.5).floor() as i32, ((to.y + geo::HALF_Z) / g - 0.5).floor() as i32);
        let lay = geo::layout();
        let mut best: Option<(f32, usize)> = None;
        let consider = |n: usize, best: &mut Option<(f32, usize)>| {
            if !tree.cost[n].is_finite() || (mover != Mover::Foot && blocked(lay, self.nodes[n], to)) {
                return;
            }
            let c = tree.cost[n] + self.nodes[n].distance(to) / mover.pace(Class::Field);
            if best.is_none_or(|(b, _)| c < b) {
                *best = Some((c, n));
            }
        };
        for (di, dj) in [(0, 0), (1, 0), (0, 1), (1, 1), (-1, 0), (0, -1), (-1, -1), (1, -1), (-1, 1)] {
            let (ii, jj) = (i + di, j + dj);
            if ii >= 0 && jj >= 0 && (ii as usize) < nx && (jj as usize) < nz {
                consider(self.grid_start + jj as usize * nx + ii as usize, &mut best);
            }
        }
        for n in 0..self.grid_start {
            if self.nodes[n].distance(to) < 60.0 {
                consider(n, &mut best);
            }
        }
        best.map(|(_, n)| n)
    }
}

/// Whether a vehicle cannot cross from `a` to `b`: a tree line in the way outside its gaps, or the river away from a
/// road (the roads cross it on bridges and fords).
fn blocked(lay: &geo::Layout, a: Vec2, b: Vec2) -> bool {
    let through_belt = lay.belts.iter().any(|belt| {
        belt.line.pts.windows(2).enumerate().any(|(i, w)| {
            geo::crossing(a, b, w[0], w[1]).is_some_and(|t| {
                let at = a + (b - a) * t;
                let s = belt.line.cum[i] + w[0].distance(at);
                !belt.gaps.iter().any(|&(g0, g1)| s > g0 && s < g1)
            })
        })
    });
    let across_river = lay.river.as_ref().is_some_and(|r| r.pts.windows(2).any(|w| geo::crossing(a, b, w[0], w[1]).is_some()));
    through_belt || across_river
}

#[derive(PartialEq)]
struct Visit(f32, usize);

impl Eq for Visit {}

impl Ord for Visit {
    fn cmp(&self, o: &Self) -> Ordering {
        // The cheapest first.
        o.0.total_cmp(&self.0).then(self.1.cmp(&o.1))
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}
