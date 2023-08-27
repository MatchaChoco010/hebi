#include "renderBuffer.h"

#include "hydra-bridge/src/bridge.rs.h"

using namespace pxr;

RenderBufferFormat convertRenderBufferFormat(HdFormat format)
{
  switch (format)
  {
  case HdFormatUNorm8:
    return RenderBufferFormat::UNorm8;
  case HdFormatUNorm8Vec2:
    return RenderBufferFormat::UNorm8Vec2;
  case HdFormatUNorm8Vec3:
    return RenderBufferFormat::UNorm8Vec3;
  case HdFormatUNorm8Vec4:
    return RenderBufferFormat::UNorm8Vec4;

  case HdFormatSNorm8:
    return RenderBufferFormat::SNorm8;
  case HdFormatSNorm8Vec2:
    return RenderBufferFormat::SNorm8Vec2;
  case HdFormatSNorm8Vec3:
    return RenderBufferFormat::SNorm8Vec3;
  case HdFormatSNorm8Vec4:
    return RenderBufferFormat::SNorm8Vec4;

  case HdFormatFloat16:
    return RenderBufferFormat::Float16;
  case HdFormatFloat16Vec2:
    return RenderBufferFormat::Float16Vec2;
  case HdFormatFloat16Vec3:
    return RenderBufferFormat::Float16Vec3;
  case HdFormatFloat16Vec4:
    return RenderBufferFormat::Float16Vec4;

  case HdFormatFloat32:
    return RenderBufferFormat::Float32;
  case HdFormatFloat32Vec2:
    return RenderBufferFormat::Float32Vec2;
  case HdFormatFloat32Vec3:
    return RenderBufferFormat::Float32Vec3;
  case HdFormatFloat32Vec4:
    return RenderBufferFormat::Float32Vec4;

  case HdFormatInt16:
    return RenderBufferFormat::Int16;
  case HdFormatInt16Vec2:
    return RenderBufferFormat::Int16Vec2;
  case HdFormatInt16Vec3:
    return RenderBufferFormat::Int16Vec3;
  case HdFormatInt16Vec4:
    return RenderBufferFormat::Int16Vec4;

  case HdFormatUInt16:
    return RenderBufferFormat::UInt16;
  case HdFormatUInt16Vec2:
    return RenderBufferFormat::UInt16Vec2;
  case HdFormatUInt16Vec3:
    return RenderBufferFormat::UInt16Vec3;
  case HdFormatUInt16Vec4:
    return RenderBufferFormat::UInt16Vec4;

  case HdFormatInt32:
    return RenderBufferFormat::Int32;
  case HdFormatInt32Vec2:
    return RenderBufferFormat::Int32Vec2;
  case HdFormatInt32Vec3:
    return RenderBufferFormat::Int32Vec3;
  case HdFormatInt32Vec4:
    return RenderBufferFormat::Int32Vec4;

  case HdFormatFloat32UInt8:
    return RenderBufferFormat::Float32UInt8;
  }
}

HdHebiRenderBuffer::HdHebiRenderBuffer(SdfPath const &id, rust::Box<BridgeRenderBuffer> renderBuffer)
    : HdRenderBuffer(id),
      _bridgeRenderBuffer(std::move(renderBuffer)),
      _width(0),
      _height(0),
      _format(HdFormatInvalid),
      _buffer(),
      _mappers(0)
{
}

HdHebiRenderBuffer::~HdHebiRenderBuffer()
{
}

void HdHebiRenderBuffer::Sync(HdSceneDelegate *sceneDelegate,
                              HdRenderParam *renderParam,
                              HdDirtyBits *dirtyBits)
{
  HdRenderBuffer::Sync(sceneDelegate, renderParam, dirtyBits);
}

void HdHebiRenderBuffer::Finalize(HdRenderParam *renderParam)
{
  _bridgeRenderBuffer->finalize();
  HdRenderBuffer::Finalize(renderParam);
}

void HdHebiRenderBuffer::_Deallocate()
{
  _width = 0;
  _height = 0;
  _format = HdFormatInvalid;
  _buffer.resize(0);
  _mappers.store(0);
}

size_t
HdHebiRenderBuffer::_GetBufferSize(GfVec2i const &dims, HdFormat format)
{
  return dims[0] * dims[1] * HdDataSizeOfFormat(format);
}

bool HdHebiRenderBuffer::Allocate(GfVec3i const &dimensions,
                                  HdFormat format,
                                  bool multiSampled)
{
  _Deallocate();

  if (dimensions[2] != 1)
  {
    TF_WARN("Render buffer allocated with dims <%d, %d, %d> and"
            " format %s; depth must be 1!",
            dimensions[0], dimensions[1], dimensions[2],
            TfEnum::GetName(format).c_str());
    return false;
  }

  _width = dimensions[0];
  _height = dimensions[1];
  _format = format;
  _buffer.resize(_GetBufferSize(GfVec2i(_width, _height), format));

  _bridgeRenderBuffer->allocate(_width, _height, convertRenderBufferFormat(format));

  return true;
}

void *HdHebiRenderBuffer::Map()
{
  _mappers.fetch_add(1);

  auto data = _bridgeRenderBuffer->read();
  for (int i = 0; i < data.size(); i++)
  {
    _buffer[i] = data[i];
  }

  return _buffer.data();
}

void HdHebiRenderBuffer::Unmap()
{
  _mappers.fetch_sub(1);
}
