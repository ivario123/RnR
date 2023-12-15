use crate::ast::{Static, Type};

use super::{TypeCheck, ValueMeta};

impl TypeCheck for Static {
    type ReturnType = Type;
    fn check(
        &self,
        env: &mut super::TypeEnv,
        _idx: usize,
    ) -> Result<Self::ReturnType, super::TypeErr> {
        // These are quite trivial to check, we just insert the
        // global in to the latest scope
        if env.is_empty() {
            return Err("Cannot declear variables in non existant scope".to_string());
        }
        let last_env = env.len();
        let scope = env.get_mut(last_env - 1).unwrap();
        scope.0.insert(
            self.id.clone(),
            ValueMeta {
                ty: Some(self.ty.clone()),
                assigned: true,
                mutable: self.mutable,
                shadowable: false,
                ref_counter: None,
            },
        );
        Ok(Type::Unit)
    }
}
