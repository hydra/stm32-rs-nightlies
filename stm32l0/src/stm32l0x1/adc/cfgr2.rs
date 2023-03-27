///Register `CFGR2` reader
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR2` writer
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OVSE` reader - Oversampler Enable
pub type OVSE_R = crate::BitReader<OVSE_A>;
///Oversampler Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVSE_A {
    ///0: Oversampler disabled
    Disabled = 0,
    ///1: Oversampler enabled
    Enabled = 1,
}
impl From<OVSE_A> for bool {
    #[inline(always)]
    fn from(variant: OVSE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVSE_A {
        match self.bits {
            false => OVSE_A::Disabled,
            true => OVSE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVSE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVSE_A::Enabled
    }
}
///Field `OVSE` writer - Oversampler Enable
pub type OVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, OVSE_A, O>;
impl<'a, const O: u8> OVSE_W<'a, O> {
    ///Oversampler disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVSE_A::Disabled)
    }
    ///Oversampler enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVSE_A::Enabled)
    }
}
///Field `OVSR` reader - Oversampling ratio
pub type OVSR_R = crate::FieldReader<u8, OVSR_A>;
///Oversampling ratio
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR_A {
    ///0: 2x
    Mul2 = 0,
    ///1: 4x
    Mul4 = 1,
    ///2: 8x
    Mul8 = 2,
    ///3: 16x
    Mul16 = 3,
    ///4: 32x
    Mul32 = 4,
    ///5: 64x
    Mul64 = 5,
    ///6: 128x
    Mul128 = 6,
    ///7: 256x
    Mul256 = 7,
}
impl From<OVSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSR_A) -> Self {
        variant as _
    }
}
impl OVSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVSR_A {
        match self.bits {
            0 => OVSR_A::Mul2,
            1 => OVSR_A::Mul4,
            2 => OVSR_A::Mul8,
            3 => OVSR_A::Mul16,
            4 => OVSR_A::Mul32,
            5 => OVSR_A::Mul64,
            6 => OVSR_A::Mul128,
            7 => OVSR_A::Mul256,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Mul2`
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == OVSR_A::Mul2
    }
    ///Checks if the value of the field is `Mul4`
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == OVSR_A::Mul4
    }
    ///Checks if the value of the field is `Mul8`
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == OVSR_A::Mul8
    }
    ///Checks if the value of the field is `Mul16`
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == OVSR_A::Mul16
    }
    ///Checks if the value of the field is `Mul32`
    #[inline(always)]
    pub fn is_mul32(&self) -> bool {
        *self == OVSR_A::Mul32
    }
    ///Checks if the value of the field is `Mul64`
    #[inline(always)]
    pub fn is_mul64(&self) -> bool {
        *self == OVSR_A::Mul64
    }
    ///Checks if the value of the field is `Mul128`
    #[inline(always)]
    pub fn is_mul128(&self) -> bool {
        *self == OVSR_A::Mul128
    }
    ///Checks if the value of the field is `Mul256`
    #[inline(always)]
    pub fn is_mul256(&self) -> bool {
        *self == OVSR_A::Mul256
    }
}
///Field `OVSR` writer - Oversampling ratio
pub type OVSR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR2_SPEC, u8, OVSR_A, 3, O>;
impl<'a, const O: u8> OVSR_W<'a, O> {
    ///2x
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(OVSR_A::Mul2)
    }
    ///4x
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(OVSR_A::Mul4)
    }
    ///8x
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(OVSR_A::Mul8)
    }
    ///16x
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(OVSR_A::Mul16)
    }
    ///32x
    #[inline(always)]
    pub fn mul32(self) -> &'a mut W {
        self.variant(OVSR_A::Mul32)
    }
    ///64x
    #[inline(always)]
    pub fn mul64(self) -> &'a mut W {
        self.variant(OVSR_A::Mul64)
    }
    ///128x
    #[inline(always)]
    pub fn mul128(self) -> &'a mut W {
        self.variant(OVSR_A::Mul128)
    }
    ///256x
    #[inline(always)]
    pub fn mul256(self) -> &'a mut W {
        self.variant(OVSR_A::Mul256)
    }
}
///Field `OVSS` reader - Oversampling shift
pub type OVSS_R = crate::FieldReader<u8, u8>;
///Field `OVSS` writer - Oversampling shift
pub type OVSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 4, O>;
///Field `TOVS` reader - Triggered Oversampling
pub type TOVS_R = crate::BitReader<TOVS_A>;
///Triggered Oversampling
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOVS_A {
    ///0: All oversampled conversions for a channel are done consecutively after a trigger
    TriggerAll = 0,
    ///1: Each oversampled conversion for a channel needs a trigger
    TriggerEach = 1,
}
impl From<TOVS_A> for bool {
    #[inline(always)]
    fn from(variant: TOVS_A) -> Self {
        variant as u8 != 0
    }
}
impl TOVS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TOVS_A {
        match self.bits {
            false => TOVS_A::TriggerAll,
            true => TOVS_A::TriggerEach,
        }
    }
    ///Checks if the value of the field is `TriggerAll`
    #[inline(always)]
    pub fn is_trigger_all(&self) -> bool {
        *self == TOVS_A::TriggerAll
    }
    ///Checks if the value of the field is `TriggerEach`
    #[inline(always)]
    pub fn is_trigger_each(&self) -> bool {
        *self == TOVS_A::TriggerEach
    }
}
///Field `TOVS` writer - Triggered Oversampling
pub type TOVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, TOVS_A, O>;
impl<'a, const O: u8> TOVS_W<'a, O> {
    ///All oversampled conversions for a channel are done consecutively after a trigger
    #[inline(always)]
    pub fn trigger_all(self) -> &'a mut W {
        self.variant(TOVS_A::TriggerAll)
    }
    ///Each oversampled conversion for a channel needs a trigger
    #[inline(always)]
    pub fn trigger_each(self) -> &'a mut W {
        self.variant(TOVS_A::TriggerEach)
    }
}
///Field `CKMODE` reader - ADC clock mode
pub type CKMODE_R = crate::FieldReader<u8, CKMODE_A>;
///ADC clock mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE_A {
    ///0: ADCCLK (Asynchronous clock mode)
    Adclk = 0,
    ///1: PCLK/2 (Synchronous clock mode)
    PclkDiv2 = 1,
    ///2: PCLK/4 (Synchronous clock mode)
    PclkDiv4 = 2,
    ///3: PCLK (Synchronous clock mode)
    Pclk = 3,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
impl CKMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CKMODE_A {
        match self.bits {
            0 => CKMODE_A::Adclk,
            1 => CKMODE_A::PclkDiv2,
            2 => CKMODE_A::PclkDiv4,
            3 => CKMODE_A::Pclk,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Adclk`
    #[inline(always)]
    pub fn is_adclk(&self) -> bool {
        *self == CKMODE_A::Adclk
    }
    ///Checks if the value of the field is `PclkDiv2`
    #[inline(always)]
    pub fn is_pclk_div2(&self) -> bool {
        *self == CKMODE_A::PclkDiv2
    }
    ///Checks if the value of the field is `PclkDiv4`
    #[inline(always)]
    pub fn is_pclk_div4(&self) -> bool {
        *self == CKMODE_A::PclkDiv4
    }
    ///Checks if the value of the field is `Pclk`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == CKMODE_A::Pclk
    }
}
///Field `CKMODE` writer - ADC clock mode
pub type CKMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR2_SPEC, u8, CKMODE_A, 2, O>;
impl<'a, const O: u8> CKMODE_W<'a, O> {
    ///ADCCLK (Asynchronous clock mode)
    #[inline(always)]
    pub fn adclk(self) -> &'a mut W {
        self.variant(CKMODE_A::Adclk)
    }
    ///PCLK/2 (Synchronous clock mode)
    #[inline(always)]
    pub fn pclk_div2(self) -> &'a mut W {
        self.variant(CKMODE_A::PclkDiv2)
    }
    ///PCLK/4 (Synchronous clock mode)
    #[inline(always)]
    pub fn pclk_div4(self) -> &'a mut W {
        self.variant(CKMODE_A::PclkDiv4)
    }
    ///PCLK (Synchronous clock mode)
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(CKMODE_A::Pclk)
    }
}
impl R {
    ///Bit 0 - Oversampler Enable
    #[inline(always)]
    pub fn ovse(&self) -> OVSE_R {
        OVSE_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:4 - Oversampling ratio
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:8 - Oversampling shift
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - Triggered Oversampling
    #[inline(always)]
    pub fn tovs(&self) -> TOVS_R {
        TOVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 30:31 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Oversampler Enable
    #[inline(always)]
    #[must_use]
    pub fn ovse(&mut self) -> OVSE_W<0> {
        OVSE_W::new(self)
    }
    ///Bits 2:4 - Oversampling ratio
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<2> {
        OVSR_W::new(self)
    }
    ///Bits 5:8 - Oversampling shift
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<5> {
        OVSS_W::new(self)
    }
    ///Bit 9 - Triggered Oversampling
    #[inline(always)]
    #[must_use]
    pub fn tovs(&mut self) -> TOVS_W<9> {
        TOVS_W::new(self)
    }
    ///Bits 30:31 - ADC clock mode
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<30> {
        CKMODE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr2](index.html) module
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr2::R](R) reader structure
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr2::W](W) writer structure
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
