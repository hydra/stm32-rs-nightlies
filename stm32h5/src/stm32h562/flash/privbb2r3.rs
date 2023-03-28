///Register `PRIVBB2R3` reader
pub struct R(crate::R<PRIVBB2R3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVBB2R3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVBB2R3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVBB2R3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PRIVBB2R3` writer
pub struct W(crate::W<PRIVBB2R3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVBB2R3_SPEC>;
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
impl From<crate::W<PRIVBB2R3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVBB2R3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRIVBB2` reader - Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute
pub type PRIVBB2_R = crate::FieldReader<u32, u32>;
///Field `PRIVBB2` writer - Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute
pub type PRIVBB2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRIVBB2R3_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute
    #[inline(always)]
    pub fn privbb2(&self) -> PRIVBB2_R {
        PRIVBB2_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Privileged / non-privileged 8-Kbyte flash Bank 2 sector attribute
    #[inline(always)]
    #[must_use]
    pub fn privbb2(&mut self) -> PRIVBB2_W<0> {
        PRIVBB2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH privilege block-based register for Bank 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [privbb2r3](index.html) module
pub struct PRIVBB2R3_SPEC;
impl crate::RegisterSpec for PRIVBB2R3_SPEC {
    type Ux = u32;
}
///`read()` method returns [privbb2r3::R](R) reader structure
impl crate::Readable for PRIVBB2R3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [privbb2r3::W](W) writer structure
impl crate::Writable for PRIVBB2R3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PRIVBB2R3 to value 0
impl crate::Resettable for PRIVBB2R3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
