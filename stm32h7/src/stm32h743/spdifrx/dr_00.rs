///Register `DR_00` reader
pub struct R(crate::R<DR_00_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_00_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_00_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_00_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DR` reader - Parity Error bit
pub type DR_R = crate::FieldReader<u32, u32>;
///Field `PE` reader - Parity Error bit
pub type PE_R = crate::BitReader<bool>;
///Field `V` reader - Validity bit
pub type V_R = crate::BitReader<bool>;
///Field `U` reader - User bit
pub type U_R = crate::BitReader<bool>;
///Field `C` reader - Channel Status bit
pub type C_R = crate::BitReader<bool>;
///Field `PT` reader - Preamble Type
pub type PT_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:23 - Parity Error bit
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 24 - Parity Error bit
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Validity bit
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - User bit
    #[inline(always)]
    pub fn u(&self) -> U_R {
        U_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Channel Status bit
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:29 - Preamble Type
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
///Data input register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr_00](index.html) module
pub struct DR_00_SPEC;
impl crate::RegisterSpec for DR_00_SPEC {
    type Ux = u32;
}
///`read()` method returns [dr_00::R](R) reader structure
impl crate::Readable for DR_00_SPEC {
    type Reader = R;
}
///`reset()` method sets DR_00 to value 0
impl crate::Resettable for DR_00_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
