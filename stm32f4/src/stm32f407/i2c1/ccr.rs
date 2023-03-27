///Register `CCR` reader
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR` writer
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCR` reader - Clock control register in Fast/Standard mode (Master mode)
pub type CCR_R = crate::FieldReader<u16, u16>;
///Field `CCR` writer - Clock control register in Fast/Standard mode (Master mode)
pub type CCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u16, u16, 12, O>;
///Field `DUTY` reader - Fast mode duty cycle
pub type DUTY_R = crate::BitReader<DUTY_A>;
///Fast mode duty cycle
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUTY_A {
    ///0: Duty cycle t_low/t_high = 2/1
    Duty21 = 0,
    ///1: Duty cycle t_low/t_high = 16/9
    Duty169 = 1,
}
impl From<DUTY_A> for bool {
    #[inline(always)]
    fn from(variant: DUTY_A) -> Self {
        variant as u8 != 0
    }
}
impl DUTY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DUTY_A {
        match self.bits {
            false => DUTY_A::Duty21,
            true => DUTY_A::Duty169,
        }
    }
    ///Checks if the value of the field is `Duty21`
    #[inline(always)]
    pub fn is_duty2_1(&self) -> bool {
        *self == DUTY_A::Duty21
    }
    ///Checks if the value of the field is `Duty169`
    #[inline(always)]
    pub fn is_duty16_9(&self) -> bool {
        *self == DUTY_A::Duty169
    }
}
///Field `DUTY` writer - Fast mode duty cycle
pub type DUTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, DUTY_A, O>;
impl<'a, const O: u8> DUTY_W<'a, O> {
    ///Duty cycle t_low/t_high = 2/1
    #[inline(always)]
    pub fn duty2_1(self) -> &'a mut W {
        self.variant(DUTY_A::Duty21)
    }
    ///Duty cycle t_low/t_high = 16/9
    #[inline(always)]
    pub fn duty16_9(self) -> &'a mut W {
        self.variant(DUTY_A::Duty169)
    }
}
///Field `F_S` reader - I2C master mode selection
pub type F_S_R = crate::BitReader<F_S_A>;
///I2C master mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F_S_A {
    ///0: Standard mode I2C
    Standard = 0,
    ///1: Fast mode I2C
    Fast = 1,
}
impl From<F_S_A> for bool {
    #[inline(always)]
    fn from(variant: F_S_A) -> Self {
        variant as u8 != 0
    }
}
impl F_S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> F_S_A {
        match self.bits {
            false => F_S_A::Standard,
            true => F_S_A::Fast,
        }
    }
    ///Checks if the value of the field is `Standard`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == F_S_A::Standard
    }
    ///Checks if the value of the field is `Fast`
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == F_S_A::Fast
    }
}
///Field `F_S` writer - I2C master mode selection
pub type F_S_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, F_S_A, O>;
impl<'a, const O: u8> F_S_W<'a, O> {
    ///Standard mode I2C
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(F_S_A::Standard)
    }
    ///Fast mode I2C
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(F_S_A::Fast)
    }
}
impl R {
    ///Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 14 - Fast mode duty cycle
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - I2C master mode selection
    #[inline(always)]
    pub fn f_s(&self) -> F_S_R {
        F_S_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - Clock control register in Fast/Standard mode (Master mode)
    #[inline(always)]
    #[must_use]
    pub fn ccr(&mut self) -> CCR_W<0> {
        CCR_W::new(self)
    }
    ///Bit 14 - Fast mode duty cycle
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<14> {
        DUTY_W::new(self)
    }
    ///Bit 15 - I2C master mode selection
    #[inline(always)]
    #[must_use]
    pub fn f_s(&mut self) -> F_S_W<15> {
        F_S_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr](index.html) module
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr::R](R) reader structure
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr::W](W) writer structure
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
