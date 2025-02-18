///Register `CFGR2` reader
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR2` writer
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - COMP channel 1 enable bit
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - COMP channel 1 enable bit
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
///Field `BRGEN` reader - Scaler bridge enable
pub type BRGEN_R = crate::BitReader<bool>;
///Field `BRGEN` writer - Scaler bridge enable
pub type BRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
///Field `SCALEN` reader - Voltage scaler enable bit
pub type SCALEN_R = crate::BitReader<bool>;
///Field `SCALEN` writer - Voltage scaler enable bit
pub type SCALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
///Field `POLARITY` reader - COMP channel 1 polarity selection bit
pub type POLARITY_R = crate::BitReader<bool>;
///Field `POLARITY` writer - COMP channel 1 polarity selection bit
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
///Field `WINMODE` reader - Window comparator mode selection bit
pub type WINMODE_R = crate::BitReader<bool>;
///Field `WINMODE` writer - Window comparator mode selection bit
pub type WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
///Field `ITEN` reader - COMP channel 1 interrupt enable
pub type ITEN_R = crate::BitReader<bool>;
///Field `ITEN` writer - COMP channel 1 interrupt enable
pub type ITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
///Field `HYST` reader - COMP channel 1 hysteresis selection bits
pub type HYST_R = crate::FieldReader<u8, u8>;
///Field `HYST` writer - COMP channel 1 hysteresis selection bits
pub type HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 2, O>;
///Field `PWRMODE` reader - Power Mode of the COMP channel 1
pub type PWRMODE_R = crate::FieldReader<u8, u8>;
///Field `PWRMODE` writer - Power Mode of the COMP channel 1
pub type PWRMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 2, O>;
///Field `INMSEL` reader - COMP channel 1 inverting input selection field
pub type INMSEL_R = crate::FieldReader<u8, u8>;
///Field `INMSEL` writer - COMP channel 1 inverting input selection field
pub type INMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 3, O>;
///Field `INPSEL` reader - COMP channel 1 non-inverting input selection bit
pub type INPSEL_R = crate::BitReader<bool>;
///Field `INPSEL` writer - COMP channel 1 non-inverting input selection bit
pub type INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
///Field `BLANKING` reader - COMP channel 1 blanking source selection bits
pub type BLANKING_R = crate::FieldReader<u8, u8>;
///Field `BLANKING` writer - COMP channel 1 blanking source selection bits
pub type BLANKING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 4, O>;
///Field `LOCK` reader - Lock bit
pub type LOCK_R = crate::BitReader<bool>;
///Field `LOCK` writer - Lock bit
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - COMP channel 1 enable bit
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Scaler bridge enable
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Voltage scaler enable bit
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - COMP channel 1 polarity selection bit
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Window comparator mode selection bit
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - COMP channel 1 interrupt enable
    #[inline(always)]
    pub fn iten(&self) -> ITEN_R {
        ITEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:9 - COMP channel 1 hysteresis selection bits
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Power Mode of the COMP channel 1
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 16:18 - COMP channel 1 inverting input selection field
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bit 20 - COMP channel 1 non-inverting input selection bit
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 24:27 - COMP channel 1 blanking source selection bits
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 31 - Lock bit
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - COMP channel 1 enable bit
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bit 1 - Scaler bridge enable
    #[inline(always)]
    #[must_use]
    pub fn brgen(&mut self) -> BRGEN_W<1> {
        BRGEN_W::new(self)
    }
    ///Bit 2 - Voltage scaler enable bit
    #[inline(always)]
    #[must_use]
    pub fn scalen(&mut self) -> SCALEN_W<2> {
        SCALEN_W::new(self)
    }
    ///Bit 3 - COMP channel 1 polarity selection bit
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<3> {
        POLARITY_W::new(self)
    }
    ///Bit 4 - Window comparator mode selection bit
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WINMODE_W<4> {
        WINMODE_W::new(self)
    }
    ///Bit 6 - COMP channel 1 interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn iten(&mut self) -> ITEN_W<6> {
        ITEN_W::new(self)
    }
    ///Bits 8:9 - COMP channel 1 hysteresis selection bits
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<8> {
        HYST_W::new(self)
    }
    ///Bits 12:13 - Power Mode of the COMP channel 1
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<12> {
        PWRMODE_W::new(self)
    }
    ///Bits 16:18 - COMP channel 1 inverting input selection field
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<16> {
        INMSEL_W::new(self)
    }
    ///Bit 20 - COMP channel 1 non-inverting input selection bit
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<20> {
        INPSEL_W::new(self)
    }
    ///Bits 24:27 - COMP channel 1 blanking source selection bits
    #[inline(always)]
    #[must_use]
    pub fn blanking(&mut self) -> BLANKING_W<24> {
        BLANKING_W::new(self)
    }
    ///Bit 31 - Lock bit
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Comparator configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr2](index.html) module
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr2::R](R) reader structure
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr2::W](W) writer structure
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
