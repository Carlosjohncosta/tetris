use super::Point;
use crate::{point_vec, Textures};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Piece<Texture> {
    pub name: String,
    pub center: Point,       //Rotation center of piece.
    pub offsets: Vec<Point>, //Offset of each block from rotation center. Add offset to center for block position.
    pub texture: Texture,
}

#[allow(unused)]
pub enum Direction {
    Clockwise,
    AntiClockwise,
}

impl<Texture> Piece<Rc<Texture>> {
    //Returns the standard tetris pieces.
    pub fn get_standard_pieces(textures: Textures<Texture>) -> Vec<Piece<Rc<Texture>>> {
        vec![
            Piece {
                name: "Square".to_owned(),
                center: Point::new(4.5, 20.5),
                offsets: point_vec![(-0.5, 0.5), (0.5, 0.5), (-0.5, -0.5), (0.5, -0.5)],
                texture: textures.yellow,
            },
            Piece {
                name: "Straight".to_owned(),
                center: Point::new(4.5, 20.5),
                offsets: point_vec![(-1.5, -0.5), (-0.5, -0.5), (0.5, -0.5), (1.5, -0.5)],
                texture: textures.light_blue,
            },
            Piece {
                name: "L".to_owned(),
                center: Point::new(4.0, 20.0),
                offsets: point_vec![(-1.0, 0.0), (0.0, 0.0), (1.0, 0.0), (1.0, 1.0)],
                texture: textures.blue,
            },
            Piece {
                name: "Backwarsds L".to_owned(),
                center: Point::new(4.0, 20.0),
                offsets: point_vec![(-1.0, 0.0), (0.0, 0.0), (1.0, 0.0), (-1.0, 1.0)],
                texture: textures.orange,
            },
            Piece {
                name: "S".to_owned(),
                center: Point::new(4.0, 20.0),
                offsets: point_vec![(-1.0, 0.0), (0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
                texture: textures.green,
            },
            Piece {
                name: "Z".to_owned(),
                center: Point::new(4.0, 20.0),
                offsets: point_vec![(1.0, 0.0), (0.0, 0.0), (0.0, 1.0), (-1.0, 1.0)],
                texture: textures.red,
            },
            Piece {
                name: "T".to_owned(),
                center: Point::new(4.0, 20.0),
                offsets: point_vec![(0.0, 0.0), (-1.0, 0.0), (1.0, 0.0), (0.0, 1.0)],
                texture: textures.purple,
            },
        ]
    }

    pub fn rotate(&mut self, direction: Direction) {
        let offset_iter = self.offsets.iter();
        self.offsets = match direction {
            Direction::Clockwise => offset_iter
                .map(|offset| Point::new(offset.y, -offset.x))
                .collect(),
            Direction::AntiClockwise => offset_iter
                .map(|offset| Point::new(-offset.y, offset.x))
                .collect(),
        };
    }

    pub fn get_block_positions(&self) -> Box<[Point]> {
        self.offsets
            .iter()
            .map(|offset| *offset + self.center)
            .collect::<Vec<Point>>()
            .into_boxed_slice()
    }
}
