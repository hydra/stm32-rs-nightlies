///Register `B2CR` reader
pub struct R(crate::R<B2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B2CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `B2CR` writer
pub struct W(crate::W<B2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<B2CR_SPEC>;
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
impl From<crate::W<B2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<B2CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PBO` reader - Physical buffer offset
pub type PBO_R = crate::FieldReader<u32, u32>;
///Field `PBO` writer - Physical buffer offset
pub type PBO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, B2CR_SPEC, u32, u32, 19, O>;
///Field `PBBA` reader - Physical buffer base address
pub type PBBA_R = crate::FieldReader<u16, u16>;
///Field `PBBA` writer - Physical buffer base address
pub type PBBA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, B2CR_SPEC, u16, u16, 9, O>;
impl R {
    ///Bits 4:22 - Physical buffer offset
    #[inline(always)]
    pub fn pbo(&self) -> PBO_R {
        PBO_R::new((self.bits >> 4) & 0x0007_ffff)
    }
    ///Bits 23:31 - Physical buffer base address
    #[inline(always)]
    pub fn pbba(&self) -> PBBA_R {
        PBBA_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    ///Bits 4:22 - Physical buffer offset
    #[inline(always)]
    #[must_use]
    pub fn pbo(&mut self) -> PBO_W<4> {
        PBO_W::new(self)
    }
    ///Bits 23:31 - Physical buffer base address
    #[inline(always)]
    #[must_use]
    pub fn pbba(&mut self) -> PBBA_W<23> {
        PBBA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Graphic MMU buffer 2 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [b2cr](index.html) module
pub struct B2CR_SPEC;
impl crate::RegisterSpec for B2CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [b2cr::R](R) reader structure
impl crate::Readable for B2CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [b2cr::W](W) writer structure
impl crate::Writable for B2CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets B2CR to value 0
impl crate::Resettable for B2CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
