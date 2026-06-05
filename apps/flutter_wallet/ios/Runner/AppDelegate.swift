import AVFoundation
import CoreImage
import Flutter
import UIKit

@main
@objc class AppDelegate: FlutterAppDelegate, FlutterImplicitEngineDelegate, UIDocumentPickerDelegate {
  private var pendingImageResult: FlutterResult?
  private var pendingScanResult: FlutterResult?

  override func application(
    _ application: UIApplication,
    didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
  ) -> Bool {
    return super.application(application, didFinishLaunchingWithOptions: launchOptions)
  }

  func didInitializeImplicitFlutterEngine(_ engineBridge: FlutterImplicitEngineBridge) {
    GeneratedPluginRegistrant.register(with: engineBridge.pluginRegistry)
    let channel = FlutterMethodChannel(
      name: "app.localwallet/qr_scanner",
      binaryMessenger: engineBridge.applicationRegistrar.messenger())
    channel.setMethodCallHandler { [weak self] call, result in
      if call.method == "scanQr" {
        self?.scanQr(result)
      } else if call.method == "pickQrImagePayload" {
        self?.pickQrImage(result)
      } else {
        result(FlutterMethodNotImplemented)
      }
    }
  }

  private func scanQr(_ result: @escaping FlutterResult) {
    if pendingScanResult != nil {
      result(FlutterError(code: "busy", message: "QR scanner already open.", details: nil))
      return
    }

    guard let presenter = topViewController() else {
      result(FlutterError(code: "unavailable", message: "QR scanner unavailable.", details: nil))
      return
    }

    pendingScanResult = result
    let scanner = QrScannerViewController { [weak self] payload in
      self?.pendingScanResult?(payload)
      self?.pendingScanResult = nil
    }
    scanner.modalPresentationStyle = .fullScreen
    presenter.present(scanner, animated: true)
  }

  private func pickQrImage(_ result: @escaping FlutterResult) {
    if pendingImageResult != nil {
      result(FlutterError(code: "busy", message: "Image picker already open.", details: nil))
      return
    }

    guard let presenter = topViewController() else {
      result(FlutterError(code: "unavailable", message: "Image picker unavailable.", details: nil))
      return
    }

    pendingImageResult = result
    let picker = UIDocumentPickerViewController(documentTypes: ["public.image"], in: .import)
    picker.delegate = self
    picker.allowsMultipleSelection = false
    presenter.present(picker, animated: true)
  }

  func documentPicker(_ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]) {
    guard let result = pendingImageResult else {
      return
    }
    pendingImageResult = nil

    guard let url = urls.first else {
      result(nil)
      return
    }

    let canAccess = url.startAccessingSecurityScopedResource()
    defer {
      if canAccess {
        url.stopAccessingSecurityScopedResource()
      }
    }

    if let payload = decodeQrImage(url) {
      result(payload)
    } else {
      result(FlutterError(code: "no_qr", message: "No QR code found in image.", details: nil))
    }
  }

  func documentPickerWasCancelled(_ controller: UIDocumentPickerViewController) {
    pendingImageResult?(nil)
    pendingImageResult = nil
  }

  private func topViewController() -> UIViewController? {
    let window = UIApplication.shared.connectedScenes
      .compactMap { $0 as? UIWindowScene }
      .flatMap { $0.windows }
      .first { $0.isKeyWindow }
    var controller = window?.rootViewController
    while let presented = controller?.presentedViewController {
      controller = presented
    }
    return controller
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
}

final class QrScannerViewController: UIViewController, AVCaptureMetadataOutputObjectsDelegate {
  private let onResult: (String?) -> Void
  private let session = AVCaptureSession()
  private var previewLayer: AVCaptureVideoPreviewLayer?
  private var completed = false

  init(onResult: @escaping (String?) -> Void) {
    self.onResult = onResult
    super.init(nibName: nil, bundle: nil)
  }

  required init?(coder: NSCoder) {
    return nil
  }

  override func viewDidLoad() {
    super.viewDidLoad()
    view.backgroundColor = .black

    let cancel = UIButton(type: .system)
    cancel.setTitle("Cancel", for: .normal)
    cancel.tintColor = .white
    cancel.backgroundColor = UIColor.black.withAlphaComponent(0.5)
    cancel.layer.cornerRadius = 8
    cancel.translatesAutoresizingMaskIntoConstraints = false
    cancel.addTarget(self, action: #selector(cancelScan), for: .touchUpInside)
    view.addSubview(cancel)
    NSLayoutConstraint.activate([
      cancel.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 16),
      cancel.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -16),
      cancel.widthAnchor.constraint(equalToConstant: 92),
      cancel.heightAnchor.constraint(equalToConstant: 44),
    ])

    startAfterPermission()
  }

  override func viewDidLayoutSubviews() {
    super.viewDidLayoutSubviews()
    previewLayer?.frame = view.bounds
  }

  private func startAfterPermission() {
    switch AVCaptureDevice.authorizationStatus(for: .video) {
    case .authorized:
      startSession()
    case .notDetermined:
      AVCaptureDevice.requestAccess(for: .video) { [weak self] granted in
        DispatchQueue.main.async {
          granted ? self?.startSession() : self?.complete(nil)
        }
      }
    default:
      complete(nil)
    }
  }

  private func startSession() {
    guard
      let device = AVCaptureDevice.default(for: .video),
      let input = try? AVCaptureDeviceInput(device: device),
      session.canAddInput(input)
    else {
      complete(nil)
      return
    }
    session.addInput(input)

    let output = AVCaptureMetadataOutput()
    guard session.canAddOutput(output) else {
      complete(nil)
      return
    }
    session.addOutput(output)
    output.setMetadataObjectsDelegate(self, queue: DispatchQueue.main)
    output.metadataObjectTypes = [.qr]

    let layer = AVCaptureVideoPreviewLayer(session: session)
    layer.videoGravity = .resizeAspectFill
    layer.frame = view.bounds
    view.layer.insertSublayer(layer, at: 0)
    previewLayer = layer

    DispatchQueue.global(qos: .userInitiated).async { [session] in
      session.startRunning()
    }
  }

  func metadataOutput(
    _ output: AVCaptureMetadataOutput,
    didOutput metadataObjects: [AVMetadataObject],
    from connection: AVCaptureConnection
  ) {
    let payload = metadataObjects
      .compactMap { $0 as? AVMetadataMachineReadableCodeObject }
      .first { $0.type == .qr }?
      .stringValue
    if let payload = payload, !payload.isEmpty {
      complete(payload)
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
    session.stopRunning()
    dismiss(animated: true) { [onResult] in
      onResult(payload)
    }
  }
}
