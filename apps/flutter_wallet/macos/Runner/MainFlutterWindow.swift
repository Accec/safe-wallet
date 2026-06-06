import AVFoundation
import Cocoa
import CoreImage
import CoreVideo
import FlutterMacOS

class MainFlutterWindow: NSWindow {
  static let defaultFrame = NSRect(x: 0, y: 0, width: 1000, height: 720)
  private var didConfigureFlutter = false
  private var qrScannerChannel: FlutterMethodChannel?
  private var appUpdateChannel: FlutterMethodChannel?
  private var pendingScanResult: FlutterResult?

  override func awakeFromNib() {
    configureFlutterWindow()
    super.awakeFromNib()
  }

  func configureFlutterWindow() {
    let flutterViewController: FlutterViewController
    if let existingController = contentViewController as? FlutterViewController {
      flutterViewController = existingController
      ensureVisibleFrame()
    } else {
      flutterViewController = FlutterViewController()
      let windowFrame = usableFrame()
      isRestorable = false
      self.contentViewController = flutterViewController
      self.setFrame(windowFrame, display: true)
      self.setContentSize(windowFrame.size)
      center()
    }

    if !didConfigureFlutter {
      RegisterGeneratedPlugins(registry: flutterViewController)
      configureQrScannerChannel(flutterViewController)
      configureAppUpdateChannel(flutterViewController)
      didConfigureFlutter = true
    }

    makeKeyAndOrderFront(nil)
    orderFrontRegardless()
    NSApp.activate(ignoringOtherApps: true)
  }

  private func configureQrScannerChannel(_ flutterViewController: FlutterViewController) {
    let channel = FlutterMethodChannel(
      name: "app.localwallet/qr_scanner",
      binaryMessenger: flutterViewController.engine.binaryMessenger)
    channel.setMethodCallHandler { [weak self, weak flutterViewController] call, result in
      guard let self = self else {
        result(FlutterError(code: "unavailable", message: "QR scanner unavailable.", details: nil))
        return
      }

      switch call.method {
      case "scanQr":
        self.scanQr(presentingFrom: flutterViewController, result: result)
      case "pickQrImagePayload":
        self.pickQrImagePayload(result)
      default:
        result(FlutterMethodNotImplemented)
      }
    }
    qrScannerChannel = channel
  }

  private func configureAppUpdateChannel(_ flutterViewController: FlutterViewController) {
    let channel = FlutterMethodChannel(
      name: "app.localwallet/app_update",
      binaryMessenger: flutterViewController.engine.binaryMessenger)
    channel.setMethodCallHandler { call, result in
      guard call.method == "installMacosUpdate" else {
        result(FlutterMethodNotImplemented)
        return
      }

      guard
        let arguments = call.arguments as? [String: Any],
        let path = arguments["path"] as? String,
        !path.isEmpty
      else {
        result(FlutterError(
          code: "invalid_args",
          message: "Downloaded update path is required.",
          details: nil))
        return
      }

      installMacosUpdate(zipPath: path, result: result)
    }
    appUpdateChannel = channel
  }

  private func scanQr(
    presentingFrom flutterViewController: FlutterViewController?,
    result: @escaping FlutterResult
  ) {
    if pendingScanResult != nil {
      result(FlutterError(code: "busy", message: "QR scanner already open.", details: nil))
      return
    }
    guard let flutterViewController = flutterViewController else {
      result(FlutterError(code: "unavailable", message: "QR scanner unavailable.", details: nil))
      return
    }

    pendingScanResult = result
    let scanner = MacQrScannerViewController { [weak self] scanResult in
      self?.pendingScanResult?(scanResult)
      self?.pendingScanResult = nil
    }
    flutterViewController.presentAsSheet(scanner)
  }

  private func pickQrImagePayload(_ result: @escaping FlutterResult) {
    let panel = NSOpenPanel()
    panel.allowedFileTypes = ["public.image"]
    panel.allowsMultipleSelection = false
    panel.canChooseDirectories = false
    panel.canChooseFiles = true

    if panel.runModal() == .OK {
      if let url = panel.url, let payload = decodeQrImage(url) {
        result(payload)
      } else {
        result(FlutterError(code: "no_qr", message: "No QR code found in image.", details: nil))
      }
    } else {
      result(nil)
    }
  }

  private func ensureVisibleFrame() {
    if frame.width > 0 && frame.height > 0 {
      return
    }
    setFrame(MainFlutterWindow.defaultFrame, display: true)
    setContentSize(MainFlutterWindow.defaultFrame.size)
    center()
  }

  private func usableFrame() -> NSRect {
    if frame.width > 0 && frame.height > 0 {
      return frame
    }
    return MainFlutterWindow.defaultFrame
  }
}

private func decodeQrImage(_ url: URL) -> String? {
  guard let image = CIImage(contentsOf: url) else {
    return nil
  }
  let detector = CIDetector(
    ofType: CIDetectorTypeQRCode,
    context: nil,
    options: [CIDetectorAccuracy: CIDetectorAccuracyHigh])
  return detector?
    .features(in: image)
    .compactMap { ($0 as? CIQRCodeFeature)?.messageString }
    .first
}

private func installMacosUpdate(zipPath: String, result: @escaping FlutterResult) {
  let fileManager = FileManager.default
  let zipURL = URL(fileURLWithPath: zipPath)
  let currentAppURL = Bundle.main.bundleURL
  let installAppURL = macosInstallTargetURL(currentAppURL: currentAppURL)

  guard fileManager.fileExists(atPath: zipURL.path) else {
    result(FlutterError(
      code: "missing_update",
      message: "Downloaded update file was not found.",
      details: nil))
    return
  }

  guard zipURL.pathExtension.lowercased() == "zip" else {
    result(FlutterError(
      code: "invalid_update",
      message: "Downloaded update is not a macOS zip archive.",
      details: nil))
    return
  }

  guard currentAppURL.pathExtension.lowercased() == "app" else {
    result(FlutterError(
      code: "invalid_app_bundle",
      message: "Current app bundle could not be found.",
      details: nil))
    return
  }

  do {
    let scriptURL = try writeMacosUpdateScript()
    let process = Process()
    process.executableURL = URL(fileURLWithPath: "/bin/sh")
    process.arguments = [scriptURL.path, zipURL.path, currentAppURL.path, installAppURL.path, String(getpid())]
    try process.run()
    result(nil)
    DispatchQueue.main.asyncAfter(deadline: .now() + 0.2) {
      NSApp.terminate(nil)
    }
  } catch {
    result(FlutterError(
      code: "install_failed",
      message: "Could not prepare macOS update installer.",
      details: error.localizedDescription))
  }
}

private func macosInstallTargetURL(currentAppURL: URL) -> URL {
  let currentURL = currentAppURL.standardizedFileURL
  if currentURL.path.hasPrefix("/Applications/") {
    return currentURL
  }
  return URL(fileURLWithPath: "/Applications/Safe Wallet.app")
}

private func writeMacosUpdateScript() throws -> URL {
  let fileManager = FileManager.default
  let scriptDirectory = fileManager.temporaryDirectory
    .appendingPathComponent("safe-wallet-updates", isDirectory: true)
  try fileManager.createDirectory(
    at: scriptDirectory,
    withIntermediateDirectories: true)

  let scriptURL = scriptDirectory.appendingPathComponent(
    "install-\(UUID().uuidString).sh")
  let script = """
#!/bin/sh
set -u

SCRIPT_PATH="$0"
ZIP_PATH="$1"
CURRENT_APP_PATH="$2"
APP_PATH="$3"
APP_PID="$4"
LAST_ERROR_LOG="${TMPDIR:-/tmp}/safe-wallet-update-error.log"

fail() {
  if [ -n "${ERROR_LOG:-}" ] && [ -f "$ERROR_LOG" ]; then
    /bin/cp "$ERROR_LOG" "$LAST_ERROR_LOG" >/dev/null 2>&1 || true
  fi
  /usr/bin/osascript -e "display alert \\"Safe Wallet update failed\\" message \\"$1\\"" >/dev/null 2>&1 || true
  /bin/rm -f "$SCRIPT_PATH"
  exit 1
}

EXTRACT_DIR="$(/usr/bin/mktemp -d "${TMPDIR:-/tmp}/safe-wallet-update.XXXXXX")" || fail "Could not create a temporary update directory."
cleanup() {
  /bin/rm -rf "$EXTRACT_DIR"
  /bin/rm -f "$SCRIPT_PATH"
}
trap cleanup EXIT

while /bin/kill -0 "$APP_PID" >/dev/null 2>&1; do
  /bin/sleep 0.2
done

/usr/bin/ditto -x -k "$ZIP_PATH" "$EXTRACT_DIR" || fail "Could not extract the downloaded update."
NEW_APP="$(/usr/bin/find "$EXTRACT_DIR" -type d -name "*.app" -print | /usr/bin/head -n 1)"
if [ -z "$NEW_APP" ]; then
  fail "The downloaded update did not contain a macOS app."
fi

APP_PARENT="$(/usr/bin/dirname "$APP_PATH")"
APP_NAME="$(/usr/bin/basename "$APP_PATH")"
BACKUP_PATH="$APP_PARENT/.$APP_NAME.updating-backup"
ERROR_LOG="$EXTRACT_DIR/install-error.log"

restore_backup() {
  if [ -e "$BACKUP_PATH" ]; then
    /bin/rm -rf "$APP_PATH" >/dev/null 2>&1 || true
    /bin/mv "$BACKUP_PATH" "$APP_PATH" >/dev/null 2>&1 || true
  fi
}

replace_without_admin() {
  : > "$ERROR_LOG"
  /bin/rm -rf "$BACKUP_PATH" 2>>"$ERROR_LOG" || return 1
  if [ -e "$APP_PATH" ]; then
    /bin/mv "$APP_PATH" "$BACKUP_PATH" 2>>"$ERROR_LOG" || return 1
  fi
  if /bin/mv "$NEW_APP" "$APP_PATH" 2>>"$ERROR_LOG"; then
    /bin/rm -rf "$BACKUP_PATH" 2>>"$ERROR_LOG" || return 1
    /bin/rm -f "$ZIP_PATH" 2>>"$ERROR_LOG" || true
    return 0
  fi
  restore_backup
  return 1
}

install_with_admin() {
  ADMIN_SCRIPT="$EXTRACT_DIR/install-admin.applescript"
  /bin/cat > "$ADMIN_SCRIPT" <<'APPLESCRIPT'
on run argv
  set appPath to item 1 of argv
  set backupPath to item 2 of argv
  set newAppPath to item 3 of argv
  set zipPath to item 4 of argv
  set commandText to "set -e; /bin/rm -rf " & quoted form of backupPath & "; if [ -e " & quoted form of appPath & " ]; then /bin/mv " & quoted form of appPath & " " & quoted form of backupPath & "; fi; if /bin/mv " & quoted form of newAppPath & " " & quoted form of appPath & "; then /bin/rm -rf " & quoted form of backupPath & " || true; /bin/rm -f " & quoted form of zipPath & " || true; else if [ -e " & quoted form of backupPath & " ]; then /bin/rm -rf " & quoted form of appPath & "; /bin/mv " & quoted form of backupPath & " " & quoted form of appPath & "; fi; exit 1; fi"
  do shell script commandText with administrator privileges
end run
APPLESCRIPT
  /usr/bin/osascript "$ADMIN_SCRIPT" "$APP_PATH" "$BACKUP_PATH" "$NEW_APP" "$ZIP_PATH" 2>>"$ERROR_LOG"
}

if replace_without_admin || install_with_admin; then
  /bin/rm -rf "$BACKUP_PATH"
  /usr/bin/open "$APP_PATH"
else
  restore_backup
  fail "The app could not be replaced. Move Safe Wallet to /Applications and make sure you have permission to modify it, then try again. Details were saved to $LAST_ERROR_LOG."
fi
"""
  try script.write(to: scriptURL, atomically: true, encoding: .utf8)
  try fileManager.setAttributes(
    [.posixPermissions: 0o700],
    ofItemAtPath: scriptURL.path)
  return scriptURL
}

final class MacQrScannerViewController: NSViewController, AVCaptureVideoDataOutputSampleBufferDelegate {
  private let onResult: (Any?) -> Void
  private let session = AVCaptureSession()
  private let sessionQueue = DispatchQueue(label: "app.localwallet.qr.session")
  private let frameQueue = DispatchQueue(label: "app.localwallet.qr.frames")
  private let detector = CIDetector(
    ofType: CIDetectorTypeQRCode,
    context: nil,
    options: [CIDetectorAccuracy: CIDetectorAccuracyHigh])
  private var previewLayer: AVCaptureVideoPreviewLayer?
  private var completed = false
  private var lastFrameScan = Date.distantPast

  init(onResult: @escaping (Any?) -> Void) {
    self.onResult = onResult
    super.init(nibName: nil, bundle: nil)
  }

  required init?(coder: NSCoder) {
    return nil
  }

  override func loadView() {
    view = NSView()
    view.wantsLayer = true
  }

  override func viewDidLoad() {
    super.viewDidLoad()
    preferredContentSize = NSSize(width: 640, height: 480)

    let cancel = NSButton(title: "Cancel", target: self, action: #selector(cancelScan))
    cancel.bezelStyle = .rounded
    cancel.translatesAutoresizingMaskIntoConstraints = false
    view.addSubview(cancel)
    NSLayoutConstraint.activate([
      cancel.topAnchor.constraint(equalTo: view.topAnchor, constant: 16),
      cancel.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -16),
    ])

    startAfterPermission()
  }

  override func viewDidLayout() {
    super.viewDidLayout()
    previewLayer?.frame = view.bounds
  }

  private func startAfterPermission() {
    switch AVCaptureDevice.authorizationStatus(for: .video) {
    case .authorized:
      startSession()
    case .notDetermined:
      AVCaptureDevice.requestAccess(for: .video) { [weak self] granted in
        DispatchQueue.main.async {
          if granted {
            self?.startSession()
          } else {
            self?.completeWithError(
              code: "permission_denied",
              message: "Camera access was denied. Enable camera permission for Safe Wallet in System Settings.")
          }
        }
      }
    case .denied:
      completeWithError(
        code: "permission_denied",
        message: "Camera access was denied. Enable camera permission for Safe Wallet in System Settings.")
    case .restricted:
      completeWithError(
        code: "permission_restricted",
        message: "Camera access is restricted on this Mac.")
    default:
      completeWithError(code: "permission_unknown", message: "Camera permission is unavailable.")
    }
  }

  private func startSession() {
    guard
      let device = AVCaptureDevice.default(for: .video),
      let input = try? AVCaptureDeviceInput(device: device),
      session.canAddInput(input)
    else {
      completeWithError(code: "camera_unavailable", message: "No available camera was found.")
      return
    }
    session.addInput(input)

    let output = AVCaptureVideoDataOutput()
    output.alwaysDiscardsLateVideoFrames = true
    output.videoSettings = [
      kCVPixelBufferPixelFormatTypeKey as String: kCVPixelFormatType_32BGRA
    ]
    guard session.canAddOutput(output) else {
      completeWithError(code: "camera_unavailable", message: "Camera output is unavailable.")
      return
    }
    session.addOutput(output)
    output.setSampleBufferDelegate(self, queue: frameQueue)

    let layer = AVCaptureVideoPreviewLayer(session: session)
    layer.videoGravity = .resizeAspectFill
    layer.frame = view.bounds
    view.layer?.insertSublayer(layer, at: 0)
    previewLayer = layer

    sessionQueue.async { [session] in
      session.startRunning()
    }
  }

  func captureOutput(
    _ output: AVCaptureOutput,
    didOutput sampleBuffer: CMSampleBuffer,
    from connection: AVCaptureConnection
  ) {
    if completed {
      return
    }

    let now = Date()
    if now.timeIntervalSince(lastFrameScan) < 0.2 {
      return
    }
    lastFrameScan = now

    guard let imageBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else {
      return
    }
    let image = CIImage(cvPixelBuffer: imageBuffer)
    let payload = detector?
      .features(in: image)
      .compactMap { ($0 as? CIQRCodeFeature)?.messageString }
      .first
    if let payload = payload, !payload.isEmpty {
      DispatchQueue.main.async { [weak self] in
        self?.complete(payload)
      }
    }
  }

  @objc private func cancelScan() {
    complete(nil)
  }

  private func complete(_ payload: String?) {
    if completed {
      return
    }
    completed = true
    sessionQueue.async { [session] in
      if session.isRunning {
        session.stopRunning()
      }
    }
    dismiss(self)
    onResult(payload)
  }

  private func completeWithError(code: String, message: String) {
    if completed {
      return
    }
    completed = true
    sessionQueue.async { [session] in
      if session.isRunning {
        session.stopRunning()
      }
    }
    dismiss(self)
    onResult(FlutterError(code: code, message: message, details: nil))
  }
}
