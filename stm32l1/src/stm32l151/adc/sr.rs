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
///Field `AWD` reader - Analog watchdog flag
pub type AWD_R = crate::BitReader<bool>;
///Field `AWD` writer - Analog watchdog flag
pub type AWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `EOC` reader - Regular channel end of conversion
pub type EOC_R = crate::BitReader<bool>;
///Field `EOC` writer - Regular channel end of conversion
pub type EOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `JEOC` reader - Injected channel end of conversion
pub type JEOC_R = crate::BitReader<bool>;
///Field `JEOC` writer - Injected channel end of conversion
pub type JEOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `JSTRT` reader - Injected channel start flag
pub type JSTRT_R = crate::BitReader<bool>;
///Field `JSTRT` writer - Injected channel start flag
pub type JSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `STRT` reader - Regular channel start flag
pub type STRT_R = crate::BitReader<bool>;
///Field `STRT` writer - Regular channel start flag
pub type STRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `OVR` reader - Overrun
pub type OVR_R = crate::BitReader<bool>;
///Field `OVR` writer - Overrun
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `ADONS` reader - ADC ON status
pub type ADONS_R = crate::BitReader<bool>;
///Field `RCNR` reader - Regular channel not ready
pub type RCNR_R = crate::BitReader<bool>;
///Field `JCNR` reader - Injected channel not ready
pub type JCNR_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Analog watchdog flag
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Regular channel end of conversion
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected channel end of conversion
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Injected channel start flag
    #[inline(always)]
    pub fn jstrt(&self) -> JSTRT_R {
        JSTRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Regular channel start flag
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Overrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ADC ON status
    #[inline(always)]
    pub fn adons(&self) -> ADONS_R {
        ADONS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Regular channel not ready
    #[inline(always)]
    pub fn rcnr(&self) -> RCNR_R {
        RCNR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Injected channel not ready
    #[inline(always)]
    pub fn jcnr(&self) -> JCNR_R {
        JCNR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Analog watchdog flag
    #[inline(always)]
    #[must_use]
    pub fn awd(&mut self) -> AWD_W<0> {
        AWD_W::new(self)
    }
    ///Bit 1 - Regular channel end of conversion
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<1> {
        EOC_W::new(self)
    }
    ///Bit 2 - Injected channel end of conversion
    #[inline(always)]
    #[must_use]
    pub fn jeoc(&mut self) -> JEOC_W<2> {
        JEOC_W::new(self)
    }
    ///Bit 3 - Injected channel start flag
    #[inline(always)]
    #[must_use]
    pub fn jstrt(&mut self) -> JSTRT_W<3> {
        JSTRT_W::new(self)
    }
    ///Bit 4 - Regular channel start flag
    #[inline(always)]
    #[must_use]
    pub fn strt(&mut self) -> STRT_W<4> {
        STRT_W::new(self)
    }
    ///Bit 5 - Overrun
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<5> {
        OVR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///status register
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
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
