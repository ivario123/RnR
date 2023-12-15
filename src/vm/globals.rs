use crate::ast::Static;

use super::{Eval, ValueMeta, Values};

impl Eval for Static {
    fn eval(
        &self,
        env: &mut super::VarEnv,
        scope: usize,
        max_iter: usize,
        iter_counter: &mut usize,
    ) -> Result<super::Values, super::VmErr> {
        let last_env = env.len();

        if last_env == 0 || last_env <= scope {
            return Err(super::VmErr::Err("Invalid scope usage.".to_string()));
        }

        let value = self.value.eval(env, 0, max_iter, iter_counter)?;

        let scope = env.get_mut(scope).unwrap();

        scope
            .0
            .insert(self.id.clone(), ValueMeta { value: Some(value) });
        Ok(Values::Lit(crate::ast::Literal::Unit))
    }
}
