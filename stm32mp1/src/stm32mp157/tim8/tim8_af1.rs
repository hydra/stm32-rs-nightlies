///Register `TIM8_AF1` reader
pub struct R(crate::R<TIM8_AF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM8_AF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM8_AF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM8_AF1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM8_AF1` writer
pub struct W(crate::W<TIM8_AF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM8_AF1_SPEC>;
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
impl From<crate::W<TIM8_AF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM8_AF1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BKINE` reader - BKINE
pub type BKINE_R = crate::BitReader<bool>;
///Field `BKINE` writer - BKINE
pub type BKINE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_AF1_SPEC, bool, O>;
///Field `BKDF1BK2E` reader - BKDF1BK2E
pub type BKDF1BK2E_R = crate::BitReader<bool>;
///Field `BKDF1BK2E` writer - BKDF1BK2E
pub type BKDF1BK2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_AF1_SPEC, bool, O>;
///Field `BKINP` reader - BKINP
pub type BKINP_R = crate::BitReader<bool>;
///Field `BKINP` writer - BKINP
pub type BKINP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_AF1_SPEC, bool, O>;
///Field `ETRSEL` reader - ETRSEL
pub type ETRSEL_R = crate::FieldReader<u8, u8>;
///Field `ETRSEL` writer - ETRSEL
pub type ETRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM8_AF1_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - BKINE
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - BKDF1BK2E
    #[inline(always)]
    pub fn bkdf1bk2e(&self) -> BKDF1BK2E_R {
        BKDF1BK2E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BKINP
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 14:17 - ETRSEL
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - BKINE
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<0> {
        BKINE_W::new(self)
    }
    ///Bit 8 - BKDF1BK2E
    #[inline(always)]
    #[must_use]
    pub fn bkdf1bk2e(&mut self) -> BKDF1BK2E_W<8> {
        BKDF1BK2E_W::new(self)
    }
    ///Bit 9 - BKINP
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BKINP_W<9> {
        BKINP_W::new(self)
    }
    ///Bits 14:17 - ETRSEL
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<14> {
        ETRSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM8 Alternate function option register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim8_af1](index.html) module
pub struct TIM8_AF1_SPEC;
impl crate::RegisterSpec for TIM8_AF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim8_af1::R](R) reader structure
impl crate::Readable for TIM8_AF1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim8_af1::W](W) writer structure
impl crate::Writable for TIM8_AF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM8_AF1 to value 0x01
impl crate::Resettable for TIM8_AF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
