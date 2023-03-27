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
///Field `HSI16ON` reader - 16 MHz high-speed internal clock enable
pub type HSI16ON_R = crate::BitReader<HSI16ON_A>;
///16 MHz high-speed internal clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16ON_A {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<HSI16ON_A> for bool {
    #[inline(always)]
    fn from(variant: HSI16ON_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI16ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSI16ON_A {
        match self.bits {
            false => HSI16ON_A::Disabled,
            true => HSI16ON_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI16ON_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI16ON_A::Enabled
    }
}
///Field `HSI16ON` writer - 16 MHz high-speed internal clock enable
pub type HSI16ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSI16ON_A, O>;
impl<'a, const O: u8> HSI16ON_W<'a, O> {
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSI16ON_A::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSI16ON_A::Enabled)
    }
}
///Field `HSI16KERON` reader - High-speed internal clock enable bit for some IP kernels
pub use HSI16ON_R as HSI16KERON_R;
///Field `HSI16RDYF` reader - Internal high-speed clock ready flag
pub type HSI16RDYF_R = crate::BitReader<HSI16RDYFR_A>;
///Internal high-speed clock ready flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16RDYFR_A {
    ///0: HSI 16 MHz oscillator not ready
    NotReady = 0,
    ///1: HSI 16 MHz oscillator ready
    Ready = 1,
}
impl From<HSI16RDYFR_A> for bool {
    #[inline(always)]
    fn from(variant: HSI16RDYFR_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI16RDYF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSI16RDYFR_A {
        match self.bits {
            false => HSI16RDYFR_A::NotReady,
            true => HSI16RDYFR_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSI16RDYFR_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSI16RDYFR_A::Ready
    }
}
///Field `HSI16RDYF` writer - Internal high-speed clock ready flag
pub type HSI16RDYF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSI16RDYFR_A, O>;
impl<'a, const O: u8> HSI16RDYF_W<'a, O> {
    ///HSI 16 MHz oscillator not ready
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(HSI16RDYFR_A::NotReady)
    }
    ///HSI 16 MHz oscillator ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(HSI16RDYFR_A::Ready)
    }
}
///Field `HSI16DIVEN` reader - HSI16DIVEN
pub type HSI16DIVEN_R = crate::BitReader<HSI16DIVEN_A>;
///HSI16DIVEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16DIVEN_A {
    ///0: no 16 MHz HSI division requested
    NotDivided = 0,
    ///1: 16 MHz HSI division by 4 requested
    Div4 = 1,
}
impl From<HSI16DIVEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSI16DIVEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI16DIVEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSI16DIVEN_A {
        match self.bits {
            false => HSI16DIVEN_A::NotDivided,
            true => HSI16DIVEN_A::Div4,
        }
    }
    ///Checks if the value of the field is `NotDivided`
    #[inline(always)]
    pub fn is_not_divided(&self) -> bool {
        *self == HSI16DIVEN_A::NotDivided
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSI16DIVEN_A::Div4
    }
}
///Field `HSI16DIVEN` writer - HSI16DIVEN
pub type HSI16DIVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSI16DIVEN_A, O>;
impl<'a, const O: u8> HSI16DIVEN_W<'a, O> {
    ///no 16 MHz HSI division requested
    #[inline(always)]
    pub fn not_divided(self) -> &'a mut W {
        self.variant(HSI16DIVEN_A::NotDivided)
    }
    ///16 MHz HSI division by 4 requested
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HSI16DIVEN_A::Div4)
    }
}
///Field `HSI16DIVF` reader - HSI16DIVF
pub type HSI16DIVF_R = crate::BitReader<HSI16DIVFR_A>;
///HSI16DIVF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16DIVFR_A {
    ///0: 16 MHz HSI clock not divided
    NotDivided = 0,
    ///1: 16 MHz HSI clock divided by 4
    Div4 = 1,
}
impl From<HSI16DIVFR_A> for bool {
    #[inline(always)]
    fn from(variant: HSI16DIVFR_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI16DIVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSI16DIVFR_A {
        match self.bits {
            false => HSI16DIVFR_A::NotDivided,
            true => HSI16DIVFR_A::Div4,
        }
    }
    ///Checks if the value of the field is `NotDivided`
    #[inline(always)]
    pub fn is_not_divided(&self) -> bool {
        *self == HSI16DIVFR_A::NotDivided
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HSI16DIVFR_A::Div4
    }
}
///Field `HSI16OUTEN` reader - 16 MHz high-speed internal clock output enable
pub type HSI16OUTEN_R = crate::BitReader<HSI16OUTEN_A>;
///16 MHz high-speed internal clock output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSI16OUTEN_A {
    ///0: HSI output clock disabled
    Disabled = 0,
    ///1: HSI output clock enabled
    Enabled = 1,
}
impl From<HSI16OUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: HSI16OUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HSI16OUTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSI16OUTEN_A {
        match self.bits {
            false => HSI16OUTEN_A::Disabled,
            true => HSI16OUTEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSI16OUTEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSI16OUTEN_A::Enabled
    }
}
///Field `HSI16OUTEN` writer - 16 MHz high-speed internal clock output enable
pub type HSI16OUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSI16OUTEN_A, O>;
impl<'a, const O: u8> HSI16OUTEN_W<'a, O> {
    ///HSI output clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSI16OUTEN_A::Disabled)
    }
    ///HSI output clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSI16OUTEN_A::Enabled)
    }
}
///Field `MSION` reader - MSI clock enable bit
pub use HSI16ON_R as MSION_R;
///Field `MSION` writer - MSI clock enable bit
pub use HSI16ON_W as MSION_W;
///Field `MSIRDY` reader - MSI clock ready flag
pub type MSIRDY_R = crate::BitReader<MSIRDYR_A>;
///MSI clock ready flag
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDYR_A {
    ///0: Oscillator is not stable
    NotReady = 0,
    ///1: Oscillator is stable
    Ready = 1,
}
impl From<MSIRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: MSIRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl MSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSIRDYR_A {
        match self.bits {
            false => MSIRDYR_A::NotReady,
            true => MSIRDYR_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == MSIRDYR_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == MSIRDYR_A::Ready
    }
}
///Field `HSEON` reader - HSE clock enable bit
pub use HSI16ON_R as HSEON_R;
///Field `HSEON` writer - HSE clock enable bit
pub use HSI16ON_W as HSEON_W;
///Field `HSERDY` reader - HSE clock ready flag
pub use MSIRDY_R as HSERDY_R;
///Field `HSEBYP` reader - HSE clock bypass bit
pub type HSEBYP_R = crate::BitReader<HSEBYP_A>;
///HSE clock bypass bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP_A {
    ///0: HSE oscillator not bypassed
    NotBypassed = 0,
    ///1: HSE oscillator bypassed
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
///Field `HSEBYP` writer - HSE clock bypass bit
pub type HSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSEBYP_A, O>;
impl<'a, const O: u8> HSEBYP_W<'a, O> {
    ///HSE oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::NotBypassed)
    }
    ///HSE oscillator bypassed
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::Bypassed)
    }
}
///Field `CSSHSEON` reader - Clock security system on HSE enable bit
pub use HSI16ON_R as CSSHSEON_R;
///Field `CSSHSEON` writer - Clock security system on HSE enable bit
pub use HSI16ON_W as CSSHSEON_W;
///Field `RTCPRE` reader - TC/LCD prescaler
pub type RTCPRE_R = crate::FieldReader<u8, RTCPRE_A>;
///TC/LCD prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCPRE_A {
    ///0: HSE divided by 2
    Div2 = 0,
    ///1: HSE divided by 4
    Div4 = 1,
    ///2: HSE divided by 8
    Div8 = 2,
    ///3: HSE divided by 16
    Div16 = 3,
}
impl From<RTCPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCPRE_A) -> Self {
        variant as _
    }
}
impl RTCPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTCPRE_A {
        match self.bits {
            0 => RTCPRE_A::Div2,
            1 => RTCPRE_A::Div4,
            2 => RTCPRE_A::Div8,
            3 => RTCPRE_A::Div16,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == RTCPRE_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == RTCPRE_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == RTCPRE_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == RTCPRE_A::Div16
    }
}
///Field `RTCPRE` writer - TC/LCD prescaler
pub type RTCPRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, RTCPRE_A, 2, O>;
impl<'a, const O: u8> RTCPRE_W<'a, O> {
    ///HSE divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(RTCPRE_A::Div2)
    }
    ///HSE divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(RTCPRE_A::Div4)
    }
    ///HSE divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(RTCPRE_A::Div8)
    }
    ///HSE divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(RTCPRE_A::Div16)
    }
}
///Field `PLLON` reader - PLL enable bit
pub use HSI16ON_R as PLLON_R;
///Field `PLLON` writer - PLL enable bit
pub use HSI16ON_W as PLLON_W;
///Field `PLLRDY` reader - PLL clock ready flag
pub type PLLRDY_R = crate::BitReader<PLLRDYR_A>;
///PLL clock ready flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDYR_A {
    ///0: PLL unlocked
    Unlocked = 0,
    ///1: PLL locked
    Locked = 1,
}
impl From<PLLRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLRDYR_A {
        match self.bits {
            false => PLLRDYR_A::Unlocked,
            true => PLLRDYR_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLRDYR_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLRDYR_A::Locked
    }
}
impl R {
    ///Bit 0 - 16 MHz high-speed internal clock enable
    #[inline(always)]
    pub fn hsi16on(&self) -> HSI16ON_R {
        HSI16ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - High-speed internal clock enable bit for some IP kernels
    #[inline(always)]
    pub fn hsi16keron(&self) -> HSI16KERON_R {
        HSI16KERON_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Internal high-speed clock ready flag
    #[inline(always)]
    pub fn hsi16rdyf(&self) -> HSI16RDYF_R {
        HSI16RDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI16DIVEN
    #[inline(always)]
    pub fn hsi16diven(&self) -> HSI16DIVEN_R {
        HSI16DIVEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSI16DIVF
    #[inline(always)]
    pub fn hsi16divf(&self) -> HSI16DIVF_R {
        HSI16DIVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - 16 MHz high-speed internal clock output enable
    #[inline(always)]
    pub fn hsi16outen(&self) -> HSI16OUTEN_R {
        HSI16OUTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - MSI clock enable bit
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MSI clock ready flag
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - HSE clock enable bit
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSE clock bypass bit
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Clock security system on HSE enable bit
    #[inline(always)]
    pub fn csshseon(&self) -> CSSHSEON_R {
        CSSHSEON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - TC/LCD prescaler
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 24 - PLL enable bit
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PLL clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - 16 MHz high-speed internal clock enable
    #[inline(always)]
    #[must_use]
    pub fn hsi16on(&mut self) -> HSI16ON_W<0> {
        HSI16ON_W::new(self)
    }
    ///Bit 2 - Internal high-speed clock ready flag
    #[inline(always)]
    #[must_use]
    pub fn hsi16rdyf(&mut self) -> HSI16RDYF_W<2> {
        HSI16RDYF_W::new(self)
    }
    ///Bit 3 - HSI16DIVEN
    #[inline(always)]
    #[must_use]
    pub fn hsi16diven(&mut self) -> HSI16DIVEN_W<3> {
        HSI16DIVEN_W::new(self)
    }
    ///Bit 5 - 16 MHz high-speed internal clock output enable
    #[inline(always)]
    #[must_use]
    pub fn hsi16outen(&mut self) -> HSI16OUTEN_W<5> {
        HSI16OUTEN_W::new(self)
    }
    ///Bit 8 - MSI clock enable bit
    #[inline(always)]
    #[must_use]
    pub fn msion(&mut self) -> MSION_W<8> {
        MSION_W::new(self)
    }
    ///Bit 16 - HSE clock enable bit
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<16> {
        HSEON_W::new(self)
    }
    ///Bit 18 - HSE clock bypass bit
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<18> {
        HSEBYP_W::new(self)
    }
    ///Bit 19 - Clock security system on HSE enable bit
    #[inline(always)]
    #[must_use]
    pub fn csshseon(&mut self) -> CSSHSEON_W<19> {
        CSSHSEON_W::new(self)
    }
    ///Bits 20:21 - TC/LCD prescaler
    #[inline(always)]
    #[must_use]
    pub fn rtcpre(&mut self) -> RTCPRE_W<20> {
        RTCPRE_W::new(self)
    }
    ///Bit 24 - PLL enable bit
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
///Clock control register
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
///`reset()` method sets CR to value 0x0300
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300;
}
