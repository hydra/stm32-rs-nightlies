///Register `LPDMA_C0LLR` reader
pub struct R(crate::R<LPDMA_C0LLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPDMA_C0LLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPDMA_C0LLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPDMA_C0LLR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPDMA_C0LLR` writer
pub struct W(crate::W<LPDMA_C0LLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPDMA_C0LLR_SPEC>;
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
impl From<crate::W<LPDMA_C0LLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPDMA_C0LLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LA` reader - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\[15:20\]
///= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file (LPDMA_CxCTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.
pub type LA_R = crate::FieldReader<u16, u16>;
///Field `LA` writer - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\[15:20\]
///= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file (LPDMA_CxCTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.
pub type LA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPDMA_C0LLR_SPEC, u16, u16, 14, O>;
///Field `ULL` reader - Update LPDMA_CxLLR register from memory This bit is used to control the update of the LPDMA_CxLLR register from the memory during the link transfer.
pub type ULL_R = crate::BitReader<bool>;
///Field `ULL` writer - Update LPDMA_CxLLR register from memory This bit is used to control the update of the LPDMA_CxLLR register from the memory during the link transfer.
pub type ULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0LLR_SPEC, bool, O>;
///Field `UDA` reader - Update LPDMA_CxDAR register from memory This bit is used to control the update of the LPDMA_CxDAR register from the memory during the link transfer.
pub type UDA_R = crate::BitReader<bool>;
///Field `UDA` writer - Update LPDMA_CxDAR register from memory This bit is used to control the update of the LPDMA_CxDAR register from the memory during the link transfer.
pub type UDA_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0LLR_SPEC, bool, O>;
///Field `USA` reader - update LPDMA_CxSAR from memory This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer.
pub type USA_R = crate::BitReader<bool>;
///Field `USA` writer - update LPDMA_CxSAR from memory This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer.
pub type USA_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0LLR_SPEC, bool, O>;
///Field `UB1` reader - Update LPDMA_CxBR1 from memory This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer.
pub type UB1_R = crate::BitReader<bool>;
///Field `UB1` writer - Update LPDMA_CxBR1 from memory This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer.
pub type UB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0LLR_SPEC, bool, O>;
///Field `UT2` reader - Update LPDMA_CxTR2 from memory This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer.
pub type UT2_R = crate::BitReader<bool>;
///Field `UT2` writer - Update LPDMA_CxTR2 from memory This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer.
pub type UT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0LLR_SPEC, bool, O>;
///Field `UT1` reader - Update LPDMA_CxTR1 from memory This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer.
pub type UT1_R = crate::BitReader<bool>;
///Field `UT1` writer - Update LPDMA_CxTR1 from memory This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer.
pub type UT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPDMA_C0LLR_SPEC, bool, O>;
impl R {
    ///Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\[15:20\]
    ///= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file (LPDMA_CxCTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.
    #[inline(always)]
    pub fn la(&self) -> LA_R {
        LA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bit 16 - Update LPDMA_CxLLR register from memory This bit is used to control the update of the LPDMA_CxLLR register from the memory during the link transfer.
    #[inline(always)]
    pub fn ull(&self) -> ULL_R {
        ULL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 27 - Update LPDMA_CxDAR register from memory This bit is used to control the update of the LPDMA_CxDAR register from the memory during the link transfer.
    #[inline(always)]
    pub fn uda(&self) -> UDA_R {
        UDA_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - update LPDMA_CxSAR from memory This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer.
    #[inline(always)]
    pub fn usa(&self) -> USA_R {
        USA_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Update LPDMA_CxBR1 from memory This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer.
    #[inline(always)]
    pub fn ub1(&self) -> UB1_R {
        UB1_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Update LPDMA_CxTR2 from memory This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer.
    #[inline(always)]
    pub fn ut2(&self) -> UT2_R {
        UT2_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Update LPDMA_CxTR1 from memory This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer.
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 2:15 - pointer (16-bit low-significant address) to the next linked-list data structure If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\[15:20\]
    ///= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list DMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list DMA internal register file (LPDMA_CxCTR1, LPDMA_CxTR2, LPDMA_CxBR1, LPDMA_CxSAR, LPDMA_CxDAR and LPDMA_CxLLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored.
    #[inline(always)]
    #[must_use]
    pub fn la(&mut self) -> LA_W<2> {
        LA_W::new(self)
    }
    ///Bit 16 - Update LPDMA_CxLLR register from memory This bit is used to control the update of the LPDMA_CxLLR register from the memory during the link transfer.
    #[inline(always)]
    #[must_use]
    pub fn ull(&mut self) -> ULL_W<16> {
        ULL_W::new(self)
    }
    ///Bit 27 - Update LPDMA_CxDAR register from memory This bit is used to control the update of the LPDMA_CxDAR register from the memory during the link transfer.
    #[inline(always)]
    #[must_use]
    pub fn uda(&mut self) -> UDA_W<27> {
        UDA_W::new(self)
    }
    ///Bit 28 - update LPDMA_CxSAR from memory This bit controls the update of the LPDMA_CxSAR register from the memory during the link transfer.
    #[inline(always)]
    #[must_use]
    pub fn usa(&mut self) -> USA_W<28> {
        USA_W::new(self)
    }
    ///Bit 29 - Update LPDMA_CxBR1 from memory This bit controls the update of the LPDMA_CxBR1 register from the memory during the link transfer.
    #[inline(always)]
    #[must_use]
    pub fn ub1(&mut self) -> UB1_W<29> {
        UB1_W::new(self)
    }
    ///Bit 30 - Update LPDMA_CxTR2 from memory This bit controls the update of the LPDMA_CxTR2 register from the memory during the link transfer.
    #[inline(always)]
    #[must_use]
    pub fn ut2(&mut self) -> UT2_W<30> {
        UT2_W::new(self)
    }
    ///Bit 31 - Update LPDMA_CxTR1 from memory This bit controls the update of the LPDMA_CxTR1 register from the memory during the link transfer.
    #[inline(always)]
    #[must_use]
    pub fn ut1(&mut self) -> UT1_W<31> {
        UT1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPDMA channel 0 linked-list address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lpdma_c0llr](index.html) module
pub struct LPDMA_C0LLR_SPEC;
impl crate::RegisterSpec for LPDMA_C0LLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lpdma_c0llr::R](R) reader structure
impl crate::Readable for LPDMA_C0LLR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lpdma_c0llr::W](W) writer structure
impl crate::Writable for LPDMA_C0LLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPDMA_C0LLR to value 0
impl crate::Resettable for LPDMA_C0LLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
