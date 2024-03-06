use super::Point;
use crate::point_vec;
use nannou::color;

#[derive(Debug, Clone)]
pub struct Piece {
    pub name: String,
    pub center: Point,       //Rotation center of piece.
    pub offsets: Vec<Point>, //Offset of each block from rotation center. Add offset to center for block position.
    pub color: color::Rgb<u8>,
}

#[allow(unused)]
pub enum Direction {
    Clockwise,
    AntiClockwise,
}

impl Piece {
    //Returns the standard tetris pieces.
    pub fn get_standard_pieces() -> Vec<Piece> {
        vec![
            Piece {
                name: "Square".to_owned(),
                center: Point::new(4.5, 20.5),
                offsets: point_vec![(-0.5, 0.5), (0.5, 0.5), (-0.5, -0.5), (0.5, -0.5)],
                color: color::YELLOW,
            },
            Piece {
                name: "Straight".to_owned(),
                center: Point::new(4.5, 20.5),
                offsets: point_vec![(-1.5, -0.5), (-0.5, -0.5), (0.5, -0.5), (1.5, -0.5)],
                color: color::CYAN,
            },
            Piece {
                name: "L".to_owned(),
                center: Point::new(4.0, 20.0),
                offsets: point_vec![(-1.0, 0.0), (0.0, 0.0), (1.0, 0.0), (1.0, 1.0)],
                color: color::BLUE,
            },
            Piece {
                name: "Backwarsds L".to_owned(),
                center: Point::new(4.0, 20.0),
                offsets: point_vec![(-1.0, 0.0), (0.0, 0.0), (1.0, 0.0), (-1.0, 1.0)],
                color: color::ORANGERED,
            },
            Piece {
                name: "S".to_owned(),
                center: Point::new(4.0, 20.0),
                offsets: point_vec![(-1.0, 0.0), (0.0, 0.0), (0.0, 1.0), (1.0, 1.0)],
                color: color::LIME,
            },
            Piece {
                name: "Z".to_owned(),
                center: Point::new(4.0, 20.0),
                offsets: point_vec![(1.0, 0.0), (0.0, 0.0), (0.0, 1.0), (-1.0, 1.0)],
                color: color::RED,
            },
            Piece {
                name: "T".to_owned(),
                center: Point::new(4.0, 20.0),
                offsets: point_vec![(0.0, 0.0), (-1.0, 0.0), (1.0, 0.0), (0.0, 1.0)],
                color: color::PURPLE,
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
