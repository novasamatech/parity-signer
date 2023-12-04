package io.parity.signer.screens.scan.camera

import android.graphics.ImageFormat
import android.graphics.Rect
import android.media.Image
import android.util.Log
import androidx.camera.core.ImageProxy
import androidx.compose.ui.unit.Density
import com.google.mlkit.vision.common.InputImage
import com.google.mlkit.vision.common.InputImage.IMAGE_FORMAT_NV21


private const val TAG = "ImageCropper"

class ImageCropper(val dencity: Density) {

	var wasLogged = false //todo dmitry

	@androidx.annotation.OptIn(androidx.camera.core.ExperimentalGetImage::class)
	fun getCuttedImage(imageProxy: ImageProxy): InputImage {
		val mediaImage = imageProxy.image
		return if (mediaImage != null && mediaImage.format == ImageFormat.YUV_420_888) {
			if (!wasLogged) {
				Log.d(TAG, "proper type of camera, cropping for recognition enabled")
				wasLogged = true
			}
			//todo dmitry calculate crop rect
			croppedNV21(mediaImage, imageProxy.cropRect).let { byteArray ->
				InputImage.fromByteArray(
					byteArray,
					imageProxy.cropRect.width(),
					imageProxy.cropRect.height(),
					imageProxy.imageInfo.rotationDegrees,
					IMAGE_FORMAT_NV21,
				)
			}
		} else {
			if (!wasLogged) {
				Log.e(
					TAG,
					"wrong format, it is ${mediaImage?.format}, cropping disabled "
				)
				wasLogged = true
			}
			InputImage.fromMediaImage(
				imageProxy.image!!,
				imageProxy.imageInfo.rotationDegrees,
			)
		}
	}

	private fun croppedNV21(mediaImage: Image, cropRect: Rect): ByteArray {
		val yBuffer = mediaImage.planes[0].buffer // Y
		val vuBuffer = mediaImage.planes[2].buffer // VU

		val ySize = yBuffer.remaining()
		val vuSize = vuBuffer.remaining()

		val nv21 = ByteArray(ySize + vuSize)

		yBuffer.get(nv21, 0, ySize)
		vuBuffer.get(nv21, ySize, vuSize)

		return cropByteArray(nv21, mediaImage.width, cropRect)
	}

	private fun cropByteArray(
		array: ByteArray,
		imageWidth: Int,
		cropRect: Rect
	): ByteArray {
		val croppedArray = ByteArray(cropRect.width() * cropRect.height())
		var i = 0
		array.forEachIndexed { index, byte ->
			val x = index % imageWidth
			val y = index / imageWidth

			if (cropRect.left <= x && x < cropRect.right && cropRect.top <= y && y < cropRect.bottom) {
				croppedArray[i] = byte
				i++
			}
		}

		return croppedArray
	}
}
