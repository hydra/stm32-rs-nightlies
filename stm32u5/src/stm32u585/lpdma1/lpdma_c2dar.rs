///Register `LPDMA_C2DAR` reader
pub struct R(crate::R<LPDMA_C2DAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPDMA_C2DAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPDMA_C2DAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPDMA_C2DAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPDMA_C2DAR` writer
pub struct W(crate::W<LPDMA_C2DAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPDMA_C2DAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LPDMA_C2DAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPDMA_C2DAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DA` reader - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2\[21:0\]) after each single destination data, reflecting the next address from which data is written. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination single (DA\[2:0\]
///versus LPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.
pub type DA_R = crate::FieldReader<u32, u32>;
///Field `DA` writer - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2\[21:0\]) after each single destination data, reflecting the next address from which data is written. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination single (DA\[2:0\]
///versus LPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.
pub type DA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDMA_C2DAR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2\[21:0\]) after each single destination data, reflecting the next address from which data is written. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination single (DA\[2:0\]
    ///versus LPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - destination address This field is the pointer to the address from which the next data is written. During the channel activity, depending on the destination addressing mode (LPDMA_CxTR1.DINC), this field is kept fixed or incremented by the data width (LPDMA_CxTR1.DDW_LOG2\[21:0\]) after each single destination data, reflecting the next address from which data is written. In linked-list mode, after a LLI data transfer is completed, this register is automatically updated by DMA from the memory, provided the LLI is set with LPDMA_CxLLR.UDA = 1. Note: A destination address must be aligned with the programmed data width of a destination single (DA\[2:0\]
    ///versus LPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<0> {
        DA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPDMA channel 2 destination address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpdma_c2dar](index.html) module
pub struct LPDMA_C2DAR_SPEC;
impl crate::RegisterSpec for LPDMA_C2DAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpdma_c2dar::R](R) reader structure
impl crate::Readable for LPDMA_C2DAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpdma_c2dar::W](W) writer structure
impl crate::Writable for LPDMA_C2DAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPDMA_C2DAR to value 0
impl crate::Resettable for LPDMA_C2DAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
