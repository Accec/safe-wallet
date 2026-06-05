import Cocoa
import FlutterMacOS

@main
class AppDelegate: FlutterAppDelegate {
  private var fallbackWindow: MainFlutterWindow?

  override func applicationDidFinishLaunching(_ notification: Notification) {
    let hasVisibleWindow = NSApp.windows.contains {
      $0.isVisible && $0.frame.width > 0 && $0.frame.height > 0
    }
    if hasVisibleWindow {
      return
    }

    let window = MainFlutterWindow(
      contentRect: MainFlutterWindow.defaultFrame,
      styleMask: [.titled, .closable, .miniaturizable, .resizable],
      backing: .buffered,
      defer: false)
    window.title = Bundle.main.object(forInfoDictionaryKey: "CFBundleName") as? String
      ?? "Safe Wallet"
    window.center()
    window.configureFlutterWindow()
    self.fallbackWindow = window
  }

  override func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
    return true
  }

  override func applicationSupportsSecureRestorableState(_ app: NSApplication) -> Bool {
    return false
  }
}
