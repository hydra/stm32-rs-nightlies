///Register `SCR` reader
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SCR` writer
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CALRAF` reader - CALRAF
pub type CALRAF_R = crate::BitReader<bool>;
///Field `CALRAF` writer - CALRAF
pub type CALRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `CALRBF` reader - CALRBF
pub type CALRBF_R = crate::BitReader<bool>;
///Field `CALRBF` writer - CALRBF
pub type CALRBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `CWUTF` reader - CWUTF
pub type CWUTF_R = crate::BitReader<bool>;
///Field `CWUTF` writer - CWUTF
pub type CWUTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `CTSF` reader - CTSF
pub type CTSF_R = crate::BitReader<bool>;
///Field `CTSF` writer - CTSF
pub type CTSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `CTSOVF` reader - CTSOVF
pub type CTSOVF_R = crate::BitReader<bool>;
///Field `CTSOVF` writer - CTSOVF
pub type CTSOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `CITSF` reader - CITSF
pub type CITSF_R = crate::BitReader<bool>;
///Field `CITSF` writer - CITSF
pub type CITSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - CALRAF
    #[inline(always)]
    pub fn calraf(&self) -> CALRAF_R {
        CALRAF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CALRBF
    #[inline(always)]
    pub fn calrbf(&self) -> CALRBF_R {
        CALRBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CWUTF
    #[inline(always)]
    pub fn cwutf(&self) -> CWUTF_R {
        CWUTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CTSF
    #[inline(always)]
    pub fn ctsf(&self) -> CTSF_R {
        CTSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CTSOVF
    #[inline(always)]
    pub fn ctsovf(&self) -> CTSOVF_R {
        CTSOVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CITSF
    #[inline(always)]
    pub fn citsf(&self) -> CITSF_R {
        CITSF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CALRAF
    #[inline(always)]
    #[must_use]
    pub fn calraf(&mut self) -> CALRAF_W<0> {
        CALRAF_W::new(self)
    }
    ///Bit 1 - CALRBF
    #[inline(always)]
    #[must_use]
    pub fn calrbf(&mut self) -> CALRBF_W<1> {
        CALRBF_W::new(self)
    }
    ///Bit 2 - CWUTF
    #[inline(always)]
    #[must_use]
    pub fn cwutf(&mut self) -> CWUTF_W<2> {
        CWUTF_W::new(self)
    }
    ///Bit 3 - CTSF
    #[inline(always)]
    #[must_use]
    pub fn ctsf(&mut self) -> CTSF_W<3> {
        CTSF_W::new(self)
    }
    ///Bit 4 - CTSOVF
    #[inline(always)]
    #[must_use]
    pub fn ctsovf(&mut self) -> CTSOVF_W<4> {
        CTSOVF_W::new(self)
    }
    ///Bit 5 - CITSF
    #[inline(always)]
    #[must_use]
    pub fn citsf(&mut self) -> CITSF_W<5> {
        CITSF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///status clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [scr](index.html) module
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [scr::R](R) reader structure
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [scr::W](W) writer structure
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
