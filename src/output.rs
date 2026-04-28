use crate::types::Feature;

pub fn write(features: Vec<Feature>) {
    for f in features {
        println!("{} @ {}", f.kind, f.offset);
    }
}
