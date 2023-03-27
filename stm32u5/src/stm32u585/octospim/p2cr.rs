///Register `P2CR` reader
pub struct R(crate::R<P2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `P2CR` writer
pub struct W(crate::W<P2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2CR_SPEC>;
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
impl From<crate::W<P2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CLKEN` reader - CLKEN
pub type CLKEN_R = crate::BitReader<bool>;
///Field `CLKEN` writer - CLKEN
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, P2CR_SPEC, bool, O>;
///Field `CLKSRC` reader - CLKSRC
pub type CLKSRC_R = crate::BitReader<bool>;
///Field `CLKSRC` writer - CLKSRC
pub type CLKSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, P2CR_SPEC, bool, O>;
///Field `DQSEN` reader - DQSEN
pub type DQSEN_R = crate::BitReader<bool>;
///Field `DQSEN` writer - DQSEN
pub type DQSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, P2CR_SPEC, bool, O>;
///Field `DQSSRC` reader - DQSSRC
pub type DQSSRC_R = crate::BitReader<bool>;
///Field `DQSSRC` writer - DQSSRC
pub type DQSSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, P2CR_SPEC, bool, O>;
///Field `NCSEN` reader - NCSEN
pub type NCSEN_R = crate::BitReader<bool>;
///Field `NCSEN` writer - NCSEN
pub type NCSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, P2CR_SPEC, bool, O>;
///Field `NCSSRC` reader - NCSSRC
pub type NCSSRC_R = crate::BitReader<bool>;
///Field `NCSSRC` writer - NCSSRC
pub type NCSSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, P2CR_SPEC, bool, O>;
///Field `IOLEN` reader - IOLEN
pub type IOLEN_R = crate::BitReader<bool>;
///Field `IOLEN` writer - IOLEN
pub type IOLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, P2CR_SPEC, bool, O>;
///Field `IOLSRC` reader - IOLSRC
pub type IOLSRC_R = crate::FieldReader<u8, u8>;
///Field `IOLSRC` writer - IOLSRC
pub type IOLSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, P2CR_SPEC, u8, u8, 2, O>;
///Field `IOHEN` reader - IOHEN
pub type IOHEN_R = crate::BitReader<bool>;
///Field `IOHEN` writer - IOHEN
pub type IOHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, P2CR_SPEC, bool, O>;
///Field `IOHSRC` reader - IOHSR
pub type IOHSRC_R = crate::FieldReader<u8, u8>;
///Field `IOHSRC` writer - IOHSR
pub type IOHSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, P2CR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - CLKEN
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CLKSRC
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - DQSEN
    #[inline(always)]
    pub fn dqsen(&self) -> DQSEN_R {
        DQSEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DQSSRC
    #[inline(always)]
    pub fn dqssrc(&self) -> DQSSRC_R {
        DQSSRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - NCSEN
    #[inline(always)]
    pub fn ncsen(&self) -> NCSEN_R {
        NCSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - NCSSRC
    #[inline(always)]
    pub fn ncssrc(&self) -> NCSSRC_R {
        NCSSRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - IOLEN
    #[inline(always)]
    pub fn iolen(&self) -> IOLEN_R {
        IOLEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - IOLSRC
    #[inline(always)]
    pub fn iolsrc(&self) -> IOLSRC_R {
        IOLSRC_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 24 - IOHEN
    #[inline(always)]
    pub fn iohen(&self) -> IOHEN_R {
        IOHEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - IOHSR
    #[inline(always)]
    pub fn iohsrc(&self) -> IOHSRC_R {
        IOHSRC_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - CLKEN
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<0> {
        CLKEN_W::new(self)
    }
    ///Bit 1 - CLKSRC
    #[inline(always)]
    #[must_use]
    pub fn clksrc(&mut self) -> CLKSRC_W<1> {
        CLKSRC_W::new(self)
    }
    ///Bit 4 - DQSEN
    #[inline(always)]
    #[must_use]
    pub fn dqsen(&mut self) -> DQSEN_W<4> {
        DQSEN_W::new(self)
    }
    ///Bit 5 - DQSSRC
    #[inline(always)]
    #[must_use]
    pub fn dqssrc(&mut self) -> DQSSRC_W<5> {
        DQSSRC_W::new(self)
    }
    ///Bit 8 - NCSEN
    #[inline(always)]
    #[must_use]
    pub fn ncsen(&mut self) -> NCSEN_W<8> {
        NCSEN_W::new(self)
    }
    ///Bit 9 - NCSSRC
    #[inline(always)]
    #[must_use]
    pub fn ncssrc(&mut self) -> NCSSRC_W<9> {
        NCSSRC_W::new(self)
    }
    ///Bit 16 - IOLEN
    #[inline(always)]
    #[must_use]
    pub fn iolen(&mut self) -> IOLEN_W<16> {
        IOLEN_W::new(self)
    }
    ///Bits 17:18 - IOLSRC
    #[inline(always)]
    #[must_use]
    pub fn iolsrc(&mut self) -> IOLSRC_W<17> {
        IOLSRC_W::new(self)
    }
    ///Bit 24 - IOHEN
    #[inline(always)]
    #[must_use]
    pub fn iohen(&mut self) -> IOHEN_W<24> {
        IOHEN_W::new(self)
    }
    ///Bits 25:26 - IOHSR
    #[inline(always)]
    #[must_use]
    pub fn iohsrc(&mut self) -> IOHSRC_W<25> {
        IOHSRC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OCTOSPI I/O manager Port 2 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [p2cr](index.html) module
pub struct P2CR_SPEC;
impl crate::RegisterSpec for P2CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [p2cr::R](R) reader structure
impl crate::Readable for P2CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [p2cr::W](W) writer structure
impl crate::Writable for P2CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets P2CR to value 0x0705_0333
impl crate::Resettable for P2CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0705_0333;
}
