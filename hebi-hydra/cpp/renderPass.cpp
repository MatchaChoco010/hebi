#include "renderPass.h"

#include <iostream>

using namespace pxr;

HdHebiRenderPass::HdHebiRenderPass(
    HdRenderIndex *index,
    HdRprimCollection const &collection)
    : HdRenderPass(index, collection)
{
}

HdHebiRenderPass::~HdHebiRenderPass()
{
}

void HdHebiRenderPass::_Execute(
    HdRenderPassStateSharedPtr const &renderPassState,
    TfTokenVector const &renderTags)
{
    std::cout << "=> Execute RenderPass" << std::endl;
    HdHebiRenderDelegate *renderDelegate = dynamic_cast<HdHebiRenderDelegate *>(GetRenderIndex()->GetRenderDelegate());
    renderDelegate->Render();
}
