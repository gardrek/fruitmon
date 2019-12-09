#version 330

in vec3 v_color;

in vec3 v_normal;

out vec4 color;

uniform vec3 u_light;

vec3 gamma(vec3 color){
    return pow(color, vec3(1.0/2.0));
}

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color = vec3(0.50, 0.50, 0.50);
    vec3 color_rgb;

    // use a default color for bright color
    //~ vec3 regular_color = vec3(0.95, 0.90, 0.85);
    vec3 regular_color = vec3(1.0, 1.0, 1.0);
    color_rgb = mix(dark_color, regular_color, brightness);

    // use vertex color for bright color
    //~ color = vec4(mix(dark_color, v_color, brightness), 1.0);

    color = vec4(color_rgb, 1.0);
}

