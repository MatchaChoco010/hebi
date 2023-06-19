#ifndef HD_HEBI_RENDER_DELEGATE_H
#define HD_HEBI_RENDER_DELEGATE_H

#include "pxr/pxr.h"
#include "pxr/imaging/hd/renderDelegate.h"
#include "pxr/imaging/hd/resourceRegistry.h"
#include "pxr/base/tf/staticTokens.h"

#include "hebi-hydra/src/bridge.rs.h"

using namespace pxr;

class HdHebiRenderDelegate final : public HdRenderDelegate
{
public:
    /// Render delegate constructor.
    HdHebiRenderDelegate();
    /// Render delegate constructor.
    HdHebiRenderDelegate(HdRenderSettingsMap const &settingsMap);
    /// Render delegate destructor.
    virtual ~HdHebiRenderDelegate();

    /// Supported types
    const TfTokenVector &GetSupportedRprimTypes() const override;
    const TfTokenVector &GetSupportedSprimTypes() const override;
    const TfTokenVector &GetSupportedBprimTypes() const override;

    // Basic value to return from the RD
    HdResourceRegistrySharedPtr GetResourceRegistry() const override;

    // Prims
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

private:
    void _Initialize();

    HdResourceRegistrySharedPtr _resourceRegistry;
    const BridgeRenderDelegate &_bridgeRenderDelegate;

    // This class does not support copying.
    HdHebiRenderDelegate(const HdHebiRenderDelegate &) = delete;
    HdHebiRenderDelegate &operator=(const HdHebiRenderDelegate &) = delete;
};

#endif
