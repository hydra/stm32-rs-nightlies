///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR` writer
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WUF` reader - Wakeup flag
pub type WUF_R = crate::BitReader<bool>;
///Field `SBF` reader - Standby flag
pub type SBF_R = crate::BitReader<bool>;
///Field `PVDO` reader - PVD output
pub type PVDO_R = crate::BitReader<bool>;
///Field `VREFINTRDY` reader - VREFINT reference voltage ready
pub type VREFINTRDY_R = crate::BitReader<bool>;
///Field `EWUP1` reader - Enable WKUP pin 1
pub type EWUP1_R = crate::BitReader<bool>;
///Field `EWUP1` writer - Enable WKUP pin 1
pub type EWUP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `EWUP2` reader - Enable WKUP pin 2
pub type EWUP2_R = crate::BitReader<bool>;
///Field `EWUP2` writer - Enable WKUP pin 2
pub type EWUP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `EWUP3` reader - Enable WKUP pin 3
pub type EWUP3_R = crate::BitReader<bool>;
///Field `EWUP3` writer - Enable WKUP pin 3
pub type EWUP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `EWUP4` reader - Enable WKUP pin 4
pub type EWUP4_R = crate::BitReader<bool>;
///Field `EWUP4` writer - Enable WKUP pin 4
pub type EWUP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `EWUP5` reader - Enable WKUP pin 5
pub type EWUP5_R = crate::BitReader<bool>;
///Field `EWUP5` writer - Enable WKUP pin 5
pub type EWUP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `EWUP6` reader - Enable WKUP pin 6
pub type EWUP6_R = crate::BitReader<bool>;
///Field `EWUP6` writer - Enable WKUP pin 6
pub type EWUP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `EWUP7` reader - Enable WKUP pin 7
pub type EWUP7_R = crate::BitReader<bool>;
///Field `EWUP7` writer - Enable WKUP pin 7
pub type EWUP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `EWUP8` reader - Enable WKUP pin 8
pub type EWUP8_R = crate::BitReader<bool>;
///Field `EWUP8` writer - Enable WKUP pin 8
pub type EWUP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Wakeup flag
    #[inline(always)]
    pub fn wuf(&self) -> WUF_R {
        WUF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Standby flag
    #[inline(always)]
    pub fn sbf(&self) -> SBF_R {
        SBF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PVD output
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VREFINT reference voltage ready
    #[inline(always)]
    pub fn vrefintrdy(&self) -> VREFINTRDY_R {
        VREFINTRDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Enable WKUP pin 1
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Enable WKUP pin 2
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Enable WKUP pin 3
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Enable WKUP pin 4
    #[inline(always)]
    pub fn ewup4(&self) -> EWUP4_R {
        EWUP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Enable WKUP pin 5
    #[inline(always)]
    pub fn ewup5(&self) -> EWUP5_R {
        EWUP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Enable WKUP pin 6
    #[inline(always)]
    pub fn ewup6(&self) -> EWUP6_R {
        EWUP6_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Enable WKUP pin 7
    #[inline(always)]
    pub fn ewup7(&self) -> EWUP7_R {
        EWUP7_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Enable WKUP pin 8
    #[inline(always)]
    pub fn ewup8(&self) -> EWUP8_R {
        EWUP8_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 8 - Enable WKUP pin 1
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<8> {
        EWUP1_W::new(self)
    }
    ///Bit 9 - Enable WKUP pin 2
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<9> {
        EWUP2_W::new(self)
    }
    ///Bit 10 - Enable WKUP pin 3
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<10> {
        EWUP3_W::new(self)
    }
    ///Bit 11 - Enable WKUP pin 4
    #[inline(always)]
    #[must_use]
    pub fn ewup4(&mut self) -> EWUP4_W<11> {
        EWUP4_W::new(self)
    }
    ///Bit 12 - Enable WKUP pin 5
    #[inline(always)]
    #[must_use]
    pub fn ewup5(&mut self) -> EWUP5_W<12> {
        EWUP5_W::new(self)
    }
    ///Bit 13 - Enable WKUP pin 6
    #[inline(always)]
    #[must_use]
    pub fn ewup6(&mut self) -> EWUP6_W<13> {
        EWUP6_W::new(self)
    }
    ///Bit 14 - Enable WKUP pin 7
    #[inline(always)]
    #[must_use]
    pub fn ewup7(&mut self) -> EWUP7_W<14> {
        EWUP7_W::new(self)
    }
    ///Bit 15 - Enable WKUP pin 8
    #[inline(always)]
    #[must_use]
    pub fn ewup8(&mut self) -> EWUP8_W<15> {
        EWUP8_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///power control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr::W](W) writer structure
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
