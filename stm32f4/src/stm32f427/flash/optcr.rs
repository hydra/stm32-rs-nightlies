///Register `OPTCR` reader
pub struct R(crate::R<OPTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPTCR` writer
pub struct W(crate::W<OPTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTCR_SPEC>;
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
impl From<crate::W<OPTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OPTLOCK` reader - Option lock
pub type OPTLOCK_R = crate::BitReader<bool>;
///Field `OPTLOCK` writer - Option lock
pub type OPTLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
///Field `OPTSTRT` reader - Option start
pub type OPTSTRT_R = crate::BitReader<bool>;
///Field `OPTSTRT` writer - Option start
pub type OPTSTRT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
///Field `BOR_LEV` reader - BOR reset Level
pub type BOR_LEV_R = crate::FieldReader<u8, u8>;
///Field `BOR_LEV` writer - BOR reset Level
pub type BOR_LEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTCR_SPEC, u8, u8, 2, O>;
///Field `WDG_SW` reader - WDG_SW User option bytes
pub type WDG_SW_R = crate::BitReader<bool>;
///Field `WDG_SW` writer - WDG_SW User option bytes
pub type WDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
///Field `nRST_STOP` reader - nRST_STOP User option bytes
pub type N_RST_STOP_R = crate::BitReader<bool>;
///Field `nRST_STOP` writer - nRST_STOP User option bytes
pub type N_RST_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
///Field `nRST_STDBY` reader - nRST_STDBY User option bytes
pub type N_RST_STDBY_R = crate::BitReader<bool>;
///Field `nRST_STDBY` writer - nRST_STDBY User option bytes
pub type N_RST_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTCR_SPEC, bool, O>;
///Field `RDP` reader - Read protect
pub type RDP_R = crate::FieldReader<u8, u8>;
///Field `RDP` writer - Read protect
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTCR_SPEC, u8, u8, 8, O>;
///Field `nWRP` reader - Not write protect
pub type N_WRP_R = crate::FieldReader<u16, u16>;
///Field `nWRP` writer - Not write protect
pub type N_WRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTCR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bit 0 - Option lock
    #[inline(always)]
    pub fn optlock(&self) -> OPTLOCK_R {
        OPTLOCK_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Option start
    #[inline(always)]
    pub fn optstrt(&self) -> OPTSTRT_R {
        OPTSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 5 - WDG_SW User option bytes
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - nRST_STOP User option bytes
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - nRST_STDBY User option bytes
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - Read protect
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:27 - Not write protect
    #[inline(always)]
    pub fn n_wrp(&self) -> N_WRP_R {
        N_WRP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bit 0 - Option lock
    #[inline(always)]
    #[must_use]
    pub fn optlock(&mut self) -> OPTLOCK_W<0> {
        OPTLOCK_W::new(self)
    }
    ///Bit 1 - Option start
    #[inline(always)]
    #[must_use]
    pub fn optstrt(&mut self) -> OPTSTRT_W<1> {
        OPTSTRT_W::new(self)
    }
    ///Bits 2:3 - BOR reset Level
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<2> {
        BOR_LEV_W::new(self)
    }
    ///Bit 5 - WDG_SW User option bytes
    #[inline(always)]
    #[must_use]
    pub fn wdg_sw(&mut self) -> WDG_SW_W<5> {
        WDG_SW_W::new(self)
    }
    ///Bit 6 - nRST_STOP User option bytes
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<6> {
        N_RST_STOP_W::new(self)
    }
    ///Bit 7 - nRST_STDBY User option bytes
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<7> {
        N_RST_STDBY_W::new(self)
    }
    ///Bits 8:15 - Read protect
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<8> {
        RDP_W::new(self)
    }
    ///Bits 16:27 - Not write protect
    #[inline(always)]
    #[must_use]
    pub fn n_wrp(&mut self) -> N_WRP_W<16> {
        N_WRP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash option control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optcr](index.html) module
pub struct OPTCR_SPEC;
impl crate::RegisterSpec for OPTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [optcr::R](R) reader structure
impl crate::Readable for OPTCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [optcr::W](W) writer structure
impl crate::Writable for OPTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPTCR to value 0x0fff_aaed
impl crate::Resettable for OPTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_aaed;
}
