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
impl R {
    ///Bit 24 - Timers clocks prescalers selection
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 24 - Timers clocks prescalers selection
    #[inline(always)]
    #[must_use]
    pub fn timpre(&mut self) -> TIMPRE_W<24> {
        TIMPRE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC Dedicated Clock Configuration Register
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
