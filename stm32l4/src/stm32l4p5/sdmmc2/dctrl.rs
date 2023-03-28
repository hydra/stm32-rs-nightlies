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
///Field `DTDIR` reader - Data transfer direction selection
pub type DTDIR_R = crate::BitReader<bool>;
///Field `DTDIR` writer - Data transfer direction selection
pub type DTDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `DTMODE` reader - Data transfer mode selection 1: Stream or SDIO multibyte data transfer
pub type DTMODE_R = crate::FieldReader<u8, u8>;
///Field `DTMODE` writer - Data transfer mode selection 1: Stream or SDIO multibyte data transfer
pub type DTMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCTRL_SPEC, u8, u8, 2, O>;
///Field `DBLOCKSIZE` reader - Data block size
pub type DBLOCKSIZE_R = crate::FieldReader<u8, u8>;
///Field `DBLOCKSIZE` writer - Data block size
pub type DBLOCKSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCTRL_SPEC, u8, u8, 4, O>;
///Field `RWSTART` reader - Read wait start
pub type RWSTART_R = crate::BitReader<bool>;
///Field `RWSTART` writer - Read wait start
pub type RWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `RWSTOP` reader - Read wait stop
pub type RWSTOP_R = crate::BitReader<bool>;
///Field `RWSTOP` writer - Read wait stop
pub type RWSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `RWMOD` reader - Read wait mode
pub type RWMOD_R = crate::BitReader<bool>;
///Field `RWMOD` writer - Read wait mode
pub type RWMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `SDIOEN` reader - SD I/O enable functions
pub type SDIOEN_R = crate::BitReader<bool>;
///Field `SDIOEN` writer - SD I/O enable functions
pub type SDIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `BOOTACKEN` reader - Enable the reception of the boot acknowledgment
pub type BOOTACKEN_R = crate::BitReader<bool>;
///Field `BOOTACKEN` writer - Enable the reception of the boot acknowledgment
pub type BOOTACKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
///Field `FIFORST` reader - FIFO reset, will flush any remaining data
pub type FIFORST_R = crate::BitReader<bool>;
///Field `FIFORST` writer - FIFO reset, will flush any remaining data
pub type FIFORST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
impl R {
    ///Bit 0 - DTEN
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data transfer direction selection
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Data block size
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - Read wait start
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Read wait stop
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Read wait mode
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SD I/O enable functions
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable the reception of the boot acknowledgment
    #[inline(always)]
    pub fn bootacken(&self) -> BOOTACKEN_R {
        BOOTACKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - FIFO reset, will flush any remaining data
    #[inline(always)]
    pub fn fiforst(&self) -> FIFORST_R {
        FIFORST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DTEN
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<0> {
        DTEN_W::new(self)
    }
    ///Bit 1 - Data transfer direction selection
    #[inline(always)]
    #[must_use]
    pub fn dtdir(&mut self) -> DTDIR_W<1> {
        DTDIR_W::new(self)
    }
    ///Bits 2:3 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer
    #[inline(always)]
    #[must_use]
    pub fn dtmode(&mut self) -> DTMODE_W<2> {
        DTMODE_W::new(self)
    }
    ///Bits 4:7 - Data block size
    #[inline(always)]
    #[must_use]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<4> {
        DBLOCKSIZE_W::new(self)
    }
    ///Bit 8 - Read wait start
    #[inline(always)]
    #[must_use]
    pub fn rwstart(&mut self) -> RWSTART_W<8> {
        RWSTART_W::new(self)
    }
    ///Bit 9 - Read wait stop
    #[inline(always)]
    #[must_use]
    pub fn rwstop(&mut self) -> RWSTOP_W<9> {
        RWSTOP_W::new(self)
    }
    ///Bit 10 - Read wait mode
    #[inline(always)]
    #[must_use]
    pub fn rwmod(&mut self) -> RWMOD_W<10> {
        RWMOD_W::new(self)
    }
    ///Bit 11 - SD I/O enable functions
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<11> {
        SDIOEN_W::new(self)
    }
    ///Bit 12 - Enable the reception of the boot acknowledgment
    #[inline(always)]
    #[must_use]
    pub fn bootacken(&mut self) -> BOOTACKEN_W<12> {
        BOOTACKEN_W::new(self)
    }
    ///Bit 13 - FIFO reset, will flush any remaining data
    #[inline(always)]
    #[must_use]
    pub fn fiforst(&mut self) -> FIFORST_W<13> {
        FIFORST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///data control register
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
