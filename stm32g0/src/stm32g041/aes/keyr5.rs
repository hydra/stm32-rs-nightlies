///Register `KEYR5` reader
pub struct R(crate::R<KEYR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR5_SPEC>) -> Self {
        R(reader)
    }
}
///Register `KEYR5` writer
pub struct W(crate::W<KEYR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR5_SPEC>;
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
impl From<crate::W<KEYR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR5_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AES_KEYR5` reader - AES key register (MSB key \[191:160\])
pub type AES_KEYR5_R = crate::FieldReader<u32, u32>;
///Field `AES_KEYR5` writer - AES key register (MSB key \[191:160\])
pub type AES_KEYR5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYR5_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - AES key register (MSB key \[191:160\])
    #[inline(always)]
    pub fn aes_keyr5(&self) -> AES_KEYR5_R {
        AES_KEYR5_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - AES key register (MSB key \[191:160\])
    #[inline(always)]
    #[must_use]
    pub fn aes_keyr5(&mut self) -> AES_KEYR5_W<0> {
        AES_KEYR5_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///key register 5
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [keyr5](index.html) module
pub struct KEYR5_SPEC;
impl crate::RegisterSpec for KEYR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [keyr5::R](R) reader structure
impl crate::Readable for KEYR5_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [keyr5::W](W) writer structure
impl crate::Writable for KEYR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets KEYR5 to value 0
impl crate::Resettable for KEYR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
