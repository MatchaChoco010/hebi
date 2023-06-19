#include "hebi-hydra/cpp/entry.h"

PXR_NAMESPACE_OPEN_SCOPE

TF_REGISTRY_FUNCTION(TfType)
{
  initialize();
  HdRendererPluginRegistry::Define<HdHebiRendererPlugin>();
}

PXR_NAMESPACE_CLOSE_SCOPE

void entry()
{
}
