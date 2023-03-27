///Register `SR2` reader
pub struct R(crate::R<SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SYSCFGF` reader - illegal access flag for SYSCFG
pub type SYSCFGF_R = crate::BitReader<bool>;
///Field `RTCF` reader - illegal access flag for RTC
pub type RTCF_R = crate::BitReader<bool>;
///Field `TAMPF` reader - illegal access flag for TAMP
pub type TAMPF_R = crate::BitReader<bool>;
///Field `PWRF` reader - illegal access flag for PWRUSART1F
pub type PWRF_R = crate::BitReader<bool>;
///Field `RCCF` reader - illegal access flag for RCC
pub type RCCF_R = crate::BitReader<bool>;
///Field `LPDMA1F` reader - illegal access flag for LPDMA
pub type LPDMA1F_R = crate::BitReader<bool>;
///Field `EXTIF` reader - illegal access flag for EXTI
pub type EXTIF_R = crate::BitReader<bool>;
///Field `TZSC2F` reader - illegal access flag for GTZC2 TZSC registers
pub type TZSC2F_R = crate::BitReader<bool>;
///Field `TZIC2F` reader - illegal access flag for GTZC2 TZIC registers
pub type TZIC2F_R = crate::BitReader<bool>;
///Field `SRAM4F` reader - illegal access flag for SRAM4
pub type SRAM4F_R = crate::BitReader<bool>;
///Field `MPCBB4_REGF` reader - illegal access flag for MPCBB4 registers
pub type MPCBB4_REGF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - illegal access flag for SYSCFG
    #[inline(always)]
    pub fn syscfgf(&self) -> SYSCFGF_R {
        SYSCFGF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for RTC
    #[inline(always)]
    pub fn rtcf(&self) -> RTCF_R {
        RTCF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for TAMP
    #[inline(always)]
    pub fn tampf(&self) -> TAMPF_R {
        TAMPF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - illegal access flag for PWRUSART1F
    #[inline(always)]
    pub fn pwrf(&self) -> PWRF_R {
        PWRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - illegal access flag for RCC
    #[inline(always)]
    pub fn rccf(&self) -> RCCF_R {
        RCCF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - illegal access flag for LPDMA
    #[inline(always)]
    pub fn lpdma1f(&self) -> LPDMA1F_R {
        LPDMA1F_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - illegal access flag for EXTI
    #[inline(always)]
    pub fn extif(&self) -> EXTIF_R {
        EXTIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 14 - illegal access flag for GTZC2 TZSC registers
    #[inline(always)]
    pub fn tzsc2f(&self) -> TZSC2F_R {
        TZSC2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access flag for GTZC2 TZIC registers
    #[inline(always)]
    pub fn tzic2f(&self) -> TZIC2F_R {
        TZIC2F_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 24 - illegal access flag for SRAM4
    #[inline(always)]
    pub fn sram4f(&self) -> SRAM4F_R {
        SRAM4F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access flag for MPCBB4 registers
    #[inline(always)]
    pub fn mpcbb4_regf(&self) -> MPCBB4_REGF_R {
        MPCBB4_REGF_R::new(((self.bits >> 25) & 1) != 0)
    }
}
///TZIC status register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr2](index.html) module
pub struct SR2_SPEC;
impl crate::RegisterSpec for SR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr2::R](R) reader structure
impl crate::Readable for SR2_SPEC {
    type Reader = R;
}
///`reset()` method sets SR2 to value 0
impl crate::Resettable for SR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
