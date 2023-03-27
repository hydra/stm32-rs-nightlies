///Register `WWDG_VERR` reader
pub struct R(crate::R<WWDG_VERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WWDG_VERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WWDG_VERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WWDG_VERR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MINREV` reader - MINREV
pub type MINREV_R = crate::FieldReader<u8, u8>;
///Field `MAJREV` reader - MAJREV
pub type MAJREV_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - MINREV
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - MAJREV
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///WWDG version register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wwdg_verr](index.html) module
pub struct WWDG_VERR_SPEC;
impl crate::RegisterSpec for WWDG_VERR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wwdg_verr::R](R) reader structure
impl crate::Readable for WWDG_VERR_SPEC {
    type Reader = R;
}
///`reset()` method sets WWDG_VERR to value 0x21
impl crate::Resettable for WWDG_VERR_SPEC {
    const RESET_VALUE: Self::Ux = 0x21;
}
