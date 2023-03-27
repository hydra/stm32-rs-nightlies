///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PE` reader - Parity error
pub type PE_R = crate::BitReader<bool>;
///Field `FE` reader - Framing error
pub type FE_R = crate::BitReader<bool>;
///Field `NF` reader - Noise detected flag
pub type NF_R = crate::BitReader<bool>;
///Field `ORE` reader - Overrun error
pub type ORE_R = crate::BitReader<bool>;
///Field `IDLE` reader - IDLE line detected
pub type IDLE_R = crate::BitReader<bool>;
///Field `RXNE` reader - Read data register not empty
pub type RXNE_R = crate::BitReader<bool>;
///Field `RXNE` writer - Read data register not empty
pub type RXNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `TC` reader - Transmission complete
pub type TC_R = crate::BitReader<bool>;
///Field `TC` writer - Transmission complete
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `TXE` reader - Transmit data register empty
pub type TXE_R = crate::BitReader<bool>;
///Field `LBD` reader - LIN break detection flag
pub type LBD_R = crate::BitReader<bool>;
///Field `LBD` writer - LIN break detection flag
pub type LBD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `CTS` reader - CTS flag
pub type CTS_R = crate::BitReader<bool>;
///Field `CTS` writer - CTS flag
pub type CTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Parity error
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Framing error
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Noise detected flag
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Overrun error
    #[inline(always)]
    pub fn ore(&self) -> ORE_R {
        ORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE line detected
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Read data register not empty
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit data register empty
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - LIN break detection flag
    #[inline(always)]
    pub fn lbd(&self) -> LBD_R {
        LBD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS flag
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 5 - Read data register not empty
    #[inline(always)]
    #[must_use]
    pub fn rxne(&mut self) -> RXNE_W<5> {
        RXNE_W::new(self)
    }
    ///Bit 6 - Transmission complete
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<6> {
        TC_W::new(self)
    }
    ///Bit 8 - LIN break detection flag
    #[inline(always)]
    #[must_use]
    pub fn lbd(&mut self) -> LBD_W<8> {
        LBD_W::new(self)
    }
    ///Bit 9 - CTS flag
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CTS_W<9> {
        CTS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0xc0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
