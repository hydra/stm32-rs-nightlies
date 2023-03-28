///Register `ITLINE9` reader
pub struct R(crate::R<ITLINE9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE9_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DMA1_CH1` reader - DMA1_CH1
pub type DMA1_CH1_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - DMA1_CH1
    #[inline(always)]
    pub fn dma1_ch1(&self) -> DMA1_CH1_R {
        DMA1_CH1_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 9 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline9](index.html) module
pub struct ITLINE9_SPEC;
impl crate::RegisterSpec for ITLINE9_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline9::R](R) reader structure
impl crate::Readable for ITLINE9_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE9 to value 0
impl crate::Resettable for ITLINE9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
