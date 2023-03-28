///Register `LPDMA_C2BR1` reader
pub struct R(crate::R<LPDMA_C2BR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPDMA_C2BR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPDMA_C2BR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPDMA_C2BR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPDMA_C2BR1` writer
pub struct W(crate::W<LPDMA_C2BR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPDMA_C2BR1_SPEC>;
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
impl From<crate::W<LPDMA_C2BR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPDMA_C2BR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BNDT` reader - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\]
///is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\]
///= 0): - if LPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if LPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all LPDMA_CxLLR.Uxx = 0 and if LPDMA_CxLLR.LA\[15:0\]
///= 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if LPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\]
///versus LPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type BNDT_R = crate::FieldReader<u16, u16>;
///Field `BNDT` writer - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\]
///is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\]
///= 0): - if LPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if LPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all LPDMA_CxLLR.Uxx = 0 and if LPDMA_CxLLR.LA\[15:0\]
///= 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if LPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\]
///versus LPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
pub type BNDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDMA_C2BR1_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\]
    ///is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\]
    ///= 0): - if LPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if LPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all LPDMA_CxLLR.Uxx = 0 and if LPDMA_CxLLR.LA\[15:0\]
    ///= 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if LPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\]
    ///versus LPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    pub fn bndt(&self) -> BNDT_R {
        BNDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - block number of data bytes to transfer from the source Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\[15:0\]
    ///is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\[15:0\]
    ///= 0): - if LPDMA_CxLLR.UB1 = 1, this field is updated by the LLI in the memory. - if LPDMA_CxLLR.UB1 = 0 and if there is at least one non null Uxx update bit, this field is internally restored to the programmed value. - if all LPDMA_CxLLR.Uxx = 0 and if LPDMA_CxLLR.LA\[15:0\]
    ///= 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if LPDMA_CxLLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\[2:0\]
    ///versus LPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued.
    #[inline(always)]
    #[must_use]
    pub fn bndt(&mut self) -> BNDT_W<0> {
        BNDT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPDMA channel 2 block register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpdma_c2br1](index.html) module
pub struct LPDMA_C2BR1_SPEC;
impl crate::RegisterSpec for LPDMA_C2BR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpdma_c2br1::R](R) reader structure
impl crate::Readable for LPDMA_C2BR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpdma_c2br1::W](W) writer structure
impl crate::Writable for LPDMA_C2BR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPDMA_C2BR1 to value 0
impl crate::Resettable for LPDMA_C2BR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
