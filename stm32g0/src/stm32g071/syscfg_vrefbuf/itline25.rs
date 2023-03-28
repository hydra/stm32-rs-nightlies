///Register `ITLINE25` reader
pub struct R(crate::R<ITLINE25_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE25_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE25_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE25_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SPI1` reader - SPI1
pub type SPI1_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - SPI1
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 25 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline25](index.html) module
pub struct ITLINE25_SPEC;
impl crate::RegisterSpec for ITLINE25_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline25::R](R) reader structure
impl crate::Readable for ITLINE25_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE25 to value 0
impl crate::Resettable for ITLINE25_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
