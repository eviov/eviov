macro_rules! op_raw {
    ($lhs:ident, $rhs:ident; $op:ident, $snake_op:ident; $assign:ident, $snake_assign:ident) => {
        impl std::ops::$op<$rhs> for $lhs {
            type Output = Self;

            fn $snake_op(mut self, other: $rhs) -> Self {
                self.0 = std::ops::$op::$snake_op(self.0, other);
                self
            }
        }

        impl std::ops::$assign<$rhs> for $lhs {
            fn $snake_assign(&mut self, rhs: $rhs) {
                *self = std::ops::$op::$snake_op(*self, rhs);
            }
        }
    };
}

macro_rules! op_newtype {
    ($lhs:ident, $rhs:ident; $op:ident, $snake_op:ident; $assign:ident, $snake_assign:ident) => {
        impl std::ops::$op<$rhs> for $lhs {
            type Output = Self;

            fn $snake_op(mut self, other: $rhs) -> Self {
                self.0 = std::ops::$op::$snake_op(self.0, other.0);
                self
            }
        }

        impl std::ops::$assign<$rhs> for $lhs {
            fn $snake_assign(&mut self, rhs: $rhs) {
                *self = std::ops::$op::$snake_op(*self, rhs);
            }
        }
    };
}

macro_rules! common_ops {
    ($($mac_raw:ident, $mac_newtype:ident : $op:ident :: $snake_op:ident, $assign:ident :: $snake_assign:ident;)*) => {$(
        #[allow(unused_macros)]
        macro_rules! $mac_raw {
            ($lhs:ident, $rhs:ident) => {
                op_raw!($lhs, $rhs; $op, $snake_op; $assign, $snake_assign);
            };
        }

        #[allow(unused_macros)]
        macro_rules! $mac_newtype {
            ($lhs:ident, $rhs:ident) => {
                op_newtype!($lhs, $rhs; $op, $snake_op; $assign, $snake_assign);
            };
        }
    )*};
}

common_ops! {
    add_raw, add_newtype: Add::add, AddAssign::add_assign;
    sub_raw, sub_newtype: Sub::sub, SubAssign::sub_assign;
    mul_raw, mul_newtype: Mul::mul, MulAssign::mul_assign;
    div_raw, div_newtype: Div::div, DivAssign::div_assign;
    rem_raw, rem_newtype: Rem::rem, RemAssign::rem_assign;
}
