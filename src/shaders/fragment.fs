#version 420

layout(location = 0) in vec2 tex_coords;
layout(location = 0) out vec4 f_color;

uniform sampler2D image;
//  'stripes' is used to pass strpies transparency data:
uniform sampler2D stripes;

void main() {

    vec4 colour = texture(image, tex_coords);
    float transparency = texture(stripes, vec2(tex_coords.x, 0)).x;
    f_color = transparency * texture(image, tex_coords);
}