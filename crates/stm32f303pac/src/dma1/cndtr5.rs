#[doc = "Register `CNDTR5` reader"]
pub type R = crate::R<Cndtr5Spec>;
#[doc = "Register `CNDTR5` writer"]
pub type W = crate::W<Cndtr5Spec>;
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub type NdtR = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NdtW<'_, Cndtr5Spec> {
        NdtW::new(self, 0)
    }
}
#[doc = "DMA channel 5 number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cndtr5Spec;
impl crate::RegisterSpec for Cndtr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cndtr5::R`](R) reader structure"]
impl crate::Readable for Cndtr5Spec {}
#[doc = "`write(|w| ..)` method takes [`cndtr5::W`](W) writer structure"]
impl crate::Writable for Cndtr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNDTR5 to value 0"]
impl crate::Resettable for Cndtr5Spec {}
