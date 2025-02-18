///Register `TXBTIE` reader
pub struct R(crate::R<TXBTIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBTIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBTIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBTIE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TXBTIE` writer
pub struct W(crate::W<TXBTIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBTIE_SPEC>;
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
impl From<crate::W<TXBTIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBTIE_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIE` reader - Transmission interrupt enable Each Tx buffer has its own TIE bit.
pub type TIE_R = crate::FieldReader<u8, u8>;
///Field `TIE` writer - Transmission interrupt enable Each Tx buffer has its own TIE bit.
pub type TIE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXBTIE_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:2 - Transmission interrupt enable Each Tx buffer has its own TIE bit.
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - Transmission interrupt enable Each Tx buffer has its own TIE bit.
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<0> {
        TIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Tx buffer transmission interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txbtie](index.html) module
pub struct TXBTIE_SPEC;
impl crate::RegisterSpec for TXBTIE_SPEC {
    type Ux = u32;
}
///`read()` method returns [txbtie::R](R) reader structure
impl crate::Readable for TXBTIE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [txbtie::W](W) writer structure
impl crate::Writable for TXBTIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TXBTIE to value 0
impl crate::Resettable for TXBTIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
