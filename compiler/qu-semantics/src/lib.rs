pub mod symbol_analyzer;
pub mod type_checker;

#[macro_export]
macro_rules! extract {
    ($value:expr, $target:pat) => {
        let $target = $value else { unreachable!(); };
    };
}
