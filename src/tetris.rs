use crate::shape::Shape;

#[derive(Debug)]
pub struct Tetris {
    width: usize,
    height: usize,
    current_shape: Shape,
    fixed_shapes: Vec<Shape>,
}

impl Tetris {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, current_shape: Shape::new_random(), fixed_shapes: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::Tetris;
    #[test]
    fn test() {
        println!("{:#?}", Tetris::new(5, 10))
    }
}
