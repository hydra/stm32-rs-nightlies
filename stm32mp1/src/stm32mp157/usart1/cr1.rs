///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UE` reader - USART enable
pub type UE_R = crate::BitReader<bool>;
///Field `UE` writer - USART enable
pub type UE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `UESM` reader - USART enable in Stop mode
pub type UESM_R = crate::BitReader<bool>;
///Field `UESM` writer - USART enable in Stop mode
pub type UESM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `RE` reader - Receiver enable
pub type RE_R = crate::BitReader<bool>;
///Field `RE` writer - Receiver enable
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `TE` reader - Transmitter enable
pub type TE_R = crate::BitReader<bool>;
///Field `TE` writer - Transmitter enable
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `IDLEIE` reader - IDLE interrupt enable
pub type IDLEIE_R = crate::BitReader<bool>;
///Field `IDLEIE` writer - IDLE interrupt enable
pub type IDLEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `RXNEIE` reader - RXNE interrupt enable
pub type RXNEIE_R = crate::BitReader<bool>;
///Field `RXNEIE` writer - RXNE interrupt enable
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `TCIE` reader - Transmission complete interrupt enable
pub type TCIE_R = crate::BitReader<bool>;
///Field `TCIE` writer - Transmission complete interrupt enable
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `TXEIE` reader - interrupt enable
pub type TXEIE_R = crate::BitReader<bool>;
///Field `TXEIE` writer - interrupt enable
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `PEIE` reader - PE interrupt enable
pub type PEIE_R = crate::BitReader<bool>;
///Field `PEIE` writer - PE interrupt enable
pub type PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `PS` reader - Parity selection
pub type PS_R = crate::BitReader<bool>;
///Field `PS` writer - Parity selection
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `PCE` reader - Parity control enable
pub type PCE_R = crate::BitReader<bool>;
///Field `PCE` writer - Parity control enable
pub type PCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `WAKE` reader - Receiver wakeup method
pub type WAKE_R = crate::BitReader<bool>;
///Field `WAKE` writer - Receiver wakeup method
pub type WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `M0` reader - Word length
pub type M0_R = crate::BitReader<bool>;
///Field `M0` writer - Word length
pub type M0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `MME` reader - Mute mode enable
pub type MME_R = crate::BitReader<bool>;
///Field `MME` writer - Mute mode enable
pub type MME_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `CMIE` reader - Character match interrupt enable
pub type CMIE_R = crate::BitReader<bool>;
///Field `CMIE` writer - Character match interrupt enable
pub type CMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `OVER8` reader - Oversampling mode
pub type OVER8_R = crate::BitReader<bool>;
///Field `OVER8` writer - Oversampling mode
pub type OVER8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `DEDT` reader - DEDT
pub type DEDT_R = crate::FieldReader<u8, u8>;
///Field `DEDT` writer - DEDT
pub type DEDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 5, O>;
///Field `DEAT` reader - DEAT
pub type DEAT_R = crate::FieldReader<u8, u8>;
///Field `DEAT` writer - DEAT
pub type DEAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 5, O>;
///Field `RTOIE` reader - Receiver timeout interrupt enable
pub type RTOIE_R = crate::BitReader<bool>;
///Field `RTOIE` writer - Receiver timeout interrupt enable
pub type RTOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `EOBIE` reader - End of Block interrupt enable
pub type EOBIE_R = crate::BitReader<bool>;
///Field `EOBIE` writer - End of Block interrupt enable
pub type EOBIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `M1` reader - Word length
pub type M1_R = crate::BitReader<bool>;
///Field `M1` writer - Word length
pub type M1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `FIFOEN` reader - FIFO mode enable
pub type FIFOEN_R = crate::BitReader<bool>;
///Field `FIFOEN` writer - FIFO mode enable
pub type FIFOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `TXFEIE` reader - TXFIFO empty interrupt enable
pub type TXFEIE_R = crate::BitReader<bool>;
///Field `TXFEIE` writer - TXFIFO empty interrupt enable
pub type TXFEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `RXFFIE` reader - RXFIFO Full interrupt enable
pub type RXFFIE_R = crate::BitReader<bool>;
///Field `RXFFIE` writer - RXFIFO Full interrupt enable
pub type RXFFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - USART enable
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:20 - DEDT
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:25 - DEAT
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bit 26 - Receiver timeout interrupt enable
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - End of Block interrupt enable
    #[inline(always)]
    pub fn eobie(&self) -> EOBIE_R {
        EOBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Word length
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - FIFO mode enable
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TXFIFO empty interrupt enable
    #[inline(always)]
    pub fn txfeie(&self) -> TXFEIE_R {
        TXFEIE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RXFIFO Full interrupt enable
    #[inline(always)]
    pub fn rxffie(&self) -> RXFFIE_R {
        RXFFIE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - USART enable
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<0> {
        UE_W::new(self)
    }
    ///Bit 1 - USART enable in Stop mode
    #[inline(always)]
    #[must_use]
    pub fn uesm(&mut self) -> UESM_W<1> {
        UESM_W::new(self)
    }
    ///Bit 2 - Receiver enable
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    ///Bit 3 - Transmitter enable
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    ///Bit 4 - IDLE interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IDLEIE_W<4> {
        IDLEIE_W::new(self)
    }
    ///Bit 5 - RXNE interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<5> {
        RXNEIE_W::new(self)
    }
    ///Bit 6 - Transmission complete interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<6> {
        TCIE_W::new(self)
    }
    ///Bit 7 - interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<7> {
        TXEIE_W::new(self)
    }
    ///Bit 8 - PE interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<8> {
        PEIE_W::new(self)
    }
    ///Bit 9 - Parity selection
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<9> {
        PS_W::new(self)
    }
    ///Bit 10 - Parity control enable
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PCE_W<10> {
        PCE_W::new(self)
    }
    ///Bit 11 - Receiver wakeup method
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<11> {
        WAKE_W::new(self)
    }
    ///Bit 12 - Word length
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0_W<12> {
        M0_W::new(self)
    }
    ///Bit 13 - Mute mode enable
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MME_W<13> {
        MME_W::new(self)
    }
    ///Bit 14 - Character match interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn cmie(&mut self) -> CMIE_W<14> {
        CMIE_W::new(self)
    }
    ///Bit 15 - Oversampling mode
    #[inline(always)]
    #[must_use]
    pub fn over8(&mut self) -> OVER8_W<15> {
        OVER8_W::new(self)
    }
    ///Bits 16:20 - DEDT
    #[inline(always)]
    #[must_use]
    pub fn dedt(&mut self) -> DEDT_W<16> {
        DEDT_W::new(self)
    }
    ///Bits 21:25 - DEAT
    #[inline(always)]
    #[must_use]
    pub fn deat(&mut self) -> DEAT_W<21> {
        DEAT_W::new(self)
    }
    ///Bit 26 - Receiver timeout interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rtoie(&mut self) -> RTOIE_W<26> {
        RTOIE_W::new(self)
    }
    ///Bit 27 - End of Block interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eobie(&mut self) -> EOBIE_W<27> {
        EOBIE_W::new(self)
    }
    ///Bit 28 - Word length
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1_W<28> {
        M1_W::new(self)
    }
    ///Bit 29 - FIFO mode enable
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FIFOEN_W<29> {
        FIFOEN_W::new(self)
    }
    ///Bit 30 - TXFIFO empty interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txfeie(&mut self) -> TXFEIE_W<30> {
        TXFEIE_W::new(self)
    }
    ///Bit 31 - RXFIFO Full interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rxffie(&mut self) -> RXFFIE_W<31> {
        RXFFIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
