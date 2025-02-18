///Register `PLL2DIVR` reader
pub struct R(crate::R<PLL2DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL2DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL2DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL2DIVR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PLL2DIVR` writer
pub struct W(crate::W<PLL2DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL2DIVR_SPEC>;
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
impl From<crate::W<PLL2DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL2DIVR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIVN2` reader - multiplication factor for PLL2 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = PLL2RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL2VCOSEL = 0 150 to 420Â MHz if PLL2VCOSEL = 1 VCO output frequency = Fref2_ck x DIVN2, when fractional value 0 has been loaded into FRACN2, with DIVN2 between 8 and 420 The input frequency Fref2_ck must be between 1 and 16MHz.
pub type DIVN2_R = crate::FieldReader<u16, u16>;
///Field `DIVN2` writer - multiplication factor for PLL2 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = PLL2RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL2VCOSEL = 0 150 to 420Â MHz if PLL2VCOSEL = 1 VCO output frequency = Fref2_ck x DIVN2, when fractional value 0 has been loaded into FRACN2, with DIVN2 between 8 and 420 The input frequency Fref2_ck must be between 1 and 16MHz.
pub type DIVN2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL2DIVR_SPEC, u16, u16, 9, O>;
///Field `DIVP2` reader - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ...
pub type DIVP2_R = crate::FieldReader<u8, u8>;
///Field `DIVP2` writer - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ...
pub type DIVP2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLL2DIVR_SPEC, u8, u8, 7, O>;
///Field `DIVQ2` reader - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ...
pub type DIVQ2_R = crate::FieldReader<u8, u8>;
///Field `DIVQ2` writer - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ...
pub type DIVQ2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLL2DIVR_SPEC, u8, u8, 7, O>;
///Field `DIVR2` reader - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ...
pub type DIVR2_R = crate::FieldReader<u8, u8>;
///Field `DIVR2` writer - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ...
pub type DIVR2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PLL2DIVR_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:8 - multiplication factor for PLL2 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = PLL2RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL2VCOSEL = 0 150 to 420Â MHz if PLL2VCOSEL = 1 VCO output frequency = Fref2_ck x DIVN2, when fractional value 0 has been loaded into FRACN2, with DIVN2 between 8 and 420 The input frequency Fref2_ck must be between 1 and 16MHz.
    #[inline(always)]
    pub fn divn2(&self) -> DIVN2_R {
        DIVN2_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ...
    #[inline(always)]
    pub fn divp2(&self) -> DIVP2_R {
        DIVP2_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ...
    #[inline(always)]
    pub fn divq2(&self) -> DIVQ2_R {
        DIVQ2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ...
    #[inline(always)]
    pub fn divr2(&self) -> DIVR2_R {
        DIVR2_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:8 - multiplication factor for PLL2 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = PLL2RDY = 0). ..........: not used ... ... Others: wrong configurations The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: 128 to 560Â MHz if PLL2VCOSEL = 0 150 to 420Â MHz if PLL2VCOSEL = 1 VCO output frequency = Fref2_ck x DIVN2, when fractional value 0 has been loaded into FRACN2, with DIVN2 between 8 and 420 The input frequency Fref2_ck must be between 1 and 16MHz.
    #[inline(always)]
    #[must_use]
    pub fn divn2(&mut self) -> DIVN2_W<0> {
        DIVN2_W::new(self)
    }
    ///Bits 9:15 - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn divp2(&mut self) -> DIVP2_W<9> {
        DIVP2_W::new(self)
    }
    ///Bits 16:22 - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn divq2(&mut self) -> DIVQ2_W<16> {
        DIVQ2_W::new(self)
    }
    ///Bits 24:30 - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = PLL2RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn divr2(&mut self) -> DIVR2_W<24> {
        DIVR2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pll2divr](index.html) module
pub struct PLL2DIVR_SPEC;
impl crate::RegisterSpec for PLL2DIVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pll2divr::R](R) reader structure
impl crate::Readable for PLL2DIVR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pll2divr::W](W) writer structure
impl crate::Writable for PLL2DIVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PLL2DIVR to value 0x0101_0280
impl crate::Resettable for PLL2DIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0280;
}
