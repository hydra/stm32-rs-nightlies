///Register `CRCEADDR` reader
pub struct R(crate::R<CRCEADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCEADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCEADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCEADDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRCEADDR` writer
pub struct W(crate::W<CRCEADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCEADDR_SPEC>;
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
impl From<crate::W<CRCEADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCEADDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CRC_END_ADDR` reader - CRC end address on bank 1
pub type CRC_END_ADDR_R = crate::FieldReader<u32, u32>;
///Field `CRC_END_ADDR` writer - CRC end address on bank 1
pub type CRC_END_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CRCEADDR_SPEC, u32, u32, 18, O>;
impl R {
    ///Bits 2:19 - CRC end address on bank 1
    #[inline(always)]
    pub fn crc_end_addr(&self) -> CRC_END_ADDR_R {
        CRC_END_ADDR_R::new((self.bits >> 2) & 0x0003_ffff)
    }
}
impl W {
    ///Bits 2:19 - CRC end address on bank 1
    #[inline(always)]
    #[must_use]
    pub fn crc_end_addr(&mut self) -> CRC_END_ADDR_W<2> {
        CRC_END_ADDR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH CRC end address register for bank 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crceaddr](index.html) module
pub struct CRCEADDR_SPEC;
impl crate::RegisterSpec for CRCEADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [crceaddr::R](R) reader structure
impl crate::Readable for CRCEADDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crceaddr::W](W) writer structure
impl crate::Writable for CRCEADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRCEADDR to value 0
impl crate::Resettable for CRCEADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
