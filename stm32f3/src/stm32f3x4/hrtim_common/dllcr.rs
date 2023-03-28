///Register `DLLCR` reader
pub struct R(crate::R<DLLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLLCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DLLCR` writer
pub struct W(crate::W<DLLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLLCR_SPEC>;
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
impl From<crate::W<DLLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLLCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CAL` reader - DLL Calibration Start
pub type CAL_R = crate::BitReader<CAL_A>;
///DLL Calibration Start
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL_A {
    ///1: Calibration start
    Start = 1,
}
impl From<CAL_A> for bool {
    #[inline(always)]
    fn from(variant: CAL_A) -> Self {
        variant as u8 != 0
    }
}
impl CAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CAL_A> {
        match self.bits {
            true => Some(CAL_A::Start),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Start`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CAL_A::Start
    }
}
///Field `CAL` writer - DLL Calibration Start
pub type CAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCR_SPEC, CAL_A, O>;
impl<'a, const O: u8> CAL_W<'a, O> {
    ///Calibration start
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CAL_A::Start)
    }
}
///Field `CALEN` reader - DLL Calibration Enable
pub type CALEN_R = crate::BitReader<CALEN_A>;
///DLL Calibration Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALEN_A {
    ///0: Periodic calibration disabled
    Disabled = 0,
    ///1: Calibration is performed periodically, as per CALRTE setting
    Enabled = 1,
}
impl From<CALEN_A> for bool {
    #[inline(always)]
    fn from(variant: CALEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CALEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CALEN_A {
        match self.bits {
            false => CALEN_A::Disabled,
            true => CALEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALEN_A::Enabled
    }
}
///Field `CALEN` writer - DLL Calibration Enable
pub type CALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLLCR_SPEC, CALEN_A, O>;
impl<'a, const O: u8> CALEN_W<'a, O> {
    ///Periodic calibration disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CALEN_A::Disabled)
    }
    ///Calibration is performed periodically, as per CALRTE setting
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CALEN_A::Enabled)
    }
}
///Field `CALRTE` reader - DLL Calibration rate
pub type CALRTE_R = crate::FieldReader<u8, CALRTE_A>;
///DLL Calibration rate
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALRTE_A {
    ///0: 1048576*t_HRTIM (7.3ms)
    Millis73 = 0,
    ///1: 131072*t_HRTIM (910µs)
    Micros910 = 1,
    ///2: 16384*t_HRTIM (114µs)
    Micros114 = 2,
    ///3: 2048*t_HRTIM (14µs)
    Micros14 = 3,
}
impl From<CALRTE_A> for u8 {
    #[inline(always)]
    fn from(variant: CALRTE_A) -> Self {
        variant as _
    }
}
impl CALRTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CALRTE_A {
        match self.bits {
            0 => CALRTE_A::Millis73,
            1 => CALRTE_A::Micros910,
            2 => CALRTE_A::Micros114,
            3 => CALRTE_A::Micros14,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Millis73`
    #[inline(always)]
    pub fn is_millis7_3(&self) -> bool {
        *self == CALRTE_A::Millis73
    }
    ///Checks if the value of the field is `Micros910`
    #[inline(always)]
    pub fn is_micros910(&self) -> bool {
        *self == CALRTE_A::Micros910
    }
    ///Checks if the value of the field is `Micros114`
    #[inline(always)]
    pub fn is_micros114(&self) -> bool {
        *self == CALRTE_A::Micros114
    }
    ///Checks if the value of the field is `Micros14`
    #[inline(always)]
    pub fn is_micros14(&self) -> bool {
        *self == CALRTE_A::Micros14
    }
}
///Field `CALRTE` writer - DLL Calibration rate
pub type CALRTE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DLLCR_SPEC, u8, CALRTE_A, 2, O>;
impl<'a, const O: u8> CALRTE_W<'a, O> {
    ///1048576*t_HRTIM (7.3ms)
    #[inline(always)]
    pub fn millis7_3(self) -> &'a mut W {
        self.variant(CALRTE_A::Millis73)
    }
    ///131072*t_HRTIM (910µs)
    #[inline(always)]
    pub fn micros910(self) -> &'a mut W {
        self.variant(CALRTE_A::Micros910)
    }
    ///16384*t_HRTIM (114µs)
    #[inline(always)]
    pub fn micros114(self) -> &'a mut W {
        self.variant(CALRTE_A::Micros114)
    }
    ///2048*t_HRTIM (14µs)
    #[inline(always)]
    pub fn micros14(self) -> &'a mut W {
        self.variant(CALRTE_A::Micros14)
    }
}
impl R {
    ///Bit 0 - DLL Calibration Start
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DLL Calibration Enable
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - DLL Calibration rate
    #[inline(always)]
    pub fn calrte(&self) -> CALRTE_R {
        CALRTE_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - DLL Calibration Start
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<0> {
        CAL_W::new(self)
    }
    ///Bit 1 - DLL Calibration Enable
    #[inline(always)]
    #[must_use]
    pub fn calen(&mut self) -> CALEN_W<1> {
        CALEN_W::new(self)
    }
    ///Bits 2:3 - DLL Calibration rate
    #[inline(always)]
    #[must_use]
    pub fn calrte(&mut self) -> CALRTE_W<2> {
        CALRTE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DLL Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dllcr](index.html) module
pub struct DLLCR_SPEC;
impl crate::RegisterSpec for DLLCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dllcr::R](R) reader structure
impl crate::Readable for DLLCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dllcr::W](W) writer structure
impl crate::Writable for DLLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DLLCR to value 0
impl crate::Resettable for DLLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
