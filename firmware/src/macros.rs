/// Define matrix
macro_rules! define_matrix {
    (
        $vis:vis struct $name:ident {
            inputs: ($($row:ident),* $(,)?),
            outputs: ($($col:ident),* $(,)?) $(,)?
        }
    ) => {
        $vis struct $name {
            inputs: (
                $(atmega_hal::port::Pin<
                    atmega_hal::port::mode::Input<atmega_hal::port::mode::PullUp>,
                    atmega_hal::port:: $col
                >),*
            ),
            outputs: (
                $(atmega_hal::port::Pin<
                    atmega_hal::port::mode::Output,
                    atmega_hal::port:: $row
                >),*
            ),
        }

        impl $name {
            pub fn new(pin: atmega_hal::Pins) -> Self {
                paste::paste!(
                    Self {
                        inputs: (
                            $(pin. [<$col:lower>] .into_pull_up_input()),*
                        ),
                        outputs: (
                            $(pin. [<$row:lower>] .into_output()),*
                        ),
                    }
                )
            }

            pub fn scan(&self) {

            }
        }
    };
}

pub(crate) use define_matrix;
