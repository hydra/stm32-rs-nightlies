///Register `DDRCTRL_ADDRMAP4` reader
pub struct R(crate::R<DDRCTRL_ADDRMAP4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ADDRMAP4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ADDRMAP4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ADDRMAP4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_ADDRMAP4` writer
pub struct W(crate::W<DDRCTRL_ADDRMAP4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ADDRMAP4_SPEC>;
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
impl From<crate::W<DDRCTRL_ADDRMAP4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ADDRMAP4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDRMAP_COL_B10` reader - ADDRMAP_COL_B10
pub type ADDRMAP_COL_B10_R = crate::FieldReader<u8, u8>;
///Field `ADDRMAP_COL_B10` writer - ADDRMAP_COL_B10
pub type ADDRMAP_COL_B10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP4_SPEC, u8, u8, 5, O>;
///Field `ADDRMAP_COL_B11` reader - ADDRMAP_COL_B11
pub type ADDRMAP_COL_B11_R = crate::FieldReader<u8, u8>;
///Field `ADDRMAP_COL_B11` writer - ADDRMAP_COL_B11
pub type ADDRMAP_COL_B11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP4_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - ADDRMAP_COL_B10
    #[inline(always)]
    pub fn addrmap_col_b10(&self) -> ADDRMAP_COL_B10_R {
        ADDRMAP_COL_B10_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - ADDRMAP_COL_B11
    #[inline(always)]
    pub fn addrmap_col_b11(&self) -> ADDRMAP_COL_B11_R {
        ADDRMAP_COL_B11_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - ADDRMAP_COL_B10
    #[inline(always)]
    #[must_use]
    pub fn addrmap_col_b10(&mut self) -> ADDRMAP_COL_B10_W<0> {
        ADDRMAP_COL_B10_W::new(self)
    }
    ///Bits 8:12 - ADDRMAP_COL_B11
    #[inline(always)]
    #[must_use]
    pub fn addrmap_col_b11(&mut self) -> ADDRMAP_COL_B11_W<8> {
        ADDRMAP_COL_B11_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL address map register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_addrmap4](index.html) module
pub struct DDRCTRL_ADDRMAP4_SPEC;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP4_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_addrmap4::R](R) reader structure
impl crate::Readable for DDRCTRL_ADDRMAP4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_addrmap4::W](W) writer structure
impl crate::Writable for DDRCTRL_ADDRMAP4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_ADDRMAP4 to value 0
impl crate::Resettable for DDRCTRL_ADDRMAP4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
