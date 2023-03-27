///Register `ISR1` reader
pub struct R(crate::R<ISR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ISR1` writer
pub struct W(crate::W<ISR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR1_SPEC>;
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
impl From<crate::W<ISR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TOHSTX` reader - Timeout high
pub type TOHSTX_R = crate::BitReader<bool>;
///Field `TOHSTX` writer - Timeout high
pub type TOHSTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
///Field `TOLPRX` reader - Timeout low
pub type TOLPRX_R = crate::BitReader<bool>;
///Field `TOLPRX` writer - Timeout low
pub type TOLPRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
///Field `ECCSE` reader - ECC single
pub type ECCSE_R = crate::BitReader<bool>;
///Field `ECCSE` writer - ECC single
pub type ECCSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
///Field `ECCME` reader - ECC multi
pub type ECCME_R = crate::BitReader<bool>;
///Field `ECCME` writer - ECC multi
pub type ECCME_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
///Field `CRCE` reader - CRC error
pub type CRCE_R = crate::BitReader<bool>;
///Field `CRCE` writer - CRC error
pub type CRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
///Field `PSE` reader - Packet size error
pub type PSE_R = crate::BitReader<bool>;
///Field `PSE` writer - Packet size error
pub type PSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
///Field `EOTPE` reader - EoTp error
pub type EOTPE_R = crate::BitReader<bool>;
///Field `EOTPE` writer - EoTp error
pub type EOTPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
///Field `LPWRE` reader - LTDC payload write error
pub type LPWRE_R = crate::BitReader<bool>;
///Field `LPWRE` writer - LTDC payload write error
pub type LPWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
///Field `GCWRE` reader - Generic command write error
pub type GCWRE_R = crate::BitReader<bool>;
///Field `GCWRE` writer - Generic command write error
pub type GCWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
///Field `GPWRE` reader - Generic payload write error
pub type GPWRE_R = crate::BitReader<bool>;
///Field `GPWRE` writer - Generic payload write error
pub type GPWRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
///Field `GPTXE` reader - Generic payload transmit error
pub type GPTXE_R = crate::BitReader<bool>;
///Field `GPTXE` writer - Generic payload transmit error
pub type GPTXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
///Field `GPRDE` reader - Generic payload read error
pub type GPRDE_R = crate::BitReader<bool>;
///Field `GPRDE` writer - Generic payload read error
pub type GPRDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
///Field `GPRXE` reader - Generic payload receive error
pub type GPRXE_R = crate::BitReader<bool>;
///Field `GPRXE` writer - Generic payload receive error
pub type GPRXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - Timeout high
    #[inline(always)]
    pub fn tohstx(&self) -> TOHSTX_R {
        TOHSTX_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timeout low
    #[inline(always)]
    pub fn tolprx(&self) -> TOLPRX_R {
        TOLPRX_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ECC single
    #[inline(always)]
    pub fn eccse(&self) -> ECCSE_R {
        ECCSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC multi
    #[inline(always)]
    pub fn eccme(&self) -> ECCME_R {
        ECCME_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC error
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Packet size error
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EoTp error
    #[inline(always)]
    pub fn eotpe(&self) -> EOTPE_R {
        EOTPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LTDC payload write error
    #[inline(always)]
    pub fn lpwre(&self) -> LPWRE_R {
        LPWRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Generic command write error
    #[inline(always)]
    pub fn gcwre(&self) -> GCWRE_R {
        GCWRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Generic payload write error
    #[inline(always)]
    pub fn gpwre(&self) -> GPWRE_R {
        GPWRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Generic payload transmit error
    #[inline(always)]
    pub fn gptxe(&self) -> GPTXE_R {
        GPTXE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Generic payload read error
    #[inline(always)]
    pub fn gprde(&self) -> GPRDE_R {
        GPRDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Generic payload receive error
    #[inline(always)]
    pub fn gprxe(&self) -> GPRXE_R {
        GPRXE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Timeout high
    #[inline(always)]
    #[must_use]
    pub fn tohstx(&mut self) -> TOHSTX_W<0> {
        TOHSTX_W::new(self)
    }
    ///Bit 1 - Timeout low
    #[inline(always)]
    #[must_use]
    pub fn tolprx(&mut self) -> TOLPRX_W<1> {
        TOLPRX_W::new(self)
    }
    ///Bit 2 - ECC single
    #[inline(always)]
    #[must_use]
    pub fn eccse(&mut self) -> ECCSE_W<2> {
        ECCSE_W::new(self)
    }
    ///Bit 3 - ECC multi
    #[inline(always)]
    #[must_use]
    pub fn eccme(&mut self) -> ECCME_W<3> {
        ECCME_W::new(self)
    }
    ///Bit 4 - CRC error
    #[inline(always)]
    #[must_use]
    pub fn crce(&mut self) -> CRCE_W<4> {
        CRCE_W::new(self)
    }
    ///Bit 5 - Packet size error
    #[inline(always)]
    #[must_use]
    pub fn pse(&mut self) -> PSE_W<5> {
        PSE_W::new(self)
    }
    ///Bit 6 - EoTp error
    #[inline(always)]
    #[must_use]
    pub fn eotpe(&mut self) -> EOTPE_W<6> {
        EOTPE_W::new(self)
    }
    ///Bit 7 - LTDC payload write error
    #[inline(always)]
    #[must_use]
    pub fn lpwre(&mut self) -> LPWRE_W<7> {
        LPWRE_W::new(self)
    }
    ///Bit 8 - Generic command write error
    #[inline(always)]
    #[must_use]
    pub fn gcwre(&mut self) -> GCWRE_W<8> {
        GCWRE_W::new(self)
    }
    ///Bit 9 - Generic payload write error
    #[inline(always)]
    #[must_use]
    pub fn gpwre(&mut self) -> GPWRE_W<9> {
        GPWRE_W::new(self)
    }
    ///Bit 10 - Generic payload transmit error
    #[inline(always)]
    #[must_use]
    pub fn gptxe(&mut self) -> GPTXE_W<10> {
        GPTXE_W::new(self)
    }
    ///Bit 11 - Generic payload read error
    #[inline(always)]
    #[must_use]
    pub fn gprde(&mut self) -> GPRDE_W<11> {
        GPRDE_W::new(self)
    }
    ///Bit 12 - Generic payload receive error
    #[inline(always)]
    #[must_use]
    pub fn gprxe(&mut self) -> GPRXE_W<12> {
        GPRXE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host interrupt and status register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr1](index.html) module
pub struct ISR1_SPEC;
impl crate::RegisterSpec for ISR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [isr1::R](R) reader structure
impl crate::Readable for ISR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [isr1::W](W) writer structure
impl crate::Writable for ISR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ISR1 to value 0
impl crate::Resettable for ISR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
