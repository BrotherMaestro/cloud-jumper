#import bevy_pbr::forward_io::VertexOutput

#import bevy_render::view::View
@group(0) @binding(0) var<uniform> view: View;

#import bevy_render::globals::Globals
@group(0) @binding(1) var<uniform> globals: Globals; 

@group(2) @binding(0) var blue_noise_texture: texture_2d<f32>;
@group(2) @binding(1) var blue_noise_sampler: sampler;

const DEPTH = 50.;
const RENDER_VOLUME = true;
const CAMERA_Z_DIST = 5.;

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

// Hard coded ellipse axis lengths and eqn coefficients:
const A = 2.;
const B = 1.2;
const C = 1.;
const COEF_A = 1 / (A*A);
const COEF_B = 1 / (B*B);
const COEF_C = 1 / (C*C);

fn raymarch(ro: vec3f, rd: vec3f) -> f32 {
    var d0 = 0.0;
    let steps = 500;
    for (var i = 0; i < steps; i = i + 1){
        var p = ro + rd * d0;
        var dS = sdEllipse(p, vec3(COEF_A, COEF_B, COEF_C));
        d0 = d0 + dS;
        if d0 > DEPTH || dS < 0.04 {
            break;
        }
    }
    return d0;
}

const EPSILON = 0.1;
const STEPS = 40;
const MARCH_SIZE = 2*(CAMERA_Z_DIST + EPSILON)/f32(STEPS);

fn densityRaymarch(ro: vec3f, rd: vec3f) -> vec4f {
    var out = vec4(1., 1., 1., 0.);
    var depth = 0.;
    var p = ro + depth * rd;
    for (var i = 0; i < STEPS; i = i + 1){
        var density = fbm(p) - sdEllipse(p, vec3(COEF_A, COEF_B, COEF_C));
        if density > 0.0 {
            // var colour = vec4(mix(vec3(1.,1.,1.),vec3(0.,0.,0.), density),density);
            // colour = vec4( colour.rgb * colour.a, colour.a);
            // out = out + colour * (1. - out.a);
            out.a = out.a + (1. - out.a)/12.; // sqrt(density);
            if (out.a > 1){
                out.a = 1.;
            }
        }
        depth = depth + MARCH_SIZE;
        p = ro + depth * rd;
    }
    return out;
}

// NOTES: TRY parameterising each axis based on elliptical length ??? 
fn noise(pos: vec3f) -> f32 {
    return textureSample(blue_noise_texture, blue_noise_sampler, (pos.xy + 2.)/169.).r;
}

fn fbm(pos : vec3f) -> f32 {
    var q = pos; // + globals.time * vec3(1.,1.,1.);
    
    var f = 0.;
    var scale = 1.;
    var factor = 2.;
    for (var i = 0; i < 5; i = i + 1){
        f = f + scale * noise(q);
        q = q * factor;
        factor = factor + 5.6;
        scale = scale * 0.5;
    }

    return f;
}
