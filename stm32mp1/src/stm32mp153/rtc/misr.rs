///Register `MISR` reader
pub struct R(crate::R<MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ALRAMF` reader - ALRAMF
pub type ALRAMF_R = crate::BitReader<bool>;
///Field `ALRBMF` reader - ALRBMF
pub type ALRBMF_R = crate::BitReader<bool>;
///Field `WUTMF` reader - WUTMF
pub type WUTMF_R = crate::BitReader<bool>;
///Field `TSMF` reader - TSMF
pub type TSMF_R = crate::BitReader<bool>;
///Field `TSOVMF` reader - TSOVMF
pub type TSOVMF_R = crate::BitReader<bool>;
///Field `ITSMF` reader - ITSMF
pub type ITSMF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - ALRAMF
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ALRBMF
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WUTMF
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TSMF
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TSOVMF
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ITSMF
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
///RTC non-secure masked interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [misr](index.html) module
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [misr::R](R) reader structure
impl crate::Readable for MISR_SPEC {
    type Reader = R;
}
///`reset()` method sets MISR to value 0
impl crate::Resettable for MISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
