mod edge;
mod face;
mod node;

use math::*;
use terrain::planet::Planet;
use terrain::types::{Vertex, Index3};

use rand::{random, thread_rng};
use rand::distributions::{IndependentSample, Range};
use std::f32;
use std::collections::HashMap;

pub use self::edge::Edge;
pub use self::face::Face;
pub use self::node::Node;

#[derive(Clone)]
pub struct Generator {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    rnd_pow: f32,
    level: u8,
}

impl Generator {
    pub fn new() -> Generator {
        let phi = ((5.0f32).sqrt() + 1.0) / 2.0;
        let du = 1.0 / (phi * phi + 1.0).sqrt();
        let dv = phi * du;
        let z = 0f32;

        fn random_elevation() -> f32 {
            random::<f32>() * 0.5
        }

        let mut nodes = vec![Node::new(Vertex::new(z, dv, du), random_elevation()),
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

        let mut edges = vec![Edge::new(0, 1),
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

        let faces = vec![Face::new(Index3::new(0, 8, 1), Index3::new(3, 7, 0)),
                         Face::new(Index3::new(0, 5, 4), Index3::new(2, 18, 1)),
                         Face::new(Index3::new(0, 10, 5), Index3::new(4, 21, 2)),
                         Face::new(Index3::new(0, 4, 8), Index3::new(1, 19, 3)),
                         Face::new(Index3::new(0, 1, 10), Index3::new(0, 8, 4)),
                         Face::new(Index3::new(1, 8, 6), Index3::new(7, 24, 5)),
                         Face::new(Index3::new(1, 6, 7), Index3::new(5, 23, 6)),
                         Face::new(Index3::new(1, 7, 10), Index3::new(6, 26, 8)),
                         Face::new(Index3::new(2, 11, 3), Index3::new(13, 17, 9)),
                         Face::new(Index3::new(2, 9, 4), Index3::new(12, 20, 10)),
                         Face::new(Index3::new(2, 4, 5), Index3::new(10, 18, 11)),
                         Face::new(Index3::new(2, 3, 9), Index3::new(9, 16, 12)),
                         Face::new(Index3::new(2, 5, 11), Index3::new(11, 22, 13)),
                         Face::new(Index3::new(3, 7, 6), Index3::new(15, 23, 14)),
                         Face::new(Index3::new(3, 11, 7), Index3::new(17, 27, 15)),
                         Face::new(Index3::new(3, 6, 9), Index3::new(14, 25, 16)),
                         Face::new(Index3::new(4, 9, 8), Index3::new(20, 28, 19)),
                         Face::new(Index3::new(5, 10, 11), Index3::new(21, 29, 22)),
                         Face::new(Index3::new(6, 8, 9), Index3::new(24, 28, 25)),
                         Face::new(Index3::new(7, 11, 10), Index3::new(27, 29, 26))];

        Generator::assign_links_to_nodes(&mut nodes, &edges, &faces);
        Generator::assign_links_to_edges(&mut edges, &faces);

        Generator {
            nodes: nodes,
            edges: edges,
            faces: faces,
            rnd_pow: 3.0,
            level: 0,
        }

    }

    fn assign_links_to_nodes(nodes: &mut Vec<Node>, edges: &Vec<Edge>, faces: &Vec<Face>) {
        for n in nodes.iter_mut() {
            n.edges.clear();
            n.faces.clear();
        }
        for (idx, e) in edges.iter().enumerate() {
            let p0 = e.a as usize;
            let p1 = e.b as usize;
            nodes[p0].add_edge(idx as u32);
            nodes[p1].add_edge(idx as u32);
        }
        for (idx, f) in faces.iter().enumerate() {
            let p0 = f.points.x as usize;
            let p1 = f.points.y as usize;
            let p2 = f.points.z as usize;
            let fidx = idx as u32;
            nodes[p0].add_face(fidx);
            nodes[p1].add_face(fidx);
            nodes[p2].add_face(fidx);
        }
    }

    fn assign_links_to_edges(edges: &mut Vec<Edge>, faces: &Vec<Face>) {
        for (idx, f) in faces.iter().enumerate() {
            let p0 = f.edges.x as usize;
            let p1 = f.edges.y as usize;
            let p2 = f.edges.z as usize;
            let fidx = idx as u32;
            edges[p0].add_face(fidx);
            edges[p1].add_face(fidx);
            edges[p2].add_face(fidx);
        }
    }

    pub fn current_level(&self) -> u8 {
        self.level
    }

    pub fn num_edges(&self) -> u32 {
        self.edges.len() as u32
    }

    pub fn num_nodes(&self) -> u32 {
        self.nodes.len() as u32
    }

    pub fn face_midpoint(&self, face: &Face) -> Vertex {
        let pindex = &face.points;
        let p0 = &self.nodes[pindex.x as usize].point;
        let p1 = &self.nodes[pindex.y as usize].point;
        let p2 = &self.nodes[pindex.z as usize].point;
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
                let mid = slerp(&p0.point, &p1.point, 0.5);
                //let mid = normalize(&p0.point + (&p1.point - &p0.point) / 2.0);
                let e = (p1.elevation + p0.elevation) / 2.0;
                (mid, e + rand(0.5) * self.rnd_pow)
            };

            let vidx = self.nodes.len() as u32;
            self.nodes.push(Node::new(midpoint, elevation));

            debug!("Splitting edge ({}, {})", e.a, e.b);
            let e0 = Edge::new(e.a, vidx);
            let e1 = Edge::new(e.b, vidx);
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
                        let idx = new_edges.len() as u32;
                        new_edges.push(Edge::new(a, b));
                        idx
                    }
                }
            };

            for f in self.faces.iter() {
                let p0 = f.points.x;
                let p1 = f.points.y;
                let p2 = f.points.z;
                debug!("Treating face ({}, {}, {})", p0, p1, p2);

                let e0 = f.edges.x;
                let e1 = f.edges.y;
                let e2 = f.edges.z;

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

                new_faces.push(Face::new(Index3::new(p0, n0, n2), Index3::new(e00, ne2, e21)));
                new_faces.push(Face::new(Index3::new(n0, p1, n1), Index3::new(e01, e10, ne0)));
                new_faces.push(Face::new(Index3::new(p2, n2, n1), Index3::new(e20, ne1, e11)));
                new_faces.push(Face::new(Index3::new(n0, n1, n2), Index3::new(ne0, ne1, ne2)));

            }
        }

        Generator::assign_links_to_nodes(&mut self.nodes, &new_edges, &new_faces);
        Generator::assign_links_to_edges(&mut new_edges, &new_faces);

        self.edges = new_edges;
        self.faces = new_faces;
        info!("After subdivision {} nodes, {} edges and {} faces",
              self.nodes.len(),
              self.edges.len(),
              self.faces.len());
    }

    fn rotation_predicate(old_node_0: &Node,
                          old_node_1: &Node,
                          new_node_0: &Node,
                          new_node_1: &Node)
                          -> bool {

        if new_node_0.faces.len() >= 7 || new_node_1.faces.len() >= 7 ||
           old_node_0.faces.len() <= 5 || old_node_1.faces.len() <= 5 {
            return false;
        }
        let old_edge_len = distance(&old_node_0.point, &old_node_1.point);
        let new_edge_len = distance(&new_node_0.point, &new_node_1.point);
        let ratio = old_edge_len / new_edge_len;
        if ratio >= 2.0 || ratio <= 0.5 {
            return false;
        }
        let v0 = (&old_node_1.point - &old_node_0.point) / old_edge_len;
        let v1 = normalize(&new_node_0.point - &old_node_0.point);
        let v2 = normalize(&new_node_1.point - &old_node_0.point);
        if v0.dot(v1) < 0.2 || v0.dot(v2) < 0.2 {
            return false;
        }
        let v3 = normalize(&new_node_0.point - &old_node_1.point);
        let v4 = normalize(&new_node_1.point - &old_node_1.point);
        if v0.dot(v3) > -0.2 || v0.dot(v4) > -0.2 {
            return false;
        }
        true
    }

    fn conditional_rotate_edge(&mut self, edge_index: u32) -> bool {
        let mut edge = self.edges[edge_index as usize].clone();
        let ef0 = edge.faces[0] as usize;
        let ef1 = edge.faces[1] as usize;
        let mut face0 = self.faces[ef0].clone();
        let mut face1 = self.faces[ef1].clone();
        let far_node_face_index_0 = face0.opposite_node_index(edge.a, edge.b);
        let far_node_face_index_1 = face1.opposite_node_index(edge.a, edge.b);
        let new_node_index_0 = face0.points[far_node_face_index_0];
        let old_node_index_0 = face0.points[far_node_face_index_0 + 1];
        let new_node_index_1 = face1.points[far_node_face_index_1];
        let old_node_index_1 = face1.points[far_node_face_index_1 + 1];
        let mut old_node_0 = self.nodes[old_node_index_0 as usize].clone();
        let mut old_node_1 = self.nodes[old_node_index_1 as usize].clone();
        let mut new_node_0 = self.nodes[new_node_index_0 as usize].clone();
        let mut new_node_1 = self.nodes[new_node_index_1 as usize].clone();
        let new_edge_index_0 = face1.edges[far_node_face_index_1 + 2];
        let new_edge_index_1 = face0.edges[far_node_face_index_0 + 2];
        let mut new_edge_0 = self.edges[new_edge_index_0 as usize].clone();
        let mut new_edge_1 = self.edges[new_edge_index_1 as usize].clone();

        if !Generator::rotation_predicate(&old_node_0, &old_node_1, &new_node_0, &new_node_1) {
            return false;
        }

        old_node_0.edges.retain(|t| *t != edge_index);
        old_node_1.edges.retain(|t| *t != edge_index);

        old_node_0.faces.retain(|t| *t != edge.faces[1]);
        old_node_1.faces.retain(|t| *t != edge.faces[0]);

        self.nodes[old_node_index_0 as usize] = old_node_0;
        self.nodes[old_node_index_1 as usize] = old_node_1;

        new_node_0.edges.push(edge_index);
        new_node_1.edges.push(edge_index);

        new_node_0.faces.push(edge.faces[1]);
        new_node_1.faces.push(edge.faces[0]);

        self.nodes[new_node_index_0 as usize] = new_node_0;
        self.nodes[new_node_index_1 as usize] = new_node_1;

        new_edge_0.faces.retain(|t| *t != edge.faces[1]);
        new_edge_1.faces.retain(|t| *t != edge.faces[0]);
        new_edge_0.faces.push(edge.faces[0]);
        new_edge_1.faces.push(edge.faces[1]);

        self.edges[new_edge_index_0 as usize] = new_edge_0;
        self.edges[new_edge_index_1 as usize] = new_edge_1;

        face0.points[far_node_face_index_0 + 2] = new_node_index_1;
        face1.points[far_node_face_index_1 + 2] = new_node_index_0;

        face0.edges[far_node_face_index_0 + 1] = new_edge_index_0;
        face1.edges[far_node_face_index_1 + 1] = new_edge_index_1;

        face0.edges[far_node_face_index_0 + 2] = edge_index;
        face1.edges[far_node_face_index_1 + 2] = edge_index;

        self.faces[ef0] = face0;
        self.faces[ef1] = face1;

        let new_edge = Edge::new(new_node_index_0, new_node_index_1);
        edge.a = new_edge.a;
        edge.b = new_edge.b;

        self.edges[edge_index as usize] = edge;

        true
    }

    pub fn distort(&mut self, degree: u32) -> bool {
        debug!("Distorting to degree {}", degree);
        let num_edges = self.edges.len() as u32;
        let mut rng = thread_rng();
        let between = Range::new(0, num_edges as u32);
        let mut i = 0;
        while i < degree {
            let mut attempts = 0;
            let mut edge_index = between.ind_sample(&mut rng);
            while !self.conditional_rotate_edge(edge_index) {
                attempts += 1;
                if attempts >= num_edges {
                    return false;
                }
                edge_index = (edge_index + 1) % num_edges;
            }
            i += 1;
        }
        true
    }

    pub fn relax(&mut self, multiplier: f32) -> f32 {
        let total_surface_area = 4.0 * f32::consts::PI;
        let ideal_face_area = total_surface_area / self.faces.len() as f32;
        let q3 = 3.0f32.sqrt();
        let ideal_distance_to_centroid = 2.0 * (q3 * ideal_face_area).sqrt() / 3.0 * 0.9;

        let mut point_shifts = vec![Vec3::new(0.0f32, 0.0, 0.0); self.nodes.len()];

        for face in self.faces.iter() {
            let index0 = face.points.x as usize;
            let index1 = face.points.y as usize;
            let index2 = face.points.z as usize;

            let n0 = &self.nodes[index0];
            let n1 = &self.nodes[index1];
            let n2 = &self.nodes[index2];

            let p0 = &n0.point;
            let p1 = &n1.point;
            let p2 = &n2.point;

            let centroid = normalize(p0 + p1 + p2);

            let v0 = &centroid - p0;
            let v1 = &centroid - p1;
            let v2 = &centroid - p2;

            let length0 = v0.length();
            let length1 = v1.length();
            let length2 = v2.length();

            point_shifts[index0] += v0 *
                                    (multiplier * (1.0 - ideal_distance_to_centroid / length0));
            point_shifts[index1] += v1 *
                                    (multiplier * (1.0 - ideal_distance_to_centroid / length1));
            point_shifts[index2] += v2 *
                                    (multiplier * (1.0 - ideal_distance_to_centroid / length2));
        }

        let mut i = 0;
        for mut vec in point_shifts.iter_mut() {
            let normal = &self.nodes[i].point;
            let mut projected = vec.clone();
            projected -= normal * vec.dot(normal);
            *vec = normalize(normal + projected);
            i += 1;
        }

        let mut rot_supp = vec![0.0f32; self.nodes.len()];

        for edge in self.edges.iter() {
            let index0 = edge.a as usize;
            let index1 = edge.b as usize;
            let old_point_0 = &self.nodes[index0].point;
            let old_point_1 = &self.nodes[index1].point;
            let new_point_0 = &point_shifts[index0];
            let new_point_1 = &point_shifts[index1];
            let ov = normalize(old_point_1 - old_point_0);
            let nv = normalize(new_point_1 - new_point_0);
            let suppression = (1.0 - ov.dot(nv)) * 0.5;
            rot_supp[index0] = rot_supp[index0].max(suppression);
            rot_supp[index1] = rot_supp[index1].max(suppression);
        }

        let mut total_shift = 0.0f32;

        for i in 0..self.nodes.len() {
            self.nodes[i].point = {
                let point = &self.nodes[i].point;
                let new_point = normalize(lerp(point, &point_shifts[i], 1.0 - rot_supp[i].sqrt()));
                total_shift += (&new_point - point).length();
                new_point
            };
        }

        total_shift
    }

    pub fn to_planet(&self) -> Planet {
        let num_tiles = self.nodes.len();
        let num_vertices = self.faces.len();

        let mut vertices = Vec::with_capacity(num_vertices + num_tiles);
        let mut borders = Vec::with_capacity(num_tiles);

        for face in self.faces.iter() {
            vertices.push(self.face_midpoint(face));
        }

        for (i, node) in self.nodes.iter().enumerate() {
            let node_index = i as u32;
            let mut border = Vec::with_capacity(node.faces.len());

            let mut face_index = node.faces[0];
            let mut midpoint = Vec3::new(0.0, 0.0, 0.0);

            loop {

                border.push(face_index);

                midpoint += &vertices[face_index as usize];

                let face = &self.faces[face_index as usize];
                let edge_index = if face.points.x == node_index {
                    face.edges[2]
                } else if face.points.y == node_index {
                    face.edges[0]
                } else if face.points.z == node_index {
                    face.edges[1]
                } else {
                    panic!("Face {:?} @ {} didn't contain node {:?} @ {}",
                           face,
                           face_index,
                           node,
                           node_index);
                };

                let edge = &self.edges[edge_index as usize];
                face_index = if edge.faces[0] == face_index {
                    edge.faces[1]
                } else if edge.faces[1] == face_index {
                    edge.faces[0]
                } else {
                    panic!("Edge {:?} @ {} didn't contain face {:?} @ {}",
                           edge,
                           edge_index,
                           face,
                           face_index)
                };

                if face_index == node.faces[0] {
                    break;
                }
            }

            vertices.push(midpoint / border.len() as f32);

            borders.push(border);
        }

        Planet::new(vertices, borders)
    }
}

fn rand(max: f32) -> f32 {
    (random::<f32>() * 2.0 * max) - max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_terrain_has_edges_for_all_nodes() {
        let terr = Generator::new();

        verify_edges_for_nodes(&terr, 5, 5);
    }

    #[test]
    fn new_terrain_has_faces_for_all_nodes() {
        let terr = Generator::new();

        verify_faces_for_nodes(&terr, 5, 5);
    }

    #[test]
    fn new_terrain_has_correct_face_to_edge_linkage() {
        let terr = Generator::new();

        verify_faces_for_edges(&terr, 2, 2);
        verify_face_to_edge_link(&terr);
    }

    #[test]
    fn subdivided_terrain_has_edges_for_all_nodes() {
        let mut terr = Generator::new();
        terr.subdivide();
        terr.subdivide();

        verify_edges_for_nodes(&terr, 5, 6);
    }

    #[test]
    fn subdivided_terrain_has_faces_for_all_nodes() {
        let mut terr = Generator::new();
        terr.subdivide();
        terr.subdivide();

        verify_faces_for_nodes(&terr, 5, 6);
    }

    #[test]
    fn subdivided_terrain_has_correct_face_to_edge_linkage() {
        let mut terr = Generator::new();
        terr.subdivide();
        terr.subdivide();

        verify_faces_for_edges(&terr, 2, 2);
        verify_face_to_edge_link(&terr);
    }

    fn verify_edges_for_nodes(terr: &Generator, min_edges: u32, max_edges: u32) {
        let num_nodes = terr.nodes.len();
        let mut seen_nodes = vec![Vec::new(); num_nodes];

        for (idx, e) in terr.edges.iter().enumerate() {
            let p0 = e.a as usize;
            let p1 = e.b as usize;
            assert!(p0 != p1,
                    "Illegal edge between {} and {} at index {}",
                    p0,
                    p1,
                    idx);
            let edges = &terr.nodes[p0].edges;
            assert!(edges.contains(&(idx as u32)),
                    "Node {} does not link back to edge {} (links: {:?})",
                    p0,
                    idx,
                    edges);
            let edges = &terr.nodes[p1].edges;
            assert!(edges.contains(&(idx as u32)),
                    "Node {} does not link back to edge {} (links: {:?})",
                    p1,
                    idx,
                    edges);

            seen_nodes[p0].push(idx as u32);
            seen_nodes[p1].push(idx as u32);
        }

        for (i, mut v) in seen_nodes.into_iter().enumerate() {
            let n = v.len() as u32;
            assert!(n > 0, "No edge leading to node {}", i);
            assert!(n >= min_edges && n <= max_edges,
                    "Illegal number of edges leading to node {}: {} (links: {:?}",
                    i,
                    n,
                    v);
            let mut actual = terr.nodes[i].edges.clone();
            actual.sort();

            v.sort();
            assert_eq!(actual,
                       v,
                       "Edge links mismatch for node {} (left=actual, right=expected)",
                       i);
        }
    }

    fn verify_faces_for_nodes(terr: &Generator, min_faces: u32, max_faces: u32) {
        let num_nodes = terr.nodes.len();
        let mut seen_nodes = vec![Vec::new(); num_nodes];

        for (idx, f) in terr.faces.iter().enumerate() {
            let p0 = f.points.x as usize;
            let p1 = f.points.y as usize;
            let p2 = f.points.z as usize;
            assert!(p0 != p1 && p0 != p2 && p1 != p2,
                    "Illegal face between {}, {} and {} at index {}",
                    p0,
                    p1,
                    p2,
                    idx);
            for p in [p0, p1, p2].iter() {
                let faces = &terr.nodes[*p].faces;

                assert!(faces.contains(&(idx as u32)),
                        "Node {} does not link back to face {} (links: {:?})",
                        p,
                        idx,
                        faces);

                seen_nodes[*p].push(idx as u32);
            }
        }

        for (i, mut v) in seen_nodes.into_iter().enumerate() {
            let n = v.len() as u32;
            assert!(n > 0, "No face includes node {}", i);
            assert!(n >= min_faces && n <= max_faces,
                    "Illegal number of faces leading to node {}: {} (links: {:?}",
                    i,
                    n,
                    v);
            let mut actual = terr.nodes[i].faces.clone();
            actual.sort();

            v.sort();
            assert_eq!(actual,
                       v,
                       "Face links mismatch for node {} (left=actual, right=expected)",
                       i);

        }
    }

    fn verify_faces_for_edges(terr: &Generator, min_faces: u32, max_faces: u32) {
        let num_edges = terr.edges.len();
        let mut seen_edges = vec![Vec::new(); num_edges];

        for (idx, f) in terr.faces.iter().enumerate() {
            let p0 = f.edges.x as usize;
            let p1 = f.edges.y as usize;
            let p2 = f.edges.z as usize;
            assert!(p0 != p1 && p0 != p2 && p1 != p2,
                    "Illegal face between edges {}, {} and {} at index {}",
                    p0,
                    p1,
                    p2,
                    idx);
            for p in [p0, p1, p2].iter() {
                let faces = &terr.edges[*p].faces;

                assert!(faces.contains(&(idx as u32)),
                        "Edge {} does not link back to face {} (links: {:?})",
                        p,
                        idx,
                        faces);

                seen_edges[*p].push(idx as u32);
            }
        }

        for (i, mut v) in seen_edges.into_iter().enumerate() {
            let n = v.len() as u32;
            assert!(n > 0, "No face includes edge {}", i);
            assert!(n >= min_faces && n <= max_faces,
                    "Illegal number of faces leading to edge {}: {} (links: {:?}",
                    i,
                    n,
                    v);
            let mut actual = terr.edges[i].faces.clone();
            actual.sort();

            v.sort();
            assert_eq!(actual,
                       v,
                       "Face links mismatch for edge {} (left=actual, right=expected)",
                       i);

        }
    }

    fn verify_face_to_edge_link(terr: &Generator) {
        for (i, f) in terr.faces.iter().enumerate() {
            let p0 = f.points.x;
            let p1 = f.points.y;
            let p2 = f.points.z;

            let ref e0 = terr.edges[f.edges.x as usize];
            let ref e1 = terr.edges[f.edges.y as usize];
            let ref e2 = terr.edges[f.edges.z as usize];

            let (e0_0, e0_1) = if p0 <= p1 { (p0, p1) } else { (p1, p0) };
            let (e1_0, e1_1) = if p1 <= p2 { (p1, p2) } else { (p2, p1) };
            let (e2_0, e2_1) = if p2 <= p0 { (p2, p0) } else { (p0, p2) };

            assert_eq!((e0_0, e0_1), (e0.a, e0.b), "Edge 0 of face {} mismatch", i);
            assert_eq!((e1_0, e1_1), (e1.a, e1.b), "Edge 1 of face {} mismatch", i);
            assert_eq!((e2_0, e2_1), (e2.a, e2.b), "Edge 2 of face {} mismatch", i);

        }
    }

}
