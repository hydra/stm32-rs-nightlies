///Register `DR16` reader
pub struct R(crate::R<DR16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR16_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DR16` writer
pub struct W(crate::W<DR16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR16_SPEC>;
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
impl From<crate::W<DR16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR16_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DR16` reader - Data register bits
pub type DR16_R = crate::FieldReader<u16, u16>;
///Field `DR16` writer - Data register bits
pub type DR16_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, DR16_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Data register bits
    #[inline(always)]
    pub fn dr16(&self) -> DR16_R {
        DR16_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - Data register bits
    #[inline(always)]
    #[must_use]
    pub fn dr16(&mut self) -> DR16_W<0> {
        DR16_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u16) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///Data register - half-word sized
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr16](index.html) module
pub struct DR16_SPEC;
impl crate::RegisterSpec for DR16_SPEC {
    type Ux = u16;
}
///`read()` method returns [dr16::R](R) reader structure
impl crate::Readable for DR16_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dr16::W](W) writer structure
impl crate::Writable for DR16_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DR16 to value 0xffff
impl crate::Resettable for DR16_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
