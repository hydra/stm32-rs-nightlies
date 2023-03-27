///Register `RCC_OCRDYR` reader
pub struct R(crate::R<RCC_OCRDYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_OCRDYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_OCRDYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_OCRDYR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HSIRDY` reader - HSIRDY
pub type HSIRDY_R = crate::BitReader<bool>;
///Field `HSIDIVRDY` reader - HSIDIVRDY
pub type HSIDIVRDY_R = crate::BitReader<bool>;
///Field `CSIRDY` reader - CSIRDY
pub type CSIRDY_R = crate::BitReader<bool>;
///Field `HSERDY` reader - HSERDY
pub type HSERDY_R = crate::BitReader<bool>;
///Field `MPUCKRDY` reader - MPUCKRDY
pub type MPUCKRDY_R = crate::BitReader<bool>;
///Field `AXICKRDY` reader - AXICKRDY
pub type AXICKRDY_R = crate::BitReader<bool>;
///Field `CKREST` reader - CKREST
pub type CKREST_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - HSIRDY
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - HSIDIVRDY
    #[inline(always)]
    pub fn hsidivrdy(&self) -> HSIDIVRDY_R {
        HSIDIVRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - CSIRDY
    #[inline(always)]
    pub fn csirdy(&self) -> CSIRDY_R {
        CSIRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - HSERDY
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 23 - MPUCKRDY
    #[inline(always)]
    pub fn mpuckrdy(&self) -> MPUCKRDY_R {
        MPUCKRDY_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - AXICKRDY
    #[inline(always)]
    pub fn axickrdy(&self) -> AXICKRDY_R {
        AXICKRDY_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CKREST
    #[inline(always)]
    pub fn ckrest(&self) -> CKREST_R {
        CKREST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
///This is a read-only access register, It contains the status flags of oscillators. Writing has no effect.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ocrdyr](index.html) module
pub struct RCC_OCRDYR_SPEC;
impl crate::RegisterSpec for RCC_OCRDYR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ocrdyr::R](R) reader structure
impl crate::Readable for RCC_OCRDYR_SPEC {
    type Reader = R;
}
///`reset()` method sets RCC_OCRDYR to value 0
impl crate::Resettable for RCC_OCRDYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
