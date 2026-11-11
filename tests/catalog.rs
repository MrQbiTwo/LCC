// Add a group in one place. Every group implements cases() -> Vec<Case>.
use crate::Case;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suite {
    Ordinary,
    Interop,
}

pub struct Group {
    pub name: &'static str,
    pub suite: Suite,
    pub cases: fn() -> Vec<Case>,
}

macro_rules! groups {
    ($($suite:ident $name:ident => $path:literal),+ $(,)?) => {
        $(#[path = $path] mod $name;)+

        pub const GROUPS: &[Group] = &[
            $(Group { name: stringify!($name), suite: Suite::$suite, cases: $name::cases },)+
        ];
    };
}

groups! {
    Ordinary core => "groups/core.rs",
    Ordinary preprocess => "groups/preprocess/cases.rs",
    Ordinary decimal => "groups/runtime/decimal.rs",
    Ordinary stdio => "groups/runtime/stdio/mod.rs",
    Ordinary calculator => "groups/runtime/calculator.rs",
    Ordinary floating => "groups/floating.rs",
    Ordinary fp_abi => "groups/fp_abi.rs",
    Ordinary fp_constant => "groups/fp_constant.rs",
    Ordinary integer_constant => "groups/integer_constant.rs",
    Ordinary constant_expression => "groups/constant_expression/mod.rs",
    Ordinary null_constant => "groups/null_constant.rs",
    Ordinary flow_labels => "groups/flow_labels.rs",
    Ordinary typedef => "groups/typedef.rs",
    Ordinary type_rules => "groups/type_rules.rs",
    Ordinary operand_constraints => "groups/operand_constraints/mod.rs",
    Ordinary cast_constraints => "groups/cast_constraints/mod.rs",
    Ordinary comparison_constraints => "groups/comparison_constraints/mod.rs",
    Ordinary conversion_constraints => "groups/conversion_constraints/mod.rs",
    Ordinary type_identity => "groups/type_identity/mod.rs",
    Ordinary conditional_composite => "groups/conditional_composite/mod.rs",
    Ordinary enumeration => "groups/enumeration.rs",
    Ordinary pp_numbers => "groups/preprocess/numbers.rs",
    Ordinary aggregate => "groups/aggregate/mod.rs",
    Ordinary initializer => "groups/initializer.rs",
    Ordinary array_bounds => "groups/array_bounds/mod.rs",
    Ordinary address_initializer => "groups/address_initializer/mod.rs",
    Ordinary storage => "groups/storage/mod.rs",
    Ordinary storage_context => "groups/storage_context/mod.rs",
    Ordinary tentative => "groups/tentative/mod.rs",
    Ordinary standard_headers => "groups/standard_headers/mod.rs",
    Ordinary aggregate_call => "groups/aggregate_call/mod.rs",
    Ordinary aggregate_abi => "groups/aggregate_abi/mod.rs",
    Ordinary aggregate_limits => "groups/aggregate_limits.rs",
    Ordinary vararg => "groups/vararg/mod.rs",
    Ordinary vararg_abi => "groups/vararg_abi/mod.rs",
    Ordinary bitfield => "groups/bitfield/mod.rs",
    Ordinary cli => "groups/cli.rs",
    Interop interop => "groups/interop/mod.rs",
    Interop address_interop => "groups/address_interop.rs",
    Interop storage_interop => "groups/storage_interop.rs",
    Interop tentative_interop => "groups/tentative_interop.rs",
}
