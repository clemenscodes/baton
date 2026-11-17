use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct GraphicsProcessingUnitAllocator;

impl Layer for GraphicsProcessingUnitAllocator {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for GraphicsProcessingUnitAllocator {}

impl GraphicsProcessingUnitAllocator {
    const NAME: &'static str = "gpu-allocator";

    const NAMES: &'static [&'static str] = &[
        "allocate",
        "Allocation",
        "allocation_scheme",
        "allocation_sizes",
        "AllocationCreateDesc",
        "AllocationError",
        "AllocationScheme",
        "AllocationSizes",
        "Allocator",
        "AllocatorCreateDesc",
        "AllocatorDebugSettings",
        "buffer_device_address",
        "CpuToGpu",
        "debug_settings",
        "device",
        "free",
        "gpu_allocator",
        "GpuAllocatorManaged",
        "GpuOnly",
        "GpuToCpu",
        "instance",
        "linear",
        "location",
        "mapped_slice",
        "mapped_slice_mut",
        "memory",
        "MemoryLocation",
        "name",
        "new",
        "offset",
        "OutOfMemory",
        "physical_device",
        "requirements",
        "Result",
        "vulkan",
    ];
}

impl TryFrom<GraphicsProcessingUnitAllocator> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: GraphicsProcessingUnitAllocator) -> Result<Self, Self::Error> {
        let foreign_name_text = GraphicsProcessingUnitAllocator::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = GraphicsProcessingUnitAllocator::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_the_graphics_processing_unit_allocator_is_a_name() {
        let graphics_processing_unit_allocator = GraphicsProcessingUnitAllocator;

        let foreign_language = ForeignLanguage::try_from(graphics_processing_unit_allocator);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
