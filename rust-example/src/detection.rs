#![allow(non_snake_case)]

use serde::Serialize;
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(Serialize, FieldNamesAsArray)]
pub(crate) struct State {
    accelerator_variant: AcceleratorVariant,
    version_variant: SomeVersionVariant,
    implementation_variant: ImplementationProviderVariant,
}

#[derive(Serialize)]
enum AcceleratorVariant {
    CPU,
    Cuda,
    ROCm,
    TPU,
}

#[derive(Serialize)]

enum SomeVersionVariant {
    Version1 = 11,
    Version2 = 12,
    VersionNotPresent = 0,
}

#[derive(Serialize)]
enum ImplementationProviderVariant {
    MPICH,
    OpenMPI,
    MSMPI,
    Undefined,
}

impl State {
    fn detect_accelerator_variant_state() -> AcceleratorVariant {
        // Do something here to determine accelerator implementation for env
        AcceleratorVariant::CPU
    }

    fn detect_some_version_variant_state() -> SomeVersionVariant {
        // Do something here to determine version information for env
        SomeVersionVariant::Version2
    }

    fn detect_implementation_variant_state() -> ImplementationProviderVariant {
        // Do something here to determine version information for env
        ImplementationProviderVariant::Undefined
    }

    pub fn new() -> Self {
        State {
            accelerator_variant: Self::detect_accelerator_variant_state(),
            version_variant: Self::detect_some_version_variant_state(),
            implementation_variant: Self::detect_implementation_variant_state(),
        }
    }
}

pub fn get_variant_names() -> Vec<&'static str> {
    return State::FIELD_NAMES_AS_ARRAY.to_vec()
}
