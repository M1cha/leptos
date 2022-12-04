//! Collection of typed events.

use std::borrow::Cow;
use wasm_bindgen::convert::FromWasmAbi;

use crate::IntoElement;

/// A trait for converting types into [web_sys events](web_sys).
pub trait IntoEvent {
  /// The [`web_sys`] event type, such as [`web_sys::MouseEvent`].
  type EventType: FromWasmAbi;

  /// The name of the event, such as `click` or `mouseover`.
  fn name(&self) -> Cow<'static, str>;

  /// Indicates if this event bubbles. For example, `click` bubbles,
  /// but `focus` does not.
  ///
  /// If this method returns true, then the event will be delegated globally,
  /// otherwise, event listeners will be directly attached to the element.
  fn bubbles(&self) -> bool {
    true
  }
}

/// Overrides the [`IntoEvent::bubbles`] method to always return
/// `false`, which forces the event to not be globally delegated.
pub struct Undelegated<Ev: IntoEvent>(Ev);

impl<Ev: IntoEvent> IntoEvent for Undelegated<Ev> {
  type EventType = Ev::EventType;

  fn name(&self) -> Cow<'static, str> {
    self.0.name()
  }

  fn bubbles(&self) -> bool {
    false
  }
}

macro_rules! generate_event_types {
  [$([$web_sys_event:ident, [$($event:ident),* $(,)?]]),* $(,)?] => {
    paste::paste! {
      $(
        $(
          #[doc = "The "]
          #[doc = stringify!([<$event:lower>])]
          #[doc = "event."]
          pub struct $event;

          impl IntoEvent for $event {
            type EventType = web_sys::MouseEvent;

            fn name(&self) -> Cow<'static, str> {
              concat!("on", stringify!([<$event:lower>])).into()
            }
          }
        )*
      )*
    }
  };
}

generate_event_types![
  // ClipboardEvent is unstable
  [Event, [Copy, Cut, Paste]],
  [
    CompositionEvent,
    [CompositionEnd, CompositionStart, CompositionUpdate]
  ],
  [KeyboardEvent, [KeyDown, Keypress, Keyup]],
  [FocusEvent, [Focus, FocusOut, FocusIn, Blur]],
  [FormEvent, [Change, Input, Invalid, Reset, Submit]],
  [
    MouseEvent,
    [
      Click,
      ContextMenu,
      DoubleClick,
      DblClick,
      Drag,
      DragEnd,
      DragEnter,
      DragExit,
      DragLeave,
      DragOver,
      DragStart,
      Drop,
      MouseDown,
      MouseEnter,
      MouseLeave,
      MouseMove,
      MouseOut,
      MouseOver,
      MouseUp,
    ]
  ],
  [ScrollEvent, [Scroll]],
  [
    PointerEvent,
    [
      PointerDown,
      PointerMove,
      PointerUp,
      PointerCancel,
      GotPointerCapture,
      LostPointerCapture,
      PointerEnter,
      PointerLeave,
      PointerOver,
      PointerOut,
    ]
  ],
  [SelectionEvent, [Select]],
  [TouchEvent, [TouchCancel, TouchEnd, TouchMove, TouchStart]],
  [WheelEvent, [Wheel]],
  [
    MediaEvent,
    [
      Abort,
      CanPlay,
      CanPlayThrough,
      DurationChange,
      Emptied,
      Encrypted,
      Ended,
      Error,
      LoadedData,
      LoadedMetadata,
      LoadStart,
      Pause,
      Play,
      Playing,
      Progress,
      RateChange,
      Seeked,
      Seeking,
      Stalled,
      Suspend,
      TimeUpdate,
      VolumeChange,
      Waiting,
    ]
  ],
  [
    AnimationEvent,
    [AnimationStart, AnimationEnd, AnimationIteration,]
  ],
  [TransitionEvent, [TransitionEnd]],
  [ToggleEvent, [Toggle]]
];
