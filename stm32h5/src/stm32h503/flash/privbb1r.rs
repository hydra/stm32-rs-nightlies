///Register `PRIVBB1R` reader
pub struct R(crate::R<PRIVBB1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVBB1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVBB1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVBB1R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PRIVBB1R` writer
pub struct W(crate::W<PRIVBB1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVBB1R_SPEC>;
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
impl From<crate::W<PRIVBB1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVBB1R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRIVBB1` reader - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)
pub type PRIVBB1_R = crate::FieldReader<u8, u8>;
///Field `PRIVBB1` writer - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)
pub type PRIVBB1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRIVBB1R_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)
    #[inline(always)]
    pub fn privbb1(&self) -> PRIVBB1_R {
        PRIVBB1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)
    #[inline(always)]
    #[must_use]
    pub fn privbb1(&mut self) -> PRIVBB1_W<0> {
        PRIVBB1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH privilege register for bank 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [privbb1r](index.html) module
pub struct PRIVBB1R_SPEC;
impl crate::RegisterSpec for PRIVBB1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [privbb1r::R](R) reader structure
impl crate::Readable for PRIVBB1R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [privbb1r::W](W) writer structure
impl crate::Writable for PRIVBB1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PRIVBB1R to value 0
impl crate::Resettable for PRIVBB1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
