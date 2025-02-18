///Register `DDRCTRL_PWRCTL` reader
pub struct R(crate::R<DDRCTRL_PWRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_PWRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_PWRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_PWRCTL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_PWRCTL` writer
pub struct W(crate::W<DDRCTRL_PWRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_PWRCTL_SPEC>;
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
impl From<crate::W<DDRCTRL_PWRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_PWRCTL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SELFREF_EN` reader - SELFREF_EN
pub type SELFREF_EN_R = crate::BitReader<bool>;
///Field `SELFREF_EN` writer - SELFREF_EN
pub type SELFREF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_PWRCTL_SPEC, bool, O>;
///Field `POWERDOWN_EN` reader - POWERDOWN_EN
pub type POWERDOWN_EN_R = crate::BitReader<bool>;
///Field `POWERDOWN_EN` writer - POWERDOWN_EN
pub type POWERDOWN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_PWRCTL_SPEC, bool, O>;
///Field `DEEPPOWERDOWN_EN` reader - DEEPPOWERDOWN_EN
pub type DEEPPOWERDOWN_EN_R = crate::BitReader<bool>;
///Field `DEEPPOWERDOWN_EN` writer - DEEPPOWERDOWN_EN
pub type DEEPPOWERDOWN_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_PWRCTL_SPEC, bool, O>;
///Field `EN_DFI_DRAM_CLK_DISABLE` reader - EN_DFI_DRAM_CLK_DISABLE
pub type EN_DFI_DRAM_CLK_DISABLE_R = crate::BitReader<bool>;
///Field `EN_DFI_DRAM_CLK_DISABLE` writer - EN_DFI_DRAM_CLK_DISABLE
pub type EN_DFI_DRAM_CLK_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_PWRCTL_SPEC, bool, O>;
///Field `SELFREF_SW` reader - SELFREF_SW
pub type SELFREF_SW_R = crate::BitReader<bool>;
///Field `SELFREF_SW` writer - SELFREF_SW
pub type SELFREF_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRCTRL_PWRCTL_SPEC, bool, O>;
///Field `DIS_CAM_DRAIN_SELFREF` reader - DIS_CAM_DRAIN_SELFREF
pub type DIS_CAM_DRAIN_SELFREF_R = crate::BitReader<bool>;
///Field `DIS_CAM_DRAIN_SELFREF` writer - DIS_CAM_DRAIN_SELFREF
pub type DIS_CAM_DRAIN_SELFREF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_PWRCTL_SPEC, bool, O>;
impl R {
    ///Bit 0 - SELFREF_EN
    #[inline(always)]
    pub fn selfref_en(&self) -> SELFREF_EN_R {
        SELFREF_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - POWERDOWN_EN
    #[inline(always)]
    pub fn powerdown_en(&self) -> POWERDOWN_EN_R {
        POWERDOWN_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DEEPPOWERDOWN_EN
    #[inline(always)]
    pub fn deeppowerdown_en(&self) -> DEEPPOWERDOWN_EN_R {
        DEEPPOWERDOWN_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EN_DFI_DRAM_CLK_DISABLE
    #[inline(always)]
    pub fn en_dfi_dram_clk_disable(&self) -> EN_DFI_DRAM_CLK_DISABLE_R {
        EN_DFI_DRAM_CLK_DISABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SELFREF_SW
    #[inline(always)]
    pub fn selfref_sw(&self) -> SELFREF_SW_R {
        SELFREF_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - DIS_CAM_DRAIN_SELFREF
    #[inline(always)]
    pub fn dis_cam_drain_selfref(&self) -> DIS_CAM_DRAIN_SELFREF_R {
        DIS_CAM_DRAIN_SELFREF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SELFREF_EN
    #[inline(always)]
    #[must_use]
    pub fn selfref_en(&mut self) -> SELFREF_EN_W<0> {
        SELFREF_EN_W::new(self)
    }
    ///Bit 1 - POWERDOWN_EN
    #[inline(always)]
    #[must_use]
    pub fn powerdown_en(&mut self) -> POWERDOWN_EN_W<1> {
        POWERDOWN_EN_W::new(self)
    }
    ///Bit 2 - DEEPPOWERDOWN_EN
    #[inline(always)]
    #[must_use]
    pub fn deeppowerdown_en(&mut self) -> DEEPPOWERDOWN_EN_W<2> {
        DEEPPOWERDOWN_EN_W::new(self)
    }
    ///Bit 3 - EN_DFI_DRAM_CLK_DISABLE
    #[inline(always)]
    #[must_use]
    pub fn en_dfi_dram_clk_disable(&mut self) -> EN_DFI_DRAM_CLK_DISABLE_W<3> {
        EN_DFI_DRAM_CLK_DISABLE_W::new(self)
    }
    ///Bit 5 - SELFREF_SW
    #[inline(always)]
    #[must_use]
    pub fn selfref_sw(&mut self) -> SELFREF_SW_W<5> {
        SELFREF_SW_W::new(self)
    }
    ///Bit 7 - DIS_CAM_DRAIN_SELFREF
    #[inline(always)]
    #[must_use]
    pub fn dis_cam_drain_selfref(&mut self) -> DIS_CAM_DRAIN_SELFREF_W<7> {
        DIS_CAM_DRAIN_SELFREF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL low power control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_pwrctl](index.html) module
pub struct DDRCTRL_PWRCTL_SPEC;
impl crate::RegisterSpec for DDRCTRL_PWRCTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_pwrctl::R](R) reader structure
impl crate::Readable for DDRCTRL_PWRCTL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_pwrctl::W](W) writer structure
impl crate::Writable for DDRCTRL_PWRCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_PWRCTL to value 0
impl crate::Resettable for DDRCTRL_PWRCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
