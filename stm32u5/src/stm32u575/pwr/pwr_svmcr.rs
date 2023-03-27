///Register `PWR_SVMCR` reader
pub struct R(crate::R<PWR_SVMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_SVMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_SVMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_SVMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_SVMCR` writer
pub struct W(crate::W<PWR_SVMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_SVMCR_SPEC>;
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
impl From<crate::W<PWR_SVMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_SVMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PVDE` reader - Power voltage detector enable
pub type PVDE_R = crate::BitReader<bool>;
///Field `PVDE` writer - Power voltage detector enable
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SVMCR_SPEC, bool, O>;
///Field `PVDLS` reader - Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:
pub type PVDLS_R = crate::FieldReader<u8, u8>;
///Field `PVDLS` writer - Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:
pub type PVDLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_SVMCR_SPEC, u8, u8, 3, O>;
///Field `UVMEN` reader - VDDUSB independent USB voltage monitor enable
pub type UVMEN_R = crate::BitReader<bool>;
///Field `UVMEN` writer - VDDUSB independent USB voltage monitor enable
pub type UVMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SVMCR_SPEC, bool, O>;
///Field `IO2VMEN` reader - VDDIO2 independent I/Os voltage monitor enable
pub type IO2VMEN_R = crate::BitReader<bool>;
///Field `IO2VMEN` writer - VDDIO2 independent I/Os voltage monitor enable
pub type IO2VMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SVMCR_SPEC, bool, O>;
///Field `AVM1EN` reader - VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
pub type AVM1EN_R = crate::BitReader<bool>;
///Field `AVM1EN` writer - VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
pub type AVM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SVMCR_SPEC, bool, O>;
///Field `AVM2EN` reader - VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
pub type AVM2EN_R = crate::BitReader<bool>;
///Field `AVM2EN` writer - VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
pub type AVM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SVMCR_SPEC, bool, O>;
///Field `USV` reader - VDDUSB independent USB supply valid
pub type USV_R = crate::BitReader<bool>;
///Field `USV` writer - VDDUSB independent USB supply valid
pub type USV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SVMCR_SPEC, bool, O>;
///Field `IO2SV` reader - VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.
pub type IO2SV_R = crate::BitReader<bool>;
///Field `IO2SV` writer - VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.
pub type IO2SV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SVMCR_SPEC, bool, O>;
///Field `ASV` reader - VDDA independent analog supply valid
pub type ASV_R = crate::BitReader<bool>;
///Field `ASV` writer - VDDA independent analog supply valid
pub type ASV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SVMCR_SPEC, bool, O>;
impl R {
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:
    #[inline(always)]
    pub fn pvdls(&self) -> PVDLS_R {
        PVDLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 24 - VDDUSB independent USB voltage monitor enable
    #[inline(always)]
    pub fn uvmen(&self) -> UVMEN_R {
        UVMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VDDIO2 independent I/Os voltage monitor enable
    #[inline(always)]
    pub fn io2vmen(&self) -> IO2VMEN_R {
        IO2VMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
    #[inline(always)]
    pub fn avm1en(&self) -> AVM1EN_R {
        AVM1EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
    #[inline(always)]
    pub fn avm2en(&self) -> AVM2EN_R {
        AVM2EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - VDDUSB independent USB supply valid
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.
    #[inline(always)]
    pub fn io2sv(&self) -> IO2SV_R {
        IO2SV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - VDDA independent analog supply valid
    #[inline(always)]
    pub fn asv(&self) -> ASV_R {
        ASV_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - Power voltage detector enable
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<4> {
        PVDE_W::new(self)
    }
    ///Bits 5:7 - Power voltage detector level selection These bits select the voltage threshold detected by the power voltage detector:
    #[inline(always)]
    #[must_use]
    pub fn pvdls(&mut self) -> PVDLS_W<5> {
        PVDLS_W::new(self)
    }
    ///Bit 24 - VDDUSB independent USB voltage monitor enable
    #[inline(always)]
    #[must_use]
    pub fn uvmen(&mut self) -> UVMEN_W<24> {
        UVMEN_W::new(self)
    }
    ///Bit 25 - VDDIO2 independent I/Os voltage monitor enable
    #[inline(always)]
    #[must_use]
    pub fn io2vmen(&mut self) -> IO2VMEN_W<25> {
        IO2VMEN_W::new(self)
    }
    ///Bit 26 - VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
    #[inline(always)]
    #[must_use]
    pub fn avm1en(&mut self) -> AVM1EN_W<26> {
        AVM1EN_W::new(self)
    }
    ///Bit 27 - VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
    #[inline(always)]
    #[must_use]
    pub fn avm2en(&mut self) -> AVM2EN_W<27> {
        AVM2EN_W::new(self)
    }
    ///Bit 28 - VDDUSB independent USB supply valid
    #[inline(always)]
    #[must_use]
    pub fn usv(&mut self) -> USV_W<28> {
        USV_W::new(self)
    }
    ///Bit 29 - VDDIO2 independent I/Os supply valid This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose. Setting this bit is mandatory to use PG\[15:2\]. If VDDIO2 is not always present in the application, the VDDIO2 voltage monitor can be used to determine whether this supply is ready or not.
    #[inline(always)]
    #[must_use]
    pub fn io2sv(&mut self) -> IO2SV_W<29> {
        IO2SV_W::new(self)
    }
    ///Bit 30 - VDDA independent analog supply valid
    #[inline(always)]
    #[must_use]
    pub fn asv(&mut self) -> ASV_W<30> {
        ASV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR supply voltage monitoring control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_svmcr](index.html) module
pub struct PWR_SVMCR_SPEC;
impl crate::RegisterSpec for PWR_SVMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_svmcr::R](R) reader structure
impl crate::Readable for PWR_SVMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_svmcr::W](W) writer structure
impl crate::Writable for PWR_SVMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_SVMCR to value 0
impl crate::Resettable for PWR_SVMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
