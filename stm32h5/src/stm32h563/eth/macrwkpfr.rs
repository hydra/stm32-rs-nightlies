///Register `MACRWKPFR` reader
pub struct R(crate::R<MACRWKPFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACRWKPFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACRWKPFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACRWKPFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACRWKPFR` writer
pub struct W(crate::W<MACRWKPFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACRWKPFR_SPEC>;
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
impl From<crate::W<MACRWKPFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACRWKPFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MACRWKPFR` reader - Remote wakeup packet filter Refer to Table�648, Table�649 and Table�650 for details on register content and programming sequence.
pub type MACRWKPFR_R = crate::FieldReader<u32, u32>;
///Field `MACRWKPFR` writer - Remote wakeup packet filter Refer to Table�648, Table�649 and Table�650 for details on register content and programming sequence.
pub type MACRWKPFR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MACRWKPFR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Remote wakeup packet filter Refer to Table�648, Table�649 and Table�650 for details on register content and programming sequence.
    #[inline(always)]
    pub fn macrwkpfr(&self) -> MACRWKPFR_R {
        MACRWKPFR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Remote wakeup packet filter Refer to Table�648, Table�649 and Table�650 for details on register content and programming sequence.
    #[inline(always)]
    #[must_use]
    pub fn macrwkpfr(&mut self) -> MACRWKPFR_W<0> {
        MACRWKPFR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Remote wakeup packet filter register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macrwkpfr](index.html) module
pub struct MACRWKPFR_SPEC;
impl crate::RegisterSpec for MACRWKPFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macrwkpfr::R](R) reader structure
impl crate::Readable for MACRWKPFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macrwkpfr::W](W) writer structure
impl crate::Writable for MACRWKPFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACRWKPFR to value 0
impl crate::Resettable for MACRWKPFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
