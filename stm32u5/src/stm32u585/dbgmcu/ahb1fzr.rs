///Register `AHB1FZR` reader
pub struct R(crate::R<AHB1FZR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1FZR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1FZR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1FZR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB1FZR` writer
pub struct W(crate::W<AHB1FZR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1FZR_SPEC>;
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
impl From<crate::W<AHB1FZR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1FZR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBG_GPDMA0_STOP` reader - GPDMA channel 0 stop in debug
pub type DBG_GPDMA0_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA0_STOP` writer - GPDMA channel 0 stop in debug
pub type DBG_GPDMA0_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA1_STOP` reader - GPDMA channel 1 stop in debug
pub type DBG_GPDMA1_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA1_STOP` writer - GPDMA channel 1 stop in debug
pub type DBG_GPDMA1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA2_STOP` reader - GPDMA channel 2 stop in debug
pub type DBG_GPDMA2_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA2_STOP` writer - GPDMA channel 2 stop in debug
pub type DBG_GPDMA2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA3_STOP` reader - GPDMA channel 3 stop in debug
pub type DBG_GPDMA3_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA3_STOP` writer - GPDMA channel 3 stop in debug
pub type DBG_GPDMA3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA4_STOP` reader - GPDMA channel 4 stop in debug
pub type DBG_GPDMA4_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA4_STOP` writer - GPDMA channel 4 stop in debug
pub type DBG_GPDMA4_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA5_STOP` reader - GPDMA channel 5 stop in debug
pub type DBG_GPDMA5_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA5_STOP` writer - GPDMA channel 5 stop in debug
pub type DBG_GPDMA5_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA6_STOP` reader - GPDMA channel 6 stop in debug
pub type DBG_GPDMA6_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA6_STOP` writer - GPDMA channel 6 stop in debug
pub type DBG_GPDMA6_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA7_STOP` reader - GPDMA channel 7 stop in debug
pub type DBG_GPDMA7_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA7_STOP` writer - GPDMA channel 7 stop in debug
pub type DBG_GPDMA7_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA8_STOP` reader - GPDMA channel 8 stop in debug
pub type DBG_GPDMA8_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA8_STOP` writer - GPDMA channel 8 stop in debug
pub type DBG_GPDMA8_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA9_STOP` reader - GPDMA channel 9 stop in debug
pub type DBG_GPDMA9_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA9_STOP` writer - GPDMA channel 9 stop in debug
pub type DBG_GPDMA9_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA10_STOP` reader - GPDMA channel 10 stop in debug
pub type DBG_GPDMA10_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA10_STOP` writer - GPDMA channel 10 stop in debug
pub type DBG_GPDMA10_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA11_STOP` reader - GPDMA channel 11 stop in debug
pub type DBG_GPDMA11_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA11_STOP` writer - GPDMA channel 11 stop in debug
pub type DBG_GPDMA11_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA12_STOP` reader - GPDMA channel 12 stop in debug
pub type DBG_GPDMA12_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA12_STOP` writer - GPDMA channel 12 stop in debug
pub type DBG_GPDMA12_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA13_STOP` reader - GPDMA channel 13 stop in debug
pub type DBG_GPDMA13_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA13_STOP` writer - GPDMA channel 13 stop in debug
pub type DBG_GPDMA13_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA14_STOP` reader - GPDMA channel 14 stop in debug
pub type DBG_GPDMA14_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA14_STOP` writer - GPDMA channel 14 stop in debug
pub type DBG_GPDMA14_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
///Field `DBG_GPDMA15_STOP` reader - GPDMA channel 15 stop in debug
pub type DBG_GPDMA15_STOP_R = crate::BitReader<bool>;
///Field `DBG_GPDMA15_STOP` writer - GPDMA channel 15 stop in debug
pub type DBG_GPDMA15_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1FZR_SPEC, bool, O>;
impl R {
    ///Bit 0 - GPDMA channel 0 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma0_stop(&self) -> DBG_GPDMA0_STOP_R {
        DBG_GPDMA0_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPDMA channel 1 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma1_stop(&self) -> DBG_GPDMA1_STOP_R {
        DBG_GPDMA1_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPDMA channel 2 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma2_stop(&self) -> DBG_GPDMA2_STOP_R {
        DBG_GPDMA2_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPDMA channel 3 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma3_stop(&self) -> DBG_GPDMA3_STOP_R {
        DBG_GPDMA3_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPDMA channel 4 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma4_stop(&self) -> DBG_GPDMA4_STOP_R {
        DBG_GPDMA4_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPDMA channel 5 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma5_stop(&self) -> DBG_GPDMA5_STOP_R {
        DBG_GPDMA5_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPDMA channel 6 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma6_stop(&self) -> DBG_GPDMA6_STOP_R {
        DBG_GPDMA6_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPDMA channel 7 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma7_stop(&self) -> DBG_GPDMA7_STOP_R {
        DBG_GPDMA7_STOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPDMA channel 8 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma8_stop(&self) -> DBG_GPDMA8_STOP_R {
        DBG_GPDMA8_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPDMA channel 9 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma9_stop(&self) -> DBG_GPDMA9_STOP_R {
        DBG_GPDMA9_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPDMA channel 10 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma10_stop(&self) -> DBG_GPDMA10_STOP_R {
        DBG_GPDMA10_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - GPDMA channel 11 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma11_stop(&self) -> DBG_GPDMA11_STOP_R {
        DBG_GPDMA11_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - GPDMA channel 12 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma12_stop(&self) -> DBG_GPDMA12_STOP_R {
        DBG_GPDMA12_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - GPDMA channel 13 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma13_stop(&self) -> DBG_GPDMA13_STOP_R {
        DBG_GPDMA13_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GPDMA channel 14 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma14_stop(&self) -> DBG_GPDMA14_STOP_R {
        DBG_GPDMA14_STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GPDMA channel 15 stop in debug
    #[inline(always)]
    pub fn dbg_gpdma15_stop(&self) -> DBG_GPDMA15_STOP_R {
        DBG_GPDMA15_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPDMA channel 0 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma0_stop(&mut self) -> DBG_GPDMA0_STOP_W<0> {
        DBG_GPDMA0_STOP_W::new(self)
    }
    ///Bit 1 - GPDMA channel 1 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma1_stop(&mut self) -> DBG_GPDMA1_STOP_W<1> {
        DBG_GPDMA1_STOP_W::new(self)
    }
    ///Bit 2 - GPDMA channel 2 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma2_stop(&mut self) -> DBG_GPDMA2_STOP_W<2> {
        DBG_GPDMA2_STOP_W::new(self)
    }
    ///Bit 3 - GPDMA channel 3 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma3_stop(&mut self) -> DBG_GPDMA3_STOP_W<3> {
        DBG_GPDMA3_STOP_W::new(self)
    }
    ///Bit 4 - GPDMA channel 4 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma4_stop(&mut self) -> DBG_GPDMA4_STOP_W<4> {
        DBG_GPDMA4_STOP_W::new(self)
    }
    ///Bit 5 - GPDMA channel 5 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma5_stop(&mut self) -> DBG_GPDMA5_STOP_W<5> {
        DBG_GPDMA5_STOP_W::new(self)
    }
    ///Bit 6 - GPDMA channel 6 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma6_stop(&mut self) -> DBG_GPDMA6_STOP_W<6> {
        DBG_GPDMA6_STOP_W::new(self)
    }
    ///Bit 7 - GPDMA channel 7 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma7_stop(&mut self) -> DBG_GPDMA7_STOP_W<7> {
        DBG_GPDMA7_STOP_W::new(self)
    }
    ///Bit 8 - GPDMA channel 8 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma8_stop(&mut self) -> DBG_GPDMA8_STOP_W<8> {
        DBG_GPDMA8_STOP_W::new(self)
    }
    ///Bit 9 - GPDMA channel 9 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma9_stop(&mut self) -> DBG_GPDMA9_STOP_W<9> {
        DBG_GPDMA9_STOP_W::new(self)
    }
    ///Bit 10 - GPDMA channel 10 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma10_stop(&mut self) -> DBG_GPDMA10_STOP_W<10> {
        DBG_GPDMA10_STOP_W::new(self)
    }
    ///Bit 11 - GPDMA channel 11 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma11_stop(&mut self) -> DBG_GPDMA11_STOP_W<11> {
        DBG_GPDMA11_STOP_W::new(self)
    }
    ///Bit 12 - GPDMA channel 12 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma12_stop(&mut self) -> DBG_GPDMA12_STOP_W<12> {
        DBG_GPDMA12_STOP_W::new(self)
    }
    ///Bit 13 - GPDMA channel 13 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma13_stop(&mut self) -> DBG_GPDMA13_STOP_W<13> {
        DBG_GPDMA13_STOP_W::new(self)
    }
    ///Bit 14 - GPDMA channel 14 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma14_stop(&mut self) -> DBG_GPDMA14_STOP_W<14> {
        DBG_GPDMA14_STOP_W::new(self)
    }
    ///Bit 15 - GPDMA channel 15 stop in debug
    #[inline(always)]
    #[must_use]
    pub fn dbg_gpdma15_stop(&mut self) -> DBG_GPDMA15_STOP_W<15> {
        DBG_GPDMA15_STOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Debug MCU AHB1 peripheral freeze register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1fzr](index.html) module
pub struct AHB1FZR_SPEC;
impl crate::RegisterSpec for AHB1FZR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb1fzr::R](R) reader structure
impl crate::Readable for AHB1FZR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb1fzr::W](W) writer structure
impl crate::Writable for AHB1FZR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB1FZR to value 0
impl crate::Resettable for AHB1FZR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
