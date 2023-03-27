///Register `RCC_MP_RSTSSETR` reader
pub struct R(crate::R<RCC_MP_RSTSSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_RSTSSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_RSTSSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_RSTSSETR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MP_RSTSSETR` writer
pub struct W(crate::W<RCC_MP_RSTSSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_RSTSSETR_SPEC>;
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
impl From<crate::W<RCC_MP_RSTSSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_RSTSSETR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PORRSTF` reader - PORRSTF
pub type PORRSTF_R = crate::BitReader<bool>;
///Field `PORRSTF` writer - PORRSTF
pub type PORRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `BORRSTF` reader - BORRSTF
pub type BORRSTF_R = crate::BitReader<bool>;
///Field `BORRSTF` writer - BORRSTF
pub type BORRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `PADRSTF` reader - PADRSTF
pub type PADRSTF_R = crate::BitReader<bool>;
///Field `PADRSTF` writer - PADRSTF
pub type PADRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `HCSSRSTF` reader - HCSSRSTF
pub type HCSSRSTF_R = crate::BitReader<bool>;
///Field `HCSSRSTF` writer - HCSSRSTF
pub type HCSSRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `VCORERSTF` reader - VCORERSTF
pub type VCORERSTF_R = crate::BitReader<bool>;
///Field `VCORERSTF` writer - VCORERSTF
pub type VCORERSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `MPSYSRSTF` reader - MPSYSRSTF
pub type MPSYSRSTF_R = crate::BitReader<bool>;
///Field `MPSYSRSTF` writer - MPSYSRSTF
pub type MPSYSRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `MCSYSRSTF` reader - MCSYSRSTF
pub type MCSYSRSTF_R = crate::BitReader<bool>;
///Field `MCSYSRSTF` writer - MCSYSRSTF
pub type MCSYSRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `IWDG1RSTF` reader - IWDG1RSTF
pub type IWDG1RSTF_R = crate::BitReader<bool>;
///Field `IWDG1RSTF` writer - IWDG1RSTF
pub type IWDG1RSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `IWDG2RSTF` reader - IWDG2RSTF
pub type IWDG2RSTF_R = crate::BitReader<bool>;
///Field `IWDG2RSTF` writer - IWDG2RSTF
pub type IWDG2RSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `STDBYRSTF` reader - STDBYRSTF
pub type STDBYRSTF_R = crate::BitReader<bool>;
///Field `STDBYRSTF` writer - STDBYRSTF
pub type STDBYRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `CSTDBYRSTF` reader - CSTDBYRSTF
pub type CSTDBYRSTF_R = crate::BitReader<bool>;
///Field `CSTDBYRSTF` writer - CSTDBYRSTF
pub type CSTDBYRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `MPUP0RSTF` reader - MPUP0RSTF
pub type MPUP0RSTF_R = crate::BitReader<bool>;
///Field `MPUP0RSTF` writer - MPUP0RSTF
pub type MPUP0RSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `MPUP1RSTF` reader - MPUP1RSTF
pub type MPUP1RSTF_R = crate::BitReader<bool>;
///Field `MPUP1RSTF` writer - MPUP1RSTF
pub type MPUP1RSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
///Field `SPARE` reader - SPARE
pub type SPARE_R = crate::BitReader<bool>;
///Field `SPARE` writer - SPARE
pub type SPARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_RSTSSETR_SPEC, bool, O>;
impl R {
    ///Bit 0 - PORRSTF
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BORRSTF
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PADRSTF
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HCSSRSTF
    #[inline(always)]
    pub fn hcssrstf(&self) -> HCSSRSTF_R {
        HCSSRSTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - VCORERSTF
    #[inline(always)]
    pub fn vcorerstf(&self) -> VCORERSTF_R {
        VCORERSTF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - MPSYSRSTF
    #[inline(always)]
    pub fn mpsysrstf(&self) -> MPSYSRSTF_R {
        MPSYSRSTF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - MCSYSRSTF
    #[inline(always)]
    pub fn mcsysrstf(&self) -> MCSYSRSTF_R {
        MCSYSRSTF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IWDG1RSTF
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> IWDG1RSTF_R {
        IWDG1RSTF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - IWDG2RSTF
    #[inline(always)]
    pub fn iwdg2rstf(&self) -> IWDG2RSTF_R {
        IWDG2RSTF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - STDBYRSTF
    #[inline(always)]
    pub fn stdbyrstf(&self) -> STDBYRSTF_R {
        STDBYRSTF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CSTDBYRSTF
    #[inline(always)]
    pub fn cstdbyrstf(&self) -> CSTDBYRSTF_R {
        CSTDBYRSTF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - MPUP0RSTF
    #[inline(always)]
    pub fn mpup0rstf(&self) -> MPUP0RSTF_R {
        MPUP0RSTF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - MPUP1RSTF
    #[inline(always)]
    pub fn mpup1rstf(&self) -> MPUP1RSTF_R {
        MPUP1RSTF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPARE
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PORRSTF
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<0> {
        PORRSTF_W::new(self)
    }
    ///Bit 1 - BORRSTF
    #[inline(always)]
    #[must_use]
    pub fn borrstf(&mut self) -> BORRSTF_W<1> {
        BORRSTF_W::new(self)
    }
    ///Bit 2 - PADRSTF
    #[inline(always)]
    #[must_use]
    pub fn padrstf(&mut self) -> PADRSTF_W<2> {
        PADRSTF_W::new(self)
    }
    ///Bit 3 - HCSSRSTF
    #[inline(always)]
    #[must_use]
    pub fn hcssrstf(&mut self) -> HCSSRSTF_W<3> {
        HCSSRSTF_W::new(self)
    }
    ///Bit 4 - VCORERSTF
    #[inline(always)]
    #[must_use]
    pub fn vcorerstf(&mut self) -> VCORERSTF_W<4> {
        VCORERSTF_W::new(self)
    }
    ///Bit 6 - MPSYSRSTF
    #[inline(always)]
    #[must_use]
    pub fn mpsysrstf(&mut self) -> MPSYSRSTF_W<6> {
        MPSYSRSTF_W::new(self)
    }
    ///Bit 7 - MCSYSRSTF
    #[inline(always)]
    #[must_use]
    pub fn mcsysrstf(&mut self) -> MCSYSRSTF_W<7> {
        MCSYSRSTF_W::new(self)
    }
    ///Bit 8 - IWDG1RSTF
    #[inline(always)]
    #[must_use]
    pub fn iwdg1rstf(&mut self) -> IWDG1RSTF_W<8> {
        IWDG1RSTF_W::new(self)
    }
    ///Bit 9 - IWDG2RSTF
    #[inline(always)]
    #[must_use]
    pub fn iwdg2rstf(&mut self) -> IWDG2RSTF_W<9> {
        IWDG2RSTF_W::new(self)
    }
    ///Bit 11 - STDBYRSTF
    #[inline(always)]
    #[must_use]
    pub fn stdbyrstf(&mut self) -> STDBYRSTF_W<11> {
        STDBYRSTF_W::new(self)
    }
    ///Bit 12 - CSTDBYRSTF
    #[inline(always)]
    #[must_use]
    pub fn cstdbyrstf(&mut self) -> CSTDBYRSTF_W<12> {
        CSTDBYRSTF_W::new(self)
    }
    ///Bit 13 - MPUP0RSTF
    #[inline(always)]
    #[must_use]
    pub fn mpup0rstf(&mut self) -> MPUP0RSTF_W<13> {
        MPUP0RSTF_W::new(self)
    }
    ///Bit 14 - MPUP1RSTF
    #[inline(always)]
    #[must_use]
    pub fn mpup1rstf(&mut self) -> MPUP1RSTF_W<14> {
        MPUP1RSTF_W::new(self)
    }
    ///Bit 15 - SPARE
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<15> {
        SPARE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mp_rstssetr](index.html) module
pub struct RCC_MP_RSTSSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_RSTSSETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mp_rstssetr::R](R) reader structure
impl crate::Readable for RCC_MP_RSTSSETR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mp_rstssetr::W](W) writer structure
impl crate::Writable for RCC_MP_RSTSSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MP_RSTSSETR to value 0
impl crate::Resettable for RCC_MP_RSTSSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
