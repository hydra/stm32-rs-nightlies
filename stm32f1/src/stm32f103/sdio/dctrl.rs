///Register `DCTRL` reader
pub struct R(crate::R<DCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCTRL` writer
pub struct W(crate::W<DCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTRL_SPEC>;
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
impl From<crate::W<DCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DTEN` reader - DTEN
pub type DTEN_R = crate::BitReader<bool>;
///Field `DTEN` writer - DTEN
pub type DTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `DTDIR` reader - DTDIR
pub type DTDIR_R = crate::BitReader<bool>;
///Field `DTDIR` writer - DTDIR
pub type DTDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `DTMODE` reader - DTMODE
pub type DTMODE_R = crate::BitReader<bool>;
///Field `DTMODE` writer - DTMODE
pub type DTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `DMAEN` reader - DMAEN
pub type DMAEN_R = crate::BitReader<bool>;
///Field `DMAEN` writer - DMAEN
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `DBLOCKSIZE` reader - DBLOCKSIZE
pub type DBLOCKSIZE_R = crate::FieldReader<u8, u8>;
///Field `DBLOCKSIZE` writer - DBLOCKSIZE
pub type DBLOCKSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCTRL_SPEC, u8, u8, 4, O>;
///Field `PWSTART` reader - PWSTART
pub type PWSTART_R = crate::BitReader<bool>;
///Field `PWSTART` writer - PWSTART
pub type PWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `PWSTOP` reader - PWSTOP
pub type PWSTOP_R = crate::BitReader<bool>;
///Field `PWSTOP` writer - PWSTOP
pub type PWSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `RWMOD` reader - RWMOD
pub type RWMOD_R = crate::BitReader<bool>;
///Field `RWMOD` writer - RWMOD
pub type RWMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `SDIOEN` reader - SDIOEN
pub type SDIOEN_R = crate::BitReader<bool>;
///Field `SDIOEN` writer - SDIOEN
pub type SDIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
impl R {
    ///Bit 0 - DTEN
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DTDIR
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DTMODE
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DMAEN
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - DBLOCKSIZE
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - PWSTART
    #[inline(always)]
    pub fn pwstart(&self) -> PWSTART_R {
        PWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PWSTOP
    #[inline(always)]
    pub fn pwstop(&self) -> PWSTOP_R {
        PWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RWMOD
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SDIOEN
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DTEN
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<0> {
        DTEN_W::new(self)
    }
    ///Bit 1 - DTDIR
    #[inline(always)]
    #[must_use]
    pub fn dtdir(&mut self) -> DTDIR_W<1> {
        DTDIR_W::new(self)
    }
    ///Bit 2 - DTMODE
    #[inline(always)]
    #[must_use]
    pub fn dtmode(&mut self) -> DTMODE_W<2> {
        DTMODE_W::new(self)
    }
    ///Bit 3 - DMAEN
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<3> {
        DMAEN_W::new(self)
    }
    ///Bits 4:7 - DBLOCKSIZE
    #[inline(always)]
    #[must_use]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<4> {
        DBLOCKSIZE_W::new(self)
    }
    ///Bit 8 - PWSTART
    #[inline(always)]
    #[must_use]
    pub fn pwstart(&mut self) -> PWSTART_W<8> {
        PWSTART_W::new(self)
    }
    ///Bit 9 - PWSTOP
    #[inline(always)]
    #[must_use]
    pub fn pwstop(&mut self) -> PWSTOP_W<9> {
        PWSTOP_W::new(self)
    }
    ///Bit 10 - RWMOD
    #[inline(always)]
    #[must_use]
    pub fn rwmod(&mut self) -> RWMOD_W<10> {
        RWMOD_W::new(self)
    }
    ///Bit 11 - SDIOEN
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<11> {
        SDIOEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SDIO data control register (SDIO_DCTRL)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dctrl](index.html) module
pub struct DCTRL_SPEC;
impl crate::RegisterSpec for DCTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [dctrl::R](R) reader structure
impl crate::Readable for DCTRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dctrl::W](W) writer structure
impl crate::Writable for DCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCTRL to value 0
impl crate::Resettable for DCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
