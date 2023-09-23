macro_rules! assert_some_or_log_err {
    ($property_name:ident, $self_name:ident) => {
        assert_some_or_log_err!($property_name, $self_name, true)
    };
    ($property_name:ident, $self_name:ident, $err_log:expr) => {
        let Some($property_name) = &$self_name.$property_name else {
            if $err_log {
                use godot::private::class_macros::godot_error;
                // #WEIRD rust (maybe?) : error log does not link back to a specific node. does indicate script origin though.
                godot_error!("{} is required!", stringify!($property_name));
            }
            return;
        };
    };
}

pub(crate) use assert_some_or_log_err;

#[cfg(test)]
mod tests {
    use super::*;
    use godot::prelude::real;

    struct SelfType {
        some_value: Option<real>,
    }
    #[test]
    fn test_exits_early_when_none() {
        let val = SelfType { some_value: None };
        assert_some_or_log_err!(some_value, val, false);
        let _ = some_value;
        assert!(false);
    }
    #[test]
    fn test_extracts_value_when_some() {
        let val = SelfType {
            some_value: Some(11.2),
        };
        assert_some_or_log_err!(some_value, val, false);
        assert_eq!(some_value, &11.2);
    }
}
