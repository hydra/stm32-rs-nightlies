///Register `FDCAN_RXBC` reader
pub struct R(crate::R<FDCAN_RXBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXBC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_RXBC` writer
pub struct W(crate::W<FDCAN_RXBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_RXBC_SPEC>;
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
impl From<crate::W<FDCAN_RXBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_RXBC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RBSA` reader - Rx Buffer Start Address
pub type RBSA_R = crate::FieldReader<u16, u16>;
///Field `RBSA` writer - Rx Buffer Start Address
pub type RBSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_RXBC_SPEC, u16, u16, 14, O>;
impl R {
    ///Bits 2:15 - Rx Buffer Start Address
    #[inline(always)]
    pub fn rbsa(&self) -> RBSA_R {
        RBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 2:15 - Rx Buffer Start Address
    #[inline(always)]
    #[must_use]
    pub fn rbsa(&mut self) -> RBSA_W<2> {
        RBSA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Rx Buffer Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_rxbc](index.html) module
pub struct FDCAN_RXBC_SPEC;
impl crate::RegisterSpec for FDCAN_RXBC_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_rxbc::R](R) reader structure
impl crate::Readable for FDCAN_RXBC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_rxbc::W](W) writer structure
impl crate::Writable for FDCAN_RXBC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_RXBC to value 0
impl crate::Resettable for FDCAN_RXBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
