///Register `FPCCR` reader
pub struct R(crate::R<FPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FPCCR` writer
pub struct W(crate::W<FPCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPCCR_SPEC>;
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
impl From<crate::W<FPCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSPACT` reader - LSPACT
pub type LSPACT_R = crate::BitReader<bool>;
///Field `LSPACT` writer - LSPACT
pub type LSPACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
///Field `USER` reader - USER
pub type USER_R = crate::BitReader<bool>;
///Field `USER` writer - USER
pub type USER_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
///Field `THREAD` reader - THREAD
pub type THREAD_R = crate::BitReader<bool>;
///Field `THREAD` writer - THREAD
pub type THREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
///Field `HFRDY` reader - HFRDY
pub type HFRDY_R = crate::BitReader<bool>;
///Field `HFRDY` writer - HFRDY
pub type HFRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
///Field `MMRDY` reader - MMRDY
pub type MMRDY_R = crate::BitReader<bool>;
///Field `MMRDY` writer - MMRDY
pub type MMRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
///Field `BFRDY` reader - BFRDY
pub type BFRDY_R = crate::BitReader<bool>;
///Field `BFRDY` writer - BFRDY
pub type BFRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
///Field `MONRDY` reader - MONRDY
pub type MONRDY_R = crate::BitReader<bool>;
///Field `MONRDY` writer - MONRDY
pub type MONRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
///Field `LSPEN` reader - LSPEN
pub type LSPEN_R = crate::BitReader<bool>;
///Field `LSPEN` writer - LSPEN
pub type LSPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
///Field `ASPEN` reader - ASPEN
pub type ASPEN_R = crate::BitReader<bool>;
///Field `ASPEN` writer - ASPEN
pub type ASPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPCCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - LSPACT
    #[inline(always)]
    pub fn lspact(&self) -> LSPACT_R {
        LSPACT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USER
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - THREAD
    #[inline(always)]
    pub fn thread(&self) -> THREAD_R {
        THREAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HFRDY
    #[inline(always)]
    pub fn hfrdy(&self) -> HFRDY_R {
        HFRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MMRDY
    #[inline(always)]
    pub fn mmrdy(&self) -> MMRDY_R {
        MMRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - BFRDY
    #[inline(always)]
    pub fn bfrdy(&self) -> BFRDY_R {
        BFRDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - MONRDY
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 30 - LSPEN
    #[inline(always)]
    pub fn lspen(&self) -> LSPEN_R {
        LSPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ASPEN
    #[inline(always)]
    pub fn aspen(&self) -> ASPEN_R {
        ASPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LSPACT
    #[inline(always)]
    #[must_use]
    pub fn lspact(&mut self) -> LSPACT_W<0> {
        LSPACT_W::new(self)
    }
    ///Bit 1 - USER
    #[inline(always)]
    #[must_use]
    pub fn user(&mut self) -> USER_W<1> {
        USER_W::new(self)
    }
    ///Bit 3 - THREAD
    #[inline(always)]
    #[must_use]
    pub fn thread(&mut self) -> THREAD_W<3> {
        THREAD_W::new(self)
    }
    ///Bit 4 - HFRDY
    #[inline(always)]
    #[must_use]
    pub fn hfrdy(&mut self) -> HFRDY_W<4> {
        HFRDY_W::new(self)
    }
    ///Bit 5 - MMRDY
    #[inline(always)]
    #[must_use]
    pub fn mmrdy(&mut self) -> MMRDY_W<5> {
        MMRDY_W::new(self)
    }
    ///Bit 6 - BFRDY
    #[inline(always)]
    #[must_use]
    pub fn bfrdy(&mut self) -> BFRDY_W<6> {
        BFRDY_W::new(self)
    }
    ///Bit 8 - MONRDY
    #[inline(always)]
    #[must_use]
    pub fn monrdy(&mut self) -> MONRDY_W<8> {
        MONRDY_W::new(self)
    }
    ///Bit 30 - LSPEN
    #[inline(always)]
    #[must_use]
    pub fn lspen(&mut self) -> LSPEN_W<30> {
        LSPEN_W::new(self)
    }
    ///Bit 31 - ASPEN
    #[inline(always)]
    #[must_use]
    pub fn aspen(&mut self) -> ASPEN_W<31> {
        ASPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Floating-point context control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fpccr](index.html) module
pub struct FPCCR_SPEC;
impl crate::RegisterSpec for FPCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fpccr::R](R) reader structure
impl crate::Readable for FPCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fpccr::W](W) writer structure
impl crate::Writable for FPCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FPCCR to value 0
impl crate::Resettable for FPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
