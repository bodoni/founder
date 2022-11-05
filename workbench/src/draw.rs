extern crate arguments;
extern crate font;
extern crate svg;

use font::Font;
use font::Glyph;
use svg::node::element::Group;
use svg::node::element::Style;
use svg::Document;

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let font = match arguments.get::<String>("font") {
        Some(font) => font,
        _ => {
            println!("Error: --font should be given.");
            return;
        }
    };
    let glyph = match arguments.get::<String>("glyph") {
        Some(glyph) => glyph.chars().next().unwrap(),
        _ => {
            println!("Error: --glyph should be given.");
            return;
        }
    };

    let font = Font::open(font).unwrap();
    let glyph = font.draw(glyph).unwrap().unwrap();
    let (width, height) = (glyph.advance_width(), glyph.height() + 2.0 * 50.0);
    let transform = format!("translate(0, {}) scale(1, -1)", glyph.bounding_box.3 + 50.0);
    let glyph = draw(&glyph).set("transform", transform);
    let style = Style::new("path { fill: none; stroke: black; stroke-width: 3; }");
    let document = Document::new()
        .set("width", width)
        .set("height", height)
        .add(style)
        .add(glyph);
    print!("{}", document);
}

fn draw(glyph: &Glyph) -> Group {
    use font::{Offset, Segment};
    use svg::node::element::path::Data;
    use svg::node::element::Path;
    use svg::node::Node;

    let mut group = Group::new();
    let mut a = Offset::default();
    for contour in glyph.iter() {
        a += contour.offset;
        let mut data = Data::new().move_to(vec![a.0, a.1]);
        for segment in contour.iter() {
            match segment {
                &Segment::Linear(b) => {
                    a += b;
                    data = data.line_by(vec![b.0, b.1]);
                }
                &Segment::Cubic(b, mut c, mut d) => {
                    c += b;
                    d += c;
                    a += d;
                    data = data.cubic_curve_by(vec![b.0, b.1, c.0, c.1, d.0, d.1]);
                }
                _ => unreachable!(),
            }
        }
        data = data.close();
        group.append(Path::new().set("d", data));
    }
    group
}
