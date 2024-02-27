use std::{any::Any, ffi::CString, ptr};

use egui::{ahash::HashMap, TextureFilter};
use gl::types::{GLchar, GLenum, GLint, GLuint};

use crate::{
    Hexgem::{
        core::Size,
        platform::HexgemWindow::{GlfwWindow, SdlWindow},
        window::get_window_struct,
    },
    Window,
};

const VS_SRC_150: &str = r#"
    #version 150
    uniform vec2 u_screen_size;
    in vec2 a_pos;
    in vec4 a_srgba; // 0-255 sRGB
    in vec2 a_tc;
    out vec4 v_rgba;
    out vec2 v_tc;

    // 0-1 linear  from  0-255 sRGB
    vec3 linear_from_srgb(vec3 srgb) {
        bvec3 cutoff = lessThan(srgb, vec3(10.31475));
        vec3 lower = srgb / vec3(3294.6);
        vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
        return mix(higher, lower, cutoff);
    }

    vec4 linear_from_srgba(vec4 srgba) {
        return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
    }

    void main() {
        gl_Position = vec4(
            2.0 * a_pos.x / u_screen_size.x - 1.0,
            1.0 - 2.0 * a_pos.y / u_screen_size.y,
            0.0,
            1.0);
        v_rgba = linear_from_srgba(a_srgba);
        v_tc = a_tc;
    }
"#;

const FS_SRC_150: &str = r#"
    #version 150
    uniform sampler2D u_sampler;
    in vec4 v_rgba;
    in vec2 v_tc;
    out vec4 f_color;

    // 0-255 sRGB  from  0-1 linear
    vec3 srgb_from_linear(vec3 rgb) {
        bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
        vec3 lower = rgb * vec3(3294.6);
        vec3 higher = vec3(269.025) * pow(rgb, vec3(1.0 / 2.4)) - vec3(14.025);
        return mix(higher, lower, vec3(cutoff));
    }

    vec4 srgba_from_linear(vec4 rgba) {
        return vec4(srgb_from_linear(rgba.rgb), 255.0 * rgba.a);
    }

    vec3 linear_from_srgb(vec3 srgb) {
        bvec3 cutoff = lessThan(srgb, vec3(10.31475));
        vec3 lower = srgb / vec3(3294.6);
        vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
        return mix(higher, lower, vec3(cutoff));
    }

    vec4 linear_from_srgba(vec4 srgba) {
        return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
    }

    void main() {
        // Need to convert from SRGBA to linear.
        vec4 texture_rgba = linear_from_srgba(texture(u_sampler, v_tc) * 255.0);
        f_color = v_rgba * texture_rgba;
    }
"#;

// VS_SRC and FS_SRC shaders taken from egui_glow crate.
const VS_SRC: &str = r#"
#if !defined(GL_ES) && __VERSION__ >= 140
#define I in
#define O out
#define V(x) x
#else
#define I attribute
#define O varying
#define V(x) vec3(x)
#endif

#ifdef GL_ES
precision mediump float;
#endif
uniform vec2 u_screen_size;
I vec2 a_pos;
I vec4 a_srgba; // 0-255 sRGB
I vec2 a_tc;
O vec4 v_rgba;
O vec2 v_tc;

// 0-1 linear  from  0-255 sRGB
vec3 linear_from_srgb(vec3 srgb) {
  bvec3 cutoff = lessThan(srgb, vec3(10.31475));
  vec3 lower = srgb / vec3(3294.6);
  vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
  return mix(higher, lower, V(cutoff));
}

vec4 linear_from_srgba(vec4 srgba) {
  return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
}

void main() {
  gl_Position = vec4(2.0 * a_pos.x / u_screen_size.x - 1.0, 1.0 - 2.0 * a_pos.y / u_screen_size.y, 0.0, 1.0);
  // egui encodes vertex colors in gamma spaces, so we must decode the colors here:
  v_rgba = linear_from_srgba(a_srgba);
  v_tc = a_tc;
}
"#;

const FS_SRC: &str = r#"
#ifdef GL_ES
precision mediump float;
#endif

uniform sampler2D u_sampler;
#if defined(GL_ES) || __VERSION__ < 140
varying vec4 v_rgba;
varying vec2 v_tc;
#else
in vec4 v_rgba;
in vec2 v_tc;
out vec4 f_color;
#endif

#ifdef GL_ES
// 0-255 sRGB  from  0-1 linear
vec3 srgb_from_linear(vec3 rgb) {
  bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
  vec3 lower = rgb * vec3(3294.6);
  vec3 higher = vec3(269.025) * pow(rgb, vec3(1.0 / 2.4)) - vec3(14.025);
  return mix(higher, lower, vec3(cutoff));
}

vec4 srgba_from_linear(vec4 rgba) {
  return vec4(srgb_from_linear(rgba.rgb), 255.0 * rgba.a);
}

#if __VERSION__ < 300
// 0-1 linear  from  0-255 sRGB
vec3 linear_from_srgb(vec3 srgb) {
  bvec3 cutoff = lessThan(srgb, vec3(10.31475));
  vec3 lower = srgb / vec3(3294.6);
  vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
  return mix(higher, lower, vec3(cutoff));
}

vec4 linear_from_srgba(vec4 srgba) {
  return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
}
#endif
#endif

#ifdef GL_ES
void main() {
#if __VERSION__ < 300
  // We must decode the colors, since WebGL doesn't come with sRGBA textures:
  vec4 texture_rgba = linear_from_srgba(texture2D(u_sampler, v_tc) * 255.0);
#else
  // The texture is set up with `SRGB8_ALPHA8`, so no need to decode here!
  vec4 texture_rgba = texture2D(u_sampler, v_tc);
#endif

  /// Multiply vertex color with texture color (in linear space).
  gl_FragColor = v_rgba * texture_rgba;

  // We must gamma-encode again since WebGL doesn't support linear blending in the framebuffer.
  gl_FragColor = srgba_from_linear(v_rgba * texture_rgba) / 255.0;

  // WebGL doesn't support linear blending in the framebuffer,
  // so we apply this hack to at least get a bit closer to the desired blending:
  gl_FragColor.a = pow(gl_FragColor.a, 1.6); // Empiric nonsense
}
#else
void main() {
  // The texture sampler is sRGB aware, and OpenGL already expects linear rgba output
  // so no need for any sRGB conversions here:
#if __VERSION__ < 140
  gl_FragColor = v_rgba * texture2D(u_sampler, v_tc);
#else
  f_color = v_rgba * texture(u_sampler, v_tc);
#endif
}
#endif
"#;

pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                std::str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    let program = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
    }

    let mut status = gl::FALSE as GLint;
    unsafe {
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
    }

    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        unsafe {
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buf = vec![0; len as usize];

        unsafe {
            gl::GetProgramInfoLog(
                program,
                len,
                core::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }

        panic!(
            "{}",
            core::str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8")
        );
    }

    program
}
struct Texture {
    size: Size<usize>,
    pixels: Vec<u8>,
    gl_id: Option<GLuint>,
    filtering: TextureFilter,
    dirty: bool,
}
impl Default for Texture {
    fn default() -> Self {
        Self {
            size: Size {
                width: 0,
                height: 0,
            },
            pixels: Default::default(),
            gl_id: Default::default(),
            filtering: TextureFilter::Nearest,
            dirty: Default::default(),
        }
    }
}
pub struct Painter {
    vertex_array: GLuint,
    program: GLuint,
    index_buffer: GLuint,
    pos_buffer: GLuint,
    tc_buffer: GLuint,
    color_buffer: GLuint,
    textures: HashMap<egui::TextureId, Texture>,
    pixels_per_point: f32,
    canvas_size: Size<u32>,
}

impl Painter {
    pub fn new(window: Box<&mut dyn Window>) -> Self {
        let (h, w) = (window.get_height(), window.get_width());
        let vert_shader: u32;
        let frag_shader: u32;
        let pixels_per_point: f32;
        #[cfg(not(target_os = "macos"))]
        {
            let os_window: &GlfwWindow = get_window_struct(window);
            pixels_per_point = os_window.window.get_content_scale().0;
            vert_shader = compile_shader(VS_SRC, gl::VERTEX_SHADER);
            frag_shader = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        };
        #[cfg(target_os = "macos")]
        {
            let macos_window: &SdlWindow = get_window_struct(window);
            let display_dpi = macos_window
                .video_subsystem
                .display_dpi(0)
                .unwrap_or((96., 96., 96.));
            let dpi_scale = 96. / display_dpi.0;
            let normalized_scale = 1. / dpi_scale;
            pixels_per_point = normalized_scale * dpi_scale;
            gl::load_with(|i| macos_window.video_subsystem.gl_get_proc_address(i) as *const _);
            vert_shader = compile_shader(VS_SRC_150, gl::VERTEX_SHADER);
            frag_shader = compile_shader(FS_SRC_150, gl::FRAGMENT_SHADER);
        };
        let program = link_program(vert_shader, frag_shader);
        let mut vertex_array = 0;
        let mut index_buffer = 0;
        let mut pos_buffer = 0;
        let mut tc_buffer = 0;
        let mut color_buffer = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vertex_array);
            gl::BindVertexArray(vertex_array);
            gl::GenBuffers(1, &mut index_buffer);
            gl::GenBuffers(1, &mut pos_buffer);
            gl::GenBuffers(1, &mut tc_buffer);
            gl::GenBuffers(1, &mut color_buffer);
        };
        Painter {
            vertex_array,
            program,
            index_buffer,
            pos_buffer,
            tc_buffer,
            color_buffer,
            textures: Default::default(),
            canvas_size: Size {
                width: w as u32,
                height: h as u32,
            },
            pixels_per_point,
        }
    }

    pub fn paint_and_update(&mut self, output: egui::FullOutput) {}
    pub fn set_pixels_per_point(&mut self, pixels_per_point: f32) {
        self.pixels_per_point = pixels_per_point;
    }
    pub fn set_size(&mut self, size: Size<u32>) {
        self.canvas_size = size;
    }
}
