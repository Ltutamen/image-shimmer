#version 420

layout(location = 0) in vec2 tex_coords;
layout(location = 0) out vec4 f_color;

uniform sampler2D image;
uniform sampler2D stripes;

void main() {

    vec4 colour = texture(image, tex_coords);
    float stripe = texture(stripes, vec2(tex_coords.x, 0)).x ;
    if(stripe > 0.5f) {
        f_color = texture(image, tex_coords);
    } else {
        discard;
    }
}