#import bevy_pbr::forward_io::VertexOutput

#import bevy_render::view::View
@group(0) @binding(0) var<uniform> view: View;

#import bevy_render::globals::Globals
@group(0) @binding(1) var<uniform> globals: Globals; 

@group(2) @binding(0) var blue_noise_texture: texture_2d<f32>;
@group(2) @binding(1) var blue_noise_sampler: sampler;

@group(2) @binding(2) var perlin_noise_texture: texture_2d<f32>;
@group(2) @binding(3) var perlin_noise_sampler: sampler;

const DEPTH = 50.;
const RENDER_VOLUME = true;
const CAMERA_Z_DIST = 5.;
const LIGHT_DIRECTION = vec3(-1., .3, .2);

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    // uv occupies the range of (-1,1)
    let uv = 2 * (mesh.uv - 0.5);
    var ro = vec3(0., 0., CAMERA_Z_DIST);
    var rd = normalize(vec3(uv, -1.));
    return densityRaymarch(ro, rd);
}

fn sdSphere(p: vec3f, radius: f32) -> f32 {
    return length(p)-radius;
}

// An ellipse is defined by x^2/a^2 + y^2/b^2 + z^2/c^2 = 1
// Each axis length is defined by a,b,c respectively.
// The input axes represents 1/a^2,1/b^2,1/c^2 and ranges from [0,inf) in value
fn sdEllipse(p: vec3f, axes: vec3f) -> f32 {
    return dot(p * p, axes) - 1.;
}

fn sdCloud(p: vec3f) -> f32 {
    return fbm(p) - sdEllipse(p, vec3(COEF_A, COEF_B, COEF_C));
}

// Hard coded ellipse axis lengths and eqn coefficients:
const A = 4.;
const B = 2.6;
const C = 1.8;
const COEF_A = 1 / (A*A);
const COEF_B = 1 / (B*B);
const COEF_C = 1 / (C*C);

const EPSILON = 0.1;
const STEPS = 40;
const MARCH_SIZE = 2*(CAMERA_Z_DIST + EPSILON)/f32(STEPS);

fn densityRaymarch(ro: vec3f, rd: vec3f) -> vec4f {
    // Normalise light direction here and assign colour
    let light_direction = normalize(LIGHT_DIRECTION);
    let light_colour = vec3(1., .65, .32);

    // Establish the minimal illumination of the cloud (base colour)
    let global_illumination = vec3(0.34, 0.34, 0.42);
    var out = vec4(global_illumination, 0.);

    var depth = 0.;
    var p = ro + depth * rd;
    for (var i = 0; i < STEPS; i = i + 1){
        let density = sdCloud(p);
        if density > 0.0 {
            // When diffuse > 0, sunlight is supposedly hitting this part of the cloud more directly 
            // Determine the isolated colour from light direction only
            let diffuse = (density - sdCloud(p - light_direction));
            let direction_mixer = 1.2 * max(diffuse, 0.);
            let direction_colour = mix(light_colour, global_illumination, exp(-direction_mixer));

            // Greater densities reduce the amount of exposed light
            let dimming_mixer = (1. - out.a)/.09;
            let dimming_colour = mix(light_colour, global_illumination, exp(-dimming_mixer));

            // Determine the resultant cloud colour
            let mixed_colour = mix(direction_colour, dimming_colour, exp(-direction_mixer-dimming_mixer));
            out = vec4(mixed_colour, out.a);

            // Cummulative density of cloud travelling from camera
            out.a = out.a + (1. - out.a)/2.28;
            out = min(out, vec4(1.));
        }
        depth = depth + MARCH_SIZE;
        p = ro + depth * rd;
    }
    return out;
}

fn noise(pos: vec3f) -> f32 {
    let offset = vec2(8.25, 15.34) + pos.z/1.72;  
    let uv = (pos.xy + offset)/64.;
    // Offset result range to approx. [-0.5 to 0.5],
    return textureSample(perlin_noise_texture, perlin_noise_sampler, uv).r - .46;
}

fn fbm(pos : vec3f) -> f32 {
    var q = pos; // + globals.time * vec3(1.,1.,1.);
    
    var f = 0.;
    var scale = 1.;
    var factor = 2.;
    for (var i = 0; i < 4; i = i + 1){
        f = f + scale * noise(q);
        q = q * factor;
        factor = factor + .021;
        scale = scale * 0.46;
    }

    return f;
}
