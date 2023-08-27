#ifndef HD_HEBI_RENDER_BUFFER_H
#define HD_HEBI_RENDER_BUFFER_H

#include "pxr/pxr.h"
#include "pxr/base/tf/diagnostic.h"
#include "pxr/base/gf/vec3i.h"
#include "pxr/imaging/hd/renderBuffer.h"

#include "rust/cxx.h"
#include "hydra-bridge/src/bridge.rs.h"

using namespace pxr;

class HdHebiRenderBuffer : public HdRenderBuffer
{
public:
  HdHebiRenderBuffer(SdfPath const &id, rust::Box<BridgeRenderBuffer> renderBuffer);
  ~HdHebiRenderBuffer() override;

  void Sync(HdSceneDelegate *sceneDelegate,
            HdRenderParam *renderParam,
            HdDirtyBits *dirtyBits) override;

  void Finalize(HdRenderParam *renderParam) override;

  bool Allocate(GfVec3i const &dimensions,
                HdFormat format,
                bool multiSampled) override;

  unsigned int GetWidth() const override { return _width; }
  unsigned int GetHeight() const override { return _height; }
  unsigned int GetDepth() const override { return 1; }
  HdFormat GetFormat() const override { return _format; }

  bool IsMultiSampled() const override { return false; }
  bool IsConverged() const override { return true; }
  void Resolve() override {}

  void *Map() override;
  void Unmap() override;
  bool IsMapped() const override
  {
    return _mappers.load() != 0;
  }

private:
  rust::Box<BridgeRenderBuffer> _bridgeRenderBuffer;
  unsigned int _width;
  unsigned int _height;
  HdFormat _format;
  std::vector<uint8_t> _buffer;
  std::atomic<int> _mappers;

  void _Deallocate();
  static size_t _GetBufferSize(GfVec2i const &dims, HdFormat format);
};

#endif
