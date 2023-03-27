///Register `UR16` reader
pub struct R(crate::R<UR16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR16_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FZIWDGSTP` reader - Freeze independent watchdog in Stop mode
pub type FZIWDGSTP_R = crate::BitReader<bool>;
///Field `PKP` reader - Private key programmed
pub type PKP_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Freeze independent watchdog in Stop mode
    #[inline(always)]
    pub fn fziwdgstp(&self) -> FZIWDGSTP_R {
        FZIWDGSTP_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - Private key programmed
    #[inline(always)]
    pub fn pkp(&self) -> PKP_R {
        PKP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
///SYSCFG user register 16
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ur16](index.html) module
pub struct UR16_SPEC;
impl crate::RegisterSpec for UR16_SPEC {
    type Ux = u32;
}
///`read()` method returns [ur16::R](R) reader structure
impl crate::Readable for UR16_SPEC {
    type Reader = R;
}
///`reset()` method sets UR16 to value 0
impl crate::Resettable for UR16_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
