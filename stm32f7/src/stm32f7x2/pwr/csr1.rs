///Register `CSR1` reader
pub struct R(crate::R<CSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR1` writer
pub struct W(crate::W<CSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR1_SPEC>;
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
impl From<crate::W<CSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WUIF` reader - Wakeup internal flag
pub type WUIF_R = crate::BitReader<bool>;
///Field `SBF` reader - Standby flag
pub type SBF_R = crate::BitReader<bool>;
///Field `PVDO` reader - PVD output
pub type PVDO_R = crate::BitReader<bool>;
///Field `BRR` reader - Backup regulator ready
pub type BRR_R = crate::BitReader<bool>;
///Field `EIWUP` reader - Enable internal wakeup
pub type EIWUP_R = crate::BitReader<bool>;
///Field `EIWUP` writer - Enable internal wakeup
pub type EIWUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR1_SPEC, bool, O>;
///Field `BRE` reader - Backup regulator enable
pub type BRE_R = crate::BitReader<bool>;
///Field `BRE` writer - Backup regulator enable
pub type BRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR1_SPEC, bool, O>;
///Field `VOSRDY` reader - Regulator voltage scaling output selection ready bit
pub type VOSRDY_R = crate::BitReader<bool>;
///Field `VOSRDY` writer - Regulator voltage scaling output selection ready bit
pub type VOSRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR1_SPEC, bool, O>;
///Field `ODRDY` reader - Over-drive mode ready
pub type ODRDY_R = crate::BitReader<bool>;
///Field `ODRDY` writer - Over-drive mode ready
pub type ODRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR1_SPEC, bool, O>;
///Field `ODSWRDY` reader - Over-drive mode switching ready
pub type ODSWRDY_R = crate::BitReader<bool>;
///Field `ODSWRDY` writer - Over-drive mode switching ready
pub type ODSWRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR1_SPEC, bool, O>;
///Field `UDRDY` reader - Under-drive ready flag
pub type UDRDY_R = crate::FieldReader<u8, u8>;
///Field `UDRDY` writer - Under-drive ready flag
pub type UDRDY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR1_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - Wakeup internal flag
    #[inline(always)]
    pub fn wuif(&self) -> WUIF_R {
        WUIF_R::new((self.bits & 1) != 0)
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
    ///Bit 3 - Backup regulator ready
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Enable internal wakeup
    #[inline(always)]
    pub fn eiwup(&self) -> EIWUP_R {
        EIWUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Backup regulator enable
    #[inline(always)]
    pub fn bre(&self) -> BRE_R {
        BRE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - Regulator voltage scaling output selection ready bit
    #[inline(always)]
    pub fn vosrdy(&self) -> VOSRDY_R {
        VOSRDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Over-drive mode ready
    #[inline(always)]
    pub fn odrdy(&self) -> ODRDY_R {
        ODRDY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Over-drive mode switching ready
    #[inline(always)]
    pub fn odswrdy(&self) -> ODSWRDY_R {
        ODSWRDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Under-drive ready flag
    #[inline(always)]
    pub fn udrdy(&self) -> UDRDY_R {
        UDRDY_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    ///Bit 8 - Enable internal wakeup
    #[inline(always)]
    #[must_use]
    pub fn eiwup(&mut self) -> EIWUP_W<8> {
        EIWUP_W::new(self)
    }
    ///Bit 9 - Backup regulator enable
    #[inline(always)]
    #[must_use]
    pub fn bre(&mut self) -> BRE_W<9> {
        BRE_W::new(self)
    }
    ///Bit 14 - Regulator voltage scaling output selection ready bit
    #[inline(always)]
    #[must_use]
    pub fn vosrdy(&mut self) -> VOSRDY_W<14> {
        VOSRDY_W::new(self)
    }
    ///Bit 16 - Over-drive mode ready
    #[inline(always)]
    #[must_use]
    pub fn odrdy(&mut self) -> ODRDY_W<16> {
        ODRDY_W::new(self)
    }
    ///Bit 17 - Over-drive mode switching ready
    #[inline(always)]
    #[must_use]
    pub fn odswrdy(&mut self) -> ODSWRDY_W<17> {
        ODSWRDY_W::new(self)
    }
    ///Bits 18:19 - Under-drive ready flag
    #[inline(always)]
    #[must_use]
    pub fn udrdy(&mut self) -> UDRDY_W<18> {
        UDRDY_W::new(self)
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
///For information about available fields see [csr1](index.html) module
pub struct CSR1_SPEC;
impl crate::RegisterSpec for CSR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr1::R](R) reader structure
impl crate::Readable for CSR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr1::W](W) writer structure
impl crate::Writable for CSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR1 to value 0
impl crate::Resettable for CSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
