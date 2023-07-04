use rufi_core::core::path::slot::slot::Slot::Nbr;
use rufi_core::core::vm::round_vm::round_vm::RoundVM;

pub trait Language {

    fn nbr<A: 'static + Copy>(&mut self, expr: impl Fn() -> A) -> A;
}

pub struct L {
    pub round_vm: RoundVM,
}

impl Language for L {
    fn nbr<A: 'static + Copy>(&mut self, expr: impl Fn() -> A) -> A {
        let vm = &mut self.round_vm;
        // Here we calculate the value of the neighbor before we nest the slot, avoiding borrowing conflicts.
        let value = match vm.neighbor() {
            Some(nbr) if nbr.clone() != vm.self_id() => {
                vm.neighbor_val().unwrap_or(&expr()).clone()
            }
            _ => expr()
        };
        vm.nest(
            Nbr(vm.index().clone()),
            vm.only_when_folding_on_self(),
            true,
            || value)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rufi_core::core::context::context::Context;
    use super::*;

    // This test will work because there are no borrowing conflicts
    #[test]
    fn it_works() {
        let context = Context::new(0, HashMap::new(), HashMap::new(), HashMap::new());
        let vm = RoundVM::new_empty(context);
        let mut l = L { round_vm: vm };
        //This time it will compile.
        let res = l.nbr(|| 1);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_nested_nbr() {
        let context = Context::new(0, HashMap::new(), HashMap::new(), HashMap::new());
        let vm = RoundVM::new_empty(context);
        let mut l = L { round_vm: vm };
        // Here in the nested nbr, l can't be borrowed mutably since it is captured immutably by the closure.
        let res = l.nbr(|| l.nbr(|| 1));
        assert_eq!(res, 1);
    }
}
