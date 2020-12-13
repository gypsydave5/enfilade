use core::option::Option;
use core::option::Option::{None, Some};
use shakmaty::Role;
use shakmaty::Role::{Bishop, King, Knight, Pawn, Queen, Rook};

const ROLES: [Role; 6] = [Pawn, Knight, Bishop, Rook, Queen, King];

pub trait RolePoints {
    fn points(&self) -> Option<u8>;
    fn higher_value_roles(&self) -> Vec<&Role>;
    fn lower_value_roles(&self) -> Vec<&Role>;
    fn is_higher_value(&self, r: Role) -> bool;
    fn is_lower_value(&self, r: Role) -> bool;
}

impl RolePoints for Role {
    fn points(&self) -> Option<u8> {
        match self {
            Pawn => Some(1),
            Knight => Some(3),
            Bishop => Some(3),
            Rook => Some(5),
            Queen => Some(9),
            King => None,
        }
    }

    fn higher_value_roles(&self) -> Vec<&Role> {
        ROLES.iter().filter(|&&r| self.is_higher_value(r)).collect()
    }

    fn lower_value_roles(&self) -> Vec<&Role> {
        ROLES.iter().filter(|&&r| self.is_lower_value(r)).collect()
    }

    fn is_higher_value(&self, r: Role) -> bool {
        if r == King {
            return true;
        };
        r.points()
            .and_then(|r| self.points().map(|p| r > p))
            .unwrap_or(false)
    }

    fn is_lower_value(&self, r: Role) -> bool {
        if self == &King {
            return true;
        };
        r.points()
            .and_then(|r| self.points().map(|p| r < p))
            .unwrap_or(false)
    }
}
