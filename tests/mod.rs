#[cfg(test)]
mod core {
    mod unit {
        mod link_test;
        mod state_test;
    }
    mod integration {
        mod core_test;
    }
}

#[cfg(test)]
mod net {
    mod unit {
        mod socket_test;
        mod group_test;
        mod route_test;
    }
    mod integration {
        mod net_test;
    }
}

#[cfg(test)]
mod guard {
    mod unit {
        mod auth_test;
        mod crypt_test;
        mod check_test;
    }
    mod integration {
        mod guard_test;
    }
}

#[cfg(test)]
mod store {
    mod unit {
        mod data_test;
        mod cache_test;
        mod file_test;
    }
    mod integration {
        mod store_test;
    }
}

#[cfg(test)]
mod log {
    mod unit {
        mod write_test;
        mod trace_test;
        mod alert_test;
    }
    mod integration {
        mod log_test;
    }
} 