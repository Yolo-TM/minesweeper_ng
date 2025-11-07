macro_rules! define_strategies {
    ($($variant:ident => $module:ident),* $(,)?) => {
        #[derive(Copy, Clone, Debug)]
        pub enum SolvingStrategy {
            $($variant),*
        }

        impl SolvingStrategy {
            const ALL: [Self; { 0 $(+ { let _ = stringify!($variant); 1 })* }] = [
                $(Self::$variant),*
            ];

            fn get_method(&self) -> fn(&Solver) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
                match self {
                    $(Self::$variant => $module::solve),*
                }
            }

            pub fn iter() -> impl Iterator<Item = SolvingStrategy> {
                Self::ALL.iter().copied()
            }

            pub fn execute(&self, solver: &Solver) -> (Vec<(u32, u32)>, Vec<(u32, u32)>) {
                self.get_method()(solver)
            }
        }
    };
}
