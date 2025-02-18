///Register `LPTIM_ISR` reader
pub struct R(crate::R<LPTIM_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CMPM` reader - Compare match The CMPM bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP registerâs value.
pub type CMPM_R = crate::BitReader<bool>;
///Field `ARRM` reader - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT registerâs value reached the LPTIM_ARR registerâs value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register.
pub type ARRM_R = crate::BitReader<bool>;
///Field `EXTTRIG` reader - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register.
pub type EXTTRIG_R = crate::BitReader<bool>;
///Field `CMPOK` reader - Compare register update OK CMPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_CMP register has been successfully completed.
pub type CMPOK_R = crate::BitReader<bool>;
///Field `ARROK` reader - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register.
pub type ARROK_R = crate::BitReader<bool>;
///Field `UP` reader - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
pub type UP_R = crate::BitReader<bool>;
///Field `DOWN` reader - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
pub type DOWN_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Compare match The CMPM bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP registerâs value.
    #[inline(always)]
    pub fn cmpm(&self) -> CMPM_R {
        CMPM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT registerâs value reached the LPTIM_ARR registerâs value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn arrm(&self) -> ARRM_R {
        ARRM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare register update OK CMPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_CMP register has been successfully completed.
    #[inline(always)]
    pub fn cmpok(&self) -> CMPOK_R {
        CMPOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register.
    #[inline(always)]
    pub fn arrok(&self) -> ARROK_R {
        ARROK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
///Interrupt and Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lptim_isr](index.html) module
pub struct LPTIM_ISR_SPEC;
impl crate::RegisterSpec for LPTIM_ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lptim_isr::R](R) reader structure
impl crate::Readable for LPTIM_ISR_SPEC {
    type Reader = R;
}
///`reset()` method sets LPTIM_ISR to value 0
impl crate::Resettable for LPTIM_ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
