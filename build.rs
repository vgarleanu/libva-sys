use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "x11")] {
        const LIB_NAME: &str = "libva-x11";
    } else if #[cfg(feature = "wayland")] {
        const LIB_NAME: &str = "libva-wayland";
    } else if #[cfg(feature = "drm")] {
        const LIB_NAME: &str = "libva-drm";
    }
}

fn main() {
    pkg_config::Config::new()
        .probe(LIB_NAME)
        .expect("Failed to find libva.");

    #[cfg(feature = "x11")]
    pkg_config::probe_library("x11").expect("Failed to find lx11.");

    let mut src = vec!["vendor/libva-utils/common/va_display.c"];

    #[cfg(feature = "drm")]
    src.push("vendor/libva-utils/common/va_display_drm.c");

    #[cfg(feature = "wayland")]
    src.push("vendor/libva-utils/common/va_display_wayland.c");

    #[cfg(feature = "x11")]
    src.push("vendor/common/va_display_x11.c");

    let mut builder = cc::Build::new();
    let build = builder.files(src.iter())
        .include("vendor/libva-utils/common");

    #[cfg(feature = "drm")]
    let build = build.define("HAVE_VA_DRM", None);

    #[cfg(feature = "x11")]
    let build = build.define("HAVE_VA_X11", None);

    #[cfg(feature = "wayland")]
    let build = build.define("HAVE_VA_WAYLAND", None);

    build.compile(LIB_NAME);
}
