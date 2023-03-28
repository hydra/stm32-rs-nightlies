///Register `MACA2LR` reader
pub struct R(crate::R<MACA2LR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACA2LR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACA2LR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACA2LR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACA2LR` writer
pub struct W(crate::W<MACA2LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACA2LR_SPEC>;
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
impl From<crate::W<MACA2LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACA2LR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MACA2L` reader - MACA2L
pub type MACA2L_R = crate::FieldReader<u32, u32>;
///Field `MACA2L` writer - MACA2L
pub type MACA2L_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACA2LR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - MACA2L
    #[inline(always)]
    pub fn maca2l(&self) -> MACA2L_R {
        MACA2L_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - MACA2L
    #[inline(always)]
    #[must_use]
    pub fn maca2l(&mut self) -> MACA2L_W<0> {
        MACA2L_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///Ethernet MAC address 2 low register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [maca2lr](index.html) module
pub struct MACA2LR_SPEC;
impl crate::RegisterSpec for MACA2LR_SPEC {
    type Ux = u32;
}
///`read()` method returns [maca2lr::R](R) reader structure
impl crate::Readable for MACA2LR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [maca2lr::W](W) writer structure
impl crate::Writable for MACA2LR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACA2LR to value 0xffff_ffff
impl crate::Resettable for MACA2LR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
