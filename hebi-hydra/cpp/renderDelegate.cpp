#include "renderDelegate.h"
#include "mesh.h"
#include "renderPass.h"

#include <iostream>

using namespace pxr;

TfTokenVector SUPPORTED_RPRIM_TYPES = {};
TfTokenVector SUPPORTED_SPRIM_TYPES = {};
TfTokenVector SUPPORTED_BPRIM_TYPES = {};

HdHebiRenderDelegate::HdHebiRenderDelegate()
    : HdRenderDelegate(),
      _bridgeRenderDelegate(new_bridge_render_delegate())
{
    _Initialize();
}

HdHebiRenderDelegate::HdHebiRenderDelegate(
    HdRenderSettingsMap const &settingsMap)
    : HdRenderDelegate(settingsMap),
      _bridgeRenderDelegate(new_bridge_render_delegate())
{
    _Initialize();
}

void HdHebiRenderDelegate::_Initialize()
{
    _bridgeRenderDelegate->init();
    _resourceRegistry = std::make_shared<HdResourceRegistry>();
}

HdHebiRenderDelegate::~HdHebiRenderDelegate()
{
    _resourceRegistry.reset();
    _bridgeRenderDelegate->destroy();
}

TfTokenVector const &
HdHebiRenderDelegate::GetSupportedRprimTypes() const
{
    SUPPORTED_RPRIM_TYPES.clear();
    auto tokens = _bridgeRenderDelegate->get_supported_rprim_types();
    for (auto token : tokens)
    {
        SUPPORTED_RPRIM_TYPES.emplace_back(TfToken(std::string(token)));
    }
    return SUPPORTED_RPRIM_TYPES;
}

TfTokenVector const &
HdHebiRenderDelegate::GetSupportedSprimTypes() const
{
    SUPPORTED_SPRIM_TYPES.clear();
    auto tokens = _bridgeRenderDelegate->get_supported_sprim_types();
    for (auto token : tokens)
    {
        SUPPORTED_SPRIM_TYPES.emplace_back(TfToken(std::string(token)));
    }
    return SUPPORTED_SPRIM_TYPES;
}

TfTokenVector const &
HdHebiRenderDelegate::GetSupportedBprimTypes() const
{
    SUPPORTED_BPRIM_TYPES.clear();
    auto tokens = _bridgeRenderDelegate->get_supported_bprim_types();
    for (auto token : tokens)
    {
        SUPPORTED_BPRIM_TYPES.emplace_back(TfToken(std::string(token)));
    }
    return SUPPORTED_BPRIM_TYPES;
}

HdResourceRegistrySharedPtr
HdHebiRenderDelegate::GetResourceRegistry() const
{
    return _resourceRegistry;
}

void HdHebiRenderDelegate::CommitResources(HdChangeTracker *tracker)
{
    std::cout << "=> CommitResources RenderDelegate" << std::endl;
}

HdRenderPassSharedPtr
HdHebiRenderDelegate::CreateRenderPass(
    HdRenderIndex *index,
    HdRprimCollection const &collection)
{
    std::cout << "Create RenderPass with Collection="
              << collection.GetName() << std::endl;

    return HdRenderPassSharedPtr(new HdTinyRenderPass(index, collection));
}

HdRprim *
HdHebiRenderDelegate::CreateRprim(TfToken const &typeId,
                                  SdfPath const &rprimId)
{
    std::cout << "Create Tiny Rprim type=" << typeId.GetText()
              << " id=" << rprimId
              << std::endl;

    if (typeId == HdPrimTypeTokens->mesh)
    {
        return new HdTinyMesh(rprimId);
    }
    else
    {
        TF_CODING_ERROR("Unknown Rprim type=%s id=%s",
                        typeId.GetText(),
                        rprimId.GetText());
    }
    return nullptr;
}

void HdHebiRenderDelegate::DestroyRprim(HdRprim *rPrim)
{
    std::cout << "Destroy Tiny Rprim id=" << rPrim->GetId() << std::endl;
    delete rPrim;
}

HdSprim *
HdHebiRenderDelegate::CreateSprim(TfToken const &typeId,
                                  SdfPath const &sprimId)
{
    TF_CODING_ERROR("Unknown Sprim type=%s id=%s",
                    typeId.GetText(),
                    sprimId.GetText());
    return nullptr;
}

HdSprim *
HdHebiRenderDelegate::CreateFallbackSprim(TfToken const &typeId)
{
    TF_CODING_ERROR("Creating unknown fallback sprim type=%s",
                    typeId.GetText());
    return nullptr;
}

void HdHebiRenderDelegate::DestroySprim(HdSprim *sPrim)
{
    TF_CODING_ERROR("Destroy Sprim not supported");
}

HdBprim *
HdHebiRenderDelegate::CreateBprim(TfToken const &typeId, SdfPath const &bprimId)
{
    if (typeId == HdPrimTypeTokens->renderBuffer)
    {
        auto id = bprimId.MakeRelativePath(SdfPath::AbsoluteRootPath()).GetText();
        auto boxRustRB = _bridgeRenderDelegate->create_render_buffer(rust::String(id));
        auto data = boxRustRB->read();
    }

    TF_CODING_ERROR("Unknown Bprim type=%s id=%s",
                    typeId.GetText(),
                    bprimId.GetText());
    return nullptr;
}

HdBprim *
HdHebiRenderDelegate::CreateFallbackBprim(TfToken const &typeId)
{
    TF_CODING_ERROR("Creating unknown fallback bprim type=%s",
                    typeId.GetText());
    return nullptr;
}

void HdHebiRenderDelegate::DestroyBprim(HdBprim *bPrim)
{
    TF_CODING_ERROR("Destroy Bprim not supported");
}

HdInstancer *
HdHebiRenderDelegate::CreateInstancer(
    HdSceneDelegate *delegate,
    SdfPath const &id)
{
    TF_CODING_ERROR("Creating Instancer not supported id=%s",
                    id.GetText());
    return nullptr;
}

void HdHebiRenderDelegate::DestroyInstancer(HdInstancer *instancer)
{
    TF_CODING_ERROR("Destroy instancer not supported");
}

HdRenderParam *
HdHebiRenderDelegate::GetRenderParam() const
{
    return nullptr;
}
