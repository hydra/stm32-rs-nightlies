///Register `ETH_MACARPAR` reader
pub struct R(crate::R<ETH_MACARPAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACARPAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACARPAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACARPAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_MACARPAR` writer
pub struct W(crate::W<ETH_MACARPAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACARPAR_SPEC>;
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
impl From<crate::W<ETH_MACARPAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACARPAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ARPPA` reader - ARPPA
pub type ARPPA_R = crate::FieldReader<u32, u32>;
///Field `ARPPA` writer - ARPPA
pub type ARPPA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACARPAR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - ARPPA
    #[inline(always)]
    pub fn arppa(&self) -> ARPPA_R {
        ARPPA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - ARPPA
    #[inline(always)]
    #[must_use]
    pub fn arppa(&mut self) -> ARPPA_W<0> {
        ARPPA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The ARP Address register contains the IPv4 Destination Address of the MAC.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_macarpar](index.html) module
pub struct ETH_MACARPAR_SPEC;
impl crate::RegisterSpec for ETH_MACARPAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_macarpar::R](R) reader structure
impl crate::Readable for ETH_MACARPAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_macarpar::W](W) writer structure
impl crate::Writable for ETH_MACARPAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_MACARPAR to value 0
impl crate::Resettable for ETH_MACARPAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
