#include "hebi-hydra/cpp/entry.h"

#include "hebi-hydra/src/lib.rs.h"

#include "pxr/imaging/hd/rendererPluginRegistry.h"

PXR_NAMESPACE_OPEN_SCOPE

TF_REGISTRY_FUNCTION(TfType)
{
  initialize();
  HdRendererPluginRegistry::Define<HdTinyRendererPlugin>();
}

PXR_NAMESPACE_CLOSE_SCOPE

void entry()
{
}
