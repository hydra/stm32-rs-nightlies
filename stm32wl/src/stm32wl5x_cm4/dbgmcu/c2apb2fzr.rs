///Register `C2APB2FZR` reader
pub struct R(crate::R<C2APB2FZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2APB2FZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2APB2FZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2APB2FZR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2APB2FZR` writer
pub struct W(crate::W<C2APB2FZR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2APB2FZR_SPEC>;
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
impl From<crate::W<C2APB2FZR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2APB2FZR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBG_TIM1_STOP` reader - DBG_TIM1_STOP
pub type DBG_TIM1_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM1_STOP` writer - DBG_TIM1_STOP
pub type DBG_TIM1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB2FZR_SPEC, bool, O>;
///Field `DBG_TIM16_STOP` reader - DBG_TIM16_STOP
pub type DBG_TIM16_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM16_STOP` writer - DBG_TIM16_STOP
pub type DBG_TIM16_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB2FZR_SPEC, bool, O>;
///Field `DBG_TIM17_STOP` reader - DBG_TIM17_STOP
pub type DBG_TIM17_STOP_R = crate::BitReader<bool>;
///Field `DBG_TIM17_STOP` writer - DBG_TIM17_STOP
pub type DBG_TIM17_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2APB2FZR_SPEC, bool, O>;
impl R {
    ///Bit 11 - DBG_TIM1_STOP
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DBG_TIM1_STOP_R {
        DBG_TIM1_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 17 - DBG_TIM16_STOP
    #[inline(always)]
    pub fn dbg_tim16_stop(&self) -> DBG_TIM16_STOP_R {
        DBG_TIM16_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DBG_TIM17_STOP
    #[inline(always)]
    pub fn dbg_tim17_stop(&self) -> DBG_TIM17_STOP_R {
        DBG_TIM17_STOP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 11 - DBG_TIM1_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim1_stop(&mut self) -> DBG_TIM1_STOP_W<11> {
        DBG_TIM1_STOP_W::new(self)
    }
    ///Bit 17 - DBG_TIM16_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim16_stop(&mut self) -> DBG_TIM16_STOP_W<17> {
        DBG_TIM16_STOP_W::new(self)
    }
    ///Bit 18 - DBG_TIM17_STOP
    #[inline(always)]
    #[must_use]
    pub fn dbg_tim17_stop(&mut self) -> DBG_TIM17_STOP_W<18> {
        DBG_TIM17_STOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DBGMCU CPU2 APB2 Peripheral Freeze Register \[dual core device
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2apb2fzr](index.html) module
pub struct C2APB2FZR_SPEC;
impl crate::RegisterSpec for C2APB2FZR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2apb2fzr::R](R) reader structure
impl crate::Readable for C2APB2FZR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2apb2fzr::W](W) writer structure
impl crate::Writable for C2APB2FZR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2APB2FZR to value 0
impl crate::Resettable for C2APB2FZR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
