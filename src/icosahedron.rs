extern crate nalgebra as na;

use na::{Point2, Point3, normalize, Vector3};

pub type Vertex = Point3<f32>;
pub type Edge = Point2<u32>;

fn make_edge(a: u32, b: u32) -> Edge {
    if a > b {
        Edge::new(b, a)
    } else {
        Edge::new(a, b)
    }
}

pub struct Face {
    pub points: Point3<u32>,
    pub edges: Point3<u32>
}

impl Face {
    pub fn new(points: Point3<u32>, edges: Point3<u32>) -> Face {
        Face {
            points: points,
            edges: edges
        }
    }
}

pub struct Icosahedron {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>
}

impl Icosahedron {

    pub fn new() -> Icosahedron {
        let phi = ((5.0f32).sqrt() + 1.0) / 2.0;
        let du = 1.0 / (phi * phi + 1.0).sqrt();
        let dv = phi * du;
        let z = 0f32;

        let vertices = vec!(
            Vertex::new(  z,  dv,  du),
            Vertex::new(  z,  dv, -du),
            Vertex::new(  z, -dv,  du),
            Vertex::new(  z, -dv, -du),
            Vertex::new( du,   z,  dv),
            Vertex::new(-du,   z,  dv),
            Vertex::new( du,   z, -dv),
            Vertex::new(-du,   z, -dv),
            Vertex::new( dv,  du,   z),
            Vertex::new( dv, -du,   z),
            Vertex::new(-dv,  du,   z),
            Vertex::new(-dv, -du,   z)
            );

        let edges = vec!(
            Edge::new( 0,  1),
            Edge::new( 0,  4),
            Edge::new( 0,  5),
            Edge::new( 0,  8),
            Edge::new( 0, 10),
            Edge::new( 1,  6),
            Edge::new( 1,  7),
            Edge::new( 1,  8),
            Edge::new( 1, 10),
            Edge::new( 2,  3),
            Edge::new( 2,  4),
            Edge::new( 2,  5),
            Edge::new( 2,  9),
            Edge::new( 2, 11),
            Edge::new( 3,  6),
            Edge::new( 3,  7),
            Edge::new( 3,  9),
            Edge::new( 3, 11),
            Edge::new( 4,  5),
            Edge::new( 4,  8),
            Edge::new( 4,  9),
            Edge::new( 5, 10),
            Edge::new( 5, 11),
            Edge::new( 6,  7),
            Edge::new( 6,  8),
            Edge::new( 6,  9),
            Edge::new( 7, 10),
            Edge::new( 7, 11),
            Edge::new( 8,  9),
            Edge::new(10, 11)
            );

        let faces = vec!(
            Face::new(Point3::new(0,  8,  1), Point3::new( 3,  7,  0)),
            Face::new(Point3::new(0,  5,  4), Point3::new( 2, 18,  1)),
            Face::new(Point3::new(0, 10,  5), Point3::new( 4, 21,  2)),
            Face::new(Point3::new(0,  4,  8), Point3::new( 1, 19,  3)),
            Face::new(Point3::new(0,  1, 10), Point3::new( 0,  8,  4)),
            Face::new(Point3::new(1,  8,  6), Point3::new( 7, 24,  5)),
            Face::new(Point3::new(1,  6,  7), Point3::new( 5, 23,  6)),
            Face::new(Point3::new(1,  7, 10), Point3::new( 6, 26,  8)),
            Face::new(Point3::new(2, 11,  3), Point3::new(13, 17,  9)),
            Face::new(Point3::new(2,  9,  4), Point3::new(12, 20, 10)),
            Face::new(Point3::new(2,  4,  5), Point3::new(10, 18, 11)),
            Face::new(Point3::new(2,  3,  9), Point3::new( 9, 16, 12)),
            Face::new(Point3::new(2,  5, 11), Point3::new(11, 22, 13)),
            Face::new(Point3::new(3,  7,  6), Point3::new(15, 23, 14)),
            Face::new(Point3::new(3, 11,  7), Point3::new(17, 27, 15)),
            Face::new(Point3::new(3,  6,  9), Point3::new(14, 25, 16)),
            Face::new(Point3::new(4,  9,  8), Point3::new(20, 28, 19)),
            Face::new(Point3::new(5, 10, 11), Point3::new(21, 29, 22)),
            Face::new(Point3::new(6,  8,  9), Point3::new(24, 28, 25)),
            Face::new(Point3::new(7, 11, 10), Point3::new(27, 29, 26))
            );

        Icosahedron {
            vertices: vertices,
            edges: edges,
            faces: faces
        }

    }

    pub fn normal(&self, face: &Point3<u32>) -> Vector3<f32> {
        // Normal is midpoint of face, normalized
        let mut normal = Vector3::new(0.0f32, 0.0, 0.0);

        for p in face.iter() {
            let vert = &self.vertices[*p as usize];
            normal += vert.coords;
        }

        normalize(&normal)
    }

    fn find_edges(&self, vertex: u32) -> Vec<Edge> {
        let mut result = Vec::with_capacity(6);

        for (i, e) in self.edges.iter().enumerate() {
            if e[0] == vertex || e[1] == vertex {
                result.push(e.clone());
            }
        }

        result
    }

    pub fn find_faces(&self, vertex: u32) -> Vec<usize> {
        let mut result = Vec::with_capacity(8);

        for (i, f) in self.faces.iter().enumerate() {
            if f.points[0] == vertex || f.points[1] == vertex || f.points[2] == vertex {
                result.push(i);
            }
        }

        result
    }

    pub fn subdivide(&mut self) {
        let num_edges = self.edges.len();
        let num_faces = self.faces.len();
        let first_new_vertex = self.vertices.len() as u32;

        for e in self.edges.iter() {

            let midpoint = {
                let p0 = &self.vertices[e[0] as usize];
                let p1 = &self.vertices[e[1] as usize];
                let delta = p1.coords - p0.coords;
                (p0.coords + delta * 0.5).clone()
            };

            self.vertices.push(Vertex::from_coordinates(normalize(&midpoint)));
        }

        let mut new_edges = Vec::with_capacity(num_edges * 2 + num_faces * 3);
        let mut new_faces = Vec::with_capacity(num_faces * 4);

        {
            let mut add_edge = |a, b| {
                let idx = new_edges.len() as u32;
                new_edges.push(make_edge(a, b));
                idx
            };

            for f in self.faces.iter() {
                let p0 = f.points[0];
                let p1 = f.points[1];
                let p2 = f.points[2];

                let e0 = f.edges[0];
                let e1 = f.edges[1];
                let e2 = f.edges[2];

                let n0 = first_new_vertex + e0;
                let n1 = first_new_vertex + e1;
                let n2 = first_new_vertex + e2;

                let e00 = add_edge(p0, n0);
                let e01 = add_edge(n0, p1);

                let e10 = add_edge(p1, n1);
                let e11 = add_edge(n1, p2);

                let e20 = add_edge(p2, n2);
                let e21 = add_edge(n2, p0);

                let ne0 = add_edge(n0, n1);
                let ne1 = add_edge(n1, n2);
                let ne2 = add_edge(n2, n0);

                new_faces.push(Face::new(Point3::new(p0, n0, n2), Point3::new(e00, ne2, e21)));
                new_faces.push(Face::new(Point3::new(n0, p1, n1), Point3::new(e01, e10, ne0)));
                new_faces.push(Face::new(Point3::new(p2, n2, n1), Point3::new(e20, ne1, e11)));
                new_faces.push(Face::new(Point3::new(n0, n1, n2), Point3::new(ne0, ne1, ne2)));

            }
        }

        self.edges = new_edges;
        self.faces = new_faces;
    }

}


