use std::ops::Add;

struct Milimeters(u32);
struct Meters(u32);

// impl Add<Meter>라고 명시하여 기본값 Self 대신 RHS타입 파라미터 지정
impl Add<Meters> for Milimeters {
    type Output = Milimeters;

    fn add(self, other: Meters) -> Milimeters {
        Milimeters(self.0 + (other.0 * 1000))
    }
}