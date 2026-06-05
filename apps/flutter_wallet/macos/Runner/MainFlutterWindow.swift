import AVFoundation
import Cocoa
import CoreImage
import CoreVideo
import FlutterMacOS

class MainFlutterWindow: NSWindow {
  static let defaultFrame = NSRect(x: 0, y: 0, width: 1000, height: 720)
  private var didConfigureFlutter = false
  private var qrScannerChannel: FlutterMethodChannel?
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
