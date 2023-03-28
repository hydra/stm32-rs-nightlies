///Register `CRC_DR` reader
pub struct R(crate::R<CRC_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_DR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRC_DR` writer
pub struct W(crate::W<CRC_DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_DR_SPEC>;
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
impl From<crate::W<CRC_DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_DR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DR` reader - DR
pub type DR_R = crate::FieldReader<u32, u32>;
///Field `DR` writer - DR
pub type DR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRC_DR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - DR
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - DR
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<0> {
        DR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CRC data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crc_dr](index.html) module
pub struct CRC_DR_SPEC;
impl crate::RegisterSpec for CRC_DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [crc_dr::R](R) reader structure
impl crate::Readable for CRC_DR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crc_dr::W](W) writer structure
impl crate::Writable for CRC_DR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRC_DR to value 0xffff_ffff
impl crate::Resettable for CRC_DR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
