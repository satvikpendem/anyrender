use wgpu::Backends;

pub(crate) fn android_surface_backends(
    mut requested: Backends,
    has_vulkan_adapter: bool,
) -> Backends {
    // Some Android drivers connect a Vulkan surface to the native window even
    // when Vulkan has no usable adapter. That prevents EGL from configuring
    // the GLES fallback on the same window. Explicit Vulkan-only stays strict.
    if requested.contains(Backends::VULKAN | Backends::GL) && !has_vulkan_adapter {
        requested.remove(Backends::VULKAN);
    }
    requested
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graphics_fallback_does_not_create_an_unusable_vulkan_surface() {
        assert_eq!(
            android_surface_backends(Backends::VULKAN | Backends::GL, false),
            Backends::GL
        );
    }

    #[test]
    fn available_vulkan_and_explicit_backend_choices_are_preserved() {
        let both = Backends::VULKAN | Backends::GL;
        assert_eq!(android_surface_backends(both, true), both);
        assert_eq!(
            android_surface_backends(Backends::VULKAN, false),
            Backends::VULKAN
        );
        assert_eq!(android_surface_backends(Backends::GL, false), Backends::GL);
    }

    #[test]
    fn unrelated_backend_flags_are_preserved() {
        let requested = Backends::VULKAN | Backends::GL | Backends::METAL;
        assert_eq!(
            android_surface_backends(requested, false),
            Backends::GL | Backends::METAL
        );
    }
}
