#import bevy_pbr::forward_io::VertexOutput

#import bevy_render::globals::Globals
@group(0) @binding(1) var<uniform> globals: Globals; 

@group(2) @binding(0) var blue_noise_texture: texture_2d<f32>;
@group(2) @binding(1) var blue_noise_sampler: sampler;

@group(2) @binding(2) var perlin_noise_texture: texture_2d<f32>;
@group(2) @binding(3) var perlin_noise_sampler: sampler;

@group(2) @binding(4) var<uniform> seed: u32;

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

    let blue_noise = textureSample(blue_noise_texture, blue_noise_sampler, uv).r;
    let offset = fract(blue_noise);

    return density_raymarch(ro, rd, offset);
}

fn sd_sphere(p: vec3f, radius: f32) -> f32 {
    return length(p)-radius;
}

// An ellipse is defined by x^2/a^2 + y^2/b^2 + z^2/c^2 = 1
// Each axis length is defined by a,b,c respectively.
// The input axes represents 1/a^2,1/b^2,1/c^2 and ranges from [0,inf) in value
fn sd_ellipse(p: vec3f, axes: vec3f) -> f32 {
    return dot(p * p, axes) - 1.;
}

fn sd_cloud(p: vec3f) -> f32 {
    return fbm(p) - sd_ellipse(p, vec3(COEF_A, COEF_B, COEF_C));
}

// Hard coded ellipse axis lengths and eqn coefficients:
const A = 4.;
const B = 2.6;
const C = 1.8;
const COEF_A = 1 / (A*A);
const COEF_B = 1 / (B*B);
const COEF_C = 1 / (C*C);

const EPSILON = 0.25;
const STEPS = 35;
const MARCH_SIZE = 2*(CAMERA_Z_DIST + EPSILON)/f32(STEPS);

// Hard coded lighting (for now)
const LIGHT_COLOUR = vec3(1., .85, .52);
const GLOBAL_ILLUMINATION = vec3(.34, .34, .42);

fn density_raymarch(ro: vec3f, rd: vec3f, offset: f32) -> vec4f {
    // Establish the minimal illumination of the cloud (base colour)
    var out = vec4(GLOBAL_ILLUMINATION, 0.);

    let march_step = MARCH_SIZE * rd;
    var p = ro + march_step * offset;
    for (var i = 0; i < STEPS; i = i + 1){
        let density = sd_cloud(p);
        if density > 0.0 {
            // Determine how much light should illuminate this part of the cloud
            // by considering the cumulative and local densities.
            let colour = lighting_colour(p, density, out.a);
            out = vec4(colour, out.a);

            // Cummulative density of cloud travelling from camera
            out.a = out.a + (1. - out.a)/2.38;
            out = min(out, vec4(1.));
        }
        p = p + march_step;
    }
    return out;
}

fn lighting_colour(pos: vec3f, density: f32, alpha: f32) -> vec3f {
    // Normalise light direction here and assign colour
    let light_direction = normalize(LIGHT_DIRECTION);

    // When diffuse > 0, sunlight is supposedly hitting this part of the cloud more directly
    // Determine the isolated colour from light direction only
    let diffuse = (density - sd_cloud(pos - light_direction));
    let direction_mixer = 2.2 * max(diffuse, 0.);
    let direction_colour = mix(LIGHT_COLOUR, GLOBAL_ILLUMINATION, exp(-direction_mixer));

    // Greater densities reduce the amount of exposed light
    let dimming_mixer = (1. - alpha)/.09;
    let dimming_colour = mix(LIGHT_COLOUR, GLOBAL_ILLUMINATION, exp(-dimming_mixer));

    // Determine the resultant cloud colour
    return mix(direction_colour, dimming_colour, exp(-direction_mixer-dimming_mixer));
}

fn noise(pos: vec3f) -> f32 {
    let time_factor = 0.32;

    // diversify user generated seeds
    let user_seed_a = f32(seed % 10000)/10000. - .5;
    let user_seed_b = f32(seed % 256);
    let user_seed_c = f32(seed % 512);
    
    // transform seeds into usable components for the texture offset
    let seed_scalar = .9 + user_seed_a;
    let seed_x = 8.25 + seed_scalar * cos(time_factor * globals.time + user_seed_b);
    let seed_y = 15.34 + seed_scalar * sin(time_factor * globals.time + user_seed_c);

    // Use seeds to generate an offset
    let offset = vec2(seed_x, seed_y) + pos.z/1.72;  
    let uv = (pos.xy + offset)/64.;

    // Result to occupy range of approx. [-0.5 to 0.5],
    return textureSample(perlin_noise_texture, perlin_noise_sampler, uv).r - .46;
}

fn fbm(pos : vec3f) -> f32 {
    var q = pos;    
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
