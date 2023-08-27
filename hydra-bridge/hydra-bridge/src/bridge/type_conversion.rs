//! Type Conversion from C++ to Rust.

use super::ffi;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RenderBufferId(pub(super) String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderBufferFormat {
    UNorm8,
    UNorm8Vec2,
    UNorm8Vec3,
    UNorm8Vec4,

    SNorm8,
    SNorm8Vec2,
    SNorm8Vec3,
    SNorm8Vec4,

    Float16,
    Float16Vec2,
    Float16Vec3,
    Float16Vec4,

    Float32,
    Float32Vec2,
    Float32Vec3,
    Float32Vec4,

    Int16,
    Int16Vec2,
    Int16Vec3,
    Int16Vec4,

    UInt16,
    UInt16Vec2,
    UInt16Vec3,
    UInt16Vec4,

    Int32,
    Int32Vec2,
    Int32Vec3,
    Int32Vec4,

    Float32UInt8,

    Invalid,
}
impl RenderBufferFormat {
    pub fn component_size(&self) -> usize {
        match self {
            RenderBufferFormat::UNorm8 => 1,
            RenderBufferFormat::UNorm8Vec2 => 2,
            RenderBufferFormat::UNorm8Vec3 => 3,
            RenderBufferFormat::UNorm8Vec4 => 4,

            RenderBufferFormat::SNorm8 => 1,
            RenderBufferFormat::SNorm8Vec2 => 2,
            RenderBufferFormat::SNorm8Vec3 => 3,
            RenderBufferFormat::SNorm8Vec4 => 4,

            RenderBufferFormat::Float16 => 2,
            RenderBufferFormat::Float16Vec2 => 4,
            RenderBufferFormat::Float16Vec3 => 6,
            RenderBufferFormat::Float16Vec4 => 8,

            RenderBufferFormat::Float32 => 4,
            RenderBufferFormat::Float32Vec2 => 8,
            RenderBufferFormat::Float32Vec3 => 12,
            RenderBufferFormat::Float32Vec4 => 16,

            RenderBufferFormat::Int16 => 2,
            RenderBufferFormat::Int16Vec2 => 4,
            RenderBufferFormat::Int16Vec3 => 6,
            RenderBufferFormat::Int16Vec4 => 8,

            RenderBufferFormat::UInt16 => 2,
            RenderBufferFormat::UInt16Vec2 => 4,
            RenderBufferFormat::UInt16Vec3 => 6,
            RenderBufferFormat::UInt16Vec4 => 8,

            RenderBufferFormat::Int32 => 4,
            RenderBufferFormat::Int32Vec2 => 8,
            RenderBufferFormat::Int32Vec3 => 12,
            RenderBufferFormat::Int32Vec4 => 16,

            RenderBufferFormat::Float32UInt8 => 5,

            RenderBufferFormat::Invalid => 0,
        }
    }
}
impl From<ffi::RenderBufferFormat> for RenderBufferFormat {
    fn from(format: ffi::RenderBufferFormat) -> Self {
        match format {
            ffi::RenderBufferFormat::UNorm8 => Self::UNorm8,
            ffi::RenderBufferFormat::UNorm8Vec2 => Self::UNorm8Vec2,
            ffi::RenderBufferFormat::UNorm8Vec3 => Self::UNorm8Vec3,
            ffi::RenderBufferFormat::UNorm8Vec4 => Self::UNorm8Vec4,

            ffi::RenderBufferFormat::SNorm8 => Self::SNorm8,
            ffi::RenderBufferFormat::SNorm8Vec2 => Self::SNorm8Vec2,
            ffi::RenderBufferFormat::SNorm8Vec3 => Self::SNorm8Vec3,
            ffi::RenderBufferFormat::SNorm8Vec4 => Self::SNorm8Vec4,

            ffi::RenderBufferFormat::Float16 => Self::Float16,
            ffi::RenderBufferFormat::Float16Vec2 => Self::Float16Vec2,
            ffi::RenderBufferFormat::Float16Vec3 => Self::Float16Vec3,
            ffi::RenderBufferFormat::Float16Vec4 => Self::Float16Vec4,

            ffi::RenderBufferFormat::Float32 => Self::Float32,
            ffi::RenderBufferFormat::Float32Vec2 => Self::Float32Vec2,
            ffi::RenderBufferFormat::Float32Vec3 => Self::Float32Vec3,
            ffi::RenderBufferFormat::Float32Vec4 => Self::Float32Vec4,

            ffi::RenderBufferFormat::Int16 => Self::Int16,
            ffi::RenderBufferFormat::Int16Vec2 => Self::Int16Vec2,
            ffi::RenderBufferFormat::Int16Vec3 => Self::Int16Vec3,
            ffi::RenderBufferFormat::Int16Vec4 => Self::Int16Vec4,

            ffi::RenderBufferFormat::UInt16 => Self::UInt16,
            ffi::RenderBufferFormat::UInt16Vec2 => Self::UInt16Vec2,
            ffi::RenderBufferFormat::UInt16Vec3 => Self::UInt16Vec3,
            ffi::RenderBufferFormat::UInt16Vec4 => Self::UInt16Vec4,

            ffi::RenderBufferFormat::Int32 => Self::Int32,
            ffi::RenderBufferFormat::Int32Vec2 => Self::Int32Vec2,
            ffi::RenderBufferFormat::Int32Vec3 => Self::Int32Vec3,
            ffi::RenderBufferFormat::Int32Vec4 => Self::Int32Vec4,

            ffi::RenderBufferFormat::Float32UInt8 => Self::Float32UInt8,

            _ => unreachable!("Invalid RenderBufferFormat"),
        }
    }
}
