///Register `GPDMA_C15BR2` reader
pub struct R(crate::R<GPDMA_C15BR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDMA_C15BR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDMA_C15BR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDMA_C15BR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GPDMA_C15BR2` writer
pub struct W(crate::W<GPDMA_C15BR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDMA_C15BR2_SPEC>;
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
impl From<crate::W<GPDMA_C15BR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDMA_C15BR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BRSAO` reader - Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\[2:0\]
///versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
pub type BRSAO_R = crate::FieldReader<u16, u16>;
///Field `BRSAO` writer - Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\[2:0\]
///versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
pub type BRSAO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDMA_C15BR2_SPEC, u16, u16, 16, O>;
///Field `BRDAO` reader - Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\[2:0\]
///versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
pub type BRDAO_R = crate::FieldReader<u16, u16>;
///Field `BRDAO` writer - Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\[2:0\]
///versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
pub type BRDAO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPDMA_C15BR2_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\[2:0\]
    ///versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
    #[inline(always)]
    pub fn brsao(&self) -> BRSAO_R {
        BRSAO_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\[2:0\]
    ///versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
    #[inline(always)]
    pub fn brdao(&self) -> BRDAO_R {
        BRDAO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Block repeated source address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRSDEC) the current source address (GPDMA_CxSAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\[2:0\]
    ///versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
    #[inline(always)]
    #[must_use]
    pub fn brsao(&mut self) -> BRSAO_W<0> {
        BRSAO_W::new(self)
    }
    ///Bits 16:31 - Block repeated destination address offset For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on GPDMA_CxBR1.BRDDEC) the current destination address (GPDMA_CxDAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\[2:0\]
    ///versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else a user setting error is reported and no transfer is issued.
    #[inline(always)]
    #[must_use]
    pub fn brdao(&mut self) -> BRDAO_W<16> {
        BRDAO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPDMA channel 15 block register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpdma_c15br2](index.html) module
pub struct GPDMA_C15BR2_SPEC;
impl crate::RegisterSpec for GPDMA_C15BR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpdma_c15br2::R](R) reader structure
impl crate::Readable for GPDMA_C15BR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gpdma_c15br2::W](W) writer structure
impl crate::Writable for GPDMA_C15BR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GPDMA_C15BR2 to value 0
impl crate::Resettable for GPDMA_C15BR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
