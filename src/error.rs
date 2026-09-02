#[derive(Debug, thiserror::Error)]
{% if kind == "lib" %}#[non_exhaustive]
{% endif %}pub{% unless kind == "lib" %}(crate){% endunless %} enum AppError {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}

pub{% unless kind == "lib" %}(crate){% endunless %} type Result<T> = core::result::Result<T, AppError>;
