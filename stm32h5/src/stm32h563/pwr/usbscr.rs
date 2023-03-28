///Register `USBSCR` reader
pub struct R(crate::R<USBSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `USBSCR` writer
pub struct W(crate::W<USBSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBSCR_SPEC>;
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
impl From<crate::W<USBSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBSCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `USB33DEN` reader - V&lt;sub>DDUSB&lt;/sub> voltage level detector enable
pub type USB33DEN_R = crate::BitReader<bool>;
///Field `USB33DEN` writer - V&lt;sub>DDUSB&lt;/sub> voltage level detector enable
pub type USB33DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSCR_SPEC, bool, O>;
///Field `USB33SV` reader - independent USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USBFS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the V&lt;sub>DDUSB&lt;/sub> voltage monitor can be used to determine whether this supply is ready or not.
pub type USB33SV_R = crate::BitReader<bool>;
///Field `USB33SV` writer - independent USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USBFS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the V&lt;sub>DDUSB&lt;/sub> voltage monitor can be used to determine whether this supply is ready or not.
pub type USB33SV_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSCR_SPEC, bool, O>;
impl R {
    ///Bit 24 - V&lt;sub>DDUSB&lt;/sub> voltage level detector enable
    #[inline(always)]
    pub fn usb33den(&self) -> USB33DEN_R {
        USB33DEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - independent USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USBFS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the V&lt;sub>DDUSB&lt;/sub> voltage monitor can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn usb33sv(&self) -> USB33SV_R {
        USB33SV_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 24 - V&lt;sub>DDUSB&lt;/sub> voltage level detector enable
    #[inline(always)]
    #[must_use]
    pub fn usb33den(&mut self) -> USB33DEN_W<24> {
        USB33DEN_W::new(self)
    }
    ///Bit 25 - independent USB supply valid This bit is used to validate the V&lt;sub>DDUSB&lt;/sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USBFS peripheral. If V&lt;sub>DDUSB&lt;/sub> is not always present in the application, the V&lt;sub>DDUSB&lt;/sub> voltage monitor can be used to determine whether this supply is ready or not.
    #[inline(always)]
    #[must_use]
    pub fn usb33sv(&mut self) -> USB33SV_W<25> {
        USB33SV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR USB supply control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [usbscr](index.html) module
pub struct USBSCR_SPEC;
impl crate::RegisterSpec for USBSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [usbscr::R](R) reader structure
impl crate::Readable for USBSCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [usbscr::W](W) writer structure
impl crate::Writable for USBSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets USBSCR to value 0
impl crate::Resettable for USBSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
