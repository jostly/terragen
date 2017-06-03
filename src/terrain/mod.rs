use na::Point3;
use rand::random;

use math::Vec3;

use std::f32;
use std::collections::HashMap;

pub type Vertex = Vec3;

pub struct Edge<V = u32> {
    pub a: V,
    pub b: V,
}

impl<V> Edge<V> {
    pub fn new(a: V, b: V) -> Edge<V> {
        Edge { a: a, b: b }
    }
}

fn make_edge(a: u32, b: u32) -> Edge {
    if a > b {
        Edge::new(b, a)
    } else {
        Edge::new(a, b)
    }
}

pub struct Node {
    pub point: Vertex,
    pub elevation: f32,
}

impl Node {
    pub fn new(point: Vertex, elevation: f32) -> Node {
        Node {
            point: point,
            elevation: elevation,
        }
    }
}

pub struct Face {
    pub points: Point3<u32>,
    pub edges: Point3<u32>,
}

impl Face {
    pub fn new(points: Point3<u32>, edges: Point3<u32>) -> Face {
        Face {
            points: points,
            edges: edges,
        }
    }
}

pub struct Terrain {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    rnd_pow: f32,
    level: u8,
}

impl Terrain {
    pub fn new() -> Terrain {
        let phi = ((5.0f32).sqrt() + 1.0) / 2.0;
        let du = 1.0 / (phi * phi + 1.0).sqrt();
        let dv = phi * du;
        let z = 0f32;

        fn random_elevation() -> f32 {
            random::<f32>() * 0.5
        }

        let nodes = vec![Node::new(Vertex::new(z, dv, du), random_elevation()),
                         Node::new(Vertex::new(z, dv, -du), random_elevation()),
                         Node::new(Vertex::new(z, -dv, du), random_elevation()),
                         Node::new(Vertex::new(z, -dv, -du), random_elevation()),
                         Node::new(Vertex::new(du, z, dv), random_elevation()),
                         Node::new(Vertex::new(-du, z, dv), random_elevation()),
                         Node::new(Vertex::new(du, z, -dv), random_elevation()),
                         Node::new(Vertex::new(-du, z, -dv), random_elevation()),
                         Node::new(Vertex::new(dv, du, z), random_elevation()),
                         Node::new(Vertex::new(dv, -du, z), random_elevation()),
                         Node::new(Vertex::new(-dv, du, z), random_elevation()),
                         Node::new(Vertex::new(-dv, -du, z), random_elevation())];

        let edges = vec![Edge::new(0, 1),
                         Edge::new(0, 4),
                         Edge::new(0, 5),
                         Edge::new(0, 8),
                         Edge::new(0, 10),
                         Edge::new(1, 6),
                         Edge::new(1, 7),
                         Edge::new(1, 8),
                         Edge::new(1, 10),
                         Edge::new(2, 3),
                         Edge::new(2, 4),
                         Edge::new(2, 5),
                         Edge::new(2, 9),
                         Edge::new(2, 11),
                         Edge::new(3, 6),
                         Edge::new(3, 7),
                         Edge::new(3, 9),
                         Edge::new(3, 11),
                         Edge::new(4, 5),
                         Edge::new(4, 8),
                         Edge::new(4, 9),
                         Edge::new(5, 10),
                         Edge::new(5, 11),
                         Edge::new(6, 7),
                         Edge::new(6, 8),
                         Edge::new(6, 9),
                         Edge::new(7, 10),
                         Edge::new(7, 11),
                         Edge::new(8, 9),
                         Edge::new(10, 11)];

        let faces = vec![Face::new(Point3::new(0, 8, 1), Point3::new(3, 7, 0)),
                         Face::new(Point3::new(0, 5, 4), Point3::new(2, 18, 1)),
                         Face::new(Point3::new(0, 10, 5), Point3::new(4, 21, 2)),
                         Face::new(Point3::new(0, 4, 8), Point3::new(1, 19, 3)),
                         Face::new(Point3::new(0, 1, 10), Point3::new(0, 8, 4)),
                         Face::new(Point3::new(1, 8, 6), Point3::new(7, 24, 5)),
                         Face::new(Point3::new(1, 6, 7), Point3::new(5, 23, 6)),
                         Face::new(Point3::new(1, 7, 10), Point3::new(6, 26, 8)),
                         Face::new(Point3::new(2, 11, 3), Point3::new(13, 17, 9)),
                         Face::new(Point3::new(2, 9, 4), Point3::new(12, 20, 10)),
                         Face::new(Point3::new(2, 4, 5), Point3::new(10, 18, 11)),
                         Face::new(Point3::new(2, 3, 9), Point3::new(9, 16, 12)),
                         Face::new(Point3::new(2, 5, 11), Point3::new(11, 22, 13)),
                         Face::new(Point3::new(3, 7, 6), Point3::new(15, 23, 14)),
                         Face::new(Point3::new(3, 11, 7), Point3::new(17, 27, 15)),
                         Face::new(Point3::new(3, 6, 9), Point3::new(14, 25, 16)),
                         Face::new(Point3::new(4, 9, 8), Point3::new(20, 28, 19)),
                         Face::new(Point3::new(5, 10, 11), Point3::new(21, 29, 22)),
                         Face::new(Point3::new(6, 8, 9), Point3::new(24, 28, 25)),
                         Face::new(Point3::new(7, 11, 10), Point3::new(27, 29, 26))];

        Terrain {
            nodes: nodes,
            edges: edges,
            faces: faces,
            rnd_pow: 3.0,
            level: 0,
        }

    }

    pub fn current_level(&self) -> u8 {
        self.level
    }

    pub fn face_midpoint(&self, face: &Face) -> Vec3 {
        let pindex = &face.points;
        let p0 = &self.nodes[pindex[0] as usize].point;
        let p1 = &self.nodes[pindex[1] as usize].point;
        let p2 = &self.nodes[pindex[2] as usize].point;
        let x = p0.x + p1.x + p2.x;
        let y = p0.y + p1.y + p2.y;
        let z = p0.z + p1.z + p2.z;
        Vec3::new(x / 3.0, y / 3.0, z / 3.0)
    }

    pub fn calculate_elevations(&self) -> (f32, f32) {
        let mut min_elev = f32::MAX;
        let mut max_elev = f32::MIN;

        for n in self.nodes.iter() {
            min_elev = min_elev.min(n.elevation);
            max_elev = max_elev.max(n.elevation);
        }

        (min_elev, max_elev)
    }

    pub fn subdivide(&mut self) {
        self.level += 1;
        debug!("Initiating subdivision to level {}", self.level);
        let num_edges = self.edges.len();
        let num_faces = self.faces.len();
        let first_new_vertex = self.nodes.len() as u32;

        self.rnd_pow *= 0.75;

        let mut new_edges = Vec::with_capacity(num_edges * 2 + num_faces * 3);
        let mut edge_index = HashMap::new();

        for e in self.edges.iter() {

            let (midpoint, elevation) = {
                let p0 = &self.nodes[e.a as usize];
                let p1 = &self.nodes[e.b as usize];
                let delta = p1.point - p0.point;
                let e = (p1.elevation + p0.elevation) / 2.0;
                ((p0.point + delta * 0.5), e + (random::<f32>() - 0.5) * self.rnd_pow)
            };

            let vidx = self.nodes.len() as u32;
            self.nodes.push(Node::new(midpoint.normal(), elevation));

            debug!("Splitting edge ({}, {})", e.a, e.b);
            let e0 = make_edge(e.a, vidx);
            let e1 = make_edge(e.b, vidx);
            debug!("Generated edge ({}, {})", e.a, vidx);
            edge_index.insert((e.a, vidx), new_edges.len() as u32);
            new_edges.push(e0);
            debug!("Generated edge ({}, {})", e.b, vidx);
            edge_index.insert((e.b, vidx), new_edges.len() as u32);
            new_edges.push(e1);
        }

        let mut new_faces = Vec::with_capacity(num_faces * 4);

        {
            let mut find_edge = |a: u32, b: u32| -> u32 {
                let key = if a <= b { (a, b) } else { (b, a) };
                match edge_index.get(&key) {
                    Some(idx) => *idx,
                    None => {
                        debug!("Failed to find edge for {:?}", key);
                        let e = make_edge(a, b);
                        let idx = new_edges.len() as u32;
                        //edge_index.insert((e[0], e[1]), idx);
                        new_edges.push(e);
                        idx
                    }
                }
            };

            for f in self.faces.iter() {
                let p0 = f.points[0];
                let p1 = f.points[1];
                let p2 = f.points[2];
                debug!("Treating face ({}, {}, {})", p0, p1, p2);

                let e0 = f.edges[0];
                let e1 = f.edges[1];
                let e2 = f.edges[2];

                let n0 = first_new_vertex + e0;
                let n1 = first_new_vertex + e1;
                let n2 = first_new_vertex + e2;

                let e00 = find_edge(p0, n0);
                let e01 = find_edge(n0, p1);

                let e10 = find_edge(p1, n1);
                let e11 = find_edge(n1, p2);

                let e20 = find_edge(p2, n2);
                let e21 = find_edge(n2, p0);

                let ne0 = find_edge(n0, n1);
                let ne1 = find_edge(n1, n2);
                let ne2 = find_edge(n2, n0);

                new_faces.push(Face::new(Point3::new(p0, n0, n2), Point3::new(e00, ne2, e21)));
                new_faces.push(Face::new(Point3::new(n0, p1, n1), Point3::new(e01, e10, ne0)));
                new_faces.push(Face::new(Point3::new(p2, n2, n1), Point3::new(e20, ne1, e11)));
                new_faces.push(Face::new(Point3::new(n0, n1, n2), Point3::new(ne0, ne1, ne2)));

            }
        }

        self.edges = new_edges;
        self.faces = new_faces;
        info!("After subdivision {} nodes, {} edges and {} faces",
              self.nodes.len(),
              self.edges.len(),
              self.faces.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_terrain_has_edges_for_all_nodes() {
        let terr = Terrain::new();

        verify_edges_for_nodes(&terr, 5, 5);
    }

    #[test]
    fn new_terrain_has_correct_face_to_edge_linkage() {
        let terr = Terrain::new();

        verify_face_to_edge_link(&terr);
    }

    #[test]
    fn subdivided_terrain_has_edges_for_all_nodes() {
        let mut terr = Terrain::new();
        terr.subdivide();

        verify_edges_for_nodes(&terr, 5, 6);
    }

    #[test]
    fn subdivided_terrain_has_correct_face_to_edge_linkage() {
        let mut terr = Terrain::new();
        terr.subdivide();

        verify_face_to_edge_link(&terr);
    }

    fn verify_edges_for_nodes(terr: &Terrain, min_edges: u32, max_edges: u32) {
        let num_nodes = terr.nodes.len();
        let mut seen_nodes = Vec::with_capacity(num_nodes);
        for _ in 0..num_nodes {
            seen_nodes.push(0);
        }

        for e in terr.edges.iter() {
            let p0 = e[0] as usize;
            let p1 = e[1] as usize;
            assert!(p0 != p1, "Illegal edge between {} and {}", p0, p1);
            seen_nodes[p0] += 1;
            seen_nodes[p1] += 1;
        }

        for (i, n) in seen_nodes.into_iter().enumerate() {
            assert!(n > 0, "No edge leading to node {}", i);
            assert!(n >= min_edges && n <= max_edges,
                    "Illegal number of edges leading to node {}: {}",
                    i,
                    n);
        }
    }

    fn verify_face_to_edge_link(terr: &Terrain) {
        for (i, f) in terr.faces.iter().enumerate() {
            let p0 = f.points[0];
            let p1 = f.points[1];
            let p2 = f.points[2];

            let e0 = terr.edges[f.edges[0] as usize];
            let e1 = terr.edges[f.edges[1] as usize];
            let e2 = terr.edges[f.edges[2] as usize];

            let (e0_0, e0_1) = if p0 <= p1 { (p0, p1) } else { (p1, p0) };
            let (e1_0, e1_1) = if p1 <= p2 { (p1, p2) } else { (p2, p1) };
            let (e2_0, e2_1) = if p2 <= p0 { (p2, p0) } else { (p0, p2) };

            assert_eq!((e0_0, e0_1),
                       (e0[0], e0[1]),
                       "Edge 0 of face {} mismatch",
                       i);
            assert_eq!((e1_0, e1_1),
                       (e1[0], e1[1]),
                       "Edge 1 of face {} mismatch",
                       i);
            assert_eq!((e2_0, e2_1),
                       (e2[0], e2[1]),
                       "Edge 2 of face {} mismatch",
                       i);

        }
    }

}
