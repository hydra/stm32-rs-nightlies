///Register `WPABR` reader
pub struct R(crate::R<WPABR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPABR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPABR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPABR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WPABR` writer
pub struct W(crate::W<WPABR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPABR_SPEC>;
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
impl From<crate::W<WPABR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPABR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ALTERNATE` reader - 31: 0\]: Alternate bytes Optional data to be sent to the external SPI device right after the address
pub type ALTERNATE_R = crate::FieldReader<u32, u32>;
///Field `ALTERNATE` writer - 31: 0\]: Alternate bytes Optional data to be sent to the external SPI device right after the address
pub type ALTERNATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPABR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - 31: 0\]: Alternate bytes Optional data to be sent to the external SPI device right after the address
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - 31: 0\]: Alternate bytes Optional data to be sent to the external SPI device right after the address
    #[inline(always)]
    #[must_use]
    pub fn alternate(&mut self) -> ALTERNATE_W<0> {
        ALTERNATE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OCTOSPI wrap alternate bytes register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wpabr](index.html) module
pub struct WPABR_SPEC;
impl crate::RegisterSpec for WPABR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wpabr::R](R) reader structure
impl crate::Readable for WPABR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wpabr::W](W) writer structure
impl crate::Writable for WPABR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WPABR to value 0
impl crate::Resettable for WPABR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
