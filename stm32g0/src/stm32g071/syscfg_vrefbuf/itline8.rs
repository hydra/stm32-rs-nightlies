///Register `ITLINE8` reader
pub struct R(crate::R<ITLINE8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE8_SPEC>) -> Self {
        R(reader)
    }
}
///Field `UCPD1` reader - UCPD1
pub type UCPD1_R = crate::BitReader<bool>;
///Field `UCPD2` reader - UCPD2
pub type UCPD2_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - UCPD1
    #[inline(always)]
    pub fn ucpd1(&self) -> UCPD1_R {
        UCPD1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - UCPD2
    #[inline(always)]
    pub fn ucpd2(&self) -> UCPD2_R {
        UCPD2_R::new(((self.bits >> 1) & 1) != 0)
    }
}
///interrupt line 8 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline8](index.html) module
pub struct ITLINE8_SPEC;
impl crate::RegisterSpec for ITLINE8_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline8::R](R) reader structure
impl crate::Readable for ITLINE8_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE8 to value 0
impl crate::Resettable for ITLINE8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
