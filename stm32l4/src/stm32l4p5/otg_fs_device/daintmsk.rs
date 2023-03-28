///Register `DAINTMSK` reader
pub struct R(crate::R<DAINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DAINTMSK` writer
pub struct W(crate::W<DAINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAINTMSK_SPEC>;
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
impl From<crate::W<DAINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IEPM` reader - IN EP interrupt mask bits
pub type IEPM_R = crate::FieldReader<u16, u16>;
///Field `IEPM` writer - IN EP interrupt mask bits
pub type IEPM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAINTMSK_SPEC, u16, u16, 16, O>;
///Field `OEPINT` reader - OUT endpoint interrupt bits
pub type OEPINT_R = crate::FieldReader<u16, u16>;
///Field `OEPINT` writer - OUT endpoint interrupt bits
pub type OEPINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAINTMSK_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - IN EP interrupt mask bits
    #[inline(always)]
    pub fn iepm(&self) -> IEPM_R {
        IEPM_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - OUT endpoint interrupt bits
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - IN EP interrupt mask bits
    #[inline(always)]
    #[must_use]
    pub fn iepm(&mut self) -> IEPM_W<0> {
        IEPM_W::new(self)
    }
    ///Bits 16:31 - OUT endpoint interrupt bits
    #[inline(always)]
    #[must_use]
    pub fn oepint(&mut self) -> OEPINT_W<16> {
        OEPINT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [daintmsk](index.html) module
pub struct DAINTMSK_SPEC;
impl crate::RegisterSpec for DAINTMSK_SPEC {
    type Ux = u32;
}
///`read()` method returns [daintmsk::R](R) reader structure
impl crate::Readable for DAINTMSK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [daintmsk::W](W) writer structure
impl crate::Writable for DAINTMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DAINTMSK to value 0
impl crate::Resettable for DAINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
