package io.parity.signer.models

import android.annotation.SuppressLint
import android.util.Log
import androidx.camera.core.ImageProxy
import com.google.mlkit.vision.barcode.BarcodeScanner
import com.google.mlkit.vision.common.InputImage
import io.parity.signer.uniffi.Action
import io.parity.signer.uniffi.qrparserGetPacketsTotal
import io.parity.signer.uniffi.qrparserTryDecodeQrSequence

/**
 * Barcode detecting function.
 * This uses experimental features
 */
@SuppressLint("UnsafeOptInUsageError")
fun SignerDataModel.processFrame(
	barcodeScanner: BarcodeScanner,
	imageProxy: ImageProxy
) {
	if (imageProxy.image == null) return
	val inputImage = InputImage.fromMediaImage(
		imageProxy.image!!,
		imageProxy.imageInfo.rotationDegrees
	)

	barcodeScanner.process(inputImage)
		.addOnSuccessListener { barcodes ->
			barcodes.forEach {
				val payloadString = it?.rawBytes?.encodeHex()
				if (!(bucket.contains(payloadString) || payloadString.isNullOrEmpty())) {
					if (total.value == null) {
						try {
							val proposeTotal =
								qrparserGetPacketsTotal(payloadString, true).toInt()
							if (proposeTotal == 1) {
								try {
									payload = qrparserTryDecodeQrSequence(
										listOf(payloadString),
										true
									)
									resetScanValues()
									navigate(Action.TRANSACTION_FETCHED, payload)
								} catch (e: java.lang.Exception) {
									Log.e("Single frame decode failed", e.toString())
								}
							} else {
								bucket += payloadString
								_total.value = proposeTotal
							}
						} catch (e: java.lang.Exception) {
							Log.e("QR sequence length estimation", e.toString())
						}
					} else {
						bucket += payloadString
						if ((bucket.size + 1) >= (total.value ?: 0)) {
							try {
								payload = qrparserTryDecodeQrSequence(
									bucket.toList(),
									true
								)
								if (payload.isNotEmpty()) {
									resetScanValues()
									navigate(Action.TRANSACTION_FETCHED, payload)
								}
							} catch (e: java.lang.Exception) {
								Log.e("failed to parse sequence", e.toString())
							}
						}
						_captured.value = bucket.size
						_progress.value = (
							(
								captured.value ?: 0
								).toFloat() / (
								(
									total.value
										?: 1
									).toFloat()
								)
							)
						Log.d("captured", captured.value.toString())
					}
				}
			}
		}
		.addOnFailureListener {
			Log.e("Scan failed", it.message.toString())
		}
		.addOnCompleteListener {
			imageProxy.close()
		}
}

/**
 * Clears camera progress
 */
fun SignerDataModel.resetScanValues() {
	bucket = arrayOf()
	_captured.value = null
	_total.value = null
	_progress.value = 0.0f
}
