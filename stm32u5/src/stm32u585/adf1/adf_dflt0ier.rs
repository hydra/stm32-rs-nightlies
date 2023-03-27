///Register `ADF_DFLT0IER` reader
pub struct R(crate::R<ADF_DFLT0IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_DFLT0IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_DFLT0IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_DFLT0IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADF_DFLT0IER` writer
pub struct W(crate::W<ADF_DFLT0IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADF_DFLT0IER_SPEC>;
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
impl From<crate::W<ADF_DFLT0IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADF_DFLT0IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FTHIE` reader - RXFIFO threshold interrupt enable
pub type FTHIE_R = crate::BitReader<bool>;
///Field `FTHIE` writer - RXFIFO threshold interrupt enable
pub type FTHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0IER_SPEC, bool, O>;
///Field `DOVRIE` reader - Data overflow interrupt enable
pub type DOVRIE_R = crate::BitReader<bool>;
///Field `DOVRIE` writer - Data overflow interrupt enable
pub type DOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0IER_SPEC, bool, O>;
///Field `SATIE` reader - Saturation detection interrupt enable
pub type SATIE_R = crate::BitReader<bool>;
///Field `SATIE` writer - Saturation detection interrupt enable
pub type SATIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0IER_SPEC, bool, O>;
///Field `CKABIE` reader - Clock absence detection interrupt enable
pub type CKABIE_R = crate::BitReader<bool>;
///Field `CKABIE` writer - Clock absence detection interrupt enable
pub type CKABIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0IER_SPEC, bool, O>;
///Field `RFOVRIE` reader - Reshape filter overrun interrupt enable
pub type RFOVRIE_R = crate::BitReader<bool>;
///Field `RFOVRIE` writer - Reshape filter overrun interrupt enable
pub type RFOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0IER_SPEC, bool, O>;
///Field `SDDETIE` reader - Sound activity detection interrupt enable
pub type SDDETIE_R = crate::BitReader<bool>;
///Field `SDDETIE` writer - Sound activity detection interrupt enable
pub type SDDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0IER_SPEC, bool, O>;
///Field `SDLVLIE` reader - SAD sound-level value ready enable
pub type SDLVLIE_R = crate::BitReader<bool>;
///Field `SDLVLIE` writer - SAD sound-level value ready enable
pub type SDLVLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DFLT0IER_SPEC, bool, O>;
impl R {
    ///Bit 0 - RXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn fthie(&self) -> FTHIE_R {
        FTHIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Data overflow interrupt enable
    #[inline(always)]
    pub fn dovrie(&self) -> DOVRIE_R {
        DOVRIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - Saturation detection interrupt enable
    #[inline(always)]
    pub fn satie(&self) -> SATIE_R {
        SATIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock absence detection interrupt enable
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reshape filter overrun interrupt enable
    #[inline(always)]
    pub fn rfovrie(&self) -> RFOVRIE_R {
        RFOVRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Sound activity detection interrupt enable
    #[inline(always)]
    pub fn sddetie(&self) -> SDDETIE_R {
        SDDETIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SAD sound-level value ready enable
    #[inline(always)]
    pub fn sdlvlie(&self) -> SDLVLIE_R {
        SDLVLIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RXFIFO threshold interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn fthie(&mut self) -> FTHIE_W<0> {
        FTHIE_W::new(self)
    }
    ///Bit 1 - Data overflow interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn dovrie(&mut self) -> DOVRIE_W<1> {
        DOVRIE_W::new(self)
    }
    ///Bit 9 - Saturation detection interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn satie(&mut self) -> SATIE_W<9> {
        SATIE_W::new(self)
    }
    ///Bit 10 - Clock absence detection interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ckabie(&mut self) -> CKABIE_W<10> {
        CKABIE_W::new(self)
    }
    ///Bit 11 - Reshape filter overrun interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rfovrie(&mut self) -> RFOVRIE_W<11> {
        RFOVRIE_W::new(self)
    }
    ///Bit 12 - Sound activity detection interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn sddetie(&mut self) -> SDDETIE_W<12> {
        SDDETIE_W::new(self)
    }
    ///Bit 13 - SAD sound-level value ready enable
    #[inline(always)]
    #[must_use]
    pub fn sdlvlie(&mut self) -> SDLVLIE_W<13> {
        SDLVLIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADF DFLT0 interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_dflt0ier](index.html) module
pub struct ADF_DFLT0IER_SPEC;
impl crate::RegisterSpec for ADF_DFLT0IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_dflt0ier::R](R) reader structure
impl crate::Readable for ADF_DFLT0IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adf_dflt0ier::W](W) writer structure
impl crate::Writable for ADF_DFLT0IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADF_DFLT0IER to value 0
impl crate::Resettable for ADF_DFLT0IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
