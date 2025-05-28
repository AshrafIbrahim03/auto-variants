pub use variants_derive::Variants;
pub trait Variants<const N: usize>
where
    Self: Sized,
{
    const VARIANTS: [Self; N];
    /// Returns a static reference to all variants of this enum
    fn variants() -> &'static [Self];
}
