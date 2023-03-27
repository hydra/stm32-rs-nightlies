///Register `RCC_PLL2DIVR` reader
pub struct R(crate::R<RCC_PLL2DIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL2DIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL2DIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL2DIVR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_PLL2DIVR` writer
pub struct W(crate::W<RCC_PLL2DIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL2DIVR_SPEC>;
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
impl From<crate::W<RCC_PLL2DIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL2DIVR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLL2N` reader - Multiplication factor for PLL2 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved VCO output frequency = Fref2_ck x PLL2N, when fractional value 0 has been loaded into PLL2FRACN, with: PLL2N between 4 and 512 input frequency Fref2_ck between 1MHz and 16MHz
pub type PLL2N_R = crate::FieldReader<u16, u16>;
///Field `PLL2N` writer - Multiplication factor for PLL2 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved VCO output frequency = Fref2_ck x PLL2N, when fractional value 0 has been loaded into PLL2FRACN, with: PLL2N between 4 and 512 input frequency Fref2_ck between 1MHz and 16MHz
pub type PLL2N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL2DIVR_SPEC, u16, u16, 9, O>;
///Field `PLL2P` reader - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2P_R = crate::FieldReader<u8, u8>;
///Field `PLL2P` writer - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL2DIVR_SPEC, u8, u8, 7, O>;
///Field `PLL2Q` reader - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2Q_R = crate::FieldReader<u8, u8>;
///Field `PLL2Q` writer - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2Q_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL2DIVR_SPEC, u8, u8, 7, O>;
///Field `PLL2R` reader - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2R_R = crate::FieldReader<u8, u8>;
///Field `PLL2R` writer - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL2DIVR_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:8 - Multiplication factor for PLL2 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved VCO output frequency = Fref2_ck x PLL2N, when fractional value 0 has been loaded into PLL2FRACN, with: PLL2N between 4 and 512 input frequency Fref2_ck between 1MHz and 16MHz
    #[inline(always)]
    pub fn pll2n(&self) -> PLL2N_R {
        PLL2N_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2p(&self) -> PLL2P_R {
        PLL2P_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:22 - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2q(&self) -> PLL2Q_R {
        PLL2Q_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bits 24:30 - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2r(&self) -> PLL2R_R {
        PLL2R_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:8 - Multiplication factor for PLL2 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0). ... ... Others: reserved VCO output frequency = Fref2_ck x PLL2N, when fractional value 0 has been loaded into PLL2FRACN, with: PLL2N between 4 and 512 input frequency Fref2_ck between 1MHz and 16MHz
    #[inline(always)]
    #[must_use]
    pub fn pll2n(&mut self) -> PLL2N_W<0> {
        PLL2N_W::new(self)
    }
    ///Bits 9:15 - PLL2 DIVP division factor Set and reset by software to control the frequency of the pll2_p_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll2p(&mut self) -> PLL2P_W<9> {
        PLL2P_W::new(self)
    }
    ///Bits 16:22 - PLL2 DIVQ division factor Set and reset by software to control the frequency of the pll2_q_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll2q(&mut self) -> PLL2Q_W<16> {
        PLL2Q_W::new(self)
    }
    ///Bits 24:30 - PLL2 DIVR division factor Set and reset by software to control the frequency of the pll2_r_ck clock. These bits can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll2r(&mut self) -> PLL2R_W<24> {
        PLL2R_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC PLL2 dividers configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_pll2divr](index.html) module
pub struct RCC_PLL2DIVR_SPEC;
impl crate::RegisterSpec for RCC_PLL2DIVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_pll2divr::R](R) reader structure
impl crate::Readable for RCC_PLL2DIVR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_pll2divr::W](W) writer structure
impl crate::Writable for RCC_PLL2DIVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_PLL2DIVR to value 0x0101_0280
impl crate::Resettable for RCC_PLL2DIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0280;
}
