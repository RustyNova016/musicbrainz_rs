macro_rules! impl_browse_includes {
    ($ty: ty, $(($args:ident, $inc: expr)),+) => {
        impl crate::BrowseQuery<$ty> {
               $(pub fn $args(&mut self) -> &mut Self  {
                     self.inner.include = self.inner.include($inc).include.to_owned();
                   self
               })*
            }

            impl_relations_includes!(crate::BrowseQuery<$ty>);
        };
}

pub(crate) use impl_browse_includes;
