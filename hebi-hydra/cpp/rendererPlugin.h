#ifndef HD_HEBI_RENDERER_PLUGIN_H
#define HD_HEBI_RENDERER_PLUGIN_H

#include "pxr/pxr.h"
#include "pxr/imaging/hd/rendererPlugin.h"
#include "pxr/imaging/hd/rendererPluginRegistry.h"

using namespace pxr;

extern "C" class HdHebiRendererPlugin final : public HdRendererPlugin
{
public:
    HdHebiRendererPlugin() = default;
    virtual ~HdHebiRendererPlugin() = default;

    /// Construct a new render delegate of type HdTinyRenderDelegate.
    virtual HdRenderDelegate *CreateRenderDelegate() override;

    /// Construct a new render delegate of type HdTinyRenderDelegate.
    virtual HdRenderDelegate *CreateRenderDelegate(
        HdRenderSettingsMap const &settingsMap) override;

    /// Destroy a render delegate created by this class's CreateRenderDelegate.
    ///   \param renderDelegate The render delegate to delete.
    virtual void DeleteRenderDelegate(
        HdRenderDelegate *renderDelegate) override;

    /// Checks to see if the plugin is supported on the running system.
    virtual bool IsSupported(bool gpuEnabled = true) const override;

private:
    // This class does not support copying.
    HdHebiRendererPlugin(const HdHebiRendererPlugin &) = delete;
    HdHebiRendererPlugin &operator=(const HdHebiRendererPlugin &) = delete;
};

#endif
