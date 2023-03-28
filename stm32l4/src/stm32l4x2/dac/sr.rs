///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMAUDR1` reader - DAC channel1 DMA underrun flag
pub type DMAUDR1_R = crate::BitReader<DMAUDR1_A>;
///DAC channel1 DMA underrun flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDR1_A {
    ///0: No DMA underrun error condition occurred for DAC channel x
    NoError = 0,
    ///1: DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
    Error = 1,
}
impl From<DMAUDR1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR1_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAUDR1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAUDR1_A {
        match self.bits {
            false => DMAUDR1_A::NoError,
            true => DMAUDR1_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == DMAUDR1_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == DMAUDR1_A::Error
    }
}
///Field `DMAUDR1` writer - DAC channel1 DMA underrun flag
pub type DMAUDR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, DMAUDR1_A, O>;
impl<'a, const O: u8> DMAUDR1_W<'a, O> {
    ///No DMA underrun error condition occurred for DAC channel x
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(DMAUDR1_A::NoError)
    }
    ///DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(DMAUDR1_A::Error)
    }
}
///Field `CAL_FLAG1` reader - DAC Channel 1 calibration offset status
pub type CAL_FLAG1_R = crate::BitReader<CAL_FLAG1_A>;
///DAC Channel 1 calibration offset status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL_FLAG1_A {
    ///0: Calibration trimming value is lower than the offset correction value
    Lower = 0,
    ///1: Calibration trimming value is equal or greater than the offset correction value
    EqualHigher = 1,
}
impl From<CAL_FLAG1_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_FLAG1_A) -> Self {
        variant as u8 != 0
    }
}
impl CAL_FLAG1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CAL_FLAG1_A {
        match self.bits {
            false => CAL_FLAG1_A::Lower,
            true => CAL_FLAG1_A::EqualHigher,
        }
    }
    ///Checks if the value of the field is `Lower`
    #[inline(always)]
    pub fn is_lower(&self) -> bool {
        *self == CAL_FLAG1_A::Lower
    }
    ///Checks if the value of the field is `EqualHigher`
    #[inline(always)]
    pub fn is_equal_higher(&self) -> bool {
        *self == CAL_FLAG1_A::EqualHigher
    }
}
///Field `BWST1` reader - DAC Channel 1 busy writing sample time flag
pub type BWST1_R = crate::BitReader<BWST1_A>;
///DAC Channel 1 busy writing sample time flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWST1_A {
    ///0: There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written
    Idle = 0,
    ///1: There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written
    Busy = 1,
}
impl From<BWST1_A> for bool {
    #[inline(always)]
    fn from(variant: BWST1_A) -> Self {
        variant as u8 != 0
    }
}
impl BWST1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BWST1_A {
        match self.bits {
            false => BWST1_A::Idle,
            true => BWST1_A::Busy,
        }
    }
    ///Checks if the value of the field is `Idle`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BWST1_A::Idle
    }
    ///Checks if the value of the field is `Busy`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BWST1_A::Busy
    }
}
///Field `BWST2` reader - DAC Channel 2 busy writing sample time flag
pub use BWST1_R as BWST2_R;
///Field `CAL_FLAG2` reader - DAC Channel 2 calibration offset status
pub use CAL_FLAG1_R as CAL_FLAG2_R;
///Field `DMAUDR2` reader - DAC channel2 DMA underrun flag
pub use DMAUDR1_R as DMAUDR2_R;
///Field `DMAUDR2` writer - DAC channel2 DMA underrun flag
pub use DMAUDR1_W as DMAUDR2_W;
impl R {
    ///Bit 13 - DAC channel1 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DAC Channel 1 calibration offset status
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DAC Channel 1 busy writing sample time flag
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 29 - DAC channel2 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DAC Channel 2 calibration offset status
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DAC Channel 2 busy writing sample time flag
    #[inline(always)]
    pub fn bwst2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 13 - DAC channel1 DMA underrun flag
    #[inline(always)]
    #[must_use]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<13> {
        DMAUDR1_W::new(self)
    }
    ///Bit 29 - DAC channel2 DMA underrun flag
    #[inline(always)]
    #[must_use]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W<29> {
        DMAUDR2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
