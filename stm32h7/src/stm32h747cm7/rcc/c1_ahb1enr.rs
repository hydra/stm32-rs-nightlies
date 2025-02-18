///Register `C1_AHB1ENR` reader
pub struct R(crate::R<C1_AHB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_AHB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_AHB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_AHB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C1_AHB1ENR` writer
pub struct W(crate::W<C1_AHB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_AHB1ENR_SPEC>;
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
impl From<crate::W<C1_AHB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_AHB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMA1EN` reader - DMA1 Clock Enable
pub type DMA1EN_R = crate::BitReader<DMA1EN_A>;
///DMA1 Clock Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN_A {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<DMA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMA1EN_A {
        match self.bits {
            false => DMA1EN_A::Disabled,
            true => DMA1EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN_A::Enabled
    }
}
///Field `DMA1EN` writer - DMA1 Clock Enable
pub type DMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_AHB1ENR_SPEC, DMA1EN_A, O>;
impl<'a, const O: u8> DMA1EN_W<'a, O> {
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::Enabled)
    }
}
///Field `DMA2EN` reader - DMA2 Clock Enable
pub use DMA1EN_R as DMA2EN_R;
///Field `ADC12EN` reader - ADC1/2 Peripheral Clocks Enable
pub use DMA1EN_R as ADC12EN_R;
///Field `ARTEN` reader - ART Clock Enable
pub use DMA1EN_R as ARTEN_R;
///Field `ETH1MACEN` reader - Ethernet MAC bus interface Clock Enable
pub use DMA1EN_R as ETH1MACEN_R;
///Field `ETH1TXEN` reader - Ethernet Transmission Clock Enable
pub use DMA1EN_R as ETH1TXEN_R;
///Field `ETH1RXEN` reader - Ethernet Reception Clock Enable
pub use DMA1EN_R as ETH1RXEN_R;
///Field `USB1OTGEN` reader - USB1OTG Peripheral Clocks Enable
pub use DMA1EN_R as USB1OTGEN_R;
///Field `USB1ULPIEN` reader - USB_PHY1 Clocks Enable
pub use DMA1EN_R as USB1ULPIEN_R;
///Field `USB2OTGEN` reader - USB2OTG Peripheral Clocks Enable
pub use DMA1EN_R as USB2OTGEN_R;
///Field `USB2ULPIEN` reader - USB_PHY2 Clocks Enable
pub use DMA1EN_R as USB2ULPIEN_R;
///Field `DMA2EN` writer - DMA2 Clock Enable
pub use DMA1EN_W as DMA2EN_W;
///Field `ADC12EN` writer - ADC1/2 Peripheral Clocks Enable
pub use DMA1EN_W as ADC12EN_W;
///Field `ARTEN` writer - ART Clock Enable
pub use DMA1EN_W as ARTEN_W;
///Field `ETH1MACEN` writer - Ethernet MAC bus interface Clock Enable
pub use DMA1EN_W as ETH1MACEN_W;
///Field `ETH1TXEN` writer - Ethernet Transmission Clock Enable
pub use DMA1EN_W as ETH1TXEN_W;
///Field `ETH1RXEN` writer - Ethernet Reception Clock Enable
pub use DMA1EN_W as ETH1RXEN_W;
///Field `USB1OTGEN` writer - USB1OTG Peripheral Clocks Enable
pub use DMA1EN_W as USB1OTGEN_W;
///Field `USB1ULPIEN` writer - USB_PHY1 Clocks Enable
pub use DMA1EN_W as USB1ULPIEN_W;
///Field `USB2OTGEN` writer - USB2OTG Peripheral Clocks Enable
pub use DMA1EN_W as USB2OTGEN_W;
///Field `USB2ULPIEN` writer - USB_PHY2 Clocks Enable
pub use DMA1EN_W as USB2ULPIEN_W;
impl R {
    ///Bit 0 - DMA1 Clock Enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 Clock Enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - ADC1/2 Peripheral Clocks Enable
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 14 - ART Clock Enable
    #[inline(always)]
    pub fn arten(&self) -> ARTEN_R {
        ARTEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Ethernet MAC bus interface Clock Enable
    #[inline(always)]
    pub fn eth1macen(&self) -> ETH1MACEN_R {
        ETH1MACEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Ethernet Transmission Clock Enable
    #[inline(always)]
    pub fn eth1txen(&self) -> ETH1TXEN_R {
        ETH1TXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Ethernet Reception Clock Enable
    #[inline(always)]
    pub fn eth1rxen(&self) -> ETH1RXEN_R {
        ETH1RXEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 25 - USB1OTG Peripheral Clocks Enable
    #[inline(always)]
    pub fn usb1otgen(&self) -> USB1OTGEN_R {
        USB1OTGEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USB_PHY1 Clocks Enable
    #[inline(always)]
    pub fn usb1ulpien(&self) -> USB1ULPIEN_R {
        USB1ULPIEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - USB2OTG Peripheral Clocks Enable
    #[inline(always)]
    pub fn usb2otgen(&self) -> USB2OTGEN_R {
        USB2OTGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - USB_PHY2 Clocks Enable
    #[inline(always)]
    pub fn usb2ulpien(&self) -> USB2ULPIEN_R {
        USB2ULPIEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA1 Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<0> {
        DMA1EN_W::new(self)
    }
    ///Bit 1 - DMA2 Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<1> {
        DMA2EN_W::new(self)
    }
    ///Bit 5 - ADC1/2 Peripheral Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn adc12en(&mut self) -> ADC12EN_W<5> {
        ADC12EN_W::new(self)
    }
    ///Bit 14 - ART Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn arten(&mut self) -> ARTEN_W<14> {
        ARTEN_W::new(self)
    }
    ///Bit 15 - Ethernet MAC bus interface Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn eth1macen(&mut self) -> ETH1MACEN_W<15> {
        ETH1MACEN_W::new(self)
    }
    ///Bit 16 - Ethernet Transmission Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn eth1txen(&mut self) -> ETH1TXEN_W<16> {
        ETH1TXEN_W::new(self)
    }
    ///Bit 17 - Ethernet Reception Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn eth1rxen(&mut self) -> ETH1RXEN_W<17> {
        ETH1RXEN_W::new(self)
    }
    ///Bit 25 - USB1OTG Peripheral Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn usb1otgen(&mut self) -> USB1OTGEN_W<25> {
        USB1OTGEN_W::new(self)
    }
    ///Bit 26 - USB_PHY1 Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn usb1ulpien(&mut self) -> USB1ULPIEN_W<26> {
        USB1ULPIEN_W::new(self)
    }
    ///Bit 27 - USB2OTG Peripheral Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn usb2otgen(&mut self) -> USB2OTGEN_W<27> {
        USB2OTGEN_W::new(self)
    }
    ///Bit 28 - USB_PHY2 Clocks Enable
    #[inline(always)]
    #[must_use]
    pub fn usb2ulpien(&mut self) -> USB2ULPIEN_W<28> {
        USB2ULPIEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB1 Clock Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1_ahb1enr](index.html) module
pub struct C1_AHB1ENR_SPEC;
impl crate::RegisterSpec for C1_AHB1ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1_ahb1enr::R](R) reader structure
impl crate::Readable for C1_AHB1ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c1_ahb1enr::W](W) writer structure
impl crate::Writable for C1_AHB1ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C1_AHB1ENR to value 0
impl crate::Resettable for C1_AHB1ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
