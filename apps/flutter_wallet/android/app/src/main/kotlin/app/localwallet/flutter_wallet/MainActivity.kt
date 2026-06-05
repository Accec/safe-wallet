package app.localwallet.flutter_wallet

import android.app.Activity
import android.content.Intent
import android.graphics.BitmapFactory
import android.net.Uri
import androidx.core.content.FileProvider
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel
import java.io.File

class MainActivity : FlutterActivity() {
    private var pendingImageResult: MethodChannel.Result? = null
    private var pendingScanResult: MethodChannel.Result? = null

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)
        MethodChannel(
            flutterEngine.dartExecutor.binaryMessenger,
            "app.localwallet/qr_scanner",
        ).setMethodCallHandler { call, result ->
            when (call.method) {
                "scanQr" -> scanQr(result)
                "pickQrImagePayload" -> pickQrImage(result)
                else -> result.notImplemented()
            }
        }
        MethodChannel(
            flutterEngine.dartExecutor.binaryMessenger,
            "app.localwallet/app_update",
        ).setMethodCallHandler { call, result ->
            when (call.method) {
                "installApk" -> installApk(call.argument<String>("path"), result)
                else -> result.notImplemented()
            }
        }
    }

    private fun scanQr(result: MethodChannel.Result) {
        if (pendingScanResult != null) {
            result.error("busy", "QR scanner already open.", null)
            return
        }

        pendingScanResult = result
        startActivityForResult(Intent(this, QrScannerActivity::class.java), REQUEST_QR_SCAN)
    }

    private fun pickQrImage(result: MethodChannel.Result) {
        if (pendingImageResult != null) {
            result.error("busy", "Image picker already open.", null)
            return
        }

        pendingImageResult = result
        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "image/*"
        }

        try {
            startActivityForResult(intent, REQUEST_QR_IMAGE)
        } catch (_: Exception) {
            pendingImageResult = null
            result.error("unavailable", "Image picker unavailable.", null)
        }
    }

    @Deprecated("Deprecated in Java")
    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (requestCode == REQUEST_QR_SCAN) {
            val result = pendingScanResult ?: return
            pendingScanResult = null
            if (resultCode == Activity.RESULT_OK) {
                result.success(data?.getStringExtra(QrScannerActivity.EXTRA_PAYLOAD))
            } else {
                result.success(null)
            }
            return
        }

        if (requestCode != REQUEST_QR_IMAGE) {
            return
        }

        val result = pendingImageResult ?: return
        pendingImageResult = null

        if (resultCode != Activity.RESULT_OK) {
            result.success(null)
            return
        }

        val uri = data?.data
        if (uri == null) {
            result.success(null)
            return
        }

        val payload = decodePickedImage(uri)
        if (payload == null) {
            result.error("no_qr", "No QR code found in image.", null)
        } else {
            result.success(payload)
        }
    }

    private fun decodePickedImage(uri: Uri): String? {
        val input = contentResolver.openInputStream(uri) ?: return null
        input.use { inputStream ->
            val bitmap = BitmapFactory.decodeStream(inputStream) ?: return null
            return QrDecoder.decodeBitmap(bitmap)
        }
    }

    private fun installApk(path: String?, result: MethodChannel.Result) {
        if (path.isNullOrBlank()) {
            result.error("invalid_path", "Downloaded update is missing.", null)
            return
        }

        val apkFile = File(path)
        if (!apkFile.exists()) {
            result.error("missing_file", "Downloaded update is missing.", null)
            return
        }

        val uri = FileProvider.getUriForFile(
            this,
            "${applicationContext.packageName}.update_file_provider",
            apkFile,
        )
        val intent = Intent(Intent.ACTION_VIEW).apply {
            setDataAndType(uri, "application/vnd.android.package-archive")
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
            addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
        }

        try {
            startActivity(intent)
            result.success(null)
        } catch (_: Exception) {
            result.error("installer_unavailable", "Update installer unavailable.", null)
        }
    }

    companion object {
        private const val REQUEST_QR_IMAGE = 7312
        private const val REQUEST_QR_SCAN = 7313
    }
}
