use serde_json::Value as JsonValue;

/// add b to a, overriding matching keys
pub fn merge_json(a: &mut JsonValue, b: &JsonValue) {
    match (a, b) {
        (&mut JsonValue::Object(ref mut a), &JsonValue::Object(ref b)) => {
            for (k, v) in b {
                merge_json(a.entry(k.clone()).or_insert(JsonValue::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

/// check whether a point is within a rectangle
#[derive(Debug, Clone)]
pub struct Rect {
    pub x1: i64,
    pub x2: i64,
    pub y1: i64,
    pub y2: i64,
}

impl Rect {
    pub fn contains(&self, x: i64, y: i64) -> bool {
        return self.x1 < x && x < self.x2 && self.y1 < y && y < self.y2;
    }
}
