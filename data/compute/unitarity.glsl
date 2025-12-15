#version 450

layout(local_size_x = 16, local_size_y = 16) in;

// Input wavefunction texture
layout(binding = 0) uniform sampler2D u_wavefunction;

// Output buffer for partial sums (one per workgroup)
layout(std430, binding = 1) buffer PartialSums {
    float partial_sums[];
};

// Shared memory for reduction within workgroup
shared float shared_sums[256]; // 16x16 = 256 threads per workgroup

void main() {
    ivec2 pixel = ivec2(gl_GlobalInvocationID.xy);
    ivec2 size = textureSize(u_wavefunction, 0);
    
    // Calculate local probability density
    float local_sum = 0.0;
    if (pixel.x < size.x && pixel.y < size.y) {
        vec2 uv = (vec2(pixel) + 0.5) / vec2(size);
        vec2 psi = texture(u_wavefunction, uv).rg;
        // |ψ|² = real² + imag²
        local_sum = dot(psi, psi);
    }
    
    // Store in shared memory
    uint local_index = gl_LocalInvocationID.y * 16u + gl_LocalInvocationID.x;
    shared_sums[local_index] = local_sum;
    barrier();
    
    // Parallel reduction in shared memory
    for (uint stride = 128u; stride > 0u; stride >>= 1u) {
        if (local_index < stride) {
            shared_sums[local_index] += shared_sums[local_index + stride];
        }
        barrier();
    }
    
    // First thread writes workgroup sum to global buffer
    if (local_index == 0u) {
        uint workgroup_index = gl_WorkGroupID.y * gl_NumWorkGroups.x + gl_WorkGroupID.x;
        partial_sums[workgroup_index] = shared_sums[0];
    }
}
