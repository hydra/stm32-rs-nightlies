///Register `KEYR` reader
pub struct R(crate::R<KEYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `KEYR` writer
pub struct W(crate::W<KEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR_SPEC>;
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
impl From<crate::W<KEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `KEY` reader - Semaphore Clear Key
pub type KEY_R = crate::FieldReader<u16, u16>;
///Field `KEY` writer - Semaphore Clear Key
pub type KEY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, KEYR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 16:31 - Semaphore Clear Key
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 16:31 - Semaphore Clear Key
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<16> {
        KEY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HSEM Interrupt clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [keyr](index.html) module
pub struct KEYR_SPEC;
impl crate::RegisterSpec for KEYR_SPEC {
    type Ux = u32;
}
///`read()` method returns [keyr::R](R) reader structure
impl crate::Readable for KEYR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [keyr::W](W) writer structure
impl crate::Writable for KEYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets KEYR to value 0
impl crate::Resettable for KEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
