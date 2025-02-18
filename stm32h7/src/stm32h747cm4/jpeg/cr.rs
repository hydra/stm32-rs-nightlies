///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `JCEN` reader - JPEG Core Enable Enable the JPEG codec Core.
pub type JCEN_R = crate::BitReader<bool>;
///Field `JCEN` writer - JPEG Core Enable Enable the JPEG codec Core.
pub type JCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `IFTIE` reader - Input FIFO Threshold Interrupt Enable This bit enables the interrupt generation when input FIFO reach the threshold.
pub type IFTIE_R = crate::BitReader<bool>;
///Field `IFTIE` writer - Input FIFO Threshold Interrupt Enable This bit enables the interrupt generation when input FIFO reach the threshold.
pub type IFTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `IFNFIE` reader - Input FIFO Not Full Interrupt Enable This bit enables the interrupt generation when input FIFO is not empty.
pub type IFNFIE_R = crate::BitReader<bool>;
///Field `IFNFIE` writer - Input FIFO Not Full Interrupt Enable This bit enables the interrupt generation when input FIFO is not empty.
pub type IFNFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `OFTIE` reader - Output FIFO Threshold Interrupt Enable This bit enables the interrupt generation when output FIFO reach the threshold.
pub type OFTIE_R = crate::BitReader<bool>;
///Field `OFTIE` writer - Output FIFO Threshold Interrupt Enable This bit enables the interrupt generation when output FIFO reach the threshold.
pub type OFTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `OFNEIE` reader - Output FIFO Not Empty Interrupt Enable This bit enables the interrupt generation when output FIFO is not empty.
pub type OFNEIE_R = crate::BitReader<bool>;
///Field `OFNEIE` writer - Output FIFO Not Empty Interrupt Enable This bit enables the interrupt generation when output FIFO is not empty.
pub type OFNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `EOCIE` reader - End of Conversion Interrupt Enable This bit enables the interrupt generation on the end of conversion.
pub type EOCIE_R = crate::BitReader<bool>;
///Field `EOCIE` writer - End of Conversion Interrupt Enable This bit enables the interrupt generation on the end of conversion.
pub type EOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `HPDIE` reader - Header Parsing Done Interrupt Enable This bit enables the interrupt generation on the Header Parsing Operation.
pub type HPDIE_R = crate::BitReader<bool>;
///Field `HPDIE` writer - Header Parsing Done Interrupt Enable This bit enables the interrupt generation on the Header Parsing Operation.
pub type HPDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `IDMAEN` reader - Input DMA Enable Enable the DMA request generation for the input FIFO.
pub type IDMAEN_R = crate::BitReader<bool>;
///Field `IDMAEN` writer - Input DMA Enable Enable the DMA request generation for the input FIFO.
pub type IDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ODMAEN` reader - Output DMA Enable Enable the DMA request generation for the output FIFO.
pub type ODMAEN_R = crate::BitReader<bool>;
///Field `ODMAEN` writer - Output DMA Enable Enable the DMA request generation for the output FIFO.
pub type ODMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `IFF` reader - Input FIFO Flush This bit flush the input FIFO. This bit is always read as 0.
pub type IFF_R = crate::BitReader<bool>;
///Field `IFF` writer - Input FIFO Flush This bit flush the input FIFO. This bit is always read as 0.
pub type IFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `OFF` reader - Output FIFO Flush This bit flush the output FIFO. This bit is always read as 0.
pub type OFF_R = crate::BitReader<bool>;
///Field `OFF` writer - Output FIFO Flush This bit flush the output FIFO. This bit is always read as 0.
pub type OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - JPEG Core Enable Enable the JPEG codec Core.
    #[inline(always)]
    pub fn jcen(&self) -> JCEN_R {
        JCEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Input FIFO Threshold Interrupt Enable This bit enables the interrupt generation when input FIFO reach the threshold.
    #[inline(always)]
    pub fn iftie(&self) -> IFTIE_R {
        IFTIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Input FIFO Not Full Interrupt Enable This bit enables the interrupt generation when input FIFO is not empty.
    #[inline(always)]
    pub fn ifnfie(&self) -> IFNFIE_R {
        IFNFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output FIFO Threshold Interrupt Enable This bit enables the interrupt generation when output FIFO reach the threshold.
    #[inline(always)]
    pub fn oftie(&self) -> OFTIE_R {
        OFTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Output FIFO Not Empty Interrupt Enable This bit enables the interrupt generation when output FIFO is not empty.
    #[inline(always)]
    pub fn ofneie(&self) -> OFNEIE_R {
        OFNEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - End of Conversion Interrupt Enable This bit enables the interrupt generation on the end of conversion.
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Header Parsing Done Interrupt Enable This bit enables the interrupt generation on the Header Parsing Operation.
    #[inline(always)]
    pub fn hpdie(&self) -> HPDIE_R {
        HPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 11 - Input DMA Enable Enable the DMA request generation for the input FIFO.
    #[inline(always)]
    pub fn idmaen(&self) -> IDMAEN_R {
        IDMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Output DMA Enable Enable the DMA request generation for the output FIFO.
    #[inline(always)]
    pub fn odmaen(&self) -> ODMAEN_R {
        ODMAEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Input FIFO Flush This bit flush the input FIFO. This bit is always read as 0.
    #[inline(always)]
    pub fn iff(&self) -> IFF_R {
        IFF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output FIFO Flush This bit flush the output FIFO. This bit is always read as 0.
    #[inline(always)]
    pub fn off(&self) -> OFF_R {
        OFF_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - JPEG Core Enable Enable the JPEG codec Core.
    #[inline(always)]
    #[must_use]
    pub fn jcen(&mut self) -> JCEN_W<0> {
        JCEN_W::new(self)
    }
    ///Bit 1 - Input FIFO Threshold Interrupt Enable This bit enables the interrupt generation when input FIFO reach the threshold.
    #[inline(always)]
    #[must_use]
    pub fn iftie(&mut self) -> IFTIE_W<1> {
        IFTIE_W::new(self)
    }
    ///Bit 2 - Input FIFO Not Full Interrupt Enable This bit enables the interrupt generation when input FIFO is not empty.
    #[inline(always)]
    #[must_use]
    pub fn ifnfie(&mut self) -> IFNFIE_W<2> {
        IFNFIE_W::new(self)
    }
    ///Bit 3 - Output FIFO Threshold Interrupt Enable This bit enables the interrupt generation when output FIFO reach the threshold.
    #[inline(always)]
    #[must_use]
    pub fn oftie(&mut self) -> OFTIE_W<3> {
        OFTIE_W::new(self)
    }
    ///Bit 4 - Output FIFO Not Empty Interrupt Enable This bit enables the interrupt generation when output FIFO is not empty.
    #[inline(always)]
    #[must_use]
    pub fn ofneie(&mut self) -> OFNEIE_W<4> {
        OFNEIE_W::new(self)
    }
    ///Bit 5 - End of Conversion Interrupt Enable This bit enables the interrupt generation on the end of conversion.
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<5> {
        EOCIE_W::new(self)
    }
    ///Bit 6 - Header Parsing Done Interrupt Enable This bit enables the interrupt generation on the Header Parsing Operation.
    #[inline(always)]
    #[must_use]
    pub fn hpdie(&mut self) -> HPDIE_W<6> {
        HPDIE_W::new(self)
    }
    ///Bit 11 - Input DMA Enable Enable the DMA request generation for the input FIFO.
    #[inline(always)]
    #[must_use]
    pub fn idmaen(&mut self) -> IDMAEN_W<11> {
        IDMAEN_W::new(self)
    }
    ///Bit 12 - Output DMA Enable Enable the DMA request generation for the output FIFO.
    #[inline(always)]
    #[must_use]
    pub fn odmaen(&mut self) -> ODMAEN_W<12> {
        ODMAEN_W::new(self)
    }
    ///Bit 13 - Input FIFO Flush This bit flush the input FIFO. This bit is always read as 0.
    #[inline(always)]
    #[must_use]
    pub fn iff(&mut self) -> IFF_W<13> {
        IFF_W::new(self)
    }
    ///Bit 14 - Output FIFO Flush This bit flush the output FIFO. This bit is always read as 0.
    #[inline(always)]
    #[must_use]
    pub fn off(&mut self) -> OFF_W<14> {
        OFF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///JPEG control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
