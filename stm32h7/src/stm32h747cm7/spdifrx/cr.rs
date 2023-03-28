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
///Field `SPDIFRXEN` reader - Peripheral Block Enable
pub type SPDIFRXEN_R = crate::FieldReader<u8, u8>;
///Field `SPDIFRXEN` writer - Peripheral Block Enable
pub type SPDIFRXEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `RXDMAEN` reader - Receiver DMA ENable for data flow
pub type RXDMAEN_R = crate::BitReader<bool>;
///Field `RXDMAEN` writer - Receiver DMA ENable for data flow
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `RXSTEO` reader - STerEO Mode
pub type RXSTEO_R = crate::BitReader<bool>;
///Field `RXSTEO` writer - STerEO Mode
pub type RXSTEO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `DRFMT` reader - RX Data format
pub type DRFMT_R = crate::FieldReader<u8, u8>;
///Field `DRFMT` writer - RX Data format
pub type DRFMT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `PMSK` reader - Mask Parity error bit
pub type PMSK_R = crate::BitReader<bool>;
///Field `PMSK` writer - Mask Parity error bit
pub type PMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `VMSK` reader - Mask of Validity bit
pub type VMSK_R = crate::BitReader<bool>;
///Field `VMSK` writer - Mask of Validity bit
pub type VMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CUMSK` reader - Mask of channel status and user bits
pub type CUMSK_R = crate::BitReader<bool>;
///Field `CUMSK` writer - Mask of channel status and user bits
pub type CUMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `PTMSK` reader - Mask of Preamble Type bits
pub type PTMSK_R = crate::BitReader<bool>;
///Field `PTMSK` writer - Mask of Preamble Type bits
pub type PTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CBDMAEN` reader - Control Buffer DMA ENable for control flow
pub type CBDMAEN_R = crate::BitReader<bool>;
///Field `CBDMAEN` writer - Control Buffer DMA ENable for control flow
pub type CBDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CHSEL` reader - Channel Selection
pub type CHSEL_R = crate::BitReader<bool>;
///Field `CHSEL` writer - Channel Selection
pub type CHSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `NBTR` reader - Maximum allowed re-tries during synchronization phase
pub type NBTR_R = crate::FieldReader<u8, u8>;
///Field `NBTR` writer - Maximum allowed re-tries during synchronization phase
pub type NBTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `WFA` reader - Wait For Activity
pub type WFA_R = crate::BitReader<bool>;
///Field `WFA` writer - Wait For Activity
pub type WFA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `INSEL` reader - input selection
pub type INSEL_R = crate::FieldReader<u8, u8>;
///Field `INSEL` writer - input selection
pub type INSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
///Field `CKSEN` reader - Symbol Clock Enable
pub type CKSEN_R = crate::BitReader<bool>;
///Field `CKSEN` writer - Symbol Clock Enable
pub type CKSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CKSBKPEN` reader - Backup Symbol Clock Enable
pub type CKSBKPEN_R = crate::BitReader<bool>;
///Field `CKSBKPEN` writer - Backup Symbol Clock Enable
pub type CKSBKPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - Peripheral Block Enable
    #[inline(always)]
    pub fn spdifrxen(&self) -> SPDIFRXEN_R {
        SPDIFRXEN_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Receiver DMA ENable for data flow
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - STerEO Mode
    #[inline(always)]
    pub fn rxsteo(&self) -> RXSTEO_R {
        RXSTEO_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - RX Data format
    #[inline(always)]
    pub fn drfmt(&self) -> DRFMT_R {
        DRFMT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Mask Parity error bit
    #[inline(always)]
    pub fn pmsk(&self) -> PMSK_R {
        PMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Mask of Validity bit
    #[inline(always)]
    pub fn vmsk(&self) -> VMSK_R {
        VMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Mask of channel status and user bits
    #[inline(always)]
    pub fn cumsk(&self) -> CUMSK_R {
        CUMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Mask of Preamble Type bits
    #[inline(always)]
    pub fn ptmsk(&self) -> PTMSK_R {
        PTMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Control Buffer DMA ENable for control flow
    #[inline(always)]
    pub fn cbdmaen(&self) -> CBDMAEN_R {
        CBDMAEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Channel Selection
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Maximum allowed re-tries during synchronization phase
    #[inline(always)]
    pub fn nbtr(&self) -> NBTR_R {
        NBTR_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - Wait For Activity
    #[inline(always)]
    pub fn wfa(&self) -> WFA_R {
        WFA_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:18 - input selection
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 20 - Symbol Clock Enable
    #[inline(always)]
    pub fn cksen(&self) -> CKSEN_R {
        CKSEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Backup Symbol Clock Enable
    #[inline(always)]
    pub fn cksbkpen(&self) -> CKSBKPEN_R {
        CKSBKPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Peripheral Block Enable
    #[inline(always)]
    #[must_use]
    pub fn spdifrxen(&mut self) -> SPDIFRXEN_W<0> {
        SPDIFRXEN_W::new(self)
    }
    ///Bit 2 - Receiver DMA ENable for data flow
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<2> {
        RXDMAEN_W::new(self)
    }
    ///Bit 3 - STerEO Mode
    #[inline(always)]
    #[must_use]
    pub fn rxsteo(&mut self) -> RXSTEO_W<3> {
        RXSTEO_W::new(self)
    }
    ///Bits 4:5 - RX Data format
    #[inline(always)]
    #[must_use]
    pub fn drfmt(&mut self) -> DRFMT_W<4> {
        DRFMT_W::new(self)
    }
    ///Bit 6 - Mask Parity error bit
    #[inline(always)]
    #[must_use]
    pub fn pmsk(&mut self) -> PMSK_W<6> {
        PMSK_W::new(self)
    }
    ///Bit 7 - Mask of Validity bit
    #[inline(always)]
    #[must_use]
    pub fn vmsk(&mut self) -> VMSK_W<7> {
        VMSK_W::new(self)
    }
    ///Bit 8 - Mask of channel status and user bits
    #[inline(always)]
    #[must_use]
    pub fn cumsk(&mut self) -> CUMSK_W<8> {
        CUMSK_W::new(self)
    }
    ///Bit 9 - Mask of Preamble Type bits
    #[inline(always)]
    #[must_use]
    pub fn ptmsk(&mut self) -> PTMSK_W<9> {
        PTMSK_W::new(self)
    }
    ///Bit 10 - Control Buffer DMA ENable for control flow
    #[inline(always)]
    #[must_use]
    pub fn cbdmaen(&mut self) -> CBDMAEN_W<10> {
        CBDMAEN_W::new(self)
    }
    ///Bit 11 - Channel Selection
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<11> {
        CHSEL_W::new(self)
    }
    ///Bits 12:13 - Maximum allowed re-tries during synchronization phase
    #[inline(always)]
    #[must_use]
    pub fn nbtr(&mut self) -> NBTR_W<12> {
        NBTR_W::new(self)
    }
    ///Bit 14 - Wait For Activity
    #[inline(always)]
    #[must_use]
    pub fn wfa(&mut self) -> WFA_W<14> {
        WFA_W::new(self)
    }
    ///Bits 16:18 - input selection
    #[inline(always)]
    #[must_use]
    pub fn insel(&mut self) -> INSEL_W<16> {
        INSEL_W::new(self)
    }
    ///Bit 20 - Symbol Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn cksen(&mut self) -> CKSEN_W<20> {
        CKSEN_W::new(self)
    }
    ///Bit 21 - Backup Symbol Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn cksbkpen(&mut self) -> CKSBKPEN_W<21> {
        CKSBKPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register
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
