use uuid::Uuid;

use super::{utilities::make_uuid, vertex::Vertex};

#[derive(Debug, Clone)]
pub struct Cell<T, U, V, const D: usize> {
    pub vertices: Vec<Vertex<T, U, D>>,
    pub uuid: Uuid,
    pub neighbors: Option<Vec<Uuid>>,
    pub data: Option<V>,
}

impl<T, U, V, const D: usize> Cell<T, U, V, D> {
    pub fn new(vertices: Vec<Vertex<T, U, D>>) -> Result<Self, &'static str> {
        if vertices.len() > D + 1 {
            return Err("Number of vertices must be less than or equal to D + 1");
        }
        let uuid = make_uuid();
        let neighbors = None;
        let data = None;
        Ok(Cell {
            vertices,
            uuid,
            neighbors,
            data,
        })
    }

    pub fn new_with_data(vertices: Vec<Vertex<T, U, D>>, data: V) -> Result<Self, &'static str> {
        if vertices.len() > D + 1 {
            return Err("Number of vertices must be less than or equal to D + 1");
        }
        let uuid = make_uuid();
        let neighbors = None;
        let data = Some(data);
        Ok(Cell {
            vertices,
            uuid,
            neighbors,
            data,
        })
    }

    pub fn number_of_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn dim(&self) -> usize {
        self.number_of_vertices() - 1
    }
}

#[cfg(test)]
mod tests {

    use crate::delaunay_core::point::Point;

    use super::*;

    #[test]
    fn make_cell_with_data() {
        let vertex1 = Vertex::new_with_data(Point::new([0.0, 0.0, 1.0]), 1);
        let vertex2 = Vertex::new_with_data(Point::new([0.0, 1.0, 0.0]), 1);
        let vertex3 = Vertex::new_with_data(Point::new([1.0, 0.0, 0.0]), 1);
        let vertex4 = Vertex::new_with_data(Point::new([1.0, 1.0, 1.0]), 2);
        let cell = Cell::new_with_data(vec![vertex1, vertex2, vertex3, vertex4], "three-one cell")
            .unwrap();

        assert_eq!(cell.vertices[0], vertex1);
        assert_eq!(cell.vertices[1], vertex2);
        assert_eq!(cell.vertices[2], vertex3);
        assert_eq!(cell.vertices[3], vertex4);
        assert_eq!(cell.vertices[0].data.unwrap(), 1);
        assert_eq!(cell.vertices[1].data.unwrap(), 1);
        assert_eq!(cell.vertices[2].data.unwrap(), 1);
        assert_eq!(cell.vertices[3].data.unwrap(), 2);
        assert_eq!(cell.dim(), 3);
        assert_eq!(cell.number_of_vertices(), 4);
        assert!(cell.neighbors.is_none());
        assert!(cell.data.is_some());
        assert_eq!(cell.data.unwrap(), "three-one cell");

        // Human readable output for cargo test -- --nocapture
        println!("Cell: {:?}", cell);
    }

    #[test]
    fn make_cell_with_data_with_too_many_vertices() {
        let vertex1 = Vertex::new_with_data(Point::new([0.0, 0.0, 1.0]), 1);
        let vertex2 = Vertex::new_with_data(Point::new([0.0, 1.0, 0.0]), 1);
        let vertex3 = Vertex::new_with_data(Point::new([1.0, 0.0, 0.0]), 1);
        let vertex4 = Vertex::new_with_data(Point::new([1.0, 1.0, 1.0]), 2);
        let vertex5 = Vertex::new_with_data(Point::new([2.0, 2.0, 2.0]), 3);
        let cell = Cell::new_with_data(
            vec![vertex1, vertex2, vertex3, vertex4, vertex5],
            "three-one cell",
        );

        assert!(cell.is_err());

        // Human readable output for cargo test -- --nocapture
        println!("{:?}", cell);
    }

    #[test]
    fn make_cell_without_data() {
        let vertex1 = Vertex::new_with_data(Point::new([0.0, 0.0, 1.0]), 1);
        let vertex2 = Vertex::new_with_data(Point::new([0.0, 1.0, 0.0]), 1);
        let vertex3 = Vertex::new_with_data(Point::new([1.0, 0.0, 0.0]), 1);
        let vertex4 = Vertex::new_with_data(Point::new([1.0, 1.0, 1.0]), 2);
        let cell: Cell<f64, i32, Option<()>, 3> =
            Cell::new(vec![vertex1, vertex2, vertex3, vertex4]).unwrap();

        assert_eq!(cell.vertices[0], vertex1);
        assert_eq!(cell.vertices[1], vertex2);
        assert_eq!(cell.vertices[2], vertex3);
        assert_eq!(cell.vertices[3], vertex4);
        assert_eq!(cell.vertices[0].data.unwrap(), 1);
        assert_eq!(cell.vertices[1].data.unwrap(), 1);
        assert_eq!(cell.vertices[2].data.unwrap(), 1);
        assert_eq!(cell.vertices[3].data.unwrap(), 2);
        assert_eq!(cell.dim(), 3);
        assert_eq!(cell.number_of_vertices(), 4);
        assert!(cell.neighbors.is_none());
        assert!(cell.data.is_none());

        // Human readable output for cargo test -- --nocapture
        println!("Cell: {:?}", cell);
    }

    #[test]
    fn make_cell_without_data_with_too_many_vertices() {
        let vertex1 = Vertex::new_with_data(Point::new([0.0, 0.0, 1.0]), 1);
        let vertex2 = Vertex::new_with_data(Point::new([0.0, 1.0, 0.0]), 1);
        let vertex3 = Vertex::new_with_data(Point::new([1.0, 0.0, 0.0]), 1);
        let vertex4 = Vertex::new_with_data(Point::new([1.0, 1.0, 1.0]), 2);
        let vertex5 = Vertex::new_with_data(Point::new([2.0, 2.0, 2.0]), 3);
        let cell: Result<Cell<f64, i32, Option<()>, 3>, &'static str> =
            Cell::new(vec![vertex1, vertex2, vertex3, vertex4, vertex5]);

        assert!(cell.is_err());

        // Human readable output for cargo test -- --nocapture
        println!("{:?}", cell);
    }
}
