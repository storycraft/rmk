/// Define matrix
macro_rules! define_matrix {
    (
        $vis:vis struct $name:ident {
            pub inputs: ($($row:ident),* $(,)?),
            pub outputs: ($($col:ident),* $(,)?) $(,)?
        }
    ) => {
        $vis struct $name {
            pub inputs: (
                $(arduino_hal::port::Pin<
                    arduino_hal::port::mode::Input<arduino_hal::port::mode::PullUp>,
                    arduino_hal::hal::port:: $col
                >),*
            ),
            pub outputs: (
                $(arduino_hal::port::Pin<
                    arduino_hal::port::mode::Output,
                    arduino_hal::hal::port:: $row
                >),*
            ),
        }

        impl $name {
            pub fn new(pin: arduino_hal::hal::Pins) -> Self {
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
        }
    };
}

pub(crate) use define_matrix;
