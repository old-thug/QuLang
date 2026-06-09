use super::TypeChecker;
use qu_entities::layout::{TypeId, TypeKind};

#[derive(Debug, Clone)]
pub enum CoerceResult {
    Identity,
    IntegerWidening,
    IntegerShrinking,
    IntegerSignChange,
    Invalid,
}

#[derive(Debug)]
pub struct TypeCoercer<'a> {
    ctx: &'a TypeChecker<'a>,
    target: TypeId,
    source: TypeId,
}

impl<'a> TypeCoercer<'a> {
    pub fn coerce(ctx: &'a TypeChecker, expected_id: TypeId, source_id: TypeId) -> CoerceResult {
        if expected_id == source_id {
            return CoerceResult::Identity;
        }

        let mut coercer = TypeCoercer {
            ctx,
            target: expected_id,
            source: source_id,
        };
        coercer.check_integer_cast_required()
    }

    fn check_integer_cast_required(&self) -> CoerceResult {
        let target = self.ctx.get_type_layout_from_id(&self.target).unwrap();
        let source = self.ctx.get_type_layout_from_id(&self.source).unwrap();

        if target.is_numeric_type() && source.is_numeric_type() {
            let width_score = target.compare_numeric_widths(source);
            if width_score < 0 {
                return CoerceResult::IntegerShrinking;
            } else if width_score > 0 {
                return CoerceResult::IntegerWidening;
            } else {
                return CoerceResult::IntegerSignChange;
            }
        }

        //dbg!("continue coerce analysis");
        CoerceResult::Invalid
    }
}
