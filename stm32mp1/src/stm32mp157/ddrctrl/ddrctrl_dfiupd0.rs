///Register `DDRCTRL_DFIUPD0` reader
pub struct R(crate::R<DDRCTRL_DFIUPD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DFIUPD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DFIUPD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DFIUPD0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_DFIUPD0` writer
pub struct W(crate::W<DDRCTRL_DFIUPD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DFIUPD0_SPEC>;
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
impl From<crate::W<DDRCTRL_DFIUPD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DFIUPD0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DFI_T_CTRLUP_MIN` reader - DFI_T_CTRLUP_MIN
pub type DFI_T_CTRLUP_MIN_R = crate::FieldReader<u16, u16>;
///Field `DFI_T_CTRLUP_MIN` writer - DFI_T_CTRLUP_MIN
pub type DFI_T_CTRLUP_MIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DFIUPD0_SPEC, u16, u16, 10, O>;
///Field `DFI_T_CTRLUP_MAX` reader - DFI_T_CTRLUP_MAX
pub type DFI_T_CTRLUP_MAX_R = crate::FieldReader<u16, u16>;
///Field `DFI_T_CTRLUP_MAX` writer - DFI_T_CTRLUP_MAX
pub type DFI_T_CTRLUP_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_DFIUPD0_SPEC, u16, u16, 10, O>;
///Field `CTRLUPD_PRE_SRX` reader - CTRLUPD_PRE_SRX
pub type CTRLUPD_PRE_SRX_R = crate::BitReader<bool>;
///Field `CTRLUPD_PRE_SRX` writer - CTRLUPD_PRE_SRX
pub type CTRLUPD_PRE_SRX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_DFIUPD0_SPEC, bool, O>;
///Field `DIS_AUTO_CTRLUPD_SRX` reader - DIS_AUTO_CTRLUPD_SRX
pub type DIS_AUTO_CTRLUPD_SRX_R = crate::BitReader<bool>;
///Field `DIS_AUTO_CTRLUPD_SRX` writer - DIS_AUTO_CTRLUPD_SRX
pub type DIS_AUTO_CTRLUPD_SRX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_DFIUPD0_SPEC, bool, O>;
///Field `DIS_AUTO_CTRLUPD` reader - DIS_AUTO_CTRLUPD
pub type DIS_AUTO_CTRLUPD_R = crate::BitReader<bool>;
///Field `DIS_AUTO_CTRLUPD` writer - DIS_AUTO_CTRLUPD
pub type DIS_AUTO_CTRLUPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_DFIUPD0_SPEC, bool, O>;
impl R {
    ///Bits 0:9 - DFI_T_CTRLUP_MIN
    #[inline(always)]
    pub fn dfi_t_ctrlup_min(&self) -> DFI_T_CTRLUP_MIN_R {
        DFI_T_CTRLUP_MIN_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - DFI_T_CTRLUP_MAX
    #[inline(always)]
    pub fn dfi_t_ctrlup_max(&self) -> DFI_T_CTRLUP_MAX_R {
        DFI_T_CTRLUP_MAX_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bit 29 - CTRLUPD_PRE_SRX
    #[inline(always)]
    pub fn ctrlupd_pre_srx(&self) -> CTRLUPD_PRE_SRX_R {
        CTRLUPD_PRE_SRX_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DIS_AUTO_CTRLUPD_SRX
    #[inline(always)]
    pub fn dis_auto_ctrlupd_srx(&self) -> DIS_AUTO_CTRLUPD_SRX_R {
        DIS_AUTO_CTRLUPD_SRX_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DIS_AUTO_CTRLUPD
    #[inline(always)]
    pub fn dis_auto_ctrlupd(&self) -> DIS_AUTO_CTRLUPD_R {
        DIS_AUTO_CTRLUPD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:9 - DFI_T_CTRLUP_MIN
    #[inline(always)]
    #[must_use]
    pub fn dfi_t_ctrlup_min(&mut self) -> DFI_T_CTRLUP_MIN_W<0> {
        DFI_T_CTRLUP_MIN_W::new(self)
    }
    ///Bits 16:25 - DFI_T_CTRLUP_MAX
    #[inline(always)]
    #[must_use]
    pub fn dfi_t_ctrlup_max(&mut self) -> DFI_T_CTRLUP_MAX_W<16> {
        DFI_T_CTRLUP_MAX_W::new(self)
    }
    ///Bit 29 - CTRLUPD_PRE_SRX
    #[inline(always)]
    #[must_use]
    pub fn ctrlupd_pre_srx(&mut self) -> CTRLUPD_PRE_SRX_W<29> {
        CTRLUPD_PRE_SRX_W::new(self)
    }
    ///Bit 30 - DIS_AUTO_CTRLUPD_SRX
    #[inline(always)]
    #[must_use]
    pub fn dis_auto_ctrlupd_srx(&mut self) -> DIS_AUTO_CTRLUPD_SRX_W<30> {
        DIS_AUTO_CTRLUPD_SRX_W::new(self)
    }
    ///Bit 31 - DIS_AUTO_CTRLUPD
    #[inline(always)]
    #[must_use]
    pub fn dis_auto_ctrlupd(&mut self) -> DIS_AUTO_CTRLUPD_W<31> {
        DIS_AUTO_CTRLUPD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL DFI update register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_dfiupd0](index.html) module
pub struct DDRCTRL_DFIUPD0_SPEC;
impl crate::RegisterSpec for DDRCTRL_DFIUPD0_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_dfiupd0::R](R) reader structure
impl crate::Readable for DDRCTRL_DFIUPD0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_dfiupd0::W](W) writer structure
impl crate::Writable for DDRCTRL_DFIUPD0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_DFIUPD0 to value 0x0040_0003
impl crate::Resettable for DDRCTRL_DFIUPD0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0003;
}
