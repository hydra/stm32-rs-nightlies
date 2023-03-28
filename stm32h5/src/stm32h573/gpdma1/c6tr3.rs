///Register `C6TR3` reader
pub struct R(crate::R<C6TR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C6TR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C6TR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C6TR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C6TR3` writer
pub struct W(crate::W<C6TR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C6TR3_SPEC>;
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
impl From<crate::W<C6TR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C6TR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SAO` reader - source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\[12:0\]
///for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.SINC = 1). Note: A source address offset must be aligned with the programmed data width of a source burst (SAO\[2:0\]
///versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued. When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\[12:0\]
///is not applied.
pub type SAO_R = crate::FieldReader<u16, u16>;
///Field `SAO` writer - source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\[12:0\]
///for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.SINC = 1). Note: A source address offset must be aligned with the programmed data width of a source burst (SAO\[2:0\]
///versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued. When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\[12:0\]
///is not applied.
pub type SAO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C6TR3_SPEC, u16, u16, 13, O>;
///Field `DAO` reader - destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\[12:0\]
///for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.DINC = 1). Note: A destination address offset must be aligned with the programmed data width of a destination burst (DAO\[2:0\]
///versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.
pub type DAO_R = crate::FieldReader<u16, u16>;
///Field `DAO` writer - destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\[12:0\]
///for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.DINC = 1). Note: A destination address offset must be aligned with the programmed data width of a destination burst (DAO\[2:0\]
///versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.
pub type DAO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C6TR3_SPEC, u16, u16, 13, O>;
impl R {
    ///Bits 0:12 - source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\[12:0\]
    ///for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.SINC = 1). Note: A source address offset must be aligned with the programmed data width of a source burst (SAO\[2:0\]
    ///versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued. When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\[12:0\]
    ///is not applied.
    #[inline(always)]
    pub fn sao(&self) -> SAO_R {
        SAO_R::new((self.bits & 0x1fff) as u16)
    }
    ///Bits 16:28 - destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\[12:0\]
    ///for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.DINC = 1). Note: A destination address offset must be aligned with the programmed data width of a destination burst (DAO\[2:0\]
    ///versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.
    #[inline(always)]
    pub fn dao(&self) -> DAO_R {
        DAO_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    ///Bits 0:12 - source address offset increment The source address, pointed by GPDMA_CxSAR, is incremented or decremented (depending on GPDMA_CxBR1.SDEC) by this offset SAO\[12:0\]
    ///for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.SINC = 1). Note: A source address offset must be aligned with the programmed data width of a source burst (SAO\[2:0\]
    ///versus GPDMA_CxTR1.SDW_LOG2\[1:0\]). Else a user setting error is reported and none transfer is issued. When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional GPDMA_CxTR3.SAO\[12:0\]
    ///is not applied.
    #[inline(always)]
    #[must_use]
    pub fn sao(&mut self) -> SAO_W<0> {
        SAO_W::new(self)
    }
    ///Bits 16:28 - destination address offset increment The destination address, pointed by GPDMA_CxDAR, is incremented or decremented (depending on GPDMA_CxBR1.DDEC) by this offset DAO\[12:0\]
    ///for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (GPDMA_CxTR1.DINC = 1). Note: A destination address offset must be aligned with the programmed data width of a destination burst (DAO\[2:0\]
    ///versus GPDMA_CxTR1.DDW_LOG2\[1:0\]). Else, a user setting error is reported and no transfer is issued.
    #[inline(always)]
    #[must_use]
    pub fn dao(&mut self) -> DAO_W<16> {
        DAO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPDMA channel 6 transfer register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c6tr3](index.html) module
pub struct C6TR3_SPEC;
impl crate::RegisterSpec for C6TR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [c6tr3::R](R) reader structure
impl crate::Readable for C6TR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c6tr3::W](W) writer structure
impl crate::Writable for C6TR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C6TR3 to value 0
impl crate::Resettable for C6TR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
