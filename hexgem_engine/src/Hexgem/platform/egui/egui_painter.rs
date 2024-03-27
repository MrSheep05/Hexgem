use std::ffi::{c_void, CString};

use egui::{
    ahash::HashMap,
    epaint::{ImageDelta, Primitive},
    Mesh, Rect, TextureFilter, TextureId,
};
use gl::types::{GLchar, GLenum, GLint, GLsizeiptr, GLuint};
use log::info;

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

const VS_SRC_100: &str = r#"
#version 100

uniform vec2 u_screen_size;

attribute vec2 a_pos;
attribute vec2 a_tc;
attribute vec4 a_srgba;

varying vec2 v_tc;
varying vec4 v_rgba;

// 0-1 linear  from  0-255 sRGB
vec3 linear_from_srgb(vec3 srgb) {
    bvec3 cutoff = lessThan(srgb, vec3(10.31475));
    vec3 lower = srgb / vec3(3294.6);
    vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
    return mix(higher, lower, vec3(cutoff));


}

// 0-1 linear  from  0-255 sRGBA
vec4 linear_from_srgba(vec4 srgba) {
    return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
}

// 0-255 sRGB  from  0-1 linear
vec3 srgb_from_linear(vec3 rgb) {
    bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
    vec3 lower = rgb * vec3(3294.6);
    vec3 higher = vec3(269.025) * pow(rgb, vec3(1.0 / 2.4)) - vec3(14.025);
    return mix(higher, lower, vec3(cutoff));
}

// 0-255 sRGBA  from  0-1 linear
vec4 srgba_from_linear(vec4 rgba) {
    return vec4(srgb_from_linear(rgba.rgb), 255.0 * rgba.a);
}

void main() {
    gl_Position = vec4(
        2.0 * a_pos.x / u_screen_size.x - 1.0,
        1.0 - 2.0 * a_pos.y / u_screen_size.y,
        0.0,
    1.0);
    v_tc = a_tc;
    v_rgba = linear_from_srgba(a_srgba);
    v_rgba.a = pow(v_rgba.a, 1.6);
}
"#;

const FS_SRC_100: &str = r#"
#version 100

uniform sampler2D u_sampler;

precision highp float;

varying vec2 v_tc;
varying vec4 v_rgba;

// 0-1 linear  from  0-255 sRGB
vec3 linear_from_srgb(vec3 srgb) {
    bvec3 cutoff = lessThan(srgb, vec3(10.31475));
    vec3 lower = srgb / vec3(3294.6);
    vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
    return mix(higher, lower, vec3(cutoff));
}

// 0-1 linear  from  0-255 sRGBA
vec4 linear_from_srgba(vec4 srgba) {
    return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
}

// 0-255 sRGB  from  0-1 linear
vec3 srgb_from_linear(vec3 rgb) {
    bvec3 cutoff = lessThan(rgb, vec3(0.0031308));
    vec3 lower = rgb * vec3(3294.6);
    vec3 higher = vec3(269.025) * pow(rgb, vec3(1.0 / 2.4)) - vec3(14.025);
    return mix(higher, lower, vec3(cutoff));
}

// 0-255 sRGBA  from  0-1 linear
vec4 srgba_from_linear(vec4 rgba) {
    return vec4(srgb_from_linear(rgba.rgb), 255.0 * rgba.a);
}

void main() {
    // We must decode the colors, since WebGL1 doesn't come with sRGBA textures:
    vec4 texture_rgba = linear_from_srgba(texture2D(u_sampler, v_tc) * 255.0);
    // WebGL1 doesn't support linear blending in the framebuffer,
    // so we do a hack here where we change the premultiplied alpha
    // to do the multiplication in gamma space instead:
    // Unmultiply alpha:
    if (texture_rgba.a > 0.0) {
        texture_rgba.rgb /= texture_rgba.a;
    }
    // Empiric tweak to make e.g. shadows look more like they should:
    texture_rgba.a *= sqrt(texture_rgba.a);
    // To gamma:
    texture_rgba = srgba_from_linear(texture_rgba) / 255.0;
    // Premultiply alpha, this time in gamma space:
    if (texture_rgba.a > 0.0) {
        texture_rgba.rgb *= texture_rgba.a;
    }
    /// Multiply vertex color with texture color (in linear space).
    gl_FragColor = v_rgba * texture_rgba;
}
"#;

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(ty) };

    let c_str = CString::new(src.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), core::ptr::null());
        gl::CompileShader(shader);
    }

    let mut status = gl::FALSE as GLint;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    }

    if status != (gl::TRUE as GLint) {
        let mut len = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buf = vec![0; len as usize];

        unsafe {
            gl::GetShaderInfoLog(
                shader,
                len,
                core::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }

        let error = core::str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8");
        panic!("Error {error}");
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
        let error = core::str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8");
        panic!("{error}");
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
impl Texture {
    fn delete(&self) {
        if let Some(id) = &self.gl_id {
            unsafe {
                gl::DeleteTextures(1, id as *const _);
            }
        }
    }

    fn update_texture_part(
        &mut self,
        x_offset: i32,
        y_offset: i32,
        width: i32,
        height: i32,
        bytes: &[u8],
    ) {
        assert!(x_offset + width <= self.size.width as _);
        assert!(y_offset + height <= self.size.height as _);

        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_SWIZZLE_A, gl::RED as _);

            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                x_offset as _,
                y_offset as _,
                width as _,
                height as _,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                bytes.as_ptr() as *const _,
            );
        }

        self.dirty = true;
    }
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
    pub fn new(window: &Box<dyn Window>) -> Self {
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
            unsafe {
                macos_window
                    .window
                    .gl_make_current(&macos_window.gl_context)
                    .expect("Cannot reattach context");
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            }

            let display_dpi = macos_window
                .video_subsystem
                .display_dpi(0)
                .unwrap_or((96., 96., 96.));
            let dpi_scale = 96. / display_dpi.0;
            let normalized_scale = 1. / dpi_scale;
            pixels_per_point = normalized_scale * dpi_scale;

            vert_shader = compile_shader(VS_SRC_100, gl::VERTEX_SHADER);
            frag_shader = compile_shader(FS_SRC_100, gl::FRAGMENT_SHADER);
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

    pub fn paint_and_update(&mut self, output: egui::FullOutput, context: &egui::Context) {
        for (id, img_delta) in &output.textures_delta.set {
            self.set_texture(id, img_delta);
        }
        self.set_pixels_per_point(output.pixels_per_point);
        let primitives = context.tessellate(output.shapes, self.pixels_per_point);
        self.paint_primitives(self.pixels_per_point, &primitives);
    }

    fn paint_primitives(
        &mut self,
        pixels_per_point: f32,
        clipped_primitives: &[egui::ClippedPrimitive],
    ) {
        self.upload_user_textures();

        unsafe {
            gl::Enable(gl::FRAMEBUFFER_SRGB);

            gl::Enable(gl::SCISSOR_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA); // premultiplied alpha
            gl::UseProgram(self.program);
            gl::ActiveTexture(gl::TEXTURE0);
        }

        let u_screen_size = CString::new("u_screen_size").unwrap();
        let u_screen_size_ptr = u_screen_size.as_ptr();
        let u_screen_size_loc = unsafe { gl::GetUniformLocation(self.program, u_screen_size_ptr) };
        let screen_size_pixels = egui::vec2(
            self.canvas_size.width as f32,
            self.canvas_size.height as f32,
        );
        let screen_size_points = screen_size_pixels / pixels_per_point;

        unsafe {
            gl::Uniform2f(
                u_screen_size_loc,
                screen_size_points.x,
                screen_size_points.y,
            );
        }

        let u_sampler = CString::new("u_sampler").unwrap();
        let u_sampler_ptr = u_sampler.as_ptr();
        let u_sampler_loc = unsafe { gl::GetUniformLocation(self.program, u_sampler_ptr) };
        unsafe {
            gl::Uniform1i(u_sampler_loc, 0);
            gl::Viewport(
                0,
                0,
                self.canvas_size.width as i32,
                self.canvas_size.height as i32,
            );
        }

        for egui::ClippedPrimitive {
            clip_rect,
            primitive,
        } in clipped_primitives
        {
            match primitive {
                Primitive::Mesh(mesh) => {
                    self.paint_mesh(mesh, clip_rect, pixels_per_point);
                    unsafe {
                        gl::Disable(gl::SCISSOR_TEST);
                    }
                }

                Primitive::Callback(_) => {
                    panic!("Custom rendering callbacks are not implemented in egui_glium");
                }
            }
        }

        unsafe {
            gl::Disable(gl::FRAMEBUFFER_SRGB);
        }
    }

    fn paint_mesh(&self, mesh: &Mesh, clip_rect: &Rect, pixels_per_point: f32) {
        debug_assert!(mesh.is_valid());

        if let Some(it) = self.textures.get(&mesh.texture_id) {
            unsafe {
                gl::BindTexture(
                    gl::TEXTURE_2D,
                    it.gl_id.expect("Texture should have a valid OpenGL id now"),
                );
            }

            let screen_size_pixels = egui::vec2(
                self.canvas_size.width as f32,
                self.canvas_size.height as f32,
            );

            let clip_min_x = pixels_per_point * clip_rect.min.x;
            let clip_min_y = pixels_per_point * clip_rect.min.y;
            let clip_max_x = pixels_per_point * clip_rect.max.x;
            let clip_max_y = pixels_per_point * clip_rect.max.y;
            let clip_min_x = clip_min_x.clamp(0.0, screen_size_pixels.x);
            let clip_min_y = clip_min_y.clamp(0.0, screen_size_pixels.y);
            let clip_max_x = clip_max_x.clamp(clip_min_x, screen_size_pixels.x);
            let clip_max_y = clip_max_y.clamp(clip_min_y, screen_size_pixels.y);
            let clip_min_x = clip_min_x.round() as i32;
            let clip_min_y = clip_min_y.round() as i32;
            let clip_max_x = clip_max_x.round() as i32;
            let clip_max_y = clip_max_y.round() as i32;

            //scissor Y coordinate is from the bottom
            unsafe {
                gl::Scissor(
                    clip_min_x,
                    self.canvas_size.height as i32 - clip_max_y,
                    clip_max_x - clip_min_x,
                    clip_max_y - clip_min_y,
                );
            }

            let indices: Vec<u16> = mesh.indices.iter().map(move |idx| *idx as u16).collect();
            let indices_len = indices.len();
            let vertices_len = mesh.vertices.len();

            unsafe {
                gl::BindVertexArray(self.vertex_array);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (indices_len * core::mem::size_of::<u16>()) as GLsizeiptr,
                    indices.as_ptr() as *const gl::types::GLvoid,
                    gl::STREAM_DRAW,
                );
            }

            let mut positions: Vec<f32> = Vec::with_capacity(2 * vertices_len);
            let mut tex_coords: Vec<f32> = Vec::with_capacity(2 * vertices_len);
            let mut colors: Vec<u8> = Vec::with_capacity(4 * vertices_len);
            for v in &mesh.vertices {
                positions.push(v.pos.x);
                positions.push(v.pos.y);

                tex_coords.push(v.uv.x);
                tex_coords.push(v.uv.y);

                colors.push(v.color[0]);
                colors.push(v.color[1]);
                colors.push(v.color[2]);
                colors.push(v.color[3]);
            }

            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, self.pos_buffer);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (positions.len() * core::mem::size_of::<f32>()) as GLsizeiptr,
                    positions.as_ptr() as *const gl::types::GLvoid,
                    gl::STREAM_DRAW,
                );
            }

            let a_pos = CString::new("a_pos").unwrap();
            let a_pos_ptr = a_pos.as_ptr();
            let a_pos_loc = unsafe { gl::GetAttribLocation(self.program, a_pos_ptr) };
            assert!(a_pos_loc >= 0);
            let a_pos_loc = a_pos_loc as u32;

            let stride = 0;
            unsafe {
                gl::VertexAttribPointer(
                    a_pos_loc,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    core::ptr::null(),
                );
                gl::EnableVertexAttribArray(a_pos_loc);

                gl::BindBuffer(gl::ARRAY_BUFFER, self.tc_buffer);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (tex_coords.len() * core::mem::size_of::<f32>()) as GLsizeiptr,
                    tex_coords.as_ptr() as *const gl::types::GLvoid,
                    gl::STREAM_DRAW,
                );
            }

            let a_tc = CString::new("a_tc").unwrap();
            let a_tc_ptr = a_tc.as_ptr();
            let a_tc_loc = unsafe { gl::GetAttribLocation(self.program, a_tc_ptr) };
            assert!(a_tc_loc >= 0);
            let a_tc_loc = a_tc_loc as u32;

            let stride = 0;
            unsafe {
                gl::VertexAttribPointer(
                    a_tc_loc,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    core::ptr::null(),
                );
                gl::EnableVertexAttribArray(a_tc_loc);

                gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (colors.len() * core::mem::size_of::<u8>()) as GLsizeiptr,
                    colors.as_ptr() as *const gl::types::GLvoid,
                    gl::STREAM_DRAW,
                );
            }

            let a_srgba = CString::new("a_srgba").unwrap();
            let a_srgba_ptr = a_srgba.as_ptr();
            let a_srgba_loc = unsafe { gl::GetAttribLocation(self.program, a_srgba_ptr) };
            assert!(a_srgba_loc >= 0);
            let a_srgba_loc = a_srgba_loc as u32;

            let stride = 0;
            unsafe {
                gl::VertexAttribPointer(
                    a_srgba_loc,
                    4,
                    gl::UNSIGNED_BYTE,
                    gl::FALSE,
                    stride,
                    core::ptr::null(),
                );
                gl::EnableVertexAttribArray(a_srgba_loc);

                gl::DrawElements(
                    gl::TRIANGLES,
                    indices_len as i32,
                    gl::UNSIGNED_SHORT,
                    core::ptr::null(),
                );
                gl::DisableVertexAttribArray(a_pos_loc);
                gl::DisableVertexAttribArray(a_tc_loc);
                gl::DisableVertexAttribArray(a_srgba_loc);
            }
        }
    }

    fn set_texture(&mut self, id: &TextureId, delta: &ImageDelta) {
        let [w, h] = delta.image.size();

        if let Some([x, y]) = delta.pos {
            if let Some(texture) = self.textures.get_mut(&id) {
                match &delta.image {
                    egui::ImageData::Color(image) => {
                        assert_eq!(
                            image.width() * image.height(),
                            image.pixels.len(),
                            "Mismatch between texture size and texel count"
                        );

                        let data: Vec<u8> =
                            image.pixels.iter().flat_map(|a| a.to_array()).collect();

                        texture.update_texture_part(x as _, y as _, w as _, h as _, &data);
                    }

                    egui::ImageData::Font(image) => {
                        assert_eq!(
                            image.width() * image.height(),
                            image.pixels.len(),
                            "Mismatch between texture size and texel count"
                        );

                        let gamma = 1.0;
                        let data: Vec<u8> = image
                            .srgba_pixels(Some(gamma))
                            .flat_map(|a| a.to_array())
                            .collect();

                        texture.update_texture_part(x as _, y as _, w as _, h as _, &data);
                    }
                }
            } else {
                eprintln!("Failed to find egui texture {:?}", id);
            }
        } else {
            let texture = match &delta.image {
                egui::ImageData::Color(image) => {
                    assert_eq!(
                        image.width() * image.height(),
                        image.pixels.len(),
                        "Mismatch between texture size and texel count"
                    );

                    let pixels = image.pixels.iter().flat_map(|a| a.to_array()).collect();

                    Texture {
                        size: Size {
                            width: w,
                            height: h,
                        },
                        pixels,
                        gl_id: None,
                        filtering: TextureFilter::Linear,
                        dirty: true,
                    }
                }
                egui::ImageData::Font(image) => {
                    assert_eq!(
                        image.width() * image.height(),
                        image.pixels.len(),
                        "Mismatch between texture size and texel count"
                    );

                    let gamma = 1.0;
                    let pixels = image
                        .srgba_pixels(Some(gamma))
                        .flat_map(|a| a.to_array())
                        .collect();

                    Texture {
                        size: Size {
                            width: w,
                            height: h,
                        },
                        pixels,
                        gl_id: None,
                        filtering: TextureFilter::Linear,
                        dirty: true,
                    }
                }
            };

            let previous = self.textures.insert(id.clone(), texture);
            if let Some(previous) = previous {
                previous.delete();
            }
        }
    }

    fn upload_user_textures(&mut self) {
        self.textures
            .values_mut()
            .filter(|user_texture| user_texture.gl_id.is_none() || user_texture.dirty)
            .for_each(|user_texture| {
                let pixels = std::mem::take(&mut user_texture.pixels);

                match user_texture.gl_id {
                    Some(texture) => unsafe {
                        gl::BindTexture(gl::TEXTURE_2D, texture);
                    },

                    None => {
                        let mut gl_texture = 0;
                        unsafe {
                            gl::GenTextures(1, &mut gl_texture);
                            gl::BindTexture(gl::TEXTURE_2D, gl_texture);
                            gl::TexParameteri(
                                gl::TEXTURE_2D,
                                gl::TEXTURE_WRAP_S,
                                gl::CLAMP_TO_EDGE as i32,
                            );
                            gl::TexParameteri(
                                gl::TEXTURE_2D,
                                gl::TEXTURE_WRAP_T,
                                gl::CLAMP_TO_EDGE as i32,
                            );
                        }

                        match user_texture.filtering {
                            TextureFilter::Nearest => unsafe {
                                gl::TexParameteri(
                                    gl::TEXTURE_2D,
                                    gl::TEXTURE_MIN_FILTER,
                                    gl::LINEAR as i32,
                                );
                                gl::TexParameteri(
                                    gl::TEXTURE_2D,
                                    gl::TEXTURE_MAG_FILTER,
                                    gl::LINEAR as i32,
                                );
                            },

                            TextureFilter::Linear => unsafe {
                                gl::TexParameteri(
                                    gl::TEXTURE_2D,
                                    gl::TEXTURE_MIN_FILTER,
                                    gl::NEAREST as i32,
                                );
                                gl::TexParameteri(
                                    gl::TEXTURE_2D,
                                    gl::TEXTURE_MAG_FILTER,
                                    gl::NEAREST as i32,
                                );
                            },
                        }
                        user_texture.gl_id = Some(gl_texture);
                    }
                }

                if !pixels.is_empty() {
                    let level = 0;
                    let internal_format = gl::RGBA;
                    let border = 0;
                    let src_format = gl::RGBA;
                    let src_type = gl::UNSIGNED_BYTE;
                    unsafe {
                        gl::TexImage2D(
                            gl::TEXTURE_2D,
                            level,
                            internal_format as i32,
                            user_texture.size.width as i32,
                            user_texture.size.height as i32,
                            border,
                            src_format,
                            src_type,
                            pixels.as_ptr() as *const c_void,
                        );
                    }
                }

                user_texture.dirty = false;
            });
    }

    pub fn set_pixels_per_point(&mut self, pixels_per_point: f32) {
        self.pixels_per_point = pixels_per_point;
    }
    pub fn set_size(&mut self, size: Size<u32>) {
        self.canvas_size = size;
    }
    pub fn get_scale(&self) -> f32 {
        self.pixels_per_point
    }
}
