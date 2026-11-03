#[cfg(not(target_os = "macos"))]
use penrose::x11rb::RustConn;
use penrose::{
    Result,
    builtin::{
        actions::{exit, key_handler, log_current_state, modify_with, send_layout_message, spawn},
        layout::{
            MainAndStack, Monocle,
            messages::{ExpandMain, IncMain, ShrinkMain},
            transformers::{Gaps, ReserveTop},
        },
    },
    core::{
        Config, WindowManager,
        bindings::{KeyEventHandler, parse_keybindings_with_xmodmap},
        layout::LayoutStack,
    },
    extensions::hooks::add_ewmh_hooks,
    map, stack,
    util::spawn_with_args,
};
#[cfg(not(target_os = "macos"))]
use penrose_ui::{bar::Position, core::TextStyle, status_bar};
use std::collections::HashMap;
use tracing_subscriber::{self, prelude::*};
#[cfg(not(target_os = "macos"))]
use x11::xlib::{Display, XErrorEvent, XSetErrorHandler};

const FONT: &str = "JetBrainsMono Nerd Font:size=10";
const BLACK: u32 = 0x282828ff;
const WHITE: u32 = 0xebdbb2ff;
const GREY: u32 = 0x3c3836ff;
const BLUE: u32 = 0x458588ff;

const MAX_MAIN: u32 = 1;
const RATIO: f32 = 0.6;
const RATIO_STEP: f32 = 0.1;
const OUTER_PX: u32 = 5;
const INNER_PX: u32 = 5;
const BAR_HEIGHT_PX: u32 = 18;

const BROWSER: &str = "librewolf";
const FILE_MANAGER: &str = "thunar";
const TERMINAL: &str = "st";

const VOLUME_UP: &str = "wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%+; pkill -RTMIN+10 dwmblocks";
const VOLUME_DOWN: &str = "wpctl set-volume @DEFAULT_AUDIO_SINK@ 5%-; pkill -RTMIN+10 dwmblocks";
const BRIGHTNESS_UP: &str = "brightnessctl set 5%+";
const BRIGHTNESS_DOWN: &str = "brightnessctl set 5%-";

#[cfg(not(target_os = "macos"))]
fn sh(command: &'static str) -> Box<dyn KeyEventHandler<RustConn>> {
    key_handler(move |_, _| spawn_with_args("sh", &["-c", command]))
}

#[cfg(not(target_os = "macos"))]
fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_owned();

        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-q" => modify_with(|cs| cs.kill_focused()),
        "M-Tab" => modify_with(|cs| cs.toggle_tag()),
        "M-bracketright" => modify_with(|cs| cs.next_screen()),
        "M-bracketleft" => modify_with(|cs| cs.previous_screen()),
        "M-grave" => modify_with(|cs| cs.next_layout()),
        "M-S-grave" => modify_with(|cs| cs.previous_layout()),
        "M-Up" => send_layout_message(|| IncMain(1)),
        "M-Down" => send_layout_message(|| IncMain(-1)),
        "M-Right" => send_layout_message(|| ExpandMain),
        "M-Left" => send_layout_message(|| ShrinkMain),
        "M-d" => spawn("dmenu_run"),
        "M-S-s" => log_current_state(),
        "M-Return" => spawn(TERMINAL),
        "M-w" => spawn(BROWSER),
        "M-r" => spawn(FILE_MANAGER),
        "M-A-Escape" => exit(),
        "M-BackSpace" => spawn("sysact"),
        "M-F3" => spawn("displayselect"),
        "XF86AudioRaiseVolume" => sh(VOLUME_UP),
        "XF86AudioLowerVolume" => sh(VOLUME_DOWN),
        "M-equal" => sh(VOLUME_UP),
        "M-minus" => sh(VOLUME_DOWN),
        "XF86MonBrightnessUp" => spawn(BRIGHTNESS_UP),
        "XF86MonBrightnessDown" => spawn(BRIGHTNESS_DOWN),
    };

    for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

#[cfg(not(target_os = "macos"))]
fn layouts() -> LayoutStack {
    stack!(
        MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
        MainAndStack::side_mirrored(MAX_MAIN, RATIO, RATIO_STEP),
        MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP),
        Monocle::boxed()
    )
    .map(|layout| ReserveTop::wrap(Gaps::wrap(layout, OUTER_PX, INNER_PX), BAR_HEIGHT_PX))
}

// The bar draws over its own Xlib connection, and Xlib's default error handler
// exits the program. On a busy X server the bar's first copy to its window can
// arrive before the window exists (BadDrawable on CopyArea). Like dwm, log X
// errors and carry on: the next redraw draws the bar.
#[cfg(not(target_os = "macos"))]
unsafe extern "C" fn log_x_error(_: *mut Display, e: *mut XErrorEvent) -> i32 {
    // SAFETY: Xlib passes a valid error event
    let e = unsafe { &*e };
    eprintln!(
        "ignoring X error {} for request {} on resource {:#x}",
        e.error_code, e.request_code, e.resourceid
    );
    0
}

#[cfg(not(target_os = "macos"))]
fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .finish()
        .init();

    // SAFETY: log_x_error only reads the event Xlib passes it
    unsafe { XSetErrorHandler(Some(log_x_error)) };

    let config = add_ewmh_hooks(Config {
        default_layouts: layouts(),
        ..Config::default()
    });

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let style = TextStyle {
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2, 2),
    };

    let bar = status_bar(BAR_HEIGHT_PX, FONT, 8, style, BLUE, GREY, Position::Top).unwrap();

    let wm = bar.add_to(WindowManager::new(
        config,
        key_bindings,
        HashMap::new(),
        conn,
    )?);

    wm.run()
}

#[cfg(target_os = "macos")]
fn main() -> Result<()> {
    panic!("not supported on OSX");
}


#[cfg(not(target_os = "macos"))]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let res = parse_keybindings_with_xmodmap(raw_key_bindings());

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
