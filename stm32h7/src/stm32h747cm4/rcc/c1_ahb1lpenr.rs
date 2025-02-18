///Register `C1_AHB1LPENR` reader
pub struct R(crate::R<C1_AHB1LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_AHB1LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_AHB1LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_AHB1LPENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C1_AHB1LPENR` writer
pub struct W(crate::W<C1_AHB1LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_AHB1LPENR_SPEC>;
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
impl From<crate::W<C1_AHB1LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_AHB1LPENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMA1LPEN` reader - DMA1 Clock Enable During CSleep Mode
pub type DMA1LPEN_R = crate::BitReader<DMA1LPEN_A>;
///DMA1 Clock Enable During CSleep Mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1LPEN_A {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<DMA1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1LPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMA1LPEN_A {
        match self.bits {
            false => DMA1LPEN_A::Disabled,
            true => DMA1LPEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1LPEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1LPEN_A::Enabled
    }
}
///Field `DMA1LPEN` writer - DMA1 Clock Enable During CSleep Mode
pub type DMA1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_AHB1LPENR_SPEC, DMA1LPEN_A, O>;
impl<'a, const O: u8> DMA1LPEN_W<'a, O> {
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1LPEN_A::Enabled)
    }
}
///Field `DMA2LPEN` reader - DMA2 Clock Enable During CSleep Mode
pub use DMA1LPEN_R as DMA2LPEN_R;
///Field `ADC12LPEN` reader - ADC1/2 Peripheral Clocks Enable During CSleep Mode
pub use DMA1LPEN_R as ADC12LPEN_R;
///Field `ARTLPEN` reader - ART Clock Enable During CSleep Mode
pub use DMA1LPEN_R as ARTLPEN_R;
///Field `ETH1MACLPEN` reader - Ethernet MAC bus interface Clock Enable During CSleep Mode
pub use DMA1LPEN_R as ETH1MACLPEN_R;
///Field `ETH1TXLPEN` reader - Ethernet Transmission Clock Enable During CSleep Mode
pub use DMA1LPEN_R as ETH1TXLPEN_R;
///Field `ETH1RXLPEN` reader - Ethernet Reception Clock Enable During CSleep Mode
pub use DMA1LPEN_R as ETH1RXLPEN_R;
///Field `USB1OTGLPEN` reader - USB1OTG peripheral clock enable during CSleep mode
pub use DMA1LPEN_R as USB1OTGLPEN_R;
///Field `USB1ULPILPEN` reader - USB_PHY1 clock enable during CSleep mode
pub use DMA1LPEN_R as USB1ULPILPEN_R;
///Field `USB2OTGLPEN` reader - USB2OTG peripheral clock enable during CSleep mode
pub use DMA1LPEN_R as USB2OTGLPEN_R;
///Field `USB2ULPILPEN` reader - USB_PHY2 clocks enable during CSleep mode
pub use DMA1LPEN_R as USB2ULPILPEN_R;
///Field `DMA2LPEN` writer - DMA2 Clock Enable During CSleep Mode
pub use DMA1LPEN_W as DMA2LPEN_W;
///Field `ADC12LPEN` writer - ADC1/2 Peripheral Clocks Enable During CSleep Mode
pub use DMA1LPEN_W as ADC12LPEN_W;
///Field `ARTLPEN` writer - ART Clock Enable During CSleep Mode
pub use DMA1LPEN_W as ARTLPEN_W;
///Field `ETH1MACLPEN` writer - Ethernet MAC bus interface Clock Enable During CSleep Mode
pub use DMA1LPEN_W as ETH1MACLPEN_W;
///Field `ETH1TXLPEN` writer - Ethernet Transmission Clock Enable During CSleep Mode
pub use DMA1LPEN_W as ETH1TXLPEN_W;
///Field `ETH1RXLPEN` writer - Ethernet Reception Clock Enable During CSleep Mode
pub use DMA1LPEN_W as ETH1RXLPEN_W;
///Field `USB1OTGLPEN` writer - USB1OTG peripheral clock enable during CSleep mode
pub use DMA1LPEN_W as USB1OTGLPEN_W;
///Field `USB1ULPILPEN` writer - USB_PHY1 clock enable during CSleep mode
pub use DMA1LPEN_W as USB1ULPILPEN_W;
///Field `USB2OTGLPEN` writer - USB2OTG peripheral clock enable during CSleep mode
pub use DMA1LPEN_W as USB2OTGLPEN_W;
///Field `USB2ULPILPEN` writer - USB_PHY2 clocks enable during CSleep mode
pub use DMA1LPEN_W as USB2ULPILPEN_W;
impl R {
    ///Bit 0 - DMA1 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - ADC1/2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn adc12lpen(&self) -> ADC12LPEN_R {
        ADC12LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 14 - ART Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn artlpen(&self) -> ARTLPEN_R {
        ARTLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Ethernet MAC bus interface Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn eth1maclpen(&self) -> ETH1MACLPEN_R {
        ETH1MACLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Ethernet Transmission Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn eth1txlpen(&self) -> ETH1TXLPEN_R {
        ETH1TXLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Ethernet Reception Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn eth1rxlpen(&self) -> ETH1RXLPEN_R {
        ETH1RXLPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 25 - USB1OTG peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn usb1otglpen(&self) -> USB1OTGLPEN_R {
        USB1OTGLPEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - USB_PHY1 clock enable during CSleep mode
    #[inline(always)]
    pub fn usb1ulpilpen(&self) -> USB1ULPILPEN_R {
        USB1ULPILPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - USB2OTG peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn usb2otglpen(&self) -> USB2OTGLPEN_R {
        USB2OTGLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - USB_PHY2 clocks enable during CSleep mode
    #[inline(always)]
    pub fn usb2ulpilpen(&self) -> USB2ULPILPEN_R {
        USB2ULPILPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA1 Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<0> {
        DMA1LPEN_W::new(self)
    }
    ///Bit 1 - DMA2 Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<1> {
        DMA2LPEN_W::new(self)
    }
    ///Bit 5 - ADC1/2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn adc12lpen(&mut self) -> ADC12LPEN_W<5> {
        ADC12LPEN_W::new(self)
    }
    ///Bit 14 - ART Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn artlpen(&mut self) -> ARTLPEN_W<14> {
        ARTLPEN_W::new(self)
    }
    ///Bit 15 - Ethernet MAC bus interface Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn eth1maclpen(&mut self) -> ETH1MACLPEN_W<15> {
        ETH1MACLPEN_W::new(self)
    }
    ///Bit 16 - Ethernet Transmission Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn eth1txlpen(&mut self) -> ETH1TXLPEN_W<16> {
        ETH1TXLPEN_W::new(self)
    }
    ///Bit 17 - Ethernet Reception Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn eth1rxlpen(&mut self) -> ETH1RXLPEN_W<17> {
        ETH1RXLPEN_W::new(self)
    }
    ///Bit 25 - USB1OTG peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn usb1otglpen(&mut self) -> USB1OTGLPEN_W<25> {
        USB1OTGLPEN_W::new(self)
    }
    ///Bit 26 - USB_PHY1 clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn usb1ulpilpen(&mut self) -> USB1ULPILPEN_W<26> {
        USB1ULPILPEN_W::new(self)
    }
    ///Bit 27 - USB2OTG peripheral clock enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn usb2otglpen(&mut self) -> USB2OTGLPEN_W<27> {
        USB2OTGLPEN_W::new(self)
    }
    ///Bit 28 - USB_PHY2 clocks enable during CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn usb2ulpilpen(&mut self) -> USB2ULPILPEN_W<28> {
        USB2ULPILPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB1 Sleep Clock Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1_ahb1lpenr](index.html) module
pub struct C1_AHB1LPENR_SPEC;
impl crate::RegisterSpec for C1_AHB1LPENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1_ahb1lpenr::R](R) reader structure
impl crate::Readable for C1_AHB1LPENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c1_ahb1lpenr::W](W) writer structure
impl crate::Writable for C1_AHB1LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C1_AHB1LPENR to value 0
impl crate::Resettable for C1_AHB1LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
