// Sample contract types
pub trait DomainContract {
    fn validate(&self) -> bool;
}