/// Components are data containers that can be attached to [`Entity`](crate::Entity).
/// # Examples
///
/// Components can take many forms: they are usually structs, but can also be of every other kind of data type, like enums or zero sized types.
/// The following examples show how components are laid out in code.
///
/// ```
/// #[derive(Component)]
/// struct Red;
/// 
/// #[derive(Component)]
/// struct ForceField {
/// name: String,
/// strength: u32,
///}
/// ```
pub trait Component: 'static + Send + Sync {}
