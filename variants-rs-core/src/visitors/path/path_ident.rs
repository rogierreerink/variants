use syn::{Error, Ident, Path, visit_mut::VisitMut};

pub struct PathIdentAppender<'a> {
    base_path: &'a Path,
    append: &'a Ident,
    _errors: &'a mut Vec<Error>,
}

impl<'a> PathIdentAppender<'a> {
    /// Append the string value of `append` to the last path segment to match
    /// the full `base_path` and replaces the span of that segment with the span
    /// of `append`.
    ///
    /// # Examples
    /// - `impl Foo {}` becomes `impl FooBar {}`
    /// - `let _ = Foo::new();` becomes `let _ = FooBar::new();`
    /// - `path::Foo::new();` becomes `path::FooBar::new();`
    /// - `return crate::Foo {};` becomes `return crate::FooBar {};`
    ///
    pub fn new(base_path: &'a Path, append: &'a Ident, errors: &'a mut Vec<Error>) -> Self {
        Self {
            base_path,
            append,
            _errors: errors,
        }
    }
}

impl VisitMut for PathIdentAppender<'_> {
    fn visit_path_mut(&mut self, node: &mut Path) {
        if !self
            .base_path
            .segments
            .iter()
            .zip(&node.segments)
            .all(|(self_seg, node_seg)| self_seg == node_seg)
        {
            return;
        }

        node.segments
            .get_mut(self.base_path.segments.len() - 1)
            .map(|seg| {
                let new_str = format!("{}{}", seg.ident, self.append);
                let new_span = self.append.span();
                seg.ident = Ident::new(&new_str, new_span);
            });
    }
}
