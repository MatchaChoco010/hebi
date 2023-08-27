#ifndef HD_HEBI_RENDER_DELEGATE_H
#define HD_HEBI_RENDER_DELEGATE_H

#include "pxr/pxr.h"
#include "pxr/base/tf/diagnostic.h"
#include "pxr/base/tf/staticTokens.h"
#include "pxr/imaging/hd/camera.h"
#include "pxr/imaging/hd/extComputation.h"
#include "pxr/imaging/hd/renderDelegate.h"
#include "pxr/imaging/hd/resourceRegistry.h"

#include "rust/cxx.h"
#include "hydra-bridge/src/bridge.rs.h"

#include "mesh.h"
#include "renderBuffer.h"
#include "renderPass.h"

using namespace pxr;

class HdHebiRenderDelegate final : public HdRenderDelegate
{
public:
    HdHebiRenderDelegate();
    HdHebiRenderDelegate(HdRenderSettingsMap const &settingsMap);
    virtual ~HdHebiRenderDelegate();

    const TfTokenVector &GetSupportedRprimTypes() const override;
    const TfTokenVector &GetSupportedSprimTypes() const override;
    const TfTokenVector &GetSupportedBprimTypes() const override;

    HdResourceRegistrySharedPtr GetResourceRegistry() const override;

    HdRenderPassSharedPtr CreateRenderPass(
        HdRenderIndex *index,
        HdRprimCollection const &collection) override;

    HdInstancer *CreateInstancer(HdSceneDelegate *delegate,
                                 SdfPath const &id) override;
    void DestroyInstancer(HdInstancer *instancer) override;

    HdRprim *CreateRprim(TfToken const &typeId,
                         SdfPath const &rprimId) override;
    void DestroyRprim(HdRprim *rPrim) override;

    HdSprim *CreateSprim(TfToken const &typeId,
                         SdfPath const &sprimId) override;
    HdSprim *CreateFallbackSprim(TfToken const &typeId) override;
    void DestroySprim(HdSprim *sprim) override;

    HdBprim *CreateBprim(TfToken const &typeId,
                         SdfPath const &bprimId) override;
    HdBprim *CreateFallbackBprim(TfToken const &typeId) override;
    void DestroyBprim(HdBprim *bprim) override;

    void CommitResources(HdChangeTracker *tracker) override;

    HdRenderParam *GetRenderParam() const override;

    HdAovDescriptor GetDefaultAovDescriptor(TfToken const &name) const override;

    void Render();

private:
    void _Initialize();

    HdResourceRegistrySharedPtr _resourceRegistry;
    rust::Box<BridgeRenderDelegate> _bridgeRenderDelegate;

    // This class does not support copying.
    HdHebiRenderDelegate(const HdHebiRenderDelegate &) = delete;
    HdHebiRenderDelegate &operator=(const HdHebiRenderDelegate &) = delete;
};

#endif
