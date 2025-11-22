#version 330 core
in vec2 vTexCoord;
in float vPhase; // Received from Vertex Shader
in float vProbability; // Received from Vertex Shader
in vec3 vWorldPos; // World position for lighting

out vec4 FragColor;

// Helper function to convert HSV to RGB
vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    // Calculate smooth normals using derivatives
    vec3 dx = dFdx(vWorldPos);
    vec3 dy = dFdy(vWorldPos);
    vec3 normal = normalize(cross(dx, dy));
    
    // Lighting setup
    vec3 lightDir = normalize(vec3(0.3, -0.8, 0.5)); // Light from above (remember -Y is up)
    float diffuse = max(dot(normal, lightDir), 0.0);
    float ambient = 0.4;
    
    // Map Phase (-PI to PI) to Hue (0.0 to 1.0)
    float hue = (vPhase + 3.14159) / (2.0 * 3.14159);
    
    // Enhanced visibility with stronger contrast
    float brightness = 0.5 + 0.5 * vProbability; // Linear mapping for stronger contrast
    float saturation = 0.75; // Slightly more saturated
    
    // Base color from phase
    vec3 baseColor = hsv2rgb(vec3(hue, saturation, 1.0));
    
    // Apply lighting
    vec3 litColor = baseColor * (ambient + diffuse * 0.6);
    
    // Add edge glow for wave peaks
    float edgeGlow = pow(vProbability, 0.5) * 0.3;
    litColor += vec3(1.0) * edgeGlow;
    
    // Final brightness modulation
    vec3 finalColor = litColor * brightness;
    
    // Darken empty areas
    vec3 darkAmbient = vec3(0.02, 0.02, 0.05);
    finalColor = mix(darkAmbient, finalColor, smoothstep(0.0, 0.1, vProbability));
    
    FragColor = vec4(finalColor, 1.0);
}