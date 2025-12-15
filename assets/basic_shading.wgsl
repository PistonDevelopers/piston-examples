
@group(0) @binding(0)
var<uniform> mvp: mat4x4<f32>;
@group(0) @binding(1)
var<uniform> m: mat4x4<f32>;
@group(0) @binding(2)
var<uniform> v: mat4x4<f32>;
@group(0) @binding(3)
var<uniform> LightPosition_worldspace: vec3<f32>;
@group(0) @binding(4)
var<uniform> ambientLight: f32;

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,

    @location(0) UV: vec2<f32>,
    @location(1) Position_worldspace: vec3<f32>,
    @location(2) EyeDirection_cameraspace: vec3<f32>,
    @location(3) LightDirection_cameraspace: vec3<f32>,
    @location(4) Normal_cameraspace: vec3<f32>,
}

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) nor: vec3<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mvp * vec4<f32>(position, 1.0);

    // Position of vertex in world space.
    out.Position_worldspace = (m * vec4<f32>(position, 1)).xyz;
    // Vector that goes from vertex to the camera, in camera space.
    let vertexPosition_cameraspace: vec3<f32> = (v * m * vec4(position, 1)).xyz;
    let EyeDirection_cameraspace: vec3<f32> = vec3<f32>(0, 0, 0) - vertexPosition_cameraspace;
    // Vector that goes from vertex to the light, in camera space.
    // M is omitted because it is identity.
    let LightPosition_cameraspace: vec3<f32> = (v * vec4<f32>(LightPosition_worldspace, 1)).xyz;
    out.LightDirection_cameraspace = LightPosition_cameraspace + EyeDirection_cameraspace;
    // Normal of the vertex, in camera space.
    // Use its inverse transpose if M scales the model.
    out.Normal_cameraspace = (v * m * vec4<f32>(nor, 0)).xyz;
    out.UV = uv;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let MaterialDiffuseColor: vec3<f32> = textureSample(t_diffuse, s_diffuse, in.UV).xyz;
    let MaterialAmbientColor: vec3<f32> = MaterialDiffuseColor * ambientLight;
    let n: vec3<f32> = normalize(in.Normal_cameraspace);
    let l: vec3<f32> = normalize(in.LightDirection_cameraspace);
    let cosTheta: f32 = clamp(dot(n, l), 0, 1);
    return vec4<f32>(MaterialAmbientColor + MaterialDiffuseColor * cosTheta, 1);
}
