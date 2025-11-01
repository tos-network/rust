use crate::spec::Target;
use crate::spec::base::tbf_base;

pub(crate) fn target() -> Target {
    tbf_base::tbf_target("v1")
}
