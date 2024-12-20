use super::*;

impl<'db> SimplifyExpr<'db> {
    /// Rewrites expressions where one half is a path referencing `self`. In
    /// this case, the expression can be rewritten to be an expression on the
    /// primary key.
    ///
    /// The caller must ensure it is an `eq` operation
    pub(super) fn rewrite_root_path_expr<'stmt>(
        &mut self,
        val: stmt::Expr<'stmt>,
    ) -> stmt::Expr<'stmt> {
        if let [field] = &self.model.primary_key.fields[..] {
            stmt::Expr::eq(*field, val)
        } else {
            todo!("composite primary keys")
        }
    }
}
