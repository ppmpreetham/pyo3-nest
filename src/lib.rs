// pyo3-nest
use pyo3::prelude::*;

/// adds multiple pyclasses to module
#[macro_export]
macro_rules! add_classes {
    ($module:expr, $($class:ty),+ $(,)?) => {
        $(
            $module.add_class::<$class>()?;
        )+
    };
}

/// adds multiple pyfunctions to module
#[macro_export]
macro_rules! add_functions {
    ($module:expr, $($func:ident),+ $(,)?) => {
        $(
            $module.add_function(wrap_pyfunction!($func, $module)?)?;
        )+
    };
}

/// Creates nested submodules with beautiful, clean syntax.
///
/// # Examples
///
/// ```rust
/// submodule!(m, "routing", add_classes!(Router, Route));
///
/// submodule!(m, "responses",
///     add_classes!(JSONResponse, HTMLResponse) &&
///     add_functions!(create_response)
/// );
///
/// submodule!(m, "responses.types", add_classes!(TextType, ImageType, AlienType));
/// ```
#[macro_export]
macro_rules! submodule {
    (@resolve $parent:expr, $path:literal) => {{
        let py = $parent.py();
        let parts: Vec<&str> = $path.split('.')
            .filter(|s| !s.is_empty())
            .collect();

        let mut current: Bound<'_, PyModule> = $parent.clone();
        let sys_modules = py.import("sys")?.getattr("modules")?;

        for &part in &parts {
            let parent_name = current.name()?.to_string();
            let full_name = format!("{}.{}", current.name()?, part);

            current = if let Ok(existing) = current.getattr(part) {
                existing.cast_into::<PyModule>()?
            } else {
                let new_mod = PyModule::new(py, part)?;

                new_mod.setattr("__name__", &full_name)?;
                new_mod.setattr("__package__", &parent_name)?;

                current.add_submodule(&new_mod)?;
                sys_modules.set_item(&full_name, &new_mod)?;

                new_mod
            };
        }
        current
    }};

    // only classes
    ($parent:expr, $path:literal, add_classes!($($class:ty),+ $(,)?)) => {{
        let leaf = submodule!(@resolve $parent, $path);
        add_classes!(leaf, $($class),+);
    }};

    // only functions
    ($parent:expr, $path:literal, add_functions!($($func:ident),+ $(,)?)) => {{
        let leaf = submodule!(@resolve $parent, $path);
        add_functions!(leaf, $($func),+);
    }};

    // classes && functions
    ($parent:expr, $path:literal,
     add_classes!($($class:ty),+ $(,)?) && add_functions!($($func:ident),+ $(,)?)) => {{
        let leaf = submodule!(@resolve $parent, $path);
        add_classes!(leaf, $($class),+);
        add_functions!(leaf, $($func),+);
    }};

    // functions && classes
    ($parent:expr, $path:literal,
     add_functions!($($func:ident),+ $(,)?) && add_classes!($($class:ty),+ $(,)?)) => {{
        let leaf = submodule!(@resolve $parent, $path);
        add_functions!(leaf, $($func),+);
        add_classes!(leaf, $($class),+);
    }};
}
