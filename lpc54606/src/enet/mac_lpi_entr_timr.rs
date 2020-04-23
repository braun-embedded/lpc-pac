#[doc = "Reader of register MAC_LPI_ENTR_TIMR"]
pub type R = crate::R<u32, super::MAC_LPI_ENTR_TIMR>;
#[doc = "Writer for register MAC_LPI_ENTR_TIMR"]
pub type W = crate::W<u32, super::MAC_LPI_ENTR_TIMR>;
#[doc = "Register MAC_LPI_ENTR_TIMR `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_LPI_ENTR_TIMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPIET`"]
pub type LPIET_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LPIET`"]
pub struct LPIET_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 3)) | (((value as u32) & 0x0001_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:19 - LPI Entry Timer This field specifies the time in microseconds the MAC will wait to enter LPI mode, after it has transmitted all the frames."]
    #[inline(always)]
    pub fn lpiet(&self) -> LPIET_R {
        LPIET_R::new(((self.bits >> 3) & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:19 - LPI Entry Timer This field specifies the time in microseconds the MAC will wait to enter LPI mode, after it has transmitted all the frames."]
    #[inline(always)]
    pub fn lpiet(&mut self) -> LPIET_W {
        LPIET_W { w: self }
    }
}
