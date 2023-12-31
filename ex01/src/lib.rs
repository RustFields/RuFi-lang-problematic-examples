use rufi_core::core::path::slot::slot::Slot::Nbr;
use rufi_core::core::vm::round_vm::round_vm::RoundVM;

pub trait Language {
    /// The nbr language construct
    /// ### Arguments
    /// * mut self - A mutable reference to the language is needed since this function will mutate the VM
    /// * `expr` - The expression to evaluate
    fn nbr<A: 'static + Copy>(&mut self, expr: impl Fn() -> A) -> A;
}

pub struct L {
    pub round_vm: RoundVM,
}

impl Language for L {
    fn nbr<A: 'static + Copy>(&mut self, expr: impl Fn() -> A) -> A {
        // Get the VM via a mutable reference
        let vm = &mut self.round_vm;
        vm.nest(
            Nbr(vm.index().clone()),
            vm.only_when_folding_on_self(),
            true,
            || {
                // Here the VM is captured immutably
                match vm.neighbor() {
                    Some(nbr) if nbr.clone() != vm.self_id() => {
                        vm.neighbor_val().unwrap_or(&expr()).clone()
                    }
                    _ => expr()
                }
            }
        ) // mutable call to nest
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rufi_core::core::context::context::Context;
    use super::*;

    #[test]
    fn it_works() {
        let context = Context::new(0, HashMap::new(), HashMap::new(), HashMap::new());
        let vm = RoundVM::new_empty(context);
        let mut l = L { round_vm: vm };
        let res = l.nbr(|| 1);
        assert_eq!(res, 1);
    }
}
