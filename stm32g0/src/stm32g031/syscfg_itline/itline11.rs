///Register `ITLINE11` reader
pub struct R(crate::R<ITLINE11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE11_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DMAMUX` reader - DMAMUX
pub type DMAMUX_R = crate::BitReader<bool>;
///Field `DMA1_CH4` reader - DMA1_CH4
pub type DMA1_CH4_R = crate::BitReader<bool>;
///Field `DMA1_CH5` reader - DMA1_CH5
pub type DMA1_CH5_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - DMAMUX
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA1_CH4
    #[inline(always)]
    pub fn dma1_ch4(&self) -> DMA1_CH4_R {
        DMA1_CH4_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMA1_CH5
    #[inline(always)]
    pub fn dma1_ch5(&self) -> DMA1_CH5_R {
        DMA1_CH5_R::new(((self.bits >> 2) & 1) != 0)
    }
}
///interrupt line 11 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline11](index.html) module
pub struct ITLINE11_SPEC;
impl crate::RegisterSpec for ITLINE11_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline11::R](R) reader structure
impl crate::Readable for ITLINE11_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE11 to value 0
impl crate::Resettable for ITLINE11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
