///Register `ITLINE17` reader
pub struct R(crate::R<ITLINE17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE17_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TIM6` reader - TIM6
pub type TIM6_R = crate::BitReader<bool>;
///Field `DAC` reader - DAC
pub type DAC_R = crate::BitReader<bool>;
///Field `LPTIM1` reader - LPTIM1
pub type LPTIM1_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TIM6
    #[inline(always)]
    pub fn tim6(&self) -> TIM6_R {
        TIM6_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DAC
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LPTIM1
    #[inline(always)]
    pub fn lptim1(&self) -> LPTIM1_R {
        LPTIM1_R::new(((self.bits >> 2) & 1) != 0)
    }
}
///interrupt line 17 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline17](index.html) module
pub struct ITLINE17_SPEC;
impl crate::RegisterSpec for ITLINE17_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline17::R](R) reader structure
impl crate::Readable for ITLINE17_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE17 to value 0
impl crate::Resettable for ITLINE17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
