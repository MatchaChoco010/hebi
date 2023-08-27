#include "rendererPlugin.h"
#include "renderDelegate.h"

#include <iostream>

using namespace pxr;

TF_REGISTRY_FUNCTION(TfType)
{
    std::cout << "Registering HdHebiRendererPlugin" << std::endl;
    HdRendererPluginRegistry::Define<HdHebiRendererPlugin>();
}

HdRenderDelegate *
HdHebiRendererPlugin::CreateRenderDelegate()
{
    return new HdHebiRenderDelegate();
}

HdRenderDelegate *
HdHebiRendererPlugin::CreateRenderDelegate(
    HdRenderSettingsMap const &settingsMap)
{
    return new HdHebiRenderDelegate(settingsMap);
}

void HdHebiRendererPlugin::DeleteRenderDelegate(HdRenderDelegate *renderDelegate)
{
    delete renderDelegate;
}

bool HdHebiRendererPlugin::IsSupported() const
{
    // Nothing more to check for now, we assume if the plugin loads correctly
    // it is supported.
    return true;
}
