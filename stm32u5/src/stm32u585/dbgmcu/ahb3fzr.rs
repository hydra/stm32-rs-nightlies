///Register `AHB3FZR` reader
pub struct R(crate::R<AHB3FZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3FZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3FZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3FZR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB3FZR` writer
pub struct W(crate::W<AHB3FZR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3FZR_SPEC>;
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
impl From<crate::W<AHB3FZR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3FZR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBG_LPDMA0_STOP` reader - LPDMA channel 0 stop in debug
pub type DBG_LPDMA0_STOP_R = crate::BitReader<bool>;
///Field `DBG_LPDMA0_STOP` writer - LPDMA channel 0 stop in debug
pub type DBG_LPDMA0_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3FZR_SPEC, bool, O>;
///Field `DBG_LPDMA1_STOP` reader - LPDMA channel 1 stop in debug
pub type DBG_LPDMA1_STOP_R = crate::BitReader<bool>;
///Field `DBG_LPDMA1_STOP` writer - LPDMA channel 1 stop in debug
pub type DBG_LPDMA1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3FZR_SPEC, bool, O>;
///Field `DBG_LPDMA2_STOP` reader - LPDMA channel 2 stop in debug
pub type DBG_LPDMA2_STOP_R = crate::BitReader<bool>;
///Field `DBG_LPDMA2_STOP` writer - LPDMA channel 2 stop in debug
pub type DBG_LPDMA2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3FZR_SPEC, bool, O>;
///Field `DBG_LPDMA3_STOP` reader - LPDMA channel 3 stop in debug
pub type DBG_LPDMA3_STOP_R = crate::BitReader<bool>;
///Field `DBG_LPDMA3_STOP` writer - LPDMA channel 3 stop in debug
pub type DBG_LPDMA3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3FZR_SPEC, bool, O>;
impl R {
    ///Bit 0 - LPDMA channel 0 stop in debug
    #[inline(always)]
    pub fn dbg_lpdma0_stop(&self) -> DBG_LPDMA0_STOP_R {
        DBG_LPDMA0_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPDMA channel 1 stop in debug
    #[inline(always)]
    pub fn dbg_lpdma1_stop(&self) -> DBG_LPDMA1_STOP_R {
        DBG_LPDMA1_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LPDMA channel 2 stop in debug
    #[inline(always)]
    pub fn dbg_lpdma2_stop(&self) -> DBG_LPDMA2_STOP_R {
        DBG_LPDMA2_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LPDMA channel 3 stop in debug
    #[inline(always)]
    pub fn dbg_lpdma3_stop(&self) -> DBG_LPDMA3_STOP_R {
        DBG_LPDMA3_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LPDMA channel 0 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_lpdma0_stop(&mut self) -> DBG_LPDMA0_STOP_W<0> {
        DBG_LPDMA0_STOP_W::new(self)
    }
    ///Bit 1 - LPDMA channel 1 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_lpdma1_stop(&mut self) -> DBG_LPDMA1_STOP_W<1> {
        DBG_LPDMA1_STOP_W::new(self)
    }
    ///Bit 2 - LPDMA channel 2 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_lpdma2_stop(&mut self) -> DBG_LPDMA2_STOP_W<2> {
        DBG_LPDMA2_STOP_W::new(self)
    }
    ///Bit 3 - LPDMA channel 3 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_lpdma3_stop(&mut self) -> DBG_LPDMA3_STOP_W<3> {
        DBG_LPDMA3_STOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Debug MCU AHB3 peripheral freeze register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb3fzr](index.html) module
pub struct AHB3FZR_SPEC;
impl crate::RegisterSpec for AHB3FZR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb3fzr::R](R) reader structure
impl crate::Readable for AHB3FZR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb3fzr::W](W) writer structure
impl crate::Writable for AHB3FZR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB3FZR to value 0
impl crate::Resettable for AHB3FZR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
