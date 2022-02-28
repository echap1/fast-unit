use std::cmp::min;
use crate::UNITS;

#[derive(Clone)]
pub(crate) struct NumberUnit {
    pub(crate) u: Vec<i16>
}

impl PartialEq for NumberUnit {
    fn eq(&self, other: &Self) -> bool {
        let k = min(self.u.len(), other.u.len());
        for i in 0..k {
            if self.u[i] != other.u[i] {
                return false
            }
        }
        if self.u.len() > other.u.len() {
            for i in (k + 1)..self.u.len() {
                if self.u[i] != 0 {
                    return false
                }
            }
        } else if other.u.len() > self.u.len() {
            for i in (k + 1)..other.u.len() {
                if other.u[i] != 0 {
                    return false
                }
            }
        }
        true
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl ToString for NumberUnit {
    fn to_string(&self) -> String {
        let mut numerator: String = "".to_string();
        let mut denominator: String = "".to_string();
        for (i, p) in self.u.iter().enumerate() {
            if *p == 0 { continue; }
            unsafe {
                let n = &UNITS.lock().unwrap()[i].name;
                if *p == 1 { numerator += n }
                else if *p == -1 { denominator += n }
                else if *p > 1 { numerator += &*(n.to_owned() + &p.to_string()) }
                else { denominator += &*(n.to_owned() + &(-p).to_string()) }
            }
        }
        return if denominator == "" {
            numerator
        } else {
            numerator + "/" + &*denominator
        }
    }
}