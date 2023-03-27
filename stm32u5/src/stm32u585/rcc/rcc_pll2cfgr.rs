///Register `RCC_PLL2CFGR` reader
pub struct R(crate::R<RCC_PLL2CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL2CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL2CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL2CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_PLL2CFGR` writer
pub struct W(crate::W<RCC_PLL2CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL2CFGR_SPEC>;
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
impl From<crate::W<RCC_PLL2CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL2CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLL2SRC` reader - PLL2 entry clock source Set and cleared by software to select PLL2 clock source. These bits can be written only when the PLL2 is disabled. In order to save power, when no PLL2 is used, the value of PLL2SRC must be 0.
pub type PLL2SRC_R = crate::FieldReader<u8, u8>;
///Field `PLL2SRC` writer - PLL2 entry clock source Set and cleared by software to select PLL2 clock source. These bits can be written only when the PLL2 is disabled. In order to save power, when no PLL2 is used, the value of PLL2SRC must be 0.
pub type PLL2SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL2CFGR_SPEC, u8, u8, 2, O>;
///Field `PLL2RGE` reader - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. This bit must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
pub type PLL2RGE_R = crate::FieldReader<u8, u8>;
///Field `PLL2RGE` writer - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. This bit must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
pub type PLL2RGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL2CFGR_SPEC, u8, u8, 2, O>;
///Field `PLL2FRACEN` reader - PLL2 fractional latch enable Set and reset by software to latch the content of PLL2FRACN into the Î£Î modulator. In order to latch the PLL2FRACN value into the Î£Î modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see for details).
pub type PLL2FRACEN_R = crate::BitReader<bool>;
///Field `PLL2FRACEN` writer - PLL2 fractional latch enable Set and reset by software to latch the content of PLL2FRACN into the Î£Î modulator. In order to latch the PLL2FRACN value into the Î£Î modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see for details).
pub type PLL2FRACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL2CFGR_SPEC, bool, O>;
///Field `PLL2M` reader - Prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2M_R = crate::FieldReader<u8, u8>;
///Field `PLL2M` writer - Prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
pub type PLL2M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL2CFGR_SPEC, u8, u8, 4, O>;
///Field `PLL2PEN` reader - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when the pll2_p_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).
pub type PLL2PEN_R = crate::BitReader<bool>;
///Field `PLL2PEN` writer - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when the pll2_p_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).
pub type PLL2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL2CFGR_SPEC, bool, O>;
///Field `PLL2QEN` reader - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when the pll2_q_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0.
pub type PLL2QEN_R = crate::BitReader<bool>;
///Field `PLL2QEN` writer - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when the pll2_q_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0.
pub type PLL2QEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL2CFGR_SPEC, bool, O>;
///Field `PLL2REN` reader - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when the pll2_r_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).
pub type PLL2REN_R = crate::BitReader<bool>;
///Field `PLL2REN` writer - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when the pll2_r_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).
pub type PLL2REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL2CFGR_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - PLL2 entry clock source Set and cleared by software to select PLL2 clock source. These bits can be written only when the PLL2 is disabled. In order to save power, when no PLL2 is used, the value of PLL2SRC must be 0.
    #[inline(always)]
    pub fn pll2src(&self) -> PLL2SRC_R {
        PLL2SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. This bit must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll2rge(&self) -> PLL2RGE_R {
        PLL2RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL2 fractional latch enable Set and reset by software to latch the content of PLL2FRACN into the Î£Î modulator. In order to latch the PLL2FRACN value into the Î£Î modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see for details).
    #[inline(always)]
    pub fn pll2fracen(&self) -> PLL2FRACEN_R {
        PLL2FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - Prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    pub fn pll2m(&self) -> PLL2M_R {
        PLL2M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 16 - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when the pll2_p_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).
    #[inline(always)]
    pub fn pll2pen(&self) -> PLL2PEN_R {
        PLL2PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when the pll2_q_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0.
    #[inline(always)]
    pub fn pll2qen(&self) -> PLL2QEN_R {
        PLL2QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when the pll2_r_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).
    #[inline(always)]
    pub fn pll2ren(&self) -> PLL2REN_R {
        PLL2REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - PLL2 entry clock source Set and cleared by software to select PLL2 clock source. These bits can be written only when the PLL2 is disabled. In order to save power, when no PLL2 is used, the value of PLL2SRC must be 0.
    #[inline(always)]
    #[must_use]
    pub fn pll2src(&mut self) -> PLL2SRC_W<0> {
        PLL2SRC_W::new(self)
    }
    ///Bits 2:3 - PLL2 input frequency range Set and reset by software to select the proper reference frequency range used for PLL2. This bit must be written before enabling the PLL2. 00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    #[must_use]
    pub fn pll2rge(&mut self) -> PLL2RGE_W<2> {
        PLL2RGE_W::new(self)
    }
    ///Bit 4 - PLL2 fractional latch enable Set and reset by software to latch the content of PLL2FRACN into the Î£Î modulator. In order to latch the PLL2FRACN value into the Î£Î modulator, PLL2FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the modulator (see for details).
    #[inline(always)]
    #[must_use]
    pub fn pll2fracen(&mut self) -> PLL2FRACEN_W<4> {
        PLL2FRACEN_W::new(self)
    }
    ///Bits 8:11 - Prescaler for PLL2 Set and cleared by software to configure the prescaler of the PLL2. The VCO2 input frequency is PLL2 input clock frequency/PLL2M. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll2m(&mut self) -> PLL2M_W<8> {
        PLL2M_W::new(self)
    }
    ///Bit 16 - PLL2 DIVP divider output enable Set and reset by software to enable the pll2_p_ck output of the PLL2. To save power, PLL2PEN and PLL2P bits must be set to 0 when the pll2_p_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).
    #[inline(always)]
    #[must_use]
    pub fn pll2pen(&mut self) -> PLL2PEN_W<16> {
        PLL2PEN_W::new(self)
    }
    ///Bit 17 - PLL2 DIVQ divider output enable Set and reset by software to enable the pll2_q_ck output of the PLL2. To save power, PLL2QEN and PLL2Q bits must be set to 0 when the pll2_q_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0.
    #[inline(always)]
    #[must_use]
    pub fn pll2qen(&mut self) -> PLL2QEN_W<17> {
        PLL2QEN_W::new(self)
    }
    ///Bit 18 - PLL2 DIVR divider output enable Set and reset by software to enable the pll2_r_ck output of the PLL2. To save power, PLL2REN and PLL2R bits must be set to 0 when the pll2_r_ck is not used. This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).
    #[inline(always)]
    #[must_use]
    pub fn pll2ren(&mut self) -> PLL2REN_W<18> {
        PLL2REN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC PLL2 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_pll2cfgr](index.html) module
pub struct RCC_PLL2CFGR_SPEC;
impl crate::RegisterSpec for RCC_PLL2CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_pll2cfgr::R](R) reader structure
impl crate::Readable for RCC_PLL2CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_pll2cfgr::W](W) writer structure
impl crate::Writable for RCC_PLL2CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_PLL2CFGR to value 0
impl crate::Resettable for RCC_PLL2CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
