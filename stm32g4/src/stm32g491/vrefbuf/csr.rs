///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR` writer
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ENVR` reader - Enable Voltage Reference
pub type ENVR_R = crate::BitReader<bool>;
///Field `ENVR` writer - Enable Voltage Reference
pub type ENVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `HIZ` reader - High impedence mode for the VREF_BUF
pub type HIZ_R = crate::BitReader<bool>;
///Field `HIZ` writer - High impedence mode for the VREF_BUF
pub type HIZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `VRR` reader - Voltage reference buffer ready
pub type VRR_R = crate::BitReader<bool>;
///Field `VRS` reader - Voltage reference scale
pub type VRS_R = crate::FieldReader<u8, u8>;
///Field `VRS` writer - Voltage reference scale
pub type VRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - Enable Voltage Reference
    #[inline(always)]
    pub fn envr(&self) -> ENVR_R {
        ENVR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - High impedence mode for the VREF_BUF
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Voltage reference buffer ready
    #[inline(always)]
    pub fn vrr(&self) -> VRR_R {
        VRR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Voltage reference scale
    #[inline(always)]
    pub fn vrs(&self) -> VRS_R {
        VRS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Enable Voltage Reference
    #[inline(always)]
    #[must_use]
    pub fn envr(&mut self) -> ENVR_W<0> {
        ENVR_W::new(self)
    }
    ///Bit 1 - High impedence mode for the VREF_BUF
    #[inline(always)]
    #[must_use]
    pub fn hiz(&mut self) -> HIZ_W<1> {
        HIZ_W::new(self)
    }
    ///Bits 4:5 - Voltage reference scale
    #[inline(always)]
    #[must_use]
    pub fn vrs(&mut self) -> VRS_W<4> {
        VRS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///VREF_BUF Control and Status Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr::W](W) writer structure
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR to value 0x02
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
