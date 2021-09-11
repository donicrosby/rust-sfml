/* automatically generated by rust-bindgen 0.59.1 */
#![allow(
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    trivial_casts,
    deref_nullptr
)]

use std::{
    ffi::c_void,
    os::raw::{c_char, c_uint},
};

// Bindgen autogenerated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Manual bindings

macro_rules! decl_opaque {
    ($($name:ident)+) => {
        $(
            #[repr(C)]
            pub struct $name {
                _opaque: [u8; 0],
            }
        )+
    };
}

decl_opaque! {
    sfClock
    sfStdString
}

/// Enumeration of the blending factors.
///
/// The factors are mapped directly to their OpenGL equivalents, specified by
/// `glBlendFunc()` or `glBlendFuncSeparate()`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendFactor {
    /// (0, 0, 0, 0)
    Zero,
    /// (1, 1, 1, 1)
    One,
    /// (src.r, src.g, src.b, src.a)
    SrcColor,
    /// (1, 1, 1, 1) - (src.r, src.g, src.b, src.a)
    OneMinusSrcColor,
    /// (dst.r, dst.g, dst.b, dst.a)
    DstColor,
    /// (1, 1, 1, 1) - (dst.r, dst.g, dst.b, dst.a)
    OneMinusDstColor,
    /// (src.a, src.a, src.a, src.a)
    SrcAlpha,
    /// (1, 1, 1, 1) - (src.a, src.a, src.a, src.a)
    OneMinusSrcAlpha,
    /// (dst.a, dst.a, dst.a, dst.a)
    DstAlpha,
    /// (1, 1, 1, 1) - (dst.a, dst.a, dst.a, dst.a)
    OneMinusDstAlpha,
}

/// Enumeration of the blending equations.
///
/// The equations are mapped directly to their OpenGL equivalents, specified by
/// `glBlendEquation()` or `glBlendEquationSeparate()`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlendEquation {
    /// Pixel = Src * SrcFactor + Dst * DstFactor.
    Add,
    /// Pixel = Src * SrcFactor - Dst * DstFactor.
    Subtract,
    /// Pixel = Dst * DstFactor - Src * SrcFactor.
    ReverseSubtract,
}

/// Blending modes for drawing.
///
/// `BlendMode` is a type that represents a blend mode.
///
/// A blend mode determines how the colors of an object you draw are mixed with the colors that
/// are already in the buffer.
///
/// The type is composed of 6 components
///
/// - Color Source Factor
/// - Color Destination Factor
/// - Color Blend Equation
/// - Alpha Source Factor
/// - Alpha Destination Factor
/// - Alpha Blend Equation
///
/// The source factor specifies how the pixel you are drawing contributes to the final color.
/// The destination factor specifies how the pixel already drawn in the buffer contributes to
/// the final color.
///
/// The color channels RGB (red, green, blue; simply referred to as color) and A
/// (alpha; the transparency) can be treated separately. This separation can be useful for
/// specific blend modes, but most often you won't need it and will simply treat the color as
/// a single unit.
///
/// The blend factors and equations correspond to their OpenGL equivalents.
/// In general, the color of the resulting pixel is calculated according to the following
/// formula (src is the color of the source pixel, dst the color of the destination pixel,
/// the other variables correspond to the public members, with the equations
/// being + or - operators):
///
/// ```ignore
/// dst.rgb = colorSrcFactor * src.rgb (colorEquation) colorDstFactor * dst.rgb
/// dst.a   = alphaSrcFactor * src.a   (alphaEquation) alphaDstFactor * dst.a
/// ```
///
/// All factors and colors are represented as floating point numbers between 0 and 1.
/// Where necessary, the result is clamped to fit in that range.
///
/// In SFML, a blend mode can be specified every time you draw a [`Drawable`] object to
/// a render target. It is part of the [`RenderStates`] compound that is passed to
/// [`RenderTarget::draw`].
///
/// [`Drawable`]: crate::graphics::Drawable
/// [`RenderStates`]: crate::graphics::RenderStates
/// [`RenderTarget::draw`]: crate::graphics::RenderTarget::draw
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlendMode {
    /// Source blending factor for the color channels
    pub color_src_factor: BlendFactor,
    /// Destination blending factor for the color channels
    pub color_dst_factor: BlendFactor,
    /// Blending equation for the color channels
    pub color_equation: BlendEquation,
    /// Source blending factor for the alpha channel
    pub alpha_src_factor: BlendFactor,
    /// Destination blending factor for the alpha channel
    pub alpha_dst_factor: BlendFactor,
    /// Blending equation for the alpha channel
    pub alpha_equation: BlendEquation,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct sfRenderStates {
    pub blendMode: BlendMode,
    pub transform: sfTransform,
    pub texture: *const sfTexture,
    pub shader: *const sfShader,
}

extern "C" {
    pub fn sfClipboard_getUnicodeString() -> *const sfUint32;
    pub fn sfClipboard_setUnicodeString(text: *const sfUint32);

    pub fn sfClock_create() -> *mut sfClock;
    pub fn sfClock_copy(clock: *const sfClock) -> *mut sfClock;
    pub fn sfClock_destroy(clock: *mut sfClock);
    pub fn sfClock_getElapsedTime(clock: *const sfClock) -> sfTime;
    pub fn sfClock_restart(clock: *mut sfClock) -> sfTime;

    pub fn sfSleep(duration: sfTime);

    pub fn sfTexture_new() -> *mut sfTexture;
    pub fn sfTexture_create(tex: *mut sfTexture, width: c_uint, height: c_uint) -> sfBool;
    pub fn sfTexture_loadFromFile(
        tex: *mut sfTexture,
        filename: *const c_char,
        area: sfIntRect,
    ) -> sfBool;
    pub fn sfTexture_loadFromMemory(
        tex: *mut sfTexture,
        data: *const c_void,
        size: usize,
        area: sfIntRect,
    ) -> sfBool;
    pub fn sfTexture_loadFromStream(
        tex: *mut sfTexture,
        stream: *mut sfInputStream,
        area: sfIntRect,
    ) -> sfBool;
    pub fn sfTexture_loadFromImage(
        tex: *mut sfTexture,
        image: *const sfImage,
        area: sfIntRect,
    ) -> sfBool;
    pub fn sfSoundBufferRecorder_create() -> *mut sfSoundBufferRecorder;
    pub fn sfSoundBufferRecorder_destroy(bufRec: *mut sfSoundBufferRecorder);
    pub fn sfSoundBufferRecorder_start(
        bufRec: *mut sfSoundBufferRecorder,
        sampRate: c_uint,
    ) -> sfBool;
    pub fn sfSoundBufferRecorder_stop(bufRec: *mut sfSoundBufferRecorder);
    pub fn sfSoundBufferRecorder_getSampleRate(bufRec: *const sfSoundBufferRecorder) -> c_uint;
    pub fn sfSoundBufferRecorder_getBuffer(
        bufRec: *const sfSoundBufferRecorder,
    ) -> *const sfSoundBuffer;
    pub fn sfSoundBufferRecorder_setDevice(
        bufRec: *mut sfSoundBufferRecorder,
        name: *const c_char,
    ) -> sfBool;
    pub fn sfSoundBufferRecorder_getDevice(
        bufRec: *const sfSoundBufferRecorder,
    ) -> *const sfStdString;
    pub fn sfStdString_getLength(s: *const sfStdString) -> usize;
    pub fn sfStdString_getData(s: *const sfStdString) -> *const c_char;
    pub fn sfCircleShape_getTransform(shape: *const sfCircleShape) -> sfTransform;
    pub fn sfCircleShape_getInverseTransform(shape: *const sfCircleShape) -> sfTransform;
    pub fn sfShape_getTransform(shape: *const sfShape) -> sfTransform;
    pub fn sfShape_getInverseTransform(shape: *const sfShape) -> sfTransform;
    pub fn sfRectangleShape_getTransform(shape: *const sfRectangleShape) -> sfTransform;
    pub fn sfRectangleShape_getInverseTransform(shape: *const sfRectangleShape) -> sfTransform;
    pub fn sfConvexShape_getTransform(shape: *const sfConvexShape) -> sfTransform;
    pub fn sfConvexShape_getInverseTransform(shape: *const sfConvexShape) -> sfTransform;
    pub fn sfSprite_getTransform(shape: *const sfSprite) -> sfTransform;
    pub fn sfSprite_getInverseTransform(shape: *const sfSprite) -> sfTransform;
    pub fn sfText_getTransform(shape: *const sfText) -> sfTransform;
    pub fn sfText_getInverseTransform(shape: *const sfText) -> sfTransform;
    pub fn sfTransform_fromMatrix(
        a00: f32,
        a01: f32,
        a02: f32,
        a10: f32,
        a11: f32,
        a12: f32,
        a20: f32,
        a21: f32,
        a22: f32,
    ) -> sfTransform;
    pub fn sfTransform_getMatrix(tf: *const sfTransform) -> *const f32;
    pub fn sfTransform_getInverse(tf: *const sfTransform) -> sfTransform;
    pub fn sfTransform_transformPoint(tf: *const sfTransform, point: sfVector2f) -> sfVector2f;
    pub fn sfTransform_transformRect(tf: *const sfTransform, rect: sfFloatRect) -> sfFloatRect;
    pub fn sfTransform_combine(tf: *mut sfTransform, other: *const sfTransform);
    pub fn sfTransform_translate(tf: *mut sfTransform, x: f32, y: f32);
    pub fn sfTransform_rotate(tf: *mut sfTransform, angle: f32);
    pub fn sfTransform_rotateWithCenter(tf: *mut sfTransform, angle: f32, cx: f32, cy: f32);
    pub fn sfTransform_scale(tf: *mut sfTransform, x: f32, y: f32);
    pub fn sfTransform_scaleWithCenter(tf: *mut sfTransform, x: f32, y: f32, cx: f32, cy: f32);
    pub fn sfRenderWindow_drawSprite(
        rw: *mut sfRenderWindow,
        object: *const sfSprite,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawText(
        rw: *mut sfRenderWindow,
        object: *const sfText,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawShape(
        rw: *mut sfRenderWindow,
        object: *const sfShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawCircleShape(
        rw: *mut sfRenderWindow,
        object: *const sfCircleShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawConvexShape(
        rw: *mut sfRenderWindow,
        object: *const sfConvexShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawRectangleShape(
        rw: *mut sfRenderWindow,
        object: *const sfRectangleShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawVertexBuffer(
        rw: *mut sfRenderWindow,
        object: *const sfVertexBuffer,
        states: *const sfRenderStates,
    );
    pub fn sfRenderWindow_drawPrimitives(
        rw: *mut sfRenderWindow,
        vertices: *const sfVertex,
        vertexCount: usize,
        type_: sfPrimitiveType,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawSprite(
        rt: *mut sfRenderTexture,
        object: *const sfSprite,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawText(
        rt: *mut sfRenderTexture,
        object: *const sfText,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawShape(
        rt: *mut sfRenderTexture,
        object: *const sfShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawCircleShape(
        rt: *mut sfRenderTexture,
        object: *const sfCircleShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawConvexShape(
        rt: *mut sfRenderTexture,
        object: *const sfConvexShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawRectangleShape(
        rt: *mut sfRenderTexture,
        object: *const sfRectangleShape,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawVertexBuffer(
        rt: *mut sfRenderTexture,
        object: *const sfVertexBuffer,
        states: *const sfRenderStates,
    );
    pub fn sfRenderTexture_drawPrimitives(
        rt: *mut sfRenderTexture,
        vertices: *const sfVertex,
        vertex_count: usize,
        type_: sfPrimitiveType,
        states: *const sfRenderStates,
    );
}
