///Register `DTR2` reader
pub struct R(crate::R<DTR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DTR2` writer
pub struct W(crate::W<DTR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTR2_SPEC>;
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
impl From<crate::W<DTR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DTGF` reader - Dead-time generator setup
pub type DTGF_R = crate::FieldReader<u8, u8>;
///Field `DTGF` writer - Dead-time generator setup
pub type DTGF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTR2_SPEC, u8, u8, 8, O>;
///Field `DTAE` reader - Deadtime Asymmetric Enable
pub type DTAE_R = crate::BitReader<bool>;
///Field `DTAE` writer - Deadtime Asymmetric Enable
pub type DTAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR2_SPEC, bool, O>;
///Field `DTPE` reader - Deadtime Preload Enable
pub type DTPE_R = crate::BitReader<bool>;
///Field `DTPE` writer - Deadtime Preload Enable
pub type DTPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTR2_SPEC, bool, O>;
impl R {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtgf(&self) -> DTGF_R {
        DTGF_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 16 - Deadtime Asymmetric Enable
    #[inline(always)]
    pub fn dtae(&self) -> DTAE_R {
        DTAE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Deadtime Preload Enable
    #[inline(always)]
    pub fn dtpe(&self) -> DTPE_R {
        DTPE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    #[must_use]
    pub fn dtgf(&mut self) -> DTGF_W<0> {
        DTGF_W::new(self)
    }
    ///Bit 16 - Deadtime Asymmetric Enable
    #[inline(always)]
    #[must_use]
    pub fn dtae(&mut self) -> DTAE_W<16> {
        DTAE_W::new(self)
    }
    ///Bit 17 - Deadtime Preload Enable
    #[inline(always)]
    #[must_use]
    pub fn dtpe(&mut self) -> DTPE_W<17> {
        DTPE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///timer Deadtime Register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dtr2](index.html) module
pub struct DTR2_SPEC;
impl crate::RegisterSpec for DTR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [dtr2::R](R) reader structure
impl crate::Readable for DTR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dtr2::W](W) writer structure
impl crate::Writable for DTR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DTR2 to value 0
impl crate::Resettable for DTR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
