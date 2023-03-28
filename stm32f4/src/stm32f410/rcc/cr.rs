///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HSION` reader - Internal high-speed clock enable
pub type HSION_R = crate::BitReader<HSION_A>;
///Internal high-speed clock enable
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION_A {
    ///0: Clock Off
    Off = 0,
    ///1: Clock On
    On = 1,
}
impl From<HSION_A> for bool {
    #[inline(always)]
    fn from(variant: HSION_A) -> Self {
        variant as u8 != 0
    }
}
impl HSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSION_A {
        match self.bits {
            false => HSION_A::Off,
            true => HSION_A::On,
        }
    }
    ///Checks if the value of the field is `Off`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HSION_A::Off
    }
    ///Checks if the value of the field is `On`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == HSION_A::On
    }
}
///Field `HSION` writer - Internal high-speed clock enable
pub type HSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSION_A, O>;
impl<'a, const O: u8> HSION_W<'a, O> {
    ///Clock Off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HSION_A::Off)
    }
    ///Clock On
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(HSION_A::On)
    }
}
///Field `HSIRDY` reader - Internal high-speed clock ready flag
pub type HSIRDY_R = crate::BitReader<HSIRDYR_A>;
///Internal high-speed clock ready flag
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDYR_A {
    ///0: Clock not ready
    NotReady = 0,
    ///1: Clock ready
    Ready = 1,
}
impl From<HSIRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSIRDYR_A {
        match self.bits {
            false => HSIRDYR_A::NotReady,
            true => HSIRDYR_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDYR_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDYR_A::Ready
    }
}
///Field `HSITRIM` reader - Internal high-speed clock trimming
pub type HSITRIM_R = crate::FieldReader<u8, u8>;
///Field `HSITRIM` writer - Internal high-speed clock trimming
pub type HSITRIM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, u8, 5, O>;
///Field `HSICAL` reader - Internal high-speed clock calibration
pub type HSICAL_R = crate::FieldReader<u8, u8>;
///Field `HSEON` reader - HSE clock enable
pub use HSION_R as HSEON_R;
///Field `HSEON` writer - HSE clock enable
pub use HSION_W as HSEON_W;
///Field `HSERDY` reader - HSE clock ready flag
pub use HSIRDY_R as HSERDY_R;
///Field `HSEBYP` reader - HSE clock bypass
pub type HSEBYP_R = crate::BitReader<HSEBYP_A>;
///HSE clock bypass
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP_A {
    ///0: HSE crystal oscillator not bypassed
    NotBypassed = 0,
    ///1: HSE crystal oscillator bypassed with external clock
    Bypassed = 1,
}
impl From<HSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSEBYP_A {
        match self.bits {
            false => HSEBYP_A::NotBypassed,
            true => HSEBYP_A::Bypassed,
        }
    }
    ///Checks if the value of the field is `NotBypassed`
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP_A::NotBypassed
    }
    ///Checks if the value of the field is `Bypassed`
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP_A::Bypassed
    }
}
///Field `HSEBYP` writer - HSE clock bypass
pub type HSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSEBYP_A, O>;
impl<'a, const O: u8> HSEBYP_W<'a, O> {
    ///HSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::NotBypassed)
    }
    ///HSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::Bypassed)
    }
}
///Field `CSSON` reader - Clock security system enable
pub type CSSON_R = crate::BitReader<CSSON_A>;
///Clock security system enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON_A {
    ///0: Clock security system disabled (clock detector OFF)
    Off = 0,
    ///1: Clock security system enable (clock detector ON if the HSE is ready, OFF if not)
    On = 1,
}
impl From<CSSON_A> for bool {
    #[inline(always)]
    fn from(variant: CSSON_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSON_A {
        match self.bits {
            false => CSSON_A::Off,
            true => CSSON_A::On,
        }
    }
    ///Checks if the value of the field is `Off`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CSSON_A::Off
    }
    ///Checks if the value of the field is `On`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CSSON_A::On
    }
}
///Field `CSSON` writer - Clock security system enable
pub type CSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CSSON_A, O>;
impl<'a, const O: u8> CSSON_W<'a, O> {
    ///Clock security system disabled (clock detector OFF)
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CSSON_A::Off)
    }
    ///Clock security system enable (clock detector ON if the HSE is ready, OFF if not)
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CSSON_A::On)
    }
}
///Field `PLLON` reader - Main PLL (PLL) enable
pub use HSION_R as PLLON_R;
///Field `PLLON` writer - Main PLL (PLL) enable
pub use HSION_W as PLLON_W;
///Field `PLLRDY` reader - Main PLL (PLL) clock ready flag
pub use HSIRDY_R as PLLRDY_R;
impl R {
    ///Bit 0 - Internal high-speed clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal high-speed clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 3:7 - Internal high-speed clock trimming
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    ///Bits 8:15 - Internal high-speed clock calibration
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSE clock bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - Main PLL (PLL) enable
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Main PLL (PLL) clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Internal high-speed clock enable
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<0> {
        HSION_W::new(self)
    }
    ///Bits 3:7 - Internal high-speed clock trimming
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<3> {
        HSITRIM_W::new(self)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<16> {
        HSEON_W::new(self)
    }
    ///Bit 18 - HSE clock bypass
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<18> {
        HSEBYP_W::new(self)
    }
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<19> {
        CSSON_W::new(self)
    }
    ///Bit 24 - Main PLL (PLL) enable
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<24> {
        PLLON_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///clock control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0x83
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x83;
}
