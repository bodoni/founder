use font::Glyph;
use svg::node::element::Group;

pub fn draw(glyph: &Glyph) -> Group {
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
                &Segment::Quadratic(b, mut c) => {
                    c += b;
                    a += c;
                    data = data.quadratic_curve_by(vec![b.0, b.1, c.0, c.1]);
                }
                &Segment::Cubic(b, mut c, mut d) => {
                    c += b;
                    d += c;
                    a += d;
                    data = data.cubic_curve_by(vec![b.0, b.1, c.0, c.1, d.0, d.1]);
                }
            }
        }
        data = data.close();
        group.append(Path::new().set("d", data));
    }
    group
}
