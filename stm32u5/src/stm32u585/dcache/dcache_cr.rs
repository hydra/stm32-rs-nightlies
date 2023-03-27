///Register `DCACHE_CR` reader
pub struct R(crate::R<DCACHE_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCACHE_CR` writer
pub struct W(crate::W<DCACHE_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_CR_SPEC>;
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
impl From<crate::W<DCACHE_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - EN
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - EN
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
///Field `CACHEINV` writer - CACHEINV
pub type CACHEINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
///Field `CACHECMD` reader - CACHECMD
pub type CACHECMD_R = crate::FieldReader<u8, u8>;
///Field `CACHECMD` writer - CACHECMD
pub type CACHECMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCACHE_CR_SPEC, u8, u8, 3, O>;
///Field `STARTCMD` writer - STARTCMD
pub type STARTCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
///Field `RHITMEN` reader - RHITMEN
pub type RHITMEN_R = crate::BitReader<bool>;
///Field `RHITMEN` writer - RHITMEN
pub type RHITMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
///Field `RMISSMEN` reader - RMISSMEN
pub type RMISSMEN_R = crate::BitReader<bool>;
///Field `RMISSMEN` writer - RMISSMEN
pub type RMISSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
///Field `RHITMRST` reader - RHITMRST
pub type RHITMRST_R = crate::BitReader<bool>;
///Field `RHITMRST` writer - RHITMRST
pub type RHITMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
///Field `RMISSMRST` reader - RMISSMRST
pub type RMISSMRST_R = crate::BitReader<bool>;
///Field `RMISSMRST` writer - RMISSMRST
pub type RMISSMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
///Field `WHITMEN` reader - WHITMEN
pub type WHITMEN_R = crate::BitReader<bool>;
///Field `WHITMEN` writer - WHITMEN
pub type WHITMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
///Field `WMISSMEN` reader - WMISSMEN
pub type WMISSMEN_R = crate::BitReader<bool>;
///Field `WMISSMEN` writer - WMISSMEN
pub type WMISSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
///Field `WHITMRST` reader - WHITMRST
pub type WHITMRST_R = crate::BitReader<bool>;
///Field `WHITMRST` writer - WHITMRST
pub type WHITMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
///Field `WMISSMRST` reader - WMISSMRST
pub type WMISSMRST_R = crate::BitReader<bool>;
///Field `WMISSMRST` writer - WMISSMRST
pub type WMISSMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
///Field `HBURST` reader - HBURST
pub type HBURST_R = crate::BitReader<bool>;
///Field `HBURST` writer - HBURST
pub type HBURST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCACHE_CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 8:10 - CACHECMD
    #[inline(always)]
    pub fn cachecmd(&self) -> CACHECMD_R {
        CACHECMD_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 16 - RHITMEN
    #[inline(always)]
    pub fn rhitmen(&self) -> RHITMEN_R {
        RHITMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RMISSMEN
    #[inline(always)]
    pub fn rmissmen(&self) -> RMISSMEN_R {
        RMISSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - RHITMRST
    #[inline(always)]
    pub fn rhitmrst(&self) -> RHITMRST_R {
        RHITMRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RMISSMRST
    #[inline(always)]
    pub fn rmissmrst(&self) -> RMISSMRST_R {
        RMISSMRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - WHITMEN
    #[inline(always)]
    pub fn whitmen(&self) -> WHITMEN_R {
        WHITMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - WMISSMEN
    #[inline(always)]
    pub fn wmissmen(&self) -> WMISSMEN_R {
        WMISSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - WHITMRST
    #[inline(always)]
    pub fn whitmrst(&self) -> WHITMRST_R {
        WHITMRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - WMISSMRST
    #[inline(always)]
    pub fn wmissmrst(&self) -> WMISSMRST_R {
        WMISSMRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - HBURST
    #[inline(always)]
    pub fn hburst(&self) -> HBURST_R {
        HBURST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bit 1 - CACHEINV
    #[inline(always)]
    #[must_use]
    pub fn cacheinv(&mut self) -> CACHEINV_W<1> {
        CACHEINV_W::new(self)
    }
    ///Bits 8:10 - CACHECMD
    #[inline(always)]
    #[must_use]
    pub fn cachecmd(&mut self) -> CACHECMD_W<8> {
        CACHECMD_W::new(self)
    }
    ///Bit 11 - STARTCMD
    #[inline(always)]
    #[must_use]
    pub fn startcmd(&mut self) -> STARTCMD_W<11> {
        STARTCMD_W::new(self)
    }
    ///Bit 16 - RHITMEN
    #[inline(always)]
    #[must_use]
    pub fn rhitmen(&mut self) -> RHITMEN_W<16> {
        RHITMEN_W::new(self)
    }
    ///Bit 17 - RMISSMEN
    #[inline(always)]
    #[must_use]
    pub fn rmissmen(&mut self) -> RMISSMEN_W<17> {
        RMISSMEN_W::new(self)
    }
    ///Bit 18 - RHITMRST
    #[inline(always)]
    #[must_use]
    pub fn rhitmrst(&mut self) -> RHITMRST_W<18> {
        RHITMRST_W::new(self)
    }
    ///Bit 19 - RMISSMRST
    #[inline(always)]
    #[must_use]
    pub fn rmissmrst(&mut self) -> RMISSMRST_W<19> {
        RMISSMRST_W::new(self)
    }
    ///Bit 20 - WHITMEN
    #[inline(always)]
    #[must_use]
    pub fn whitmen(&mut self) -> WHITMEN_W<20> {
        WHITMEN_W::new(self)
    }
    ///Bit 21 - WMISSMEN
    #[inline(always)]
    #[must_use]
    pub fn wmissmen(&mut self) -> WMISSMEN_W<21> {
        WMISSMEN_W::new(self)
    }
    ///Bit 22 - WHITMRST
    #[inline(always)]
    #[must_use]
    pub fn whitmrst(&mut self) -> WHITMRST_W<22> {
        WHITMRST_W::new(self)
    }
    ///Bit 23 - WMISSMRST
    #[inline(always)]
    #[must_use]
    pub fn wmissmrst(&mut self) -> WMISSMRST_W<23> {
        WMISSMRST_W::new(self)
    }
    ///Bit 31 - HBURST
    #[inline(always)]
    #[must_use]
    pub fn hburst(&mut self) -> HBURST_W<31> {
        HBURST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DCACHE control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcache_cr](index.html) module
pub struct DCACHE_CR_SPEC;
impl crate::RegisterSpec for DCACHE_CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcache_cr::R](R) reader structure
impl crate::Readable for DCACHE_CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dcache_cr::W](W) writer structure
impl crate::Writable for DCACHE_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCACHE_CR to value 0
impl crate::Resettable for DCACHE_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
