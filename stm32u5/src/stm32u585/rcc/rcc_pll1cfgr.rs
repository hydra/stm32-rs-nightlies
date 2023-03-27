///Register `RCC_PLL1CFGR` reader
pub struct R(crate::R<RCC_PLL1CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL1CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL1CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL1CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_PLL1CFGR` writer
pub struct W(crate::W<RCC_PLL1CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL1CFGR_SPEC>;
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
impl From<crate::W<RCC_PLL1CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL1CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLL1SRC` reader - PLL1 entry clock source Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, the value of PLL1SRC must be 0.
pub type PLL1SRC_R = crate::FieldReader<u8, u8>;
///Field `PLL1SRC` writer - PLL1 entry clock source Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, the value of PLL1SRC must be 0.
pub type PLL1SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL1CFGR_SPEC, u8, u8, 2, O>;
///Field `PLL1RGE` reader - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
pub type PLL1RGE_R = crate::FieldReader<u8, u8>;
///Field `PLL1RGE` writer - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
pub type PLL1RGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL1CFGR_SPEC, u8, u8, 2, O>;
///Field `PLL1FRACEN` reader - PLL1 fractional latch enable Set and reset by software to latch the content of PLL1FRACN into the Î£Î modulator. In order to latch the PLL1FRACN value into the Î£Î modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see for details).
pub type PLL1FRACEN_R = crate::BitReader<bool>;
///Field `PLL1FRACEN` writer - PLL1 fractional latch enable Set and reset by software to latch the content of PLL1FRACN into the Î£Î modulator. In order to latch the PLL1FRACN value into the Î£Î modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see for details).
pub type PLL1FRACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL1CFGR_SPEC, bool, O>;
///Field `PLL1M` reader - Prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type PLL1M_R = crate::FieldReader<u8, u8>;
///Field `PLL1M` writer - Prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
pub type PLL1M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL1CFGR_SPEC, u8, u8, 4, O>;
///Field `PLL1MBOOST` reader - Prescaler for EPOD booster input clock Set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1 input clock frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPOD Boost mode is disabled (see ). others: reserved
pub type PLL1MBOOST_R = crate::FieldReader<u8, u8>;
///Field `PLL1MBOOST` writer - Prescaler for EPOD booster input clock Set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1 input clock frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPOD Boost mode is disabled (see ). others: reserved
pub type PLL1MBOOST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_PLL1CFGR_SPEC, u8, u8, 4, O>;
///Field `PLL1PEN` reader - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1_p_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
pub type PLL1PEN_R = crate::BitReader<bool>;
///Field `PLL1PEN` writer - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1_p_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
pub type PLL1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL1CFGR_SPEC, bool, O>;
///Field `PLL1QEN` reader - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1_q_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
pub type PLL1QEN_R = crate::BitReader<bool>;
///Field `PLL1QEN` writer - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1_q_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
pub type PLL1QEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL1CFGR_SPEC, bool, O>;
///Field `PLL1REN` reader - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
pub type PLL1REN_R = crate::BitReader<bool>;
///Field `PLL1REN` writer - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
pub type PLL1REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL1CFGR_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - PLL1 entry clock source Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, the value of PLL1SRC must be 0.
    #[inline(always)]
    pub fn pll1src(&self) -> PLL1SRC_R {
        PLL1SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll1rge(&self) -> PLL1RGE_R {
        PLL1RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL1 fractional latch enable Set and reset by software to latch the content of PLL1FRACN into the Î£Î modulator. In order to latch the PLL1FRACN value into the Î£Î modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see for details).
    #[inline(always)]
    pub fn pll1fracen(&self) -> PLL1FRACEN_R {
        PLL1FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - Prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    pub fn pll1m(&self) -> PLL1M_R {
        PLL1M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Prescaler for EPOD booster input clock Set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1 input clock frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPOD Boost mode is disabled (see ). others: reserved
    #[inline(always)]
    pub fn pll1mboost(&self) -> PLL1MBOOST_R {
        PLL1MBOOST_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1_p_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
    #[inline(always)]
    pub fn pll1pen(&self) -> PLL1PEN_R {
        PLL1PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1_q_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
    #[inline(always)]
    pub fn pll1qen(&self) -> PLL1QEN_R {
        PLL1QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
    #[inline(always)]
    pub fn pll1ren(&self) -> PLL1REN_R {
        PLL1REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - PLL1 entry clock source Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled. In order to save power, when no PLL1 is used, the value of PLL1SRC must be 0.
    #[inline(always)]
    #[must_use]
    pub fn pll1src(&mut self) -> PLL1SRC_W<0> {
        PLL1SRC_W::new(self)
    }
    ///Bits 2:3 - PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    #[must_use]
    pub fn pll1rge(&mut self) -> PLL1RGE_W<2> {
        PLL1RGE_W::new(self)
    }
    ///Bit 4 - PLL1 fractional latch enable Set and reset by software to latch the content of PLL1FRACN into the Î£Î modulator. In order to latch the PLL1FRACN value into the Î£Î modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see for details).
    #[inline(always)]
    #[must_use]
    pub fn pll1fracen(&mut self) -> PLL1FRACEN_W<4> {
        PLL1FRACEN_W::new(self)
    }
    ///Bits 8:11 - Prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll1m(&mut self) -> PLL1M_W<8> {
        PLL1M_W::new(self)
    }
    ///Bits 12:15 - Prescaler for EPOD booster input clock Set and cleared by software to configure the prescaler of the PLL1, used for the EPOD booster. The EPOD booster input frequency is PLL1 input clock frequency/PLL1MBOOST. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and EPOD Boost mode is disabled (see ). others: reserved
    #[inline(always)]
    #[must_use]
    pub fn pll1mboost(&mut self) -> PLL1MBOOST_W<12> {
        PLL1MBOOST_W::new(self)
    }
    ///Bit 16 - PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1_p_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
    #[inline(always)]
    #[must_use]
    pub fn pll1pen(&mut self) -> PLL1PEN_W<16> {
        PLL1PEN_W::new(self)
    }
    ///Bit 17 - PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1_q_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
    #[inline(always)]
    #[must_use]
    pub fn pll1qen(&mut self) -> PLL1QEN_W<17> {
        PLL1QEN_W::new(self)
    }
    ///Bit 18 - PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
    #[inline(always)]
    #[must_use]
    pub fn pll1ren(&mut self) -> PLL1REN_W<18> {
        PLL1REN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC PLL1 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_pll1cfgr](index.html) module
pub struct RCC_PLL1CFGR_SPEC;
impl crate::RegisterSpec for RCC_PLL1CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_pll1cfgr::R](R) reader structure
impl crate::Readable for RCC_PLL1CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_pll1cfgr::W](W) writer structure
impl crate::Writable for RCC_PLL1CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_PLL1CFGR to value 0
impl crate::Resettable for RCC_PLL1CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
