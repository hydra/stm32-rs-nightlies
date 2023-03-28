///Register `OPAMP2_HSOTR` reader
pub struct R(crate::R<OPAMP2_HSOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP2_HSOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP2_HSOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP2_HSOTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPAMP2_HSOTR` writer
pub struct W(crate::W<OPAMP2_HSOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP2_HSOTR_SPEC>;
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
impl From<crate::W<OPAMP2_HSOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP2_HSOTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRIMLPOFFSETN` reader - Trim for NMOS differential pairs
pub type TRIMLPOFFSETN_R = crate::FieldReader<u8, u8>;
///Field `TRIMLPOFFSETN` writer - Trim for NMOS differential pairs
pub type TRIMLPOFFSETN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP2_HSOTR_SPEC, u8, u8, 5, O>;
///Field `TRIMLPOFFSETP` reader - Trim for PMOS differential pairs
pub type TRIMLPOFFSETP_R = crate::FieldReader<u8, u8>;
///Field `TRIMLPOFFSETP` writer - Trim for PMOS differential pairs
pub type TRIMLPOFFSETP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP2_HSOTR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - Trim for NMOS differential pairs
    #[inline(always)]
    pub fn trimlpoffsetn(&self) -> TRIMLPOFFSETN_R {
        TRIMLPOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Trim for PMOS differential pairs
    #[inline(always)]
    pub fn trimlpoffsetp(&self) -> TRIMLPOFFSETP_R {
        TRIMLPOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - Trim for NMOS differential pairs
    #[inline(always)]
    #[must_use]
    pub fn trimlpoffsetn(&mut self) -> TRIMLPOFFSETN_W<0> {
        TRIMLPOFFSETN_W::new(self)
    }
    ///Bits 8:12 - Trim for PMOS differential pairs
    #[inline(always)]
    #[must_use]
    pub fn trimlpoffsetp(&mut self) -> TRIMLPOFFSETP_W<8> {
        TRIMLPOFFSETP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OPAMP2 offset trimming register in low-power mode
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp2_hsotr](index.html) module
pub struct OPAMP2_HSOTR_SPEC;
impl crate::RegisterSpec for OPAMP2_HSOTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [opamp2_hsotr::R](R) reader structure
impl crate::Readable for OPAMP2_HSOTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [opamp2_hsotr::W](W) writer structure
impl crate::Writable for OPAMP2_HSOTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPAMP2_HSOTR to value 0
impl crate::Resettable for OPAMP2_HSOTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
