use crate::color::Color;
use array2d::Array2D;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub pixels: Array2D<Color>,
}

fn to_rgb(val: f64) -> i32 {
    (val * 255.0).round() as i32
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            pixels: Array2D::filled_with(
                Color {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                },
                height,
                width,
            ),
        }
    }

    pub fn at(&self, x: usize, y: usize) -> &Color {
        &self.pixels[(y, x)]
    }

    pub fn to_rgb(&self, x: usize, y: usize) -> (i32, i32, i32) {
        let pix = &self.pixels[(x, y)];
        (to_rgb(pix.red), to_rgb(pix.green), to_rgb(pix.blue))
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) -> &mut Canvas {
        match self.pixels.set(y, x, c) {
            Ok(_) => self,
            Err(_) => self,
        }
    }

    pub fn write_to_ppm(&self, output: &mut impl Write) {
        // PPM header
        writeln!(output, "P3").unwrap();
        writeln!(
            output,
            "{} {}",
            self.pixels.num_columns(),
            self.pixels.num_rows()
        )
        .unwrap();
        writeln!(output, "255").unwrap();

        for row_iter in self.pixels.rows_iter() {
            for element in row_iter {
                //
                write!(
                    output,
                    "{} {} {} ",
                    to_rgb(element.red),
                    to_rgb(element.green),
                    to_rgb(element.blue)
                )
                .unwrap();
            }
            writeln!(output, "").unwrap();
        }
    }
}

#[cfg(test)]
mod tests {

    use std::io::BufWriter;

    use crate::canvas::{to_rgb, Canvas};

    #[test]
    fn test_construct() {
        let array = Canvas::new(10, 20);
        assert_eq!(array.pixels.num_rows(), 20);
        assert_eq!(array.pixels.num_columns(), 10);
        for row_iter in array.pixels.rows_iter() {
            for element in row_iter {
                assert_eq!(element.red, 0.0);
                assert_eq!(element.green, 0.0);
                assert_eq!(element.blue, 0.0);
            }
        }
    }

    #[test]
    fn test_write_red() {
        let mut c = Canvas::new(10, 20);
        let red = crate::color::Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        };
        let c = c.write_pixel(2, 3, red);
        assert_eq!(c.at(2, 3), &red);
    }

    #[test]
    fn test_rgb() {
        assert_eq!(to_rgb(0.0), 0);
        assert_eq!(to_rgb(1.0), 255);
    }

    #[test]
    fn test_ppm() {
        let mut c = Canvas::new(5, 3);
        let c1 = crate::color::Color {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
        };
        let c2 = crate::color::Color {
            red: 0.0,
            green: 0.5,
            blue: 0.0,
        };
        let c3 = crate::color::Color {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
        };

        let c = c
            .write_pixel(0, 0, c1)
            .write_pixel(2, 1, c2)
            .write_pixel(4, 2, c3);

        //let mut output = String::new();
        let mut buf = BufWriter::new(Vec::new());

        c.write_to_ppm(&mut buf);
        let bytes = buf.into_inner().unwrap();
        let string = String::from_utf8(bytes).unwrap();
        let result = r#"P3
5 3
255
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 
"#;
        assert_eq!(string, String::from(result));
    }
}
