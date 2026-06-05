# Flutter Wallet Motion Design

## Goal

Safe Wallet improves its Flutter motion system so page changes, screen entrances, and list item reveals feel more polished while staying fast and restrained for a wallet application.

## Direction

The motion direction is "more tactile and smoother" rather than "more animated." Animations should help the interface feel layered and responsive without drawing attention away from balances, assets, transfers, and security actions.

GSAP is not a runtime dependency for this Flutter application. The implementation applies GSAP motion principles to Flutter: prefer transform and opacity, sequence related animations with timeline-like delays, keep shared motion tokens centralized, and respect reduced-motion preferences.

## Scope

- Update the shared `screen_motion` components and constants.
- Add semantic motion tokens for short, base, long, page, entrance, and list stagger timing.
- Enhance page transitions in `AnimatedPageStack` so the incoming page gets visual priority and the outgoing page exits faster.
- Enhance `MotionEntrance` so callers can apply layer-based delays for header, actions, and content sections.
- Tune `StaggeredListItem` for smoother first-item sequencing with a capped delay for long lists.
- Keep `MotionSwitcher` focused on local state changes, using fade and small slide transforms only.
- Apply new layer delays only at clear page structure points already using the motion layer or nearby wrappers.

Out of scope:

- Adding JavaScript GSAP to the Flutter app.
- Reworking layout, branding, wallet flows, or Rust/FFI behavior.
- Animating width, height, margin, padding, or other layout-heavy properties.
- Adding route-level animation infrastructure beyond the existing page stack.

## Architecture

The existing `apps/flutter_wallet/lib/src/core/ui/screen_motion` folder remains the motion boundary. It exposes small widgets that feature screens can opt into without owning animation details.

`constants.dart` becomes the single source for motion tokens:

- Shared durations for quick state changes, normal content motion, and longer page/entrance motion.
- Shared curves for entering, exiting, and emphasized content.
- Shared offsets for page motion, screen entrance, switcher motion, and list item motion.
- Shared stagger step and maximum stagger count.

`MotionEntrance` gains optional delay support. This gives screens a simple way to express visual order, such as brand/header first, form/content second, secondary actions last. The widget continues to short-circuit when `MediaQuery.disableAnimations` is enabled.

`AnimatedPageStack` keeps all page widgets mounted only as needed for the active and recently-active pages. Incoming pages use the primary entrance curve and offset. Outgoing pages use a shorter exit duration and smaller movement so the transition reads as a page handoff instead of two full entrances happening at once.

`StaggeredListItem` keeps its timer-based initial reveal, but the delay is tuned and capped through constants. The first visible items can sequence clearly, while long lists avoid delayed, sluggish tails.

## Motion Behavior

Page transitions:

- Incoming page fades in and moves from a small vertical offset to rest.
- Outgoing page fades out faster with less movement.
- Offstage, pointer, semantics, and ticker behavior continue to protect inactive pages.

Screen entrances:

- Content fades and translates from a small vertical offset.
- Optional delay lets callers create a timeline-like sequence.
- Delay is ignored when animations are disabled.

List reveals:

- Items use opacity and slide only.
- Delay is based on index, with a fixed cap so list length does not increase total reveal time indefinitely.
- Item motion stays subtle enough for dense wallet data.

Local switchers:

- `MotionSwitcher` keeps the current fade-plus-small-slide pattern.
- Curves and offsets use the shared tokens so it matches the rest of the system.

## Accessibility

All shared motion widgets continue to use `motionDisabled(context)`. When disabled, they render the child immediately with no delayed timer, no entrance state change, and no page animation.

The changes should not hide active content from assistive technologies. Existing `ExcludeSemantics`, `IgnorePointer`, `Offstage`, and `TickerMode` behavior in `AnimatedPageStack` remains in place.

## Testing

Flutter tests cover:

- Motion widgets render their children with animation enabled and disabled.
- `AnimatedPageStack` keeps the active child visible and inactive children non-interactive.
- `StaggeredListItem` reveals after the capped delay and renders immediately when animations are disabled.
- Existing app tests still pass after tuning shared motion constants.

Manual verification focuses on unlocking, switching wallet sections, empty-state/list transitions, and local state changes such as biometric availability or transfer preview updates.
