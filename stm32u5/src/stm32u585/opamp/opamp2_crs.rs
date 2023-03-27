///Register `OPAMP2_CRS` reader
pub struct R(crate::R<OPAMP2_CRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP2_CRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP2_CRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP2_CRS_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPAMP2_CRS` writer
pub struct W(crate::W<OPAMP2_CRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP2_CRS_SPEC>;
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
impl From<crate::W<OPAMP2_CRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP2_CRS_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OPAEN` reader - OPAEN
pub type OPAEN_R = crate::BitReader<bool>;
///Field `OPAEN` writer - OPAEN
pub type OPAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CRS_SPEC, bool, O>;
///Field `OPALPM` reader - OPALPM
pub type OPALPM_R = crate::BitReader<bool>;
///Field `OPALPM` writer - OPALPM
pub type OPALPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CRS_SPEC, bool, O>;
///Field `OPAMODE` reader - OPAMODE
pub type OPAMODE_R = crate::FieldReader<u8, u8>;
///Field `OPAMODE` writer - OPAMODE
pub type OPAMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP2_CRS_SPEC, u8, u8, 2, O>;
///Field `PGA_GAIN` reader - PGA_GAIN
pub type PGA_GAIN_R = crate::FieldReader<u8, u8>;
///Field `PGA_GAIN` writer - PGA_GAIN
pub type PGA_GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP2_CRS_SPEC, u8, u8, 2, O>;
///Field `VM_SEL` reader - VM_SEL
pub type VM_SEL_R = crate::FieldReader<u8, u8>;
///Field `VM_SEL` writer - VM_SEL
pub type VM_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPAMP2_CRS_SPEC, u8, u8, 2, O>;
///Field `VP_SEL` reader - VP_SEL
pub type VP_SEL_R = crate::BitReader<bool>;
///Field `VP_SEL` writer - VP_SEL
pub type VP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CRS_SPEC, bool, O>;
///Field `CALON` reader - CALON
pub type CALON_R = crate::BitReader<bool>;
///Field `CALON` writer - CALON
pub type CALON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CRS_SPEC, bool, O>;
///Field `CALSEL` reader - CALSEL
pub type CALSEL_R = crate::BitReader<bool>;
///Field `CALSEL` writer - CALSEL
pub type CALSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CRS_SPEC, bool, O>;
///Field `USERTRIM` reader - USERTRIM
pub type USERTRIM_R = crate::BitReader<bool>;
///Field `USERTRIM` writer - USERTRIM
pub type USERTRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CRS_SPEC, bool, O>;
///Field `CALOUT` reader - CALOUT
pub type CALOUT_R = crate::BitReader<bool>;
///Field `OPAHSM` reader - OPAHSM
pub type OPAHSM_R = crate::BitReader<bool>;
///Field `OPAHSM` writer - OPAHSM
pub type OPAHSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CRS_SPEC, bool, O>;
impl R {
    ///Bit 0 - OPAEN
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OPALPM
    #[inline(always)]
    pub fn opalpm(&self) -> OPALPM_R {
        OPALPM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - OPAMODE
    #[inline(always)]
    pub fn opamode(&self) -> OPAMODE_R {
        OPAMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - PGA_GAIN
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - VM_SEL
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - VP_SEL
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - CALON
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CALSEL
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - USERTRIM
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CALOUT
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 30 - OPAHSM
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - OPAEN
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OPAEN_W<0> {
        OPAEN_W::new(self)
    }
    ///Bit 1 - OPALPM
    #[inline(always)]
    #[must_use]
    pub fn opalpm(&mut self) -> OPALPM_W<1> {
        OPALPM_W::new(self)
    }
    ///Bits 2:3 - OPAMODE
    #[inline(always)]
    #[must_use]
    pub fn opamode(&mut self) -> OPAMODE_W<2> {
        OPAMODE_W::new(self)
    }
    ///Bits 4:5 - PGA_GAIN
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<4> {
        PGA_GAIN_W::new(self)
    }
    ///Bits 8:9 - VM_SEL
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<8> {
        VM_SEL_W::new(self)
    }
    ///Bit 10 - VP_SEL
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<10> {
        VP_SEL_W::new(self)
    }
    ///Bit 12 - CALON
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<12> {
        CALON_W::new(self)
    }
    ///Bit 13 - CALSEL
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<13> {
        CALSEL_W::new(self)
    }
    ///Bit 14 - USERTRIM
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> USERTRIM_W<14> {
        USERTRIM_W::new(self)
    }
    ///Bit 30 - OPAHSM
    #[inline(always)]
    #[must_use]
    pub fn opahsm(&mut self) -> OPAHSM_W<30> {
        OPAHSM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp2_crs](index.html) module
pub struct OPAMP2_CRS_SPEC;
impl crate::RegisterSpec for OPAMP2_CRS_SPEC {
    type Ux = u32;
}
///`read()` method returns [opamp2_crs::R](R) reader structure
impl crate::Readable for OPAMP2_CRS_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [opamp2_crs::W](W) writer structure
impl crate::Writable for OPAMP2_CRS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPAMP2_CRS to value 0
impl crate::Resettable for OPAMP2_CRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
