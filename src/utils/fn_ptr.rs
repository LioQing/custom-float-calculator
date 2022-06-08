#[derive(Clone, PartialEq, Eq)]
pub struct FnPtr<Arg, Ret>(pub fn(Arg) -> Ret);