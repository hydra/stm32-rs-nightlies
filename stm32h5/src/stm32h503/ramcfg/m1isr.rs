///Register `M1ISR` reader
pub struct R(crate::R<M1ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M1ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M1ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M1ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SEDC` reader - ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.
pub type SEDC_R = crate::BitReader<bool>;
///Field `DED` reader - ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.
pub type DED_R = crate::BitReader<bool>;
///Field `SRAMBUSY` reader - SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features.
pub type SRAMBUSY_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.
    #[inline(always)]
    pub fn sedc(&self) -> SEDC_R {
        SEDC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register.
    #[inline(always)]
    pub fn ded(&self) -> DED_R {
        DED_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features.
    #[inline(always)]
    pub fn srambusy(&self) -> SRAMBUSY_R {
        SRAMBUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
///RAMCFG memory interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m1isr](index.html) module
pub struct M1ISR_SPEC;
impl crate::RegisterSpec for M1ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [m1isr::R](R) reader structure
impl crate::Readable for M1ISR_SPEC {
    type Reader = R;
}
///`reset()` method sets M1ISR to value 0
impl crate::Resettable for M1ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
