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
///Field `FLASH_RDY` reader - Flash ready flag
pub type FLASH_RDY_R = crate::BitReader<bool>;
///Field `REGLPS` reader - Low-power regulator started
pub type REGLPS_R = crate::BitReader<bool>;
///Field `REGLPF` reader - Low-power regulator flag
pub type REGLPF_R = crate::BitReader<bool>;
///Field `VOSF` reader - Voltage scaling flag
pub type VOSF_R = crate::BitReader<bool>;
///Field `PVDO` reader - Power voltage detector output
pub type PVDO_R = crate::BitReader<bool>;
impl R {
    ///Bit 7 - Flash ready flag
    #[inline(always)]
    pub fn flash_rdy(&self) -> FLASH_RDY_R {
        FLASH_RDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Low-power regulator started
    #[inline(always)]
    pub fn reglps(&self) -> REGLPS_R {
        REGLPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Low-power regulator flag
    #[inline(always)]
    pub fn reglpf(&self) -> REGLPF_R {
        REGLPF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Voltage scaling flag
    #[inline(always)]
    pub fn vosf(&self) -> VOSF_R {
        VOSF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Power voltage detector output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 11) & 1) != 0)
    }
}
///Power status register 2
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
