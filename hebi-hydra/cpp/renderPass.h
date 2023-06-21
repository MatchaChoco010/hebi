#ifndef HD_HEBI_RENDER_PASS_H
#define HD_HEBI_RENDER_PASS_H

#include "pxr/pxr.h"
#include "pxr/imaging/hd/renderPass.h"

#include "renderDelegate.h"

using namespace pxr;

class HdHebiRenderPass final : public HdRenderPass
{
public:
    HdHebiRenderPass(HdRenderIndex *index,
                     HdRprimCollection const &collection);
    virtual ~HdHebiRenderPass();

protected:
    void _Execute(
        HdRenderPassStateSharedPtr const &renderPassState,
        TfTokenVector const &renderTags) override;
};

#endif
