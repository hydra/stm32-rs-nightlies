///Register `RCC_PLL3CFGR` reader
pub struct R(crate::R<RCC_PLL3CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL3CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL3CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL3CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_PLL3CFGR` writer
pub struct W(crate::W<RCC_PLL3CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL3CFGR_SPEC>;
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
impl From<crate::W<RCC_PLL3CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL3CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLL3SRC` reader - PLL3 entry clock source Set and cleared by software to select PLL3 clock source. These bits can be written only when the PLL3 is disabled. In order to save power, when no PLL3 is used, the value of PLL3SRC must be 00.
pub type PLL3SRC_R = crate::FieldReader<u8, u8>;
///Field `PLL3SRC` writer - PLL3 entry clock source Set and cleared by software to select PLL3 clock source. These bits can be written only when the PLL3 is disabled. In order to save power, when no PLL3 is used, the value of PLL3SRC must be 00.
pub type PLL3SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL3CFGR_SPEC, u8, u8, 2, O>;
///Field `PLL3RGE` reader - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
pub type PLL3RGE_R = crate::FieldReader<u8, u8>;
///Field `PLL3RGE` writer - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
pub type PLL3RGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL3CFGR_SPEC, u8, u8, 2, O>;
///Field `PLL3FRACEN` reader - PLL3 fractional latch enable Set and reset by software to latch the content of PLL3FRACN into the Î£Î modulator. In order to latch the PLL3FRACN value into the Î£Î modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see for details).
pub type PLL3FRACEN_R = crate::BitReader<bool>;
///Field `PLL3FRACEN` writer - PLL3 fractional latch enable Set and reset by software to latch the content of PLL3FRACN into the Î£Î modulator. In order to latch the PLL3FRACN value into the Î£Î modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see for details).
pub type PLL3FRACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL3CFGR_SPEC, bool, O>;
///Field `PLL3M` reader - Prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3M_R = crate::FieldReader<u8, u8>;
///Field `PLL3M` writer - Prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
pub type PLL3M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_PLL3CFGR_SPEC, u8, u8, 4, O>;
///Field `PLL3PEN` reader - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when the pll3_p_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
pub type PLL3PEN_R = crate::BitReader<bool>;
///Field `PLL3PEN` writer - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when the pll3_p_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
pub type PLL3PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL3CFGR_SPEC, bool, O>;
///Field `PLL3QEN` reader - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when the pll3_q_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
pub type PLL3QEN_R = crate::BitReader<bool>;
///Field `PLL3QEN` writer - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when the pll3_q_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
pub type PLL3QEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL3CFGR_SPEC, bool, O>;
///Field `PLL3REN` reader - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
pub type PLL3REN_R = crate::BitReader<bool>;
///Field `PLL3REN` writer - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
pub type PLL3REN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL3CFGR_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - PLL3 entry clock source Set and cleared by software to select PLL3 clock source. These bits can be written only when the PLL3 is disabled. In order to save power, when no PLL3 is used, the value of PLL3SRC must be 00.
    #[inline(always)]
    pub fn pll3src(&self) -> PLL3SRC_R {
        PLL3SRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    pub fn pll3rge(&self) -> PLL3RGE_R {
        PLL3RGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - PLL3 fractional latch enable Set and reset by software to latch the content of PLL3FRACN into the Î£Î modulator. In order to latch the PLL3FRACN value into the Î£Î modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see for details).
    #[inline(always)]
    pub fn pll3fracen(&self) -> PLL3FRACEN_R {
        PLL3FRACEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - Prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    pub fn pll3m(&self) -> PLL3M_R {
        PLL3M_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 16 - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when the pll3_p_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
    #[inline(always)]
    pub fn pll3pen(&self) -> PLL3PEN_R {
        PLL3PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when the pll3_q_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
    #[inline(always)]
    pub fn pll3qen(&self) -> PLL3QEN_R {
        PLL3QEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
    #[inline(always)]
    pub fn pll3ren(&self) -> PLL3REN_R {
        PLL3REN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - PLL3 entry clock source Set and cleared by software to select PLL3 clock source. These bits can be written only when the PLL3 is disabled. In order to save power, when no PLL3 is used, the value of PLL3SRC must be 00.
    #[inline(always)]
    #[must_use]
    pub fn pll3src(&mut self) -> PLL3SRC_W<0> {
        PLL3SRC_W::new(self)
    }
    ///Bits 2:3 - PLL3 input frequency range Set and reset by software to select the proper reference frequency range used for PLL3. This bit must be written before enabling the PLL3. 00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
    #[inline(always)]
    #[must_use]
    pub fn pll3rge(&mut self) -> PLL3RGE_W<2> {
        PLL3RGE_W::new(self)
    }
    ///Bit 4 - PLL3 fractional latch enable Set and reset by software to latch the content of PLL3FRACN into the Î£Î modulator. In order to latch the PLL3FRACN value into the Î£Î modulator, PLL3FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the modulator (see for details).
    #[inline(always)]
    #[must_use]
    pub fn pll3fracen(&mut self) -> PLL3FRACEN_W<4> {
        PLL3FRACEN_W::new(self)
    }
    ///Bits 8:11 - Prescaler for PLL3 Set and cleared by software to configure the prescaler of the PLL3. The VCO3 input frequency is PLL3 input clock frequency/PLL3M. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0). ...
    #[inline(always)]
    #[must_use]
    pub fn pll3m(&mut self) -> PLL3M_W<8> {
        PLL3M_W::new(self)
    }
    ///Bit 16 - PLL3 DIVP divider output enable Set and reset by software to enable the pll3_p_ck output of the PLL3. To save power, PLL3PEN and PLL3P bits must be set to 0 when the pll3_p_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
    #[inline(always)]
    #[must_use]
    pub fn pll3pen(&mut self) -> PLL3PEN_W<16> {
        PLL3PEN_W::new(self)
    }
    ///Bit 17 - PLL3 DIVQ divider output enable Set and reset by software to enable the pll3_q_ck output of the PLL3. To save power, PLL3QEN and PLL3Q bits must be set to 0 when the pll3_q_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
    #[inline(always)]
    #[must_use]
    pub fn pll3qen(&mut self) -> PLL3QEN_W<17> {
        PLL3QEN_W::new(self)
    }
    ///Bit 18 - PLL3 DIVR divider output enable Set and reset by software to enable the pll3_r_ck output of the PLL3. To save power, PLL3REN and PLL3R bits must be set to 0 when the pll3_r_ck is not used. This bit can be written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
    #[inline(always)]
    #[must_use]
    pub fn pll3ren(&mut self) -> PLL3REN_W<18> {
        PLL3REN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC PLL3 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_pll3cfgr](index.html) module
pub struct RCC_PLL3CFGR_SPEC;
impl crate::RegisterSpec for RCC_PLL3CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_pll3cfgr::R](R) reader structure
impl crate::Readable for RCC_PLL3CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_pll3cfgr::W](W) writer structure
impl crate::Writable for RCC_PLL3CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_PLL3CFGR to value 0
impl crate::Resettable for RCC_PLL3CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
