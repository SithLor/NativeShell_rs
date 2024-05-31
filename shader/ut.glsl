//https://www.shadertoy.com/view/43jXWt
//
//Shader Inputs
//uniform vec3      iResolution;           // viewport resolution (in pixels)
//uniform float     iTime;                 // shader playback time (in seconds)
//uniform float     iTimeDelta;            // render time (in seconds)
//uniform float     iFrameRate;            // shader frame rate
//uniform int       iFrame;                // shader playback frame
//uniform float     iChannelTime[4];       // channel playback time (in seconds)
//uniform vec3      iChannelResolution[4]; // channel resolution (in pixels)
//uniform vec4      iMouse;                // mouse pixel coords. xy: current (if MLB down), zw: click
//uniform samplerXX iChannel0..3;          // input channel. XX = 2D/Cube
//uniform vec4      iDate;                 // (year, month, day, time in seconds)
//uniform float     iSampleRate;           // sound sample rate (i.e., 44100)

//scale down the resolution to 1/4
vec4 scaledown_with_sampling(vec2 uv, float scale, float sampling)
{
    vec4 color = vec4(0.0);
    for (float i = 0.0; i < sampling; i++)
    {
        for (float j = 0.0; j < sampling; j++)
        {
            vec2 offset = vec2(i, j) / sampling;
            color += texture(iChannel0, (uv + offset) * scale);
        }
    }
    return color / (sampling * sampling);
}

void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;

    // Time varying pixel color
    vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));

    // Scale down the resolution with sampling
    vec4 scaled_color = scaledown_with_sampling(uv, 0.25, 4.0);

    // Output to screen
    fragColor = vec4(scaled_color.rgb,1.0);
}