#[allow(missing_docs)]
#[macro_export]
macro_rules! op_raw {
    ($lhs:ty, $rhs:ty; $op:ident, $snake_op:ident; $assign:ident, $snake_assign:ident) => {
        $crate::op_raw!($lhs, $rhs; $op, $snake_op; $assign, $snake_assign; Self);
    };
    ($lhs:ty, $rhs:ty; $op:ident, $snake_op:ident; $assign:ident, $snake_assign:ident; $output:ident) => {
        impl std::ops::$op<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn $snake_op(self, other: $rhs) -> $output {
                $output(std::ops::$op::$snake_op(self.0, other))
            }
        }

        impl std::ops::$assign<$rhs> for $lhs {
            #[inline]
            fn $snake_assign(&mut self, rhs: $rhs) {
                *self = std::ops::$op::$snake_op(*self, rhs);
            }
        }
    };
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! op_newtype {
    ($lhs:ty, $rhs:ty; $op:ident, $snake_op:ident; $assign:ident, $snake_assign:ident) => {
        $crate::op_newtype!($lhs, $rhs; $op, $snake_op; $assign, $snake_assign; Self);
    };
    ($lhs:ty, $rhs:ty; $op:ident, $snake_op:ident; $assign:ident, $snake_assign:ident; $output:ident) => {
        impl std::ops::$op<$rhs> for $lhs {
            type Output = $output;

            #[inline]
            fn $snake_op(self, other: $rhs) -> $output {
                $output(std::ops::$op::$snake_op(self.0, other.0))
            }
        }

        impl std::ops::$assign<$rhs> for $lhs {
            #[inline]
            fn $snake_assign(&mut self, rhs: $rhs) {
                *self = std::ops::$op::$snake_op(*self, rhs);
            }
        }
    };
}

macro_rules! common_ops {
    ($($mac_raw:ident, $mac_newtype:ident : $op:ident :: $snake_op:ident, $assign:ident :: $snake_assign:ident;)*) => {$(
        #[allow(missing_docs)]
        #[macro_export]
        macro_rules! $mac_raw {
            ($lhs:ty, $rhs:ty) => {
                $crate::op_raw!($lhs, $rhs; $op, $snake_op; $assign, $snake_assign; Self);
            };
            ($lhs:ty, $rhs:ty => $output:ident) => {
                $crate::op_raw!($lhs, $rhs; $op, $snake_op; $assign, $snake_assign; $output);
            };
        }

        #[allow(missing_docs)]
        #[macro_export]
        macro_rules! $mac_newtype {
            ($lhs:ty, $rhs:ty) => {
                $crate::op_newtype!($lhs, $rhs; $op, $snake_op; $assign, $snake_assign; Self);
            };
            ($lhs:ty, $rhs:ty => $output:ident) => {
                $crate::op_newtype!($lhs, $rhs; $op, $snake_op; $assign, $snake_assign; $output);
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
