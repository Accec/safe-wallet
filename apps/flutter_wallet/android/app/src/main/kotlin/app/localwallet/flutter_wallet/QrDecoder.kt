package app.localwallet.flutter_wallet

import android.graphics.Bitmap
import android.media.Image
import com.google.zxing.BarcodeFormat
import com.google.zxing.BinaryBitmap
import com.google.zxing.DecodeHintType
import com.google.zxing.MultiFormatReader
import com.google.zxing.NotFoundException
import com.google.zxing.PlanarYUVLuminanceSource
import com.google.zxing.RGBLuminanceSource
import com.google.zxing.common.HybridBinarizer

object QrDecoder {
    private val hints = mapOf(DecodeHintType.POSSIBLE_FORMATS to listOf(BarcodeFormat.QR_CODE))

    fun decodeBitmap(bitmap: Bitmap): String? {
        val pixels = IntArray(bitmap.width * bitmap.height)
        bitmap.getPixels(pixels, 0, bitmap.width, 0, 0, bitmap.width, bitmap.height)
        val source = RGBLuminanceSource(bitmap.width, bitmap.height, pixels)
        return decode(BinaryBitmap(HybridBinarizer(source)))
    }

    fun decodeCameraImage(image: Image): String? {
        val source = PlanarYUVLuminanceSource(
            copyLumaPlane(image),
            image.width,
            image.height,
            0,
            0,
            image.width,
            image.height,
            false,
        )
        return decode(BinaryBitmap(HybridBinarizer(source)))
    }

    private fun decode(binaryBitmap: BinaryBitmap): String? {
        val reader = MultiFormatReader()
        reader.setHints(hints)
        return try {
            reader.decodeWithState(binaryBitmap).text
        } catch (_: NotFoundException) {
            null
        } finally {
            reader.reset()
        }
    }

    private fun copyLumaPlane(image: Image): ByteArray {
        val plane = image.planes[0]
        val buffer = plane.buffer
        val rowStride = plane.rowStride
        val pixelStride = plane.pixelStride
        val bytes = ByteArray(image.width * image.height)
        var offset = 0

        for (y in 0 until image.height) {
            val rowStart = y * rowStride
            for (x in 0 until image.width) {
                bytes[offset++] = buffer.get(rowStart + x * pixelStride)
            }
        }

        return bytes
    }
}
