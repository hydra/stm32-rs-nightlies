///Register `BCR1` reader
pub struct R(crate::R<BCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BCR1` writer
pub struct W(crate::W<BCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCR1_SPEC>;
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
impl From<crate::W<BCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MODE` reader - Audio block mode
pub type MODE_R = crate::FieldReader<u8, u8>;
///Field `MODE` writer - Audio block mode
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR1_SPEC, u8, u8, 2, O>;
///Field `PRTCFG` reader - Protocol configuration
pub type PRTCFG_R = crate::FieldReader<u8, u8>;
///Field `PRTCFG` writer - Protocol configuration
pub type PRTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR1_SPEC, u8, u8, 2, O>;
///Field `DS` reader - Data size
pub type DS_R = crate::FieldReader<u8, u8>;
///Field `DS` writer - Data size
pub type DS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR1_SPEC, u8, u8, 3, O>;
///Field `LSBFIRST` reader - Least significant bit first
pub type LSBFIRST_R = crate::BitReader<bool>;
///Field `LSBFIRST` writer - Least significant bit first
pub type LSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
///Field `CKSTR` reader - Clock strobing edge
pub type CKSTR_R = crate::BitReader<bool>;
///Field `CKSTR` writer - Clock strobing edge
pub type CKSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
///Field `SYNCEN` reader - Synchronization enable
pub type SYNCEN_R = crate::FieldReader<u8, u8>;
///Field `SYNCEN` writer - Synchronization enable
pub type SYNCEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR1_SPEC, u8, u8, 2, O>;
///Field `MONO` reader - Mono mode
pub type MONO_R = crate::BitReader<bool>;
///Field `MONO` writer - Mono mode
pub type MONO_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
///Field `OUTDRIV` reader - Output drive
pub type OUTDRIV_R = crate::BitReader<bool>;
///Field `OUTDRIV` writer - Output drive
pub type OUTDRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
///Field `SAIAEN` reader - Audio block A enable
pub type SAIAEN_R = crate::BitReader<bool>;
///Field `SAIAEN` writer - Audio block A enable
pub type SAIAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
///Field `DMAEN` reader - DMA enable
pub type DMAEN_R = crate::BitReader<bool>;
///Field `DMAEN` writer - DMA enable
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
///Field `NODIV` reader - No divider
pub type NODIV_R = crate::BitReader<bool>;
///Field `NODIV` writer - No divider
pub type NODIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
///Field `MCKDIV` reader - Master clock divider
pub type MCKDIV_R = crate::FieldReader<u8, u8>;
///Field `MCKDIV` writer - Master clock divider
pub type MCKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCR1_SPEC, u8, u8, 6, O>;
///Field `OSR` reader - OSR
pub type OSR_R = crate::BitReader<bool>;
///Field `OSR` writer - OSR
pub type OSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
///Field `MCKEN` reader - MCKEN
pub type MCKEN_R = crate::BitReader<bool>;
///Field `MCKEN` writer - MCKEN
pub type MCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BCR1_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - Audio block mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Protocol configuration
    #[inline(always)]
    pub fn prtcfg(&self) -> PRTCFG_R {
        PRTCFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 5:7 - Data size
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - Least significant bit first
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clock strobing edge
    #[inline(always)]
    pub fn ckstr(&self) -> CKSTR_R {
        CKSTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - Synchronization enable
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Mono mode
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Output drive
    #[inline(always)]
    pub fn outdriv(&self) -> OUTDRIV_R {
        OUTDRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Audio block A enable
    #[inline(always)]
    pub fn saiaen(&self) -> SAIAEN_R {
        SAIAEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DMA enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - No divider
    #[inline(always)]
    pub fn nodiv(&self) -> NODIV_R {
        NODIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:25 - Master clock divider
    #[inline(always)]
    pub fn mckdiv(&self) -> MCKDIV_R {
        MCKDIV_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 26 - OSR
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - MCKEN
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Audio block mode
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    ///Bits 2:3 - Protocol configuration
    #[inline(always)]
    #[must_use]
    pub fn prtcfg(&mut self) -> PRTCFG_W<2> {
        PRTCFG_W::new(self)
    }
    ///Bits 5:7 - Data size
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<5> {
        DS_W::new(self)
    }
    ///Bit 8 - Least significant bit first
    #[inline(always)]
    #[must_use]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<8> {
        LSBFIRST_W::new(self)
    }
    ///Bit 9 - Clock strobing edge
    #[inline(always)]
    #[must_use]
    pub fn ckstr(&mut self) -> CKSTR_W<9> {
        CKSTR_W::new(self)
    }
    ///Bits 10:11 - Synchronization enable
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<10> {
        SYNCEN_W::new(self)
    }
    ///Bit 12 - Mono mode
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MONO_W<12> {
        MONO_W::new(self)
    }
    ///Bit 13 - Output drive
    #[inline(always)]
    #[must_use]
    pub fn outdriv(&mut self) -> OUTDRIV_W<13> {
        OUTDRIV_W::new(self)
    }
    ///Bit 16 - Audio block A enable
    #[inline(always)]
    #[must_use]
    pub fn saiaen(&mut self) -> SAIAEN_W<16> {
        SAIAEN_W::new(self)
    }
    ///Bit 17 - DMA enable
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<17> {
        DMAEN_W::new(self)
    }
    ///Bit 19 - No divider
    #[inline(always)]
    #[must_use]
    pub fn nodiv(&mut self) -> NODIV_W<19> {
        NODIV_W::new(self)
    }
    ///Bits 20:25 - Master clock divider
    #[inline(always)]
    #[must_use]
    pub fn mckdiv(&mut self) -> MCKDIV_W<20> {
        MCKDIV_W::new(self)
    }
    ///Bit 26 - OSR
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OSR_W<26> {
        OSR_W::new(self)
    }
    ///Bit 27 - MCKEN
    #[inline(always)]
    #[must_use]
    pub fn mcken(&mut self) -> MCKEN_W<27> {
        MCKEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///B Configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bcr1](index.html) module
pub struct BCR1_SPEC;
impl crate::RegisterSpec for BCR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [bcr1::R](R) reader structure
impl crate::Readable for BCR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bcr1::W](W) writer structure
impl crate::Writable for BCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BCR1 to value 0x40
impl crate::Resettable for BCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
