use wgpu::Limits;

pub(crate) fn requested_device_limits(
    override_limits: Option<Limits>,
    available: &Limits,
) -> Limits {
    // Default limits are preferences, not renderer requirements. In particular,
    // graphics-only GLES adapters can render without WebGPU's compute limits.
    // Keep explicit overrides intact so unsupported requirements still fail.
    override_limits.unwrap_or_else(|| {
        Limits {
            // Retain the existing iOS simulator varying limit.
            max_inter_stage_shader_variables: 15,
            ..Limits::default()
        }
        .or_worse_values_from(available)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_fit_a_graphics_only_downlevel_adapter() {
        let available = Limits::downlevel_webgl2_defaults();
        let requested = requested_device_limits(None, &available);
        assert!(requested.check_limits(&available));
        assert_eq!(requested.max_compute_workgroups_per_dimension, 0);
        assert_eq!(requested.max_storage_buffers_per_shader_stage, 0);
    }

    #[test]
    fn defaults_respect_alignment_and_smaller_resource_limits() {
        let available = Limits {
            min_uniform_buffer_offset_alignment: 512,
            min_storage_buffer_offset_alignment: 512,
            max_sampled_textures_per_shader_stage: 8,
            max_inter_stage_shader_variables: 8,
            ..Limits::downlevel_webgl2_defaults()
        };
        let requested = requested_device_limits(None, &available);
        assert!(requested.check_limits(&available));
        assert_eq!(requested.min_uniform_buffer_offset_alignment, 512);
        assert_eq!(requested.max_inter_stage_shader_variables, 8);
    }

    #[test]
    fn capable_adapters_keep_the_existing_defaults() {
        let available = Limits::default();
        let expected = Limits {
            max_inter_stage_shader_variables: 15,
            ..Limits::default()
        };
        assert_eq!(requested_device_limits(None, &available), expected);
    }

    #[test]
    fn explicit_requirements_remain_strict() {
        let available = Limits::downlevel_webgl2_defaults();
        let required = Limits::default();
        let requested = requested_device_limits(Some(required.clone()), &available);
        assert_eq!(requested, required);
        assert!(!requested.check_limits(&available));
    }
}
