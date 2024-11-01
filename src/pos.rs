use std::hash::Hash;
use std::hash::Hasher;

use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::number::complete::double;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Clone, Debug)]
pub(crate) enum PosVal {
    /// Axis set { A, B, C }
    A(f64),
    B(f64),
    C(f64),
    // Extruder
    E(f64),
    /// sets the federate for all subsequent moved.
    F(f64),
    /// Sets the laser power for the move
    S(f64),

    /// Axis set { U, V }
    U(f64),
    V(f64),

    /// Axis set { W, X, Y, Z }
    W(f64),
    X(f64),
    Y(f64),
    Z(f64),
}

impl Eq for PosVal {}

/// Bit wise comparison cant' compare directly [NAN and inf]
impl PartialEq for PosVal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::A(x), Self::A(y)) => x.to_bits() == y.to_bits(),
            (Self::B(x), Self::B(y)) => x.to_bits() == y.to_bits(),
            (Self::C(x), Self::C(y)) => x.to_bits() == y.to_bits(),

            (Self::E(x), Self::E(y)) => x.to_bits() == y.to_bits(),
            (Self::F(x), Self::F(y)) => x.to_bits() == y.to_bits(),
            (Self::S(x), Self::S(y)) => x.to_bits() == y.to_bits(),

            (Self::U(x), Self::U(y)) => x.to_bits() == y.to_bits(),
            (Self::V(x), Self::V(y)) => x.to_bits() == y.to_bits(),

            (Self::W(x), Self::W(y)) => x.to_bits() == y.to_bits(),
            (Self::X(x), Self::X(y)) => x.to_bits() == y.to_bits(),
            (Self::Y(x), Self::Y(y)) => x.to_bits() == y.to_bits(),
            (Self::Z(x), Self::Z(y)) => x.to_bits() == y.to_bits(),
            _ => false,
        }
    }
}

impl Hash for PosVal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::A(_) => "A".hash(state),
            Self::B(_) => "B".hash(state),
            Self::C(_) => "C".hash(state),
            Self::E(_) => "E".hash(state),
            Self::F(_) => "F".hash(state),
            Self::S(_) => "S".hash(state),
            Self::U(_) => "U".hash(state),
            Self::V(_) => "V".hash(state),
            Self::W(_) => "W".hash(state),
            Self::X(_) => "X".hash(state),
            Self::Y(_) => "Y".hash(state),
            Self::Z(_) => "Z".hash(state),
        }
    }
}

// TODO: can I use a macro here!!
pub fn parse_a(i: &str) -> IResult<&str, PosVal> {
    map(preceded(tag("A"), double), PosVal::A)(i)
}
pub fn parse_b(i: &str) -> IResult<&str, PosVal> {
    map(preceded(tag("B"), double), PosVal::B)(i)
}
pub fn parse_c(i: &str) -> IResult<&str, PosVal> {
    map(preceded(tag("C"), double), PosVal::C)(i)
}
pub fn parse_e(i: &str) -> IResult<&str, PosVal> {
    map(preceded(tag("E"), double), PosVal::E)(i)
}
pub fn parse_f(i: &str) -> IResult<&str, PosVal> {
    map(preceded(tag("F"), double), PosVal::F)(i)
}
pub fn parse_s(i: &str) -> IResult<&str, PosVal> {
    map(preceded(tag("S"), double), PosVal::S)(i)
}
pub fn parse_u(i: &str) -> IResult<&str, PosVal> {
    map(preceded(tag("U"), double), PosVal::U)(i)
}
pub fn parse_v(i: &str) -> IResult<&str, PosVal> {
    map(preceded(tag("V"), double), PosVal::V)(i)
}
pub fn parse_w(i: &str) -> IResult<&str, PosVal> {
    map(preceded(tag("W"), double), PosVal::W)(i)
}
pub fn parse_x(i: &str) -> IResult<&str, PosVal> {
    map(preceded(tag("X"), double), PosVal::X)(i)
}
pub fn parse_y(i: &str) -> IResult<&str, PosVal> {
    map(preceded(tag("Y"), double), PosVal::Y)(i)
}
pub fn parse_z(i: &str) -> IResult<&str, PosVal> {
    map(preceded(tag("Z"), double), PosVal::Z)(i)
}
