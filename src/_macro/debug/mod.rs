#[macro_export]
macro_rules! function_path {
  () => {{
    concat!(module_path!(), "::", function_name!())
  }};
}

#[macro_export]
macro_rules! trace_var {
  ($var: expr) => {{
    #[cfg(debug_assertions)] {
      use log::*;
      trace!("{} = {:?}", stringify!($var), $var);
    }
  }};
}

#[macro_export]
macro_rules! debug_var {
  ($var: expr) => {{
    #[cfg(debug_assertions)] {
      use log::*;
      debug!("{} = {:?}", stringify!($var), $var);
    }
  }};
}

#[macro_export]
macro_rules! info_var {
  ($var: expr) => {{
    #[cfg(debug_assertions)] {
      use log::*;
      info!("{} = {:?}", stringify!($var), $var);
    }
  }};
}

#[macro_export]
macro_rules! trace_enter {
  () => {{
    #[cfg(debug_assertions)] {
      use log::*;
      trace!("[ENTER] {} @ line {}", function_name!(), line!());
    }
  }};
}

#[macro_export]
macro_rules! trace_exit {
  () => {{
    #[cfg(debug_assertions)] {
      use log::*;
      trace!("[EXIT] {} @ line {}", function_name!(), line!());
    }
  }};
}

#[macro_export]
macro_rules! trace_result {
  ($var: expr) => {{
    #[cfg(debug_assertions)] {
      use log::*;
      trace!(
        "[EXIT] {} @ line {} with {}: {:?}",
        function_name!(),
        line!(),
        stringify!($var),
        $var
      );
    }
  }};
}
