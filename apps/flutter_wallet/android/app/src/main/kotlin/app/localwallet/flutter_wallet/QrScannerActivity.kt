package app.localwallet.flutter_wallet

import android.Manifest
import android.annotation.SuppressLint
import android.app.Activity
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.graphics.Color
import android.graphics.ImageFormat
import android.graphics.SurfaceTexture
import android.hardware.camera2.CameraCaptureSession
import android.hardware.camera2.CameraCharacteristics
import android.hardware.camera2.CameraDevice
import android.hardware.camera2.CameraManager
import android.hardware.camera2.CaptureRequest
import android.media.ImageReader
import android.os.Bundle
import android.os.Handler
import android.os.HandlerThread
import android.util.Size
import android.view.Gravity
import android.view.Surface
import android.view.TextureView
import android.widget.FrameLayout
import android.widget.TextView

class QrScannerActivity : Activity() {
    private lateinit var textureView: TextureView
    private lateinit var cameraThread: HandlerThread
    private lateinit var cameraHandler: Handler
    private var cameraDevice: CameraDevice? = null
    private var cameraSession: CameraCaptureSession? = null
    private var imageReader: ImageReader? = null
    @Volatile private var decoding = false
    @Volatile private var completed = false

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        textureView = TextureView(this)
        val label = TextView(this).apply {
            text = "Align the payment QR code"
            setTextColor(Color.WHITE)
            setBackgroundColor(0x66000000)
            gravity = Gravity.CENTER
            setPadding(24, 16, 24, 16)
        }
        val layout = FrameLayout(this).apply {
            addView(textureView, FrameLayout.LayoutParams.MATCH_PARENT, FrameLayout.LayoutParams.MATCH_PARENT)
            addView(
                label,
                FrameLayout.LayoutParams(
                    FrameLayout.LayoutParams.MATCH_PARENT,
                    FrameLayout.LayoutParams.WRAP_CONTENT,
                    Gravity.BOTTOM,
                ),
            )
        }
        setContentView(layout)

        cameraThread = HandlerThread("qr-camera")
        cameraThread.start()
        cameraHandler = Handler(cameraThread.looper)

        if (checkSelfPermission(Manifest.permission.CAMERA) == PackageManager.PERMISSION_GRANTED) {
            startWhenReady()
        } else {
            requestPermissions(arrayOf(Manifest.permission.CAMERA), REQUEST_CAMERA)
        }
    }

    override fun onRequestPermissionsResult(
        requestCode: Int,
        permissions: Array<out String>,
        grantResults: IntArray,
    ) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if (requestCode == REQUEST_CAMERA && grantResults.firstOrNull() == PackageManager.PERMISSION_GRANTED) {
            startWhenReady()
        } else {
            finishWithoutPayload()
        }
    }

    private fun startWhenReady() {
        if (textureView.isAvailable) {
            openCamera()
        } else {
            textureView.surfaceTextureListener = object : TextureView.SurfaceTextureListener {
                override fun onSurfaceTextureAvailable(surface: SurfaceTexture, width: Int, height: Int) {
                    openCamera()
                }

                override fun onSurfaceTextureSizeChanged(surface: SurfaceTexture, width: Int, height: Int) = Unit
                override fun onSurfaceTextureDestroyed(surface: SurfaceTexture): Boolean = true
                override fun onSurfaceTextureUpdated(surface: SurfaceTexture) = Unit
            }
        }
    }

    @SuppressLint("MissingPermission")
    private fun openCamera() {
        val manager = getSystemService(Context.CAMERA_SERVICE) as CameraManager
        val cameraId = manager.cameraIdList.firstOrNull { id ->
            manager.getCameraCharacteristics(id)
                .get(CameraCharacteristics.LENS_FACING) == CameraCharacteristics.LENS_FACING_BACK
        } ?: manager.cameraIdList.firstOrNull()

        if (cameraId == null) {
            finishWithoutPayload()
            return
        }

        manager.openCamera(cameraId, object : CameraDevice.StateCallback() {
            override fun onOpened(camera: CameraDevice) {
                cameraDevice = camera
                createCameraSession(camera, manager.getCameraCharacteristics(cameraId))
            }

            override fun onDisconnected(camera: CameraDevice) {
                camera.close()
                finishWithoutPayload()
            }

            override fun onError(camera: CameraDevice, error: Int) {
                camera.close()
                finishWithoutPayload()
            }
        }, cameraHandler)
    }

    private fun createCameraSession(camera: CameraDevice, characteristics: CameraCharacteristics) {
        val map = characteristics.get(CameraCharacteristics.SCALER_STREAM_CONFIGURATION_MAP)
        val previewSize = map?.getOutputSizes(SurfaceTexture::class.java)?.let(::chooseSize) ?: Size(1280, 720)
        val imageSize = map?.getOutputSizes(ImageFormat.YUV_420_888)?.let(::chooseSize) ?: previewSize
        val texture = textureView.surfaceTexture ?: run {
            finishWithoutPayload()
            return
        }

        texture.setDefaultBufferSize(previewSize.width, previewSize.height)
        val previewSurface = Surface(texture)
        imageReader = ImageReader.newInstance(
            imageSize.width,
            imageSize.height,
            ImageFormat.YUV_420_888,
            2,
        ).apply {
            setOnImageAvailableListener({ reader ->
                val image = reader.acquireLatestImage() ?: return@setOnImageAvailableListener
                if (completed || decoding) {
                    image.close()
                    return@setOnImageAvailableListener
                }

                decoding = true
                try {
                    val payload = QrDecoder.decodeCameraImage(image)
                    if (!payload.isNullOrBlank()) {
                        finishWithPayload(payload)
                    }
                } finally {
                    decoding = false
                    image.close()
                }
            }, cameraHandler)
        }

        val captureRequest = camera.createCaptureRequest(CameraDevice.TEMPLATE_PREVIEW).apply {
            addTarget(previewSurface)
            addTarget(imageReader!!.surface)
            set(CaptureRequest.CONTROL_AF_MODE, CaptureRequest.CONTROL_AF_MODE_CONTINUOUS_PICTURE)
        }

        camera.createCaptureSession(
            listOf(previewSurface, imageReader!!.surface),
            object : CameraCaptureSession.StateCallback() {
                override fun onConfigured(session: CameraCaptureSession) {
                    cameraSession = session
                    session.setRepeatingRequest(captureRequest.build(), null, cameraHandler)
                }

                override fun onConfigureFailed(session: CameraCaptureSession) {
                    finishWithoutPayload()
                }
            },
            cameraHandler,
        )
    }

    private fun chooseSize(sizes: Array<Size>): Size {
        return sizes
            .filter { it.width <= 1280 && it.height <= 720 }
            .maxByOrNull { it.width * it.height }
            ?: sizes.maxByOrNull { it.width * it.height }
            ?: Size(1280, 720)
    }

    private fun finishWithPayload(payload: String) {
        if (completed) {
            return
        }
        completed = true
        runOnUiThread {
            setResult(RESULT_OK, Intent().putExtra(EXTRA_PAYLOAD, payload))
            finish()
        }
    }

    private fun finishWithoutPayload() {
        if (completed) {
            return
        }
        completed = true
        runOnUiThread {
            setResult(RESULT_CANCELED)
            finish()
        }
    }

    override fun onDestroy() {
        cameraSession?.close()
        cameraDevice?.close()
        imageReader?.close()
        cameraThread.quitSafely()
        super.onDestroy()
    }

    companion object {
        const val EXTRA_PAYLOAD = "payload"
        private const val REQUEST_CAMERA = 8241
    }
}
