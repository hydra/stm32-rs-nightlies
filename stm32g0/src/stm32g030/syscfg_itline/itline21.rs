///Register `ITLINE21` reader
pub struct R(crate::R<ITLINE21_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE21_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE21_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE21_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TIM16` reader - TIM16
pub type TIM16_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TIM16
    #[inline(always)]
    pub fn tim16(&self) -> TIM16_R {
        TIM16_R::new((self.bits & 1) != 0)
    }
}
///interrupt line 21 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline21](index.html) module
pub struct ITLINE21_SPEC;
impl crate::RegisterSpec for ITLINE21_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline21::R](R) reader structure
impl crate::Readable for ITLINE21_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE21 to value 0
impl crate::Resettable for ITLINE21_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
