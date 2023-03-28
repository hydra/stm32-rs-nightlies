///Register `PCR` reader
pub struct R(crate::R<PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCR` writer
pub struct W(crate::W<PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SPEC>;
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
impl From<crate::W<PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ETTXE` reader - EoTp Transmission Enable
pub type ETTXE_R = crate::BitReader<bool>;
///Field `ETTXE` writer - EoTp Transmission Enable
pub type ETTXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
///Field `ETRXE` reader - EoTp Reception Enable
pub type ETRXE_R = crate::BitReader<bool>;
///Field `ETRXE` writer - EoTp Reception Enable
pub type ETRXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
///Field `BTAE` reader - Bus Turn Around Enable
pub type BTAE_R = crate::BitReader<bool>;
///Field `BTAE` writer - Bus Turn Around Enable
pub type BTAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
///Field `ECCRXE` reader - ECC Reception Enable
pub type ECCRXE_R = crate::BitReader<bool>;
///Field `ECCRXE` writer - ECC Reception Enable
pub type ECCRXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
///Field `CRCRXE` reader - CRC Reception Enable
pub type CRCRXE_R = crate::BitReader<bool>;
///Field `CRCRXE` writer - CRC Reception Enable
pub type CRCRXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - EoTp Transmission Enable
    #[inline(always)]
    pub fn ettxe(&self) -> ETTXE_R {
        ETTXE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EoTp Reception Enable
    #[inline(always)]
    pub fn etrxe(&self) -> ETRXE_R {
        ETRXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Bus Turn Around Enable
    #[inline(always)]
    pub fn btae(&self) -> BTAE_R {
        BTAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC Reception Enable
    #[inline(always)]
    pub fn eccrxe(&self) -> ECCRXE_R {
        ECCRXE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC Reception Enable
    #[inline(always)]
    pub fn crcrxe(&self) -> CRCRXE_R {
        CRCRXE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - EoTp Transmission Enable
    #[inline(always)]
    #[must_use]
    pub fn ettxe(&mut self) -> ETTXE_W<0> {
        ETTXE_W::new(self)
    }
    ///Bit 1 - EoTp Reception Enable
    #[inline(always)]
    #[must_use]
    pub fn etrxe(&mut self) -> ETRXE_W<1> {
        ETRXE_W::new(self)
    }
    ///Bit 2 - Bus Turn Around Enable
    #[inline(always)]
    #[must_use]
    pub fn btae(&mut self) -> BTAE_W<2> {
        BTAE_W::new(self)
    }
    ///Bit 3 - ECC Reception Enable
    #[inline(always)]
    #[must_use]
    pub fn eccrxe(&mut self) -> ECCRXE_W<3> {
        ECCRXE_W::new(self)
    }
    ///Bit 4 - CRC Reception Enable
    #[inline(always)]
    #[must_use]
    pub fn crcrxe(&mut self) -> CRCRXE_W<4> {
        CRCRXE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host Protocol Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcr](index.html) module
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcr::R](R) reader structure
impl crate::Readable for PCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pcr::W](W) writer structure
impl crate::Writable for PCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PCR to value 0
impl crate::Resettable for PCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
