///Register `FMC_BTR3` reader
pub struct R(crate::R<FMC_BTR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BTR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BTR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BTR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FMC_BTR3` writer
pub struct W(crate::W<FMC_BTR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_BTR3_SPEC>;
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
impl From<crate::W<FMC_BTR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_BTR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDSET` reader - ADDSET
pub type ADDSET_R = crate::FieldReader<u8, u8>;
///Field `ADDSET` writer - ADDSET
pub type ADDSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BTR3_SPEC, u8, u8, 4, O>;
///Field `ADDHLD` reader - ADDHLD
pub type ADDHLD_R = crate::FieldReader<u8, u8>;
///Field `ADDHLD` writer - ADDHLD
pub type ADDHLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BTR3_SPEC, u8, u8, 4, O>;
///Field `DATAST` reader - DATAST
pub type DATAST_R = crate::FieldReader<u8, u8>;
///Field `DATAST` writer - DATAST
pub type DATAST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BTR3_SPEC, u8, u8, 8, O>;
///Field `BUSTURN` reader - BUSTURN
pub type BUSTURN_R = crate::FieldReader<u8, u8>;
///Field `BUSTURN` writer - BUSTURN
pub type BUSTURN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BTR3_SPEC, u8, u8, 4, O>;
///Field `CLKDIV` reader - CLKDIV
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
///Field `CLKDIV` writer - CLKDIV
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BTR3_SPEC, u8, u8, 4, O>;
///Field `DATLAT` reader - DATLAT
pub type DATLAT_R = crate::FieldReader<u8, u8>;
///Field `DATLAT` writer - DATLAT
pub type DATLAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BTR3_SPEC, u8, u8, 4, O>;
///Field `ACCMOD` reader - ACCMOD
pub type ACCMOD_R = crate::FieldReader<u8, u8>;
///Field `ACCMOD` writer - ACCMOD
pub type ACCMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BTR3_SPEC, u8, u8, 2, O>;
///Field `DATAHLD` reader - DATAHLD
pub type DATAHLD_R = crate::FieldReader<u8, u8>;
///Field `DATAHLD` writer - DATAHLD
pub type DATAHLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMC_BTR3_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:3 - ADDSET
    #[inline(always)]
    pub fn addset(&self) -> ADDSET_R {
        ADDSET_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - ADDHLD
    #[inline(always)]
    pub fn addhld(&self) -> ADDHLD_R {
        ADDHLD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:15 - DATAST
    #[inline(always)]
    pub fn datast(&self) -> DATAST_R {
        DATAST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - BUSTURN
    #[inline(always)]
    pub fn busturn(&self) -> BUSTURN_R {
        BUSTURN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - CLKDIV
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - DATLAT
    #[inline(always)]
    pub fn datlat(&self) -> DATLAT_R {
        DATLAT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:29 - ACCMOD
    #[inline(always)]
    pub fn accmod(&self) -> ACCMOD_R {
        ACCMOD_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - DATAHLD
    #[inline(always)]
    pub fn datahld(&self) -> DATAHLD_R {
        DATAHLD_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:3 - ADDSET
    #[inline(always)]
    #[must_use]
    pub fn addset(&mut self) -> ADDSET_W<0> {
        ADDSET_W::new(self)
    }
    ///Bits 4:7 - ADDHLD
    #[inline(always)]
    #[must_use]
    pub fn addhld(&mut self) -> ADDHLD_W<4> {
        ADDHLD_W::new(self)
    }
    ///Bits 8:15 - DATAST
    #[inline(always)]
    #[must_use]
    pub fn datast(&mut self) -> DATAST_W<8> {
        DATAST_W::new(self)
    }
    ///Bits 16:19 - BUSTURN
    #[inline(always)]
    #[must_use]
    pub fn busturn(&mut self) -> BUSTURN_W<16> {
        BUSTURN_W::new(self)
    }
    ///Bits 20:23 - CLKDIV
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<20> {
        CLKDIV_W::new(self)
    }
    ///Bits 24:27 - DATLAT
    #[inline(always)]
    #[must_use]
    pub fn datlat(&mut self) -> DATLAT_W<24> {
        DATLAT_W::new(self)
    }
    ///Bits 28:29 - ACCMOD
    #[inline(always)]
    #[must_use]
    pub fn accmod(&mut self) -> ACCMOD_W<28> {
        ACCMOD_W::new(self)
    }
    ///Bits 30:31 - DATAHLD
    #[inline(always)]
    #[must_use]
    pub fn datahld(&mut self) -> DATAHLD_W<30> {
        DATAHLD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register contains the control information of each memory bank, used for SRAMs, PSRAM and NOR Flash memories.If the EXTMOD bit is set in the FMC_BCRx register, then this register is partitioned for write and read access, that is, two registers are available: one to configure read accesses (this register) and one to configure write accesses (FMC_BWTRx registers).
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fmc_btr3](index.html) module
pub struct FMC_BTR3_SPEC;
impl crate::RegisterSpec for FMC_BTR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [fmc_btr3::R](R) reader structure
impl crate::Readable for FMC_BTR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fmc_btr3::W](W) writer structure
impl crate::Writable for FMC_BTR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FMC_BTR3 to value 0x0fff_ffff
impl crate::Resettable for FMC_BTR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}
