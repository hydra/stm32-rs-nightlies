///Register `OPAMP1_CSR` reader
pub struct R(crate::R<OPAMP1_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP1_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP1_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP1_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPAMP1_CSR` writer
pub struct W(crate::W<OPAMP1_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP1_CSR_SPEC>;
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
impl From<crate::W<OPAMP1_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP1_CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OPAEN` reader - Operational amplifier Enable
pub type OPAEN_R = crate::BitReader<bool>;
///Field `OPAEN` writer - Operational amplifier Enable
pub type OPAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, bool, O>;
///Field `FORCE_VP` reader - Force internal reference on VP (reserved for test
pub type FORCE_VP_R = crate::BitReader<bool>;
///Field `FORCE_VP` writer - Force internal reference on VP (reserved for test
pub type FORCE_VP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, bool, O>;
///Field `VP_SEL` reader - Operational amplifier PGA mode
pub type VP_SEL_R = crate::FieldReader<u8, u8>;
///Field `VP_SEL` writer - Operational amplifier PGA mode
pub type VP_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP1_CSR_SPEC, u8, u8, 2, O>;
///Field `VM_SEL` reader - Inverting input selection
pub type VM_SEL_R = crate::FieldReader<u8, u8>;
///Field `VM_SEL` writer - Inverting input selection
pub type VM_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP1_CSR_SPEC, u8, u8, 2, O>;
///Field `OPAHSM` reader - Operational amplifier high-speed mode
pub type OPAHSM_R = crate::BitReader<bool>;
///Field `OPAHSM` writer - Operational amplifier high-speed mode
pub type OPAHSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, bool, O>;
///Field `CALON` reader - Calibration mode enabled
pub type CALON_R = crate::BitReader<bool>;
///Field `CALON` writer - Calibration mode enabled
pub type CALON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, bool, O>;
///Field `CALSEL` reader - Calibration selection
pub type CALSEL_R = crate::FieldReader<u8, u8>;
///Field `CALSEL` writer - Calibration selection
pub type CALSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP1_CSR_SPEC, u8, u8, 2, O>;
///Field `PGA_GAIN` reader - allows to switch from AOP offset trimmed values to AOP offset
pub type PGA_GAIN_R = crate::FieldReader<u8, u8>;
///Field `PGA_GAIN` writer - allows to switch from AOP offset trimmed values to AOP offset
pub type PGA_GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP1_CSR_SPEC, u8, u8, 4, O>;
///Field `USERTRIM` reader - User trimming enable
pub type USERTRIM_R = crate::BitReader<bool>;
///Field `USERTRIM` writer - User trimming enable
pub type USERTRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, bool, O>;
///Field `TSTREF` reader - OPAMP calibration reference voltage output control (reserved for test)
pub type TSTREF_R = crate::BitReader<bool>;
///Field `TSTREF` writer - OPAMP calibration reference voltage output control (reserved for test)
pub type TSTREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, bool, O>;
///Field `CALOUT` reader - Operational amplifier calibration output
pub type CALOUT_R = crate::BitReader<bool>;
///Field `CALOUT` writer - Operational amplifier calibration output
pub type CALOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Force internal reference on VP (reserved for test
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Operational amplifier PGA mode
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 5:6 - Inverting input selection
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 8 - Operational amplifier high-speed mode
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Calibration mode enabled
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:17 - allows to switch from AOP offset trimmed values to AOP offset
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    ///Bit 18 - User trimming enable
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 29 - OPAMP calibration reference voltage output control (reserved for test)
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Operational amplifier calibration output
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OPAEN_W<0> {
        OPAEN_W::new(self)
    }
    ///Bit 1 - Force internal reference on VP (reserved for test
    #[inline(always)]
    #[must_use]
    pub fn force_vp(&mut self) -> FORCE_VP_W<1> {
        FORCE_VP_W::new(self)
    }
    ///Bits 2:3 - Operational amplifier PGA mode
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<2> {
        VP_SEL_W::new(self)
    }
    ///Bits 5:6 - Inverting input selection
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<5> {
        VM_SEL_W::new(self)
    }
    ///Bit 8 - Operational amplifier high-speed mode
    #[inline(always)]
    #[must_use]
    pub fn opahsm(&mut self) -> OPAHSM_W<8> {
        OPAHSM_W::new(self)
    }
    ///Bit 11 - Calibration mode enabled
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<11> {
        CALON_W::new(self)
    }
    ///Bits 12:13 - Calibration selection
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<12> {
        CALSEL_W::new(self)
    }
    ///Bits 14:17 - allows to switch from AOP offset trimmed values to AOP offset
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<14> {
        PGA_GAIN_W::new(self)
    }
    ///Bit 18 - User trimming enable
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> USERTRIM_W<18> {
        USERTRIM_W::new(self)
    }
    ///Bit 29 - OPAMP calibration reference voltage output control (reserved for test)
    #[inline(always)]
    #[must_use]
    pub fn tstref(&mut self) -> TSTREF_W<29> {
        TSTREF_W::new(self)
    }
    ///Bit 30 - Operational amplifier calibration output
    #[inline(always)]
    #[must_use]
    pub fn calout(&mut self) -> CALOUT_W<30> {
        CALOUT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OPAMP1 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp1_csr](index.html) module
pub struct OPAMP1_CSR_SPEC;
impl crate::RegisterSpec for OPAMP1_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [opamp1_csr::R](R) reader structure
impl crate::Readable for OPAMP1_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [opamp1_csr::W](W) writer structure
impl crate::Writable for OPAMP1_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPAMP1_CSR to value 0
impl crate::Resettable for OPAMP1_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
