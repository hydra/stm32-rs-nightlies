///Register `RAM5ISR` reader
pub struct R(crate::R<RAM5ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM5ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM5ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM5ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SEDC` reader - SEDC
pub type SEDC_R = crate::BitReader<bool>;
///Field `DED` reader - DED
pub type DED_R = crate::BitReader<bool>;
///Field `SRAMBUSY` reader - SRAMBUSY
pub type SRAMBUSY_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - SEDC
    #[inline(always)]
    pub fn sedc(&self) -> SEDC_R {
        SEDC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DED
    #[inline(always)]
    pub fn ded(&self) -> DED_R {
        DED_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - SRAMBUSY
    #[inline(always)]
    pub fn srambusy(&self) -> SRAMBUSY_R {
        SRAMBUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
///RAMCFG RAMx interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram5isr](index.html) module
pub struct RAM5ISR_SPEC;
impl crate::RegisterSpec for RAM5ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ram5isr::R](R) reader structure
impl crate::Readable for RAM5ISR_SPEC {
    type Reader = R;
}
///`reset()` method sets RAM5ISR to value 0
impl crate::Resettable for RAM5ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
