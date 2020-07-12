

in vec3 ScreenPos_FS_in;

uniform vec2 frustum;

vec3 headlightEffect(vec3 rgb) {
    // NDC
    // x -1 to 1, left to right
    // y -1 to 1, bottom to top
    vec2 angPos = ScreenPos_FS_in.xy * frustum / 2.0;
    
}