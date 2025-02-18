///Register `SMPR1` reader
pub struct R(crate::R<SMPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SMPR1` writer
pub struct W(crate::W<SMPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR1_SPEC>;
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
impl From<crate::W<SMPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SMPx_x` reader - Sample time bits
pub type SMPX_X_R = crate::FieldReader<u32, SMPX_X_A>;
///Sample time bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum SMPX_X_A {
    ///0: 1.5 ADC clock cycles
    Cycles15 = 0,
    ///1: 7.5 ADC clock cycles
    Cycles75 = 1,
    ///2: 13.5 ADC clock cycles
    Cycles135 = 2,
    ///3: 28.5 ADC clock cycles
    Cycles285 = 3,
    ///4: 41.5 ADC clock cycles
    Cycles415 = 4,
    ///5: 55.5 ADC clock cycles
    Cycles555 = 5,
    ///6: 71.5 ADC clock cycles
    Cycles715 = 6,
    ///7: 239.5 ADC clock cycles
    Cycles2395 = 7,
}
impl From<SMPX_X_A> for u32 {
    #[inline(always)]
    fn from(variant: SMPX_X_A) -> Self {
        variant as _
    }
}
impl SMPX_X_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SMPX_X_A> {
        match self.bits {
            0 => Some(SMPX_X_A::Cycles15),
            1 => Some(SMPX_X_A::Cycles75),
            2 => Some(SMPX_X_A::Cycles135),
            3 => Some(SMPX_X_A::Cycles285),
            4 => Some(SMPX_X_A::Cycles415),
            5 => Some(SMPX_X_A::Cycles555),
            6 => Some(SMPX_X_A::Cycles715),
            7 => Some(SMPX_X_A::Cycles2395),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Cycles15`
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMPX_X_A::Cycles15
    }
    ///Checks if the value of the field is `Cycles75`
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMPX_X_A::Cycles75
    }
    ///Checks if the value of the field is `Cycles135`
    #[inline(always)]
    pub fn is_cycles13_5(&self) -> bool {
        *self == SMPX_X_A::Cycles135
    }
    ///Checks if the value of the field is `Cycles285`
    #[inline(always)]
    pub fn is_cycles28_5(&self) -> bool {
        *self == SMPX_X_A::Cycles285
    }
    ///Checks if the value of the field is `Cycles415`
    #[inline(always)]
    pub fn is_cycles41_5(&self) -> bool {
        *self == SMPX_X_A::Cycles415
    }
    ///Checks if the value of the field is `Cycles555`
    #[inline(always)]
    pub fn is_cycles55_5(&self) -> bool {
        *self == SMPX_X_A::Cycles555
    }
    ///Checks if the value of the field is `Cycles715`
    #[inline(always)]
    pub fn is_cycles71_5(&self) -> bool {
        *self == SMPX_X_A::Cycles715
    }
    ///Checks if the value of the field is `Cycles2395`
    #[inline(always)]
    pub fn is_cycles239_5(&self) -> bool {
        *self == SMPX_X_A::Cycles2395
    }
}
///Field `SMPx_x` writer - Sample time bits
pub type SMPX_X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR1_SPEC, u32, SMPX_X_A, 32, O>;
impl<'a, const O: u8> SMPX_X_W<'a, O> {
    ///1.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles15)
    }
    ///7.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles75)
    }
    ///13.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles13_5(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles135)
    }
    ///28.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles28_5(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles285)
    }
    ///41.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles41_5(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles415)
    }
    ///55.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles55_5(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles555)
    }
    ///71.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles71_5(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles715)
    }
    ///239.5 ADC clock cycles
    #[inline(always)]
    pub fn cycles239_5(self) -> &'a mut W {
        self.variant(SMPX_X_A::Cycles2395)
    }
}
impl R {
    ///Bits 0:31 - Sample time bits
    #[inline(always)]
    pub fn smpx_x(&self) -> SMPX_X_R {
        SMPX_X_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Sample time bits
    #[inline(always)]
    #[must_use]
    pub fn smpx_x(&mut self) -> SMPX_X_W<0> {
        SMPX_X_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///sample time register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smpr1](index.html) module
pub struct SMPR1_SPEC;
impl crate::RegisterSpec for SMPR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [smpr1::R](R) reader structure
impl crate::Readable for SMPR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [smpr1::W](W) writer structure
impl crate::Writable for SMPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SMPR1 to value 0
impl crate::Resettable for SMPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
