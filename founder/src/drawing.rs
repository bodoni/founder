use font::{Glyph, Metrics, Number, Offset, Segment};
use svg::node::element;
use svg::node::Node;

pub fn draw(glyph: &Glyph) -> element::Group {
    let mut group = element::Group::new();
    let mut data = element::path::Data::new();
    let mut a = Offset::default();
    for contour in glyph.iter() {
        a += contour.offset;
        data = data.move_to(vec![a.0, a.1]);
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
    }
    if !data.is_empty() {
        group.append(element::Path::new().set("d", data));
    }
    group
}

pub fn transform(
    glyph: &Glyph,
    metrics: &Metrics,
    document_size: Number,
    mode: &str,
) -> (Number, Number, Number) {
    let (x, y, scale);
    match mode {
        "free" => {
            let (left, bottom, right, top) = glyph.bounding_box;
            let glyph_size = (right - left).max(top - bottom);
            scale = document_size / glyph_size;
            x = -left + (glyph_size - (right - left)) / 2.0;
            y = top + (glyph_size - (top - bottom)) / 2.0;
        }
        "local" => {
            let (left, _, right, _) = glyph.bounding_box;
            let glyph_size = metrics.ascender - metrics.descender;
            scale = document_size / glyph_size;
            x = -glyph.side_bearings.0 + (glyph_size - (right - left)) / 2.0;
            y = metrics.ascender;
        }
        _ => unreachable!(),
    }
    (x, y, scale)
}
