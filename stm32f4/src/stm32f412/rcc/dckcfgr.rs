///Register `DCKCFGR` reader
pub struct R(crate::R<DCKCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCKCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCKCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCKCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCKCFGR` writer
pub struct W(crate::W<DCKCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCKCFGR_SPEC>;
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
impl From<crate::W<DCKCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCKCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CKDFSDM1ASEL` reader - DFSDM1 audio clock selection
pub type CKDFSDM1ASEL_R = crate::FieldReader<u8, CKDFSDM1ASEL_A>;
///DFSDM1 audio clock selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKDFSDM1ASEL_A {
    ///0: CK_I2S_APB1 selected as audio clock
    I2s1 = 0,
    ///1: CK_I2S_APB2 selected as audio clock
    I2s2 = 1,
}
impl From<CKDFSDM1ASEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKDFSDM1ASEL_A) -> Self {
        variant as _
    }
}
impl CKDFSDM1ASEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CKDFSDM1ASEL_A> {
        match self.bits {
            0 => Some(CKDFSDM1ASEL_A::I2s1),
            1 => Some(CKDFSDM1ASEL_A::I2s2),
            _ => None,
        }
    }
    ///Checks if the value of the field is `I2s1`
    #[inline(always)]
    pub fn is_i2s1(&self) -> bool {
        *self == CKDFSDM1ASEL_A::I2s1
    }
    ///Checks if the value of the field is `I2s2`
    #[inline(always)]
    pub fn is_i2s2(&self) -> bool {
        *self == CKDFSDM1ASEL_A::I2s2
    }
}
///Field `CKDFSDM1ASEL` writer - DFSDM1 audio clock selection
pub type CKDFSDM1ASEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCKCFGR_SPEC, u8, CKDFSDM1ASEL_A, 5, O>;
impl<'a, const O: u8> CKDFSDM1ASEL_W<'a, O> {
    ///CK_I2S_APB1 selected as audio clock
    #[inline(always)]
    pub fn i2s1(self) -> &'a mut W {
        self.variant(CKDFSDM1ASEL_A::I2s1)
    }
    ///CK_I2S_APB2 selected as audio clock
    #[inline(always)]
    pub fn i2s2(self) -> &'a mut W {
        self.variant(CKDFSDM1ASEL_A::I2s2)
    }
}
///Field `TIMPRE` reader - Timers clocks prescalers selection
pub type TIMPRE_R = crate::BitReader<TIMPRE_A>;
///Timers clocks prescalers selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPRE_A {
    ///0: If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx
    Mul2 = 0,
    ///1: If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx
    Mul4 = 1,
}
impl From<TIMPRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIMPRE_A {
        match self.bits {
            false => TIMPRE_A::Mul2,
            true => TIMPRE_A::Mul4,
        }
    }
    ///Checks if the value of the field is `Mul2`
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == TIMPRE_A::Mul2
    }
    ///Checks if the value of the field is `Mul4`
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == TIMPRE_A::Mul4
    }
}
///Field `TIMPRE` writer - Timers clocks prescalers selection
pub type TIMPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR_SPEC, TIMPRE_A, O>;
impl<'a, const O: u8> TIMPRE_W<'a, O> {
    ///If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(TIMPRE_A::Mul2)
    }
    ///If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(TIMPRE_A::Mul4)
    }
}
///Field `I2S1SRC` reader - I2S APB1 clocks source selection (I2S2/3)
pub type I2S1SRC_R = crate::FieldReader<u8, I2S1SRC_A>;
///I2S APB1 clocks source selection (I2S2/3)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2S1SRC_A {
    ///0: I2Sx clock frequency = f(PLLI2S_R)
    Plli2sr = 0,
    ///1: I2Sx clock frequency = I2S_CKIN Alternate function input frequency
    I2sCkin = 1,
    ///2: I2Sx clock frequency = f(PLL_R)
    Pllr = 2,
    ///3: I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\[22\])
    HsiHse = 3,
}
impl From<I2S1SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: I2S1SRC_A) -> Self {
        variant as _
    }
}
impl I2S1SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2S1SRC_A {
        match self.bits {
            0 => I2S1SRC_A::Plli2sr,
            1 => I2S1SRC_A::I2sCkin,
            2 => I2S1SRC_A::Pllr,
            3 => I2S1SRC_A::HsiHse,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Plli2sr`
    #[inline(always)]
    pub fn is_plli2sr(&self) -> bool {
        *self == I2S1SRC_A::Plli2sr
    }
    ///Checks if the value of the field is `I2sCkin`
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == I2S1SRC_A::I2sCkin
    }
    ///Checks if the value of the field is `Pllr`
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == I2S1SRC_A::Pllr
    }
    ///Checks if the value of the field is `HsiHse`
    #[inline(always)]
    pub fn is_hsi_hse(&self) -> bool {
        *self == I2S1SRC_A::HsiHse
    }
}
///Field `I2S1SRC` writer - I2S APB1 clocks source selection (I2S2/3)
pub type I2S1SRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR_SPEC, u8, I2S1SRC_A, 2, O>;
impl<'a, const O: u8> I2S1SRC_W<'a, O> {
    ///I2Sx clock frequency = f(PLLI2S_R)
    #[inline(always)]
    pub fn plli2sr(self) -> &'a mut W {
        self.variant(I2S1SRC_A::Plli2sr)
    }
    ///I2Sx clock frequency = I2S_CKIN Alternate function input frequency
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(I2S1SRC_A::I2sCkin)
    }
    ///I2Sx clock frequency = f(PLL_R)
    #[inline(always)]
    pub fn pllr(self) -> &'a mut W {
        self.variant(I2S1SRC_A::Pllr)
    }
    ///I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\[22\])
    #[inline(always)]
    pub fn hsi_hse(self) -> &'a mut W {
        self.variant(I2S1SRC_A::HsiHse)
    }
}
///Field `I2S2SRC` reader - I2S APB2 clocks source selection (I2S1/4/5)
pub use I2S1SRC_R as I2S2SRC_R;
///Field `I2S2SRC` writer - I2S APB2 clocks source selection (I2S1/4/5)
pub use I2S1SRC_W as I2S2SRC_W;
///Field `CKDFSDM1SEL` reader - DFSDM1 Kernel clock selection
pub type CKDFSDM1SEL_R = crate::BitReader<CKDFSDM1SEL_A>;
///DFSDM1 Kernel clock selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKDFSDM1SEL_A {
    ///0: APB2 clock used as Kernel clock
    Apb2 = 0,
    ///1: System clock used as Kernel clock
    Sysclk = 1,
}
impl From<CKDFSDM1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CKDFSDM1SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CKDFSDM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CKDFSDM1SEL_A {
        match self.bits {
            false => CKDFSDM1SEL_A::Apb2,
            true => CKDFSDM1SEL_A::Sysclk,
        }
    }
    ///Checks if the value of the field is `Apb2`
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == CKDFSDM1SEL_A::Apb2
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == CKDFSDM1SEL_A::Sysclk
    }
}
///Field `CKDFSDM1SEL` writer - DFSDM1 Kernel clock selection
pub type CKDFSDM1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR_SPEC, CKDFSDM1SEL_A, O>;
impl<'a, const O: u8> CKDFSDM1SEL_W<'a, O> {
    ///APB2 clock used as Kernel clock
    #[inline(always)]
    pub fn apb2(self) -> &'a mut W {
        self.variant(CKDFSDM1SEL_A::Apb2)
    }
    ///System clock used as Kernel clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(CKDFSDM1SEL_A::Sysclk)
    }
}
impl R {
    ///Bits 15:19 - DFSDM1 audio clock selection
    #[inline(always)]
    pub fn ckdfsdm1asel(&self) -> CKDFSDM1ASEL_R {
        CKDFSDM1ASEL_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bit 24 - Timers clocks prescalers selection
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - I2S APB1 clocks source selection (I2S2/3)
    #[inline(always)]
    pub fn i2s1src(&self) -> I2S1SRC_R {
        I2S1SRC_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:28 - I2S APB2 clocks source selection (I2S1/4/5)
    #[inline(always)]
    pub fn i2s2src(&self) -> I2S2SRC_R {
        I2S2SRC_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bit 31 - DFSDM1 Kernel clock selection
    #[inline(always)]
    pub fn ckdfsdm1sel(&self) -> CKDFSDM1SEL_R {
        CKDFSDM1SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 15:19 - DFSDM1 audio clock selection
    #[inline(always)]
    #[must_use]
    pub fn ckdfsdm1asel(&mut self) -> CKDFSDM1ASEL_W<15> {
        CKDFSDM1ASEL_W::new(self)
    }
    ///Bit 24 - Timers clocks prescalers selection
    #[inline(always)]
    #[must_use]
    pub fn timpre(&mut self) -> TIMPRE_W<24> {
        TIMPRE_W::new(self)
    }
    ///Bits 25:26 - I2S APB1 clocks source selection (I2S2/3)
    #[inline(always)]
    #[must_use]
    pub fn i2s1src(&mut self) -> I2S1SRC_W<25> {
        I2S1SRC_W::new(self)
    }
    ///Bits 27:28 - I2S APB2 clocks source selection (I2S1/4/5)
    #[inline(always)]
    #[must_use]
    pub fn i2s2src(&mut self) -> I2S2SRC_W<27> {
        I2S2SRC_W::new(self)
    }
    ///Bit 31 - DFSDM1 Kernel clock selection
    #[inline(always)]
    #[must_use]
    pub fn ckdfsdm1sel(&mut self) -> CKDFSDM1SEL_W<31> {
        CKDFSDM1SEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Dedicated Clock Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dckcfgr](index.html) module
pub struct DCKCFGR_SPEC;
impl crate::RegisterSpec for DCKCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dckcfgr::R](R) reader structure
impl crate::Readable for DCKCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dckcfgr::W](W) writer structure
impl crate::Writable for DCKCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCKCFGR to value 0
impl crate::Resettable for DCKCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
